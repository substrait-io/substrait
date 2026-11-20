# SPDX-License-Identifier: Apache-2.0
from antlr4.tree.Tree import TerminalNode

from tests.parser.antlr_parser.FuncTestCaseParser import FuncTestCaseParser
from tests.parser.antlr_parser.FuncTestCaseParserVisitor import (
    FuncTestCaseParserVisitor,
)
from tests.parser.nodes import (
    AggregateArgument,
    CaseGroup,
    FuncCallArg,
    TestFile,
    TestCase,
    CaseLiteral,
    SubstraitError,
    type_str_is_outer_nullable,
)


class TestCaseVisitor(FuncTestCaseParserVisitor):
    def __init__(self, file_path):
        self.file_path = file_path

    def visitDoc(self, ctx: FuncTestCaseParser.DocContext):
        version, include, dependencies = self.visitHeader(ctx.header())
        testcases = []
        for group in ctx.testGroup():
            _, group_tests = self.visit(group)
            for test_case in group_tests:
                test_case.base_uri = include
            testcases.extend(group_tests)

        return TestFile(self.file_path, version, include, dependencies, testcases)

    def visitHeader(self, ctx: FuncTestCaseParser.HeaderContext):
        version = self.visitVersion(ctx.version())
        include = self.visitInclude(ctx.include())
        dependencies = []
        for dependency_ctx in ctx.dependency():
            dependencies.extend(self.visitDependency(dependency_ctx))
        return version, include, dependencies

    def visitVersion(self, ctx: FuncTestCaseParser.VersionContext):
        return ctx.FormatVersion().getText()

    def visitInclude(self, ctx: FuncTestCaseParser.IncludeContext):
        # TODO handle multiple includes
        return ctx.ExtensionUrn().getText()

    def visitDependency(self, ctx: FuncTestCaseParser.DependencyContext):
        return [ctx.ExtensionUrn().getText()]

    def visitTestGroupDescription(
        self, ctx: FuncTestCaseParser.TestGroupDescriptionContext
    ):
        if ctx:
            group = ctx.DescriptionLine().getText().strip("#").strip()
            return CaseGroup(group, "")
        else:
            return CaseGroup("", "")

    def visitScalarFuncTestGroup(
        self, ctx: FuncTestCaseParser.ScalarFuncTestGroupContext
    ):
        group = self.visitTestGroupDescription(ctx.testGroupDescription())
        test_cases = []
        for test_case in ctx.testCase():
            testcase = self.visitTestCase(test_case)
            testcase.group = group
            test_cases.append(testcase)
        return group, test_cases

    def visitAggregateFuncTestGroup(
        self, ctx: FuncTestCaseParser.AggregateFuncTestGroupContext
    ):
        group = self.visitTestGroupDescription(ctx.testGroupDescription())
        test_cases = []
        for test_case in ctx.aggFuncTestCase():
            testcase = self.visitAggFuncTestCase(test_case)
            testcase.group = group
            test_cases.append(testcase)
        return group, test_cases

    def visitWindowFuncTestGroup(
        self, ctx: FuncTestCaseParser.WindowFuncTestGroupContext
    ):
        group = self.visitTestGroupDescription(ctx.testGroupDescription())
        test_cases = []
        for test_case in ctx.windowFuncTestCase():
            testcase = self.visitWindowFuncTestCase(test_case)
            testcase.group = group
            test_cases.append(testcase)
        return group, test_cases

    def visitTestCase(self, ctx: FuncTestCaseParser.TestCaseContext):
        # TODO Implement this method
        args = self.visitArguments(ctx.arguments())
        result = self.visitResult(ctx.result())
        options = {}
        if ctx.funcOptions() is not None:
            options = self.visitFuncOptions(ctx.funcOptions())
        return TestCase(
            func_name=ctx.identifier().getText(),
            base_uri="",
            group=None,
            options=options,
            rows=None,
            args=args,
            result=result,
            comment="",
        )

    def visitAggFuncTestCase(self, ctx: FuncTestCaseParser.AggFuncTestCaseContext):
        testcase = self.visit(ctx.aggFuncCall())
        testcase.result = self.visitResult(ctx.result())
        if ctx.funcOptions() is not None:
            testcase.options = self.visitFuncOptions(ctx.funcOptions())
        return testcase

    def visitWindowFuncTestCase(
        self, ctx: FuncTestCaseParser.WindowFuncTestCaseContext
    ):
        testcase = self.visitWindowFuncCall(ctx.windowFuncCall())
        testcase.result = self.visitWindowResult(ctx.windowResult())
        if isinstance(testcase.result, CaseLiteral) and len(
            testcase.result.value
        ) != len(testcase.rows):
            raise ParseError(
                f"Window result has {len(testcase.result.value)} value(s) but frame "
                f"'{ctx.windowFuncCall().frameRef.text}' has {len(testcase.rows)} row(s)"
            )
        if ctx.funcOptions() is not None:
            testcase.options = self.visitFuncOptions(ctx.funcOptions())
        return testcase

    def visitWindowResult(self, ctx: FuncTestCaseParser.WindowResultContext):
        if ctx.substraitError() is not None:
            return self.visitSubstraitError(ctx.substraitError())
        return self.visitDataColumn(ctx.dataColumn())

    def visitSingleArgAggregateFuncCall(
        self, ctx: FuncTestCaseParser.SingleArgAggregateFuncCallContext
    ):
        arg = self.visitDataColumn(ctx.dataColumn())
        return TestCase(
            func_name=ctx.identifier().getText(),
            base_uri="",
            group=None,
            options={},
            rows=None,
            args=[arg],
            result=SubstraitError("uninitialized"),
            comment="",
        )

    def visitCompactAggregateFuncCall(
        self, ctx: FuncTestCaseParser.CompactAggregateFuncCallContext
    ):
        rows = self.visitTableRows(ctx.tableRows())
        args = []
        if ctx.aggregateFuncArgs() is not None:
            args = self.visitAggregateFuncArgs(ctx.aggregateFuncArgs())
            for arg in ctx.aggregateFuncArgs().aggregateFuncArg():
                if arg.dataType() is not None:
                    column_index = int(arg.ColumnName().getText()[3:])
                    self._validate_table_column(
                        ctx.tableRows(), column_index, arg.dataType()
                    )
        return TestCase(
            func_name=ctx.identifier().getText(),
            base_uri="",
            group=None,
            options={},
            rows=rows,
            args=args,
            result=SubstraitError("uninitialized"),
            comment="",
        )

    def visitMultiArgAggregateFuncCall(
        self, ctx: FuncTestCaseParser.MultiArgAggregateFuncCallContext
    ):
        table_name, column_types, rows = self.visitTableData(ctx.tableData())
        args = []
        if ctx.qualifiedAggregateFuncArgs() is not None:
            args = self.visitQualifiedAggregateFuncArgs(
                ctx.qualifiedAggregateFuncArgs()
            )
        for arg in args:
            if arg.scalar_value is None:
                if arg.table_name != table_name:
                    raise ParseError(
                        "Table name in argument does not match the table name in the function call"
                    )
                self._bind_column_type(arg, column_types)

        return TestCase(
            func_name=ctx.identifier().getText(),
            base_uri="",
            group=None,
            options={},
            rows=rows,
            args=args,
            result=SubstraitError("uninitialized"),
            comment="",
        )

    def visitWindowFuncCall(self, ctx: FuncTestCaseParser.WindowFuncCallContext):
        table_name, column_types, rows = self.visitTableData(ctx.tableData())

        frame_ref = ctx.frameRef.text
        if frame_ref != table_name:
            raise ParseError(
                f"OVER references frame '{frame_ref}' but DEFINE declared '{table_name}'"
            )

        args = []
        if ctx.windowFuncArgs() is not None:
            args = self.visitWindowFuncArgs(ctx.windowFuncArgs())
        for arg in args:
            if arg.scalar_value is None:
                self._bind_column_type(arg, column_types)

        return TestCase(
            func_name=ctx.identifier().getText(),
            base_uri="",
            group=None,
            options={},
            rows=rows,
            args=args,
            result=SubstraitError("uninitialized"),
            comment="",
        )

    @staticmethod
    def _bind_column_type(arg, column_types):
        column_index = int(arg.column_name[3:])
        if column_index >= len(column_types):
            raise ParseError(f"Column '{arg.column_name}' is not declared by the frame")
        arg.column_type = column_types[column_index]

    def visitAggregateFuncArgs(self, ctx: FuncTestCaseParser.AggregateFuncArgsContext):
        args = []
        for arg in ctx.aggregateFuncArg():
            args.append(self.visitAggregateFuncArg(arg))
        return args

    def visitAggregateFuncArg(self, ctx: FuncTestCaseParser.AggregateFuncArgContext):
        if ctx.argument() is not None:
            return AggregateArgument("", "", "", self.visitArgument(ctx.argument()))
        data_type = self.visitDataType(ctx.dataType())
        return AggregateArgument(
            ctx.ColumnName().getText(), data_type, "", scalar_value=None
        )

    def visitQualifiedAggregateFuncArgs(
        self, ctx: FuncTestCaseParser.QualifiedAggregateFuncArgsContext
    ):
        args = []
        for arg in ctx.qualifiedAggregateFuncArg():
            args.append(self.visitQualifiedAggregateFuncArg(arg))
        return args

    def visitQualifiedAggregateFuncArg(
        self, ctx: FuncTestCaseParser.QualifiedAggregateFuncArgContext
    ):
        if ctx.argument() is not None:
            return AggregateArgument("", "", "", self.visitArgument(ctx.argument()))
        table_name = ctx.Identifier().getText()
        return AggregateArgument(
            ctx.ColumnName().getText(), "", table_name, scalar_value=None
        )

    def visitWindowFuncArgs(self, ctx: FuncTestCaseParser.WindowFuncArgsContext):
        return [self.visitWindowFuncArg(arg) for arg in ctx.windowFuncArg()]

    def visitWindowFuncArg(self, ctx: FuncTestCaseParser.WindowFuncArgContext):
        if ctx.argument() is not None:
            return AggregateArgument("", "", "", self.visitArgument(ctx.argument()))
        return AggregateArgument(ctx.ColumnName().getText(), "", "", scalar_value=None)

    def visitTableRows(self, ctx: FuncTestCaseParser.TableRowsContext):
        parsed_rows = [
            [self.visitLiteral(literal) for literal in row.literal()]
            for row in ctx.columnValues()
        ]
        rows = [[value for value, _ in row] for row in parsed_rows]
        for row in rows[1:]:
            if len(row) != len(rows[0]):
                raise ParseError(
                    f"All rows in a table must have the same number of values, "
                    f"but got rows with {len(rows[0])} and {len(row)} values"
                )
        for column in zip(*parsed_rows, strict=True):
            kinds = {kind for _, kind in column} - {"null"}
            if len(kinds) > 1:
                raise ParseError("All values in a column must have the same type")
        return rows

    def visitTableData(self, ctx: FuncTestCaseParser.TableDataContext):
        table_name = ctx.Identifier().getText()
        rows = self.visitTableRows(ctx.tableRows())
        column_types = []
        for dataType in ctx.dataType():
            column_types.append(self.visitDataType(dataType))
        for row in rows:
            if len(row) != len(column_types):
                raise ParseError(
                    f"Table '{table_name}' declares {len(column_types)} column(s) "
                    f"but row has {len(row)} value(s): {row}"
                )
        for column_index, data_type in enumerate(ctx.dataType()):
            self._validate_table_column(ctx.tableRows(), column_index, data_type)
        return table_name, column_types, rows

    def _validate_table_column(self, ctx, column_index, data_type):
        for row in ctx.columnValues():
            # Compact aggregate tests use an empty row for an empty input.
            if not row.literal():
                continue
            if column_index >= len(row.literal()):
                raise ParseError(
                    f"Column 'col{column_index}' is not declared by the table"
                )
            _, kind = self.visitLiteral(row.literal()[column_index])
            self._validate_literal_type(kind, data_type)

    def visitDataColumn(self, ctx: FuncTestCaseParser.DataColumnContext):
        column = self.visitColumnValues(ctx.columnValues(), ctx.dataType())
        column_type = ctx.dataType().getText()
        return CaseLiteral(
            value=column,
            type=column_type,
            nullable=type_str_is_outer_nullable(column_type),
        )

    def visitColumnValues(
        self, ctx: FuncTestCaseParser.ColumnValuesContext, data_type=None
    ):
        values = []
        type_str = ""
        for literal in ctx.literal():
            value, curr_type = self.visitLiteral(literal)
            if curr_type != "null":
                if type_str == "":
                    type_str = curr_type
                elif type_str != curr_type:
                    raise ParseError("All values in a column must have the same type")
            values.append(value)
        if data_type is not None and type_str:
            self._validate_literal_type(type_str, data_type)
        return values

    @staticmethod
    def _concrete_type(ctx):
        while isinstance(
            ctx,
            (
                FuncTestCaseParser.DataTypeContext,
                FuncTestCaseParser.ScalarTypeContext,
                FuncTestCaseParser.ParameterizedTypeContext,
            ),
        ):
            ctx = ctx.getChild(0)
        return ctx

    def _validate_literal_type(self, kind, data_type):
        if kind == "null":
            return
        allowed_types = {
            "number": (
                FuncTestCaseParser.IntTypeContext,
                FuncTestCaseParser.FloatTypeContext,
                FuncTestCaseParser.DecimalTypeContext,
            ),
            "str": (
                FuncTestCaseParser.StringTypeContext,
                FuncTestCaseParser.BinaryTypeContext,
                FuncTestCaseParser.FixedCharTypeContext,
                FuncTestCaseParser.VarCharTypeContext,
                FuncTestCaseParser.FixedBinaryTypeContext,
            ),
            "bool": (FuncTestCaseParser.BooleanTypeContext,),
            "uuid": (FuncTestCaseParser.UuidTypeContext,),
            "date": (FuncTestCaseParser.DateTypeContext,),
            "pt": (FuncTestCaseParser.PrecisionTimeTypeContext,),
            "pts": (FuncTestCaseParser.PrecisionTimestampTypeContext,),
            "ptstz": (FuncTestCaseParser.PrecisionTimestampTZTypeContext,),
            "iyear": (FuncTestCaseParser.IntervalYearTypeContext,),
            "iday": (FuncTestCaseParser.IntervalDayTypeContext,),
            "icompound": (FuncTestCaseParser.IntervalCompoundTypeContext,),
            "list": (FuncTestCaseParser.ListTypeContext,),
            "struct": (
                FuncTestCaseParser.StructTypeContext,
                FuncTestCaseParser.UserDefinedTypeContext,
            ),
            "map": (FuncTestCaseParser.MapTypeContext,),
        }
        if not isinstance(self._concrete_type(data_type), allowed_types[kind]):
            raise ParseError(
                f"Literal kind '{kind}' does not match declared type '{data_type.getText()}'"
            )

    def visitLiteral(self, ctx: FuncTestCaseParser.LiteralContext):
        if ctx.numericLiteral() is not None:
            return self.visitNumericLiteral(ctx.numericLiteral()), "number"
        if ctx.StringLiteral() is not None:
            return ctx.getText(), "str"
        if ctx.UuidLiteral() is not None:
            return ctx.getText(), "uuid"
        if ctx.BooleanLiteral() is not None:
            return ctx.getText(), "bool"
        if ctx.DateLiteral() is not None:
            return ctx.getText(), "date"
        if ctx.TimeLiteral() is not None:
            return ctx.getText(), "pt"
        if ctx.TimestampLiteral() is not None:
            return ctx.getText(), "pts"
        if ctx.TimestampTzLiteral() is not None:
            return ctx.getText(), "ptstz"
        if ctx.IntervalDayLiteral() is not None:
            return ctx.getText(), "iday"
        if ctx.IntervalYearLiteral() is not None:
            return ctx.getText(), "iyear"
        if ctx.IntervalCompoundLiteral() is not None:
            return ctx.getText(), "icompound"
        if ctx.NullLiteral() is not None:
            return ctx.getText(), "null"

    def visitFuncOptions(self, ctx: FuncTestCaseParser.FuncOptionsContext):
        options = {}
        for option in ctx.funcOption():
            key, value = self.visitFuncOption(option)
            options[key] = value
        return options

    def visitFuncOption(self, ctx: FuncTestCaseParser.FuncOptionContext):
        key = ctx.optionName().getText()
        value = ctx.optionValue().getText()
        return key, value

    def visitArguments(self, ctx: FuncTestCaseParser.ArgumentsContext):
        arguments = []
        for arg in ctx.argument():
            arguments.append(self.visitArgument(arg))
        return arguments

    def visitArgument(self, ctx: FuncTestCaseParser.ArgumentContext):
        if ctx.intArg() is not None:
            return self.visitIntArg(ctx.intArg())
        if ctx.floatArg() is not None:
            return self.visitFloatArg(ctx.floatArg())
        if ctx.booleanArg() is not None:
            return self.visitBooleanArg(ctx.booleanArg())
        if ctx.stringArg() is not None:
            return self.visitStringArg(ctx.stringArg())
        if ctx.decimalArg() is not None:
            return self.visitDecimalArg(ctx.decimalArg())
        if ctx.dateArg() is not None:
            return self.visitDateArg(ctx.dateArg())
        if ctx.uuidArg() is not None:
            return self.visitUuidArg(ctx.uuidArg())
        if ctx.intervalDayArg() is not None:
            return self.visitIntervalDayArg(ctx.intervalDayArg())
        if ctx.intervalYearArg() is not None:
            return self.visitIntervalYearArg(ctx.intervalYearArg())
        if ctx.fixedCharArg() is not None:
            return self.visitFixedCharArg(ctx.fixedCharArg())
        if ctx.varCharArg() is not None:
            return self.visitVarCharArg(ctx.varCharArg())
        if ctx.fixedBinaryArg() is not None:
            return self.visitFixedBinaryArg(ctx.fixedBinaryArg())
        if ctx.precisionTimeArg() is not None:
            return self.visitPrecisionTimeArg(ctx.precisionTimeArg())
        if ctx.precisionTimestampArg() is not None:
            return self.visitPrecisionTimestampArg(ctx.precisionTimestampArg())
        if ctx.precisionTimestampTZArg() is not None:
            return self.visitPrecisionTimestampTZArg(ctx.precisionTimestampTZArg())
        if ctx.nullArg() is not None:
            return self.visitNullArg(ctx.nullArg())
        if ctx.listArg() is not None:
            return self.visitListArg(ctx.listArg())
        if ctx.structArg() is not None:
            return self.visitStructArg(ctx.structArg())
        if ctx.mapArg() is not None:
            return self.visitMapArg(ctx.mapArg())
        if ctx.userDefinedArg() is not None:
            return self.visitUserDefinedArg(ctx.userDefinedArg())
        if ctx.lambdaArg() is not None:
            return self.visitLambdaArg(ctx.lambdaArg())
        if ctx.funcCallArg() is not None:
            return self.visitFuncCallArg(ctx.funcCallArg())
        if ctx.enumArg() is not None:
            return self.visitEnumArg(ctx.enumArg())

        return CaseLiteral(value="unknown_value", type="unknown_type")

    def visitNumericLiteral(self, ctx: FuncTestCaseParser.NumericLiteralContext):
        if ctx.IntegerLiteral() is not None:
            return ctx.IntegerLiteral().getText()
        if ctx.DecimalLiteral() is not None:
            return ctx.DecimalLiteral().getText()
        return self.visitFloatLiteral(ctx.floatLiteral())

    def visitFloatLiteral(self, ctx: FuncTestCaseParser.FloatLiteralContext):
        if ctx.FloatLiteral() is not None:
            return ctx.FloatLiteral().getText()
        return ctx.NaN().getText()

    def visitNullArg(self, ctx: FuncTestCaseParser.NullArgContext):
        datatype = self._normalized_type_text(ctx.dataType())
        if not type_str_is_outer_nullable(datatype):
            raise ParseError(
                f"Null literal must have a nullable type (add '?'), "
                f"got: null::{datatype}"
            )
        return CaseLiteral(value=None, type=datatype, nullable=True)

    @staticmethod
    def _normalized_type_text(ctx):
        if isinstance(ctx, TerminalNode):
            text = ctx.getText()
            return (
                text
                if ctx.getSymbol().type == FuncTestCaseParser.Identifier
                else text.lower()
            )
        return "".join(
            TestCaseVisitor._normalized_type_text(child) for child in ctx.getChildren()
        )

    def visitIntArg(self, ctx: FuncTestCaseParser.IntArgContext):
        type_str = ctx.intType().getText().lower()
        return CaseLiteral(
            value=ctx.IntegerLiteral().getText(),
            type=type_str,
            nullable=ctx.intType().isnull is not None,
        )

    def visitFloatArg(self, ctx: FuncTestCaseParser.FloatArgContext):
        type_str = ctx.floatType().getText().lower()
        return CaseLiteral(
            value=self.visitNumericLiteral(ctx.numericLiteral()),
            type=type_str,
            nullable=ctx.floatType().isnull is not None,
        )

    def visitBooleanArg(self, ctx: FuncTestCaseParser.BooleanArgContext):
        type_str = ctx.booleanType().getText().lower()
        return CaseLiteral(
            value=ctx.BooleanLiteral().getText(),
            type=type_str,
            nullable=ctx.booleanType().isnull is not None,
        )

    def visitStringArg(self, ctx: FuncTestCaseParser.StringArgContext):
        type_str = ctx.stringType().getText().lower()
        return CaseLiteral(
            value=ctx.StringLiteral().getText(),
            type=type_str,
            nullable=ctx.stringType().isnull is not None,
        )

    def visitDecimalArg(self, ctx: FuncTestCaseParser.DecimalArgContext):
        return CaseLiteral(
            value=self.visitNumericLiteral(ctx.numericLiteral()),
            type=ctx.decimalType().getText().lower(),
            nullable=ctx.decimalType().isnull is not None,
        )

    def visitDateArg(self, ctx: FuncTestCaseParser.DateArgContext):
        type_str = ctx.dateType().getText().lower()
        return CaseLiteral(
            value=ctx.DateLiteral().getText(),
            type=type_str,
            nullable=ctx.dateType().isnull is not None,
        )

    def visitUuidArg(self, ctx: FuncTestCaseParser.UuidArgContext):
        return CaseLiteral(
            value=ctx.UuidLiteral().getText(),
            type=ctx.uuidType().getText().lower(),
            nullable=ctx.uuidType().isnull is not None,
        )

    def visitIntervalDayArg(self, ctx: FuncTestCaseParser.IntervalDayArgContext):
        type_str = ctx.intervalDayType().getText().lower()
        return CaseLiteral(
            value=ctx.IntervalDayLiteral().getText(),
            type=type_str,
            nullable=ctx.intervalDayType().isnull is not None,
        )

    def visitIntervalYearArg(self, ctx: FuncTestCaseParser.IntervalYearArgContext):
        type_str = ctx.intervalYearType().getText().lower()
        return CaseLiteral(
            value=ctx.IntervalYearLiteral().getText(),
            type=type_str,
            nullable=ctx.intervalYearType().isnull is not None,
        )

    def visitFixedCharArg(self, ctx: FuncTestCaseParser.FixedCharArgContext):
        type_str = ctx.fixedCharType().getText().lower()
        return CaseLiteral(
            value=ctx.StringLiteral().getText(),
            type=type_str,
            nullable=ctx.fixedCharType().isnull is not None,
        )

    def visitVarCharArg(self, ctx: FuncTestCaseParser.VarCharArgContext):
        type_str = ctx.varCharType().getText().lower()
        return CaseLiteral(
            value=ctx.StringLiteral().getText(),
            type=type_str,
            nullable=ctx.varCharType().isnull is not None,
        )

    def visitFixedBinaryArg(self, ctx: FuncTestCaseParser.FixedBinaryArgContext):
        type_str = ctx.fixedBinaryType().getText().lower()
        return CaseLiteral(
            value=ctx.StringLiteral().getText(),
            type=type_str,
            nullable=ctx.fixedBinaryType().isnull is not None,
        )

    def visitPrecisionTimeArg(self, ctx: FuncTestCaseParser.PrecisionTimeArgContext):
        type_str = ctx.precisionTimeType().getText().lower()
        return CaseLiteral(
            value=ctx.TimeLiteral().getText(),
            type=type_str,
            nullable=ctx.precisionTimeType().isnull is not None,
        )

    def visitPrecisionTimestampArg(
        self, ctx: FuncTestCaseParser.PrecisionTimestampArgContext
    ):
        type_str = ctx.precisionTimestampType().getText().lower()
        return CaseLiteral(
            value=ctx.TimestampLiteral().getText(),
            type=type_str,
            nullable=ctx.precisionTimestampType().isnull is not None,
        )

    def visitPrecisionTimestampTZArg(
        self, ctx: FuncTestCaseParser.PrecisionTimestampTZArgContext
    ):
        type_str = ctx.precisionTimestampTZType().getText().lower()
        return CaseLiteral(
            value=ctx.TimestampTzLiteral().getText(),
            type=type_str,
            nullable=ctx.precisionTimestampTZType().isnull is not None,
        )

    def visitListArg(self, ctx: FuncTestCaseParser.ListArgContext):
        return CaseLiteral(
            value=self.visitLiteralList(ctx.literalList(), ctx.listType().elemType),
            type=ctx.listType().getText(),
            nullable=ctx.listType().isnull is not None,
        )

    def visitStructArg(self, ctx: FuncTestCaseParser.StructArgContext):
        return CaseLiteral(
            value=self.visitLiteralStruct(
                ctx.literalStruct(), ctx.structType().dataType()
            ),
            type=ctx.structType().getText(),
            nullable=ctx.structType().isnull is not None,
        )

    def visitMapArg(self, ctx: FuncTestCaseParser.MapArgContext):
        return CaseLiteral(
            value=self.visitLiteralMap(
                ctx.literalMap(), ctx.mapType().keyType, ctx.mapType().valueType
            ),
            type=ctx.mapType().getText(),
            nullable=ctx.mapType().isnull is not None,
        )

    def visitUserDefinedArg(self, ctx: FuncTestCaseParser.UserDefinedArgContext):
        return CaseLiteral(
            value=self.visitLiteralStruct(ctx.literalStruct()),
            type=ctx.userDefinedType().getText(),
            nullable=ctx.userDefinedType().isnull is not None,
        )

    def visitLiteralList(
        self, ctx: FuncTestCaseParser.LiteralListContext, element_type=None
    ):
        return [
            self.visitCompoundLiteral(element, element_type)
            for element in ctx.compoundLiteral()
        ]

    def visitLiteralStruct(
        self, ctx: FuncTestCaseParser.LiteralStructContext, field_types=None
    ):
        elements = ctx.compoundLiteral()
        if field_types is not None and len(elements) != len(field_types):
            raise ParseError(
                f"Struct literal has {len(elements)} field(s) but type declares {len(field_types)}"
            )
        return [
            self.visitCompoundLiteral(
                element, field_types[index] if field_types is not None else None
            )
            for index, element in enumerate(elements)
        ]

    def visitLiteralMap(
        self, ctx: FuncTestCaseParser.LiteralMapContext, key_type=None, value_type=None
    ):
        return [
            {
                "key": self.visitCompoundLiteral(entry.key, key_type),
                "value": self.visitCompoundLiteral(entry.value, value_type),
            }
            for entry in ctx.mapEntry()
        ]

    def visitCompoundLiteral(
        self, ctx: FuncTestCaseParser.CompoundLiteralContext, data_type=None
    ):
        if ctx.literal() is not None:
            value, kind = self.visitLiteral(ctx.literal())
            if data_type is not None:
                self._validate_literal_type(kind, data_type)
            return value
        if ctx.literalList() is not None:
            element_type = None
            if data_type is not None:
                self._validate_literal_type("list", data_type)
                element_type = self._concrete_type(data_type).elemType
            return self.visitLiteralList(ctx.literalList(), element_type)
        if ctx.literalStruct() is not None:
            field_types = None
            if data_type is not None:
                self._validate_literal_type("struct", data_type)
                concrete_type = self._concrete_type(data_type)
                if isinstance(concrete_type, FuncTestCaseParser.StructTypeContext):
                    field_types = concrete_type.dataType()
            return self.visitLiteralStruct(ctx.literalStruct(), field_types)
        if ctx.literalMap() is not None:
            key_type = value_type = None
            if data_type is not None:
                self._validate_literal_type("map", data_type)
                concrete_type = self._concrete_type(data_type)
                key_type, value_type = concrete_type.keyType, concrete_type.valueType
            return self.visitLiteralMap(ctx.literalMap(), key_type, value_type)
        raise ParseError("Unknown compound literal")

    def visitLambdaArg(self, ctx: FuncTestCaseParser.LambdaArgContext):
        lambda_type = ctx.funcType().getText()
        return CaseLiteral(
            value="lambda",
            type=lambda_type,
            nullable=ctx.funcType().isnull is not None,
        )

    def visitFuncCallArg(self, ctx: FuncTestCaseParser.FuncCallArgContext):
        return FuncCallArg(
            func_name=ctx.identifier().getText(),
            args=self.visitArguments(ctx.arguments()),
        )

    def visitEnumArg(self, ctx: FuncTestCaseParser.EnumArgContext):
        enum_value = ctx.Identifier().getText()
        return CaseLiteral(value=enum_value, type="enum")

    def visitDataType(self, ctx: FuncTestCaseParser.DataTypeContext):
        return ctx.getText()

    def visitResult(self, ctx: FuncTestCaseParser.ResultContext):
        if ctx.argument() is not None:
            return self.visitArgument(ctx.argument())
        return self.visitSubstraitError(ctx.substraitError())

    def visitSubstraitError(self, ctx: FuncTestCaseParser.SubstraitErrorContext):
        if ctx.ErrorResult() is not None:
            return SubstraitError("error")
        if ctx.UndefineResult() is not None:
            return SubstraitError("undefined")
        return SubstraitError("unknown_error")


class ParseError(Exception):
    def __init__(self, message="Parsing error occurred"):
        self.message = message
        super().__init__(self.message)
