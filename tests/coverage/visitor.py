# SPDX-License-Identifier: Apache-2.0
from tests.coverage.antlr_parser.FuncTestCaseParser import FuncTestCaseParser
from tests.coverage.antlr_parser.FuncTestCaseParserVisitor import (
    FuncTestCaseParserVisitor,
)
from tests.coverage.nodes import (
    CaseGroup,
    TestFile,
    TestCase,
    CaseLiteral,
    SubstraitError,
)


class TestCaseVisitor(FuncTestCaseParserVisitor):
    def __init__(self, file_path):
        self.file_path = file_path

    def visitDoc(self, ctx: FuncTestCaseParser.DocContext):
        version, include = self.visitHeader(ctx.header())
        testcases = []
        for group in ctx.testGroup():
            _, group_tests = self.visitTestGroup(group)
            for test_case in group_tests:
                test_case.base_uri = include
            testcases.extend(group_tests)

        return TestFile(self.file_path, version, include, testcases)

    def visitHeader(self, ctx: FuncTestCaseParser.HeaderContext):
        version = self.visitVersion(ctx.version())
        include = self.visitInclude(ctx.include())
        return version, include

    def visitVersion(self, ctx: FuncTestCaseParser.VersionContext):
        return ctx.FORMAT_VERSION().getText()

    def visitInclude(self, ctx: FuncTestCaseParser.IncludeContext):
        # TODO handle multiple includes
        return ctx.STRING_LITERAL(0).getText().strip("'")

    def visitTestGroupDescription(
        self, ctx: FuncTestCaseParser.TestGroupDescriptionContext
    ):
        group = ctx.DESCRIPTION_LINE().getText().strip("#").strip()
        return CaseGroup(group, "")

    def visitTestGroup(self, ctx: FuncTestCaseParser.TestGroupContext):
        group = self.visitTestGroupDescription(ctx.testGroupDescription())
        test_cases = []
        for test_case in ctx.testCase():
            testcase = self.visitTestCase(test_case)
            testcase.group = group
            test_cases.append(testcase)
        return group, test_cases

    def visitTestCase(self, ctx: FuncTestCaseParser.TestCaseContext):
        # TODO Implement this method
        args = self.visitArguments(ctx.arguments())
        result = self.visitResult(ctx.result())
        options = dict()
        if ctx.func_options() is not None:
            options = self.visitFunc_options(ctx.func_options())
        return TestCase(
            func_name=ctx.IDENTIFIER().getText(),
            base_uri="",
            group=None,
            options=options,
            args=args,
            result=result,
            comment="",
        )

    def visitFunc_options(self, ctx: FuncTestCaseParser.Func_optionsContext):
        options = {}
        for option in ctx.func_option():
            key, value = self.visitFunc_option(option)
            options[key] = value
        return options

    def visitFunc_option(self, ctx: FuncTestCaseParser.Func_optionContext):
        key = ctx.option_name().getText()
        value = ctx.option_value().getText()
        return key, value

    def visitArguments(self, ctx: FuncTestCaseParser.ArgumentsContext):
        arguments = []
        for arg in ctx.argument():
            arguments.append(self.visitArgument(arg))
        return arguments

    def visitArgument(self, ctx: FuncTestCaseParser.ArgumentContext):
        if ctx.i8Arg() is not None:
            return self.visitI8Arg(ctx.i8Arg())
        if ctx.i16Arg() is not None:
            return self.visitI16Arg(ctx.i16Arg())
        if ctx.i32Arg() is not None:
            return self.visitI32Arg(ctx.i32Arg())
        if ctx.i64Arg() is not None:
            return self.visitI64Arg(ctx.i64Arg())
        if ctx.fp32Arg() is not None:
            return self.visitFp32Arg(ctx.fp32Arg())
        if ctx.fp64Arg() is not None:
            return self.visitFp64Arg(ctx.fp64Arg())
        if ctx.booleanArg() is not None:
            return self.visitBooleanArg(ctx.booleanArg())
        if ctx.stringArg() is not None:
            return self.visitStringArg(ctx.stringArg())
        if ctx.decimalArg() is not None:
            return self.visitDecimalArg(ctx.decimalArg())
        if ctx.dateArg() is not None:
            return self.visitDateArg(ctx.dateArg())
        if ctx.timeArg() is not None:
            return self.visitTimeArg(ctx.timeArg())
        if ctx.timestampArg() is not None:
            return self.visitTimestampArg(ctx.timestampArg())
        if ctx.timestampTzArg() is not None:
            return self.visitTimestampTzArg(ctx.timestampTzArg())
        if ctx.intervalDayArg() is not None:
            return self.visitIntervalDayArg(ctx.intervalDayArg())
        if ctx.intervalYearArg() is not None:
            return self.visitIntervalYearArg(ctx.intervalYearArg())
        if ctx.nullArg() is not None:
            return self.visitNullArg(ctx.nullArg())

        return CaseLiteral(value="unknown_value", type="unknown_type")

    def visitNumericLiteral(self, ctx: FuncTestCaseParser.NumericLiteralContext):
        if ctx.INTEGER_LITERAL() is not None:
            return ctx.INTEGER_LITERAL().getText()
        if ctx.DECIMAL_LITERAL() is not None:
            return ctx.DECIMAL_LITERAL().getText()
        return ctx.FLOAT_LITERAL

    def visitNullArg(self, ctx: FuncTestCaseParser.NullArgContext):
        datatype = ctx.datatype().getText()
        return CaseLiteral(value=None, type=datatype)

    def visitI8Arg(self, ctx: FuncTestCaseParser.I8ArgContext):
        return CaseLiteral(value=ctx.INTEGER_LITERAL().getText(), type="i8")

    def visitI16Arg(self, ctx: FuncTestCaseParser.I16ArgContext):
        return CaseLiteral(value=ctx.INTEGER_LITERAL().getText(), type="i16")

    def visitI32Arg(self, ctx: FuncTestCaseParser.I32ArgContext):
        return CaseLiteral(value=ctx.INTEGER_LITERAL().getText(), type="i32")

    def visitI64Arg(self, ctx: FuncTestCaseParser.I64ArgContext):
        return CaseLiteral(value=ctx.INTEGER_LITERAL().getText(), type="i64")

    def visitFp32Arg(self, ctx: FuncTestCaseParser.Fp32ArgContext):
        # TODO add checks on number of decimal places
        return CaseLiteral(
            value=self.visitNumericLiteral(ctx.numericLiteral()),
            type=ctx.FP32().getText().lower(),
        )

    def visitFp64Arg(self, ctx: FuncTestCaseParser.Fp64ArgContext):
        return CaseLiteral(
            value=self.visitNumericLiteral(ctx.numericLiteral()),
            type=ctx.FP64().getText().lower(),
        )

    def visitBooleanArg(self, ctx: FuncTestCaseParser.BooleanArgContext):
        return CaseLiteral(value=ctx.BOOLEAN_LITERAL().getText(), type="bool")

    def visitStringArg(self, ctx: FuncTestCaseParser.StringArgContext):
        return CaseLiteral(value=ctx.STRING_LITERAL().getText(), type="str")

    def visitDecimalArg(self, ctx: FuncTestCaseParser.DecimalArgContext):
        return CaseLiteral(
            value=self.visitNumericLiteral(ctx.numericLiteral()),
            type=ctx.decimalType().getText().lower(),
        )

    def visitDateArg(self, ctx: FuncTestCaseParser.DateArgContext):
        return CaseLiteral(value=ctx.DATE_LITERAL().getText().strip("'"), type="date")

    def visitTimeArg(self, ctx: FuncTestCaseParser.TimeArgContext):
        return CaseLiteral(value=ctx.TIME_LITERAL().getText().strip("'"), type="time")

    def visitTimestampArg(self, ctx: FuncTestCaseParser.TimestampArgContext):
        return CaseLiteral(
            value=ctx.TIMESTAMP_LITERAL().getText().strip("'"), type="ts"
        )

    def visitTimestampTzArg(self, ctx: FuncTestCaseParser.TimestampTzArgContext):
        return CaseLiteral(
            value=ctx.TIMESTAMP_TZ_LITERAL().getText().strip("'"), type="tstz"
        )

    def visitIntervalDayArg(self, ctx: FuncTestCaseParser.IntervalDayArgContext):
        return CaseLiteral(
            value=ctx.INTERVAL_DAY_LITERAL().getText().strip("'"), type="iday"
        )

    def visitIntervalYearArg(self, ctx: FuncTestCaseParser.IntervalYearArgContext):
        return CaseLiteral(
            value=ctx.INTERVAL_YEAR_LITERAL().getText().strip("'"), type="iyear"
        )

    def visitResult(self, ctx: FuncTestCaseParser.ResultContext):
        if ctx.argument() is not None:
            return self.visitArgument(ctx.argument())
        return self.visitSubstraitError(ctx.substraitError())

    def visitSubstraitError(self, ctx: FuncTestCaseParser.SubstraitErrorContext):
        if ctx.ERROR_RESULT() is not None:
            return SubstraitError("error")
        if ctx.UNDEFINED_RESULT() is not None:
            return SubstraitError("undefined")
        return SubstraitError("unknown_error")
