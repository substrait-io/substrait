# SPDX-License-Identifier: Apache-2.0
# Generated from FuncTestCaseParser.g4 by ANTLR 4.13.2
from antlr4 import *

if "." in __name__:
    from .FuncTestCaseParser import FuncTestCaseParser
else:
    from FuncTestCaseParser import FuncTestCaseParser


# This class defines a complete listener for a parse tree produced by FuncTestCaseParser.
class FuncTestCaseParserListener(ParseTreeListener):
    # Enter a parse tree produced by FuncTestCaseParser#doc.
    def enterDoc(self, ctx: FuncTestCaseParser.DocContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#doc.
    def exitDoc(self, ctx: FuncTestCaseParser.DocContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#header.
    def enterHeader(self, ctx: FuncTestCaseParser.HeaderContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#header.
    def exitHeader(self, ctx: FuncTestCaseParser.HeaderContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#version.
    def enterVersion(self, ctx: FuncTestCaseParser.VersionContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#version.
    def exitVersion(self, ctx: FuncTestCaseParser.VersionContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#include.
    def enterInclude(self, ctx: FuncTestCaseParser.IncludeContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#include.
    def exitInclude(self, ctx: FuncTestCaseParser.IncludeContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#testGroupDescription.
    def enterTestGroupDescription(
        self, ctx: FuncTestCaseParser.TestGroupDescriptionContext
    ):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#testGroupDescription.
    def exitTestGroupDescription(
        self, ctx: FuncTestCaseParser.TestGroupDescriptionContext
    ):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#testCase.
    def enterTestCase(self, ctx: FuncTestCaseParser.TestCaseContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#testCase.
    def exitTestCase(self, ctx: FuncTestCaseParser.TestCaseContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#testGroup.
    def enterTestGroup(self, ctx: FuncTestCaseParser.TestGroupContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#testGroup.
    def exitTestGroup(self, ctx: FuncTestCaseParser.TestGroupContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#arguments.
    def enterArguments(self, ctx: FuncTestCaseParser.ArgumentsContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#arguments.
    def exitArguments(self, ctx: FuncTestCaseParser.ArgumentsContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#result.
    def enterResult(self, ctx: FuncTestCaseParser.ResultContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#result.
    def exitResult(self, ctx: FuncTestCaseParser.ResultContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#argument.
    def enterArgument(self, ctx: FuncTestCaseParser.ArgumentContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#argument.
    def exitArgument(self, ctx: FuncTestCaseParser.ArgumentContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#numericLiteral.
    def enterNumericLiteral(self, ctx: FuncTestCaseParser.NumericLiteralContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#numericLiteral.
    def exitNumericLiteral(self, ctx: FuncTestCaseParser.NumericLiteralContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#floatLiteral.
    def enterFloatLiteral(self, ctx: FuncTestCaseParser.FloatLiteralContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#floatLiteral.
    def exitFloatLiteral(self, ctx: FuncTestCaseParser.FloatLiteralContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#nullArg.
    def enterNullArg(self, ctx: FuncTestCaseParser.NullArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#nullArg.
    def exitNullArg(self, ctx: FuncTestCaseParser.NullArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intArg.
    def enterIntArg(self, ctx: FuncTestCaseParser.IntArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intArg.
    def exitIntArg(self, ctx: FuncTestCaseParser.IntArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#floatArg.
    def enterFloatArg(self, ctx: FuncTestCaseParser.FloatArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#floatArg.
    def exitFloatArg(self, ctx: FuncTestCaseParser.FloatArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#decimalArg.
    def enterDecimalArg(self, ctx: FuncTestCaseParser.DecimalArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#decimalArg.
    def exitDecimalArg(self, ctx: FuncTestCaseParser.DecimalArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#booleanArg.
    def enterBooleanArg(self, ctx: FuncTestCaseParser.BooleanArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#booleanArg.
    def exitBooleanArg(self, ctx: FuncTestCaseParser.BooleanArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#stringArg.
    def enterStringArg(self, ctx: FuncTestCaseParser.StringArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#stringArg.
    def exitStringArg(self, ctx: FuncTestCaseParser.StringArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#dateArg.
    def enterDateArg(self, ctx: FuncTestCaseParser.DateArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#dateArg.
    def exitDateArg(self, ctx: FuncTestCaseParser.DateArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timeArg.
    def enterTimeArg(self, ctx: FuncTestCaseParser.TimeArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timeArg.
    def exitTimeArg(self, ctx: FuncTestCaseParser.TimeArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timestampArg.
    def enterTimestampArg(self, ctx: FuncTestCaseParser.TimestampArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timestampArg.
    def exitTimestampArg(self, ctx: FuncTestCaseParser.TimestampArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timestampTzArg.
    def enterTimestampTzArg(self, ctx: FuncTestCaseParser.TimestampTzArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timestampTzArg.
    def exitTimestampTzArg(self, ctx: FuncTestCaseParser.TimestampTzArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalYearArg.
    def enterIntervalYearArg(self, ctx: FuncTestCaseParser.IntervalYearArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalYearArg.
    def exitIntervalYearArg(self, ctx: FuncTestCaseParser.IntervalYearArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalDayArg.
    def enterIntervalDayArg(self, ctx: FuncTestCaseParser.IntervalDayArgContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalDayArg.
    def exitIntervalDayArg(self, ctx: FuncTestCaseParser.IntervalDayArgContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalYearLiteral.
    def enterIntervalYearLiteral(
        self, ctx: FuncTestCaseParser.IntervalYearLiteralContext
    ):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalYearLiteral.
    def exitIntervalYearLiteral(
        self, ctx: FuncTestCaseParser.IntervalYearLiteralContext
    ):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalDayLiteral.
    def enterIntervalDayLiteral(
        self, ctx: FuncTestCaseParser.IntervalDayLiteralContext
    ):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalDayLiteral.
    def exitIntervalDayLiteral(self, ctx: FuncTestCaseParser.IntervalDayLiteralContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timeInterval.
    def enterTimeInterval(self, ctx: FuncTestCaseParser.TimeIntervalContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timeInterval.
    def exitTimeInterval(self, ctx: FuncTestCaseParser.TimeIntervalContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#datatype.
    def enterDatatype(self, ctx: FuncTestCaseParser.DatatypeContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#datatype.
    def exitDatatype(self, ctx: FuncTestCaseParser.DatatypeContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#Boolean.
    def enterBoolean(self, ctx: FuncTestCaseParser.BooleanContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#Boolean.
    def exitBoolean(self, ctx: FuncTestCaseParser.BooleanContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#i8.
    def enterI8(self, ctx: FuncTestCaseParser.I8Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#i8.
    def exitI8(self, ctx: FuncTestCaseParser.I8Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#i16.
    def enterI16(self, ctx: FuncTestCaseParser.I16Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#i16.
    def exitI16(self, ctx: FuncTestCaseParser.I16Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#i32.
    def enterI32(self, ctx: FuncTestCaseParser.I32Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#i32.
    def exitI32(self, ctx: FuncTestCaseParser.I32Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#i64.
    def enterI64(self, ctx: FuncTestCaseParser.I64Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#i64.
    def exitI64(self, ctx: FuncTestCaseParser.I64Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#fp32.
    def enterFp32(self, ctx: FuncTestCaseParser.Fp32Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#fp32.
    def exitFp32(self, ctx: FuncTestCaseParser.Fp32Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#fp64.
    def enterFp64(self, ctx: FuncTestCaseParser.Fp64Context):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#fp64.
    def exitFp64(self, ctx: FuncTestCaseParser.Fp64Context):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#string.
    def enterString(self, ctx: FuncTestCaseParser.StringContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#string.
    def exitString(self, ctx: FuncTestCaseParser.StringContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#binary.
    def enterBinary(self, ctx: FuncTestCaseParser.BinaryContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#binary.
    def exitBinary(self, ctx: FuncTestCaseParser.BinaryContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timestamp.
    def enterTimestamp(self, ctx: FuncTestCaseParser.TimestampContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timestamp.
    def exitTimestamp(self, ctx: FuncTestCaseParser.TimestampContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#timestampTz.
    def enterTimestampTz(self, ctx: FuncTestCaseParser.TimestampTzContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#timestampTz.
    def exitTimestampTz(self, ctx: FuncTestCaseParser.TimestampTzContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#date.
    def enterDate(self, ctx: FuncTestCaseParser.DateContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#date.
    def exitDate(self, ctx: FuncTestCaseParser.DateContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#time.
    def enterTime(self, ctx: FuncTestCaseParser.TimeContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#time.
    def exitTime(self, ctx: FuncTestCaseParser.TimeContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalDay.
    def enterIntervalDay(self, ctx: FuncTestCaseParser.IntervalDayContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalDay.
    def exitIntervalDay(self, ctx: FuncTestCaseParser.IntervalDayContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#intervalYear.
    def enterIntervalYear(self, ctx: FuncTestCaseParser.IntervalYearContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#intervalYear.
    def exitIntervalYear(self, ctx: FuncTestCaseParser.IntervalYearContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#uuid.
    def enterUuid(self, ctx: FuncTestCaseParser.UuidContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#uuid.
    def exitUuid(self, ctx: FuncTestCaseParser.UuidContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#userDefined.
    def enterUserDefined(self, ctx: FuncTestCaseParser.UserDefinedContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#userDefined.
    def exitUserDefined(self, ctx: FuncTestCaseParser.UserDefinedContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#fixedChar.
    def enterFixedChar(self, ctx: FuncTestCaseParser.FixedCharContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#fixedChar.
    def exitFixedChar(self, ctx: FuncTestCaseParser.FixedCharContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#varChar.
    def enterVarChar(self, ctx: FuncTestCaseParser.VarCharContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#varChar.
    def exitVarChar(self, ctx: FuncTestCaseParser.VarCharContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#fixedBinary.
    def enterFixedBinary(self, ctx: FuncTestCaseParser.FixedBinaryContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#fixedBinary.
    def exitFixedBinary(self, ctx: FuncTestCaseParser.FixedBinaryContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#decimal.
    def enterDecimal(self, ctx: FuncTestCaseParser.DecimalContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#decimal.
    def exitDecimal(self, ctx: FuncTestCaseParser.DecimalContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#precisionTimestamp.
    def enterPrecisionTimestamp(
        self, ctx: FuncTestCaseParser.PrecisionTimestampContext
    ):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#precisionTimestamp.
    def exitPrecisionTimestamp(self, ctx: FuncTestCaseParser.PrecisionTimestampContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#precisionTimestampTZ.
    def enterPrecisionTimestampTZ(
        self, ctx: FuncTestCaseParser.PrecisionTimestampTZContext
    ):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#precisionTimestampTZ.
    def exitPrecisionTimestampTZ(
        self, ctx: FuncTestCaseParser.PrecisionTimestampTZContext
    ):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#parameterizedType.
    def enterParameterizedType(self, ctx: FuncTestCaseParser.ParameterizedTypeContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#parameterizedType.
    def exitParameterizedType(self, ctx: FuncTestCaseParser.ParameterizedTypeContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#integerLiteral.
    def enterIntegerLiteral(self, ctx: FuncTestCaseParser.IntegerLiteralContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#integerLiteral.
    def exitIntegerLiteral(self, ctx: FuncTestCaseParser.IntegerLiteralContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#substraitError.
    def enterSubstraitError(self, ctx: FuncTestCaseParser.SubstraitErrorContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#substraitError.
    def exitSubstraitError(self, ctx: FuncTestCaseParser.SubstraitErrorContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#func_option.
    def enterFunc_option(self, ctx: FuncTestCaseParser.Func_optionContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#func_option.
    def exitFunc_option(self, ctx: FuncTestCaseParser.Func_optionContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#option_name.
    def enterOption_name(self, ctx: FuncTestCaseParser.Option_nameContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#option_name.
    def exitOption_name(self, ctx: FuncTestCaseParser.Option_nameContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#option_value.
    def enterOption_value(self, ctx: FuncTestCaseParser.Option_valueContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#option_value.
    def exitOption_value(self, ctx: FuncTestCaseParser.Option_valueContext):
        pass

    # Enter a parse tree produced by FuncTestCaseParser#func_options.
    def enterFunc_options(self, ctx: FuncTestCaseParser.Func_optionsContext):
        pass

    # Exit a parse tree produced by FuncTestCaseParser#func_options.
    def exitFunc_options(self, ctx: FuncTestCaseParser.Func_optionsContext):
        pass


del FuncTestCaseParser
