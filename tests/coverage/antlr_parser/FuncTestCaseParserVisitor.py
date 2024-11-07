# SPDX-License-Identifier: Apache-2.0
# Generated from FuncTestCaseParser.g4 by ANTLR 4.13.2
from antlr4 import *

if "." in __name__:
    from .FuncTestCaseParser import FuncTestCaseParser
else:
    from FuncTestCaseParser import FuncTestCaseParser

# This class defines a complete generic visitor for a parse tree produced by FuncTestCaseParser.


class FuncTestCaseParserVisitor(ParseTreeVisitor):
    # Visit a parse tree produced by FuncTestCaseParser#doc.
    def visitDoc(self, ctx: FuncTestCaseParser.DocContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#header.
    def visitHeader(self, ctx: FuncTestCaseParser.HeaderContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#version.
    def visitVersion(self, ctx: FuncTestCaseParser.VersionContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#include.
    def visitInclude(self, ctx: FuncTestCaseParser.IncludeContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#testGroupDescription.
    def visitTestGroupDescription(
        self, ctx: FuncTestCaseParser.TestGroupDescriptionContext
    ):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#testCase.
    def visitTestCase(self, ctx: FuncTestCaseParser.TestCaseContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#testGroup.
    def visitTestGroup(self, ctx: FuncTestCaseParser.TestGroupContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#arguments.
    def visitArguments(self, ctx: FuncTestCaseParser.ArgumentsContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#result.
    def visitResult(self, ctx: FuncTestCaseParser.ResultContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#argument.
    def visitArgument(self, ctx: FuncTestCaseParser.ArgumentContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#numericLiteral.
    def visitNumericLiteral(self, ctx: FuncTestCaseParser.NumericLiteralContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#floatLiteral.
    def visitFloatLiteral(self, ctx: FuncTestCaseParser.FloatLiteralContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#nullArg.
    def visitNullArg(self, ctx: FuncTestCaseParser.NullArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intArg.
    def visitIntArg(self, ctx: FuncTestCaseParser.IntArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#floatArg.
    def visitFloatArg(self, ctx: FuncTestCaseParser.FloatArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#decimalArg.
    def visitDecimalArg(self, ctx: FuncTestCaseParser.DecimalArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#booleanArg.
    def visitBooleanArg(self, ctx: FuncTestCaseParser.BooleanArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#stringArg.
    def visitStringArg(self, ctx: FuncTestCaseParser.StringArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#dateArg.
    def visitDateArg(self, ctx: FuncTestCaseParser.DateArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timeArg.
    def visitTimeArg(self, ctx: FuncTestCaseParser.TimeArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timestampArg.
    def visitTimestampArg(self, ctx: FuncTestCaseParser.TimestampArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timestampTzArg.
    def visitTimestampTzArg(self, ctx: FuncTestCaseParser.TimestampTzArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalYearArg.
    def visitIntervalYearArg(self, ctx: FuncTestCaseParser.IntervalYearArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalDayArg.
    def visitIntervalDayArg(self, ctx: FuncTestCaseParser.IntervalDayArgContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalYearLiteral.
    def visitIntervalYearLiteral(
        self, ctx: FuncTestCaseParser.IntervalYearLiteralContext
    ):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalDayLiteral.
    def visitIntervalDayLiteral(
        self, ctx: FuncTestCaseParser.IntervalDayLiteralContext
    ):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timeInterval.
    def visitTimeInterval(self, ctx: FuncTestCaseParser.TimeIntervalContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#datatype.
    def visitDatatype(self, ctx: FuncTestCaseParser.DatatypeContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#Boolean.
    def visitBoolean(self, ctx: FuncTestCaseParser.BooleanContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#i8.
    def visitI8(self, ctx: FuncTestCaseParser.I8Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#i16.
    def visitI16(self, ctx: FuncTestCaseParser.I16Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#i32.
    def visitI32(self, ctx: FuncTestCaseParser.I32Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#i64.
    def visitI64(self, ctx: FuncTestCaseParser.I64Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#fp32.
    def visitFp32(self, ctx: FuncTestCaseParser.Fp32Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#fp64.
    def visitFp64(self, ctx: FuncTestCaseParser.Fp64Context):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#string.
    def visitString(self, ctx: FuncTestCaseParser.StringContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#binary.
    def visitBinary(self, ctx: FuncTestCaseParser.BinaryContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timestamp.
    def visitTimestamp(self, ctx: FuncTestCaseParser.TimestampContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#timestampTz.
    def visitTimestampTz(self, ctx: FuncTestCaseParser.TimestampTzContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#date.
    def visitDate(self, ctx: FuncTestCaseParser.DateContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#time.
    def visitTime(self, ctx: FuncTestCaseParser.TimeContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalDay.
    def visitIntervalDay(self, ctx: FuncTestCaseParser.IntervalDayContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#intervalYear.
    def visitIntervalYear(self, ctx: FuncTestCaseParser.IntervalYearContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#uuid.
    def visitUuid(self, ctx: FuncTestCaseParser.UuidContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#userDefined.
    def visitUserDefined(self, ctx: FuncTestCaseParser.UserDefinedContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#fixedChar.
    def visitFixedChar(self, ctx: FuncTestCaseParser.FixedCharContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#varChar.
    def visitVarChar(self, ctx: FuncTestCaseParser.VarCharContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#fixedBinary.
    def visitFixedBinary(self, ctx: FuncTestCaseParser.FixedBinaryContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#decimal.
    def visitDecimal(self, ctx: FuncTestCaseParser.DecimalContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#precisionTimestamp.
    def visitPrecisionTimestamp(
        self, ctx: FuncTestCaseParser.PrecisionTimestampContext
    ):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#precisionTimestampTZ.
    def visitPrecisionTimestampTZ(
        self, ctx: FuncTestCaseParser.PrecisionTimestampTZContext
    ):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#parameterizedType.
    def visitParameterizedType(self, ctx: FuncTestCaseParser.ParameterizedTypeContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#integerLiteral.
    def visitIntegerLiteral(self, ctx: FuncTestCaseParser.IntegerLiteralContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#substraitError.
    def visitSubstraitError(self, ctx: FuncTestCaseParser.SubstraitErrorContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#func_option.
    def visitFunc_option(self, ctx: FuncTestCaseParser.Func_optionContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#option_name.
    def visitOption_name(self, ctx: FuncTestCaseParser.Option_nameContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#option_value.
    def visitOption_value(self, ctx: FuncTestCaseParser.Option_valueContext):
        return self.visitChildren(ctx)

    # Visit a parse tree produced by FuncTestCaseParser#func_options.
    def visitFunc_options(self, ctx: FuncTestCaseParser.Func_optionsContext):
        return self.visitChildren(ctx)


del FuncTestCaseParser
