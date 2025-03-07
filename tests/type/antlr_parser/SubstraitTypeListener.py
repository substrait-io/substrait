# SPDX-License-Identifier: Apache-2.0
# Generated from SubstraitType.g4 by ANTLR 4.13.2
from antlr4 import *
if "." in __name__:
    from .SubstraitTypeParser import SubstraitTypeParser
else:
    from SubstraitTypeParser import SubstraitTypeParser

# This class defines a complete listener for a parse tree produced by SubstraitTypeParser.
class SubstraitTypeListener(ParseTreeListener):

    # Enter a parse tree produced by SubstraitTypeParser#startRule.
    def enterStartRule(self, ctx:SubstraitTypeParser.StartRuleContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#startRule.
    def exitStartRule(self, ctx:SubstraitTypeParser.StartRuleContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#typeStatement.
    def enterTypeStatement(self, ctx:SubstraitTypeParser.TypeStatementContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#typeStatement.
    def exitTypeStatement(self, ctx:SubstraitTypeParser.TypeStatementContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#boolean.
    def enterBoolean(self, ctx:SubstraitTypeParser.BooleanContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#boolean.
    def exitBoolean(self, ctx:SubstraitTypeParser.BooleanContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#i8.
    def enterI8(self, ctx:SubstraitTypeParser.I8Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#i8.
    def exitI8(self, ctx:SubstraitTypeParser.I8Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#i16.
    def enterI16(self, ctx:SubstraitTypeParser.I16Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#i16.
    def exitI16(self, ctx:SubstraitTypeParser.I16Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#i32.
    def enterI32(self, ctx:SubstraitTypeParser.I32Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#i32.
    def exitI32(self, ctx:SubstraitTypeParser.I32Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#i64.
    def enterI64(self, ctx:SubstraitTypeParser.I64Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#i64.
    def exitI64(self, ctx:SubstraitTypeParser.I64Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#fp32.
    def enterFp32(self, ctx:SubstraitTypeParser.Fp32Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#fp32.
    def exitFp32(self, ctx:SubstraitTypeParser.Fp32Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#fp64.
    def enterFp64(self, ctx:SubstraitTypeParser.Fp64Context):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#fp64.
    def exitFp64(self, ctx:SubstraitTypeParser.Fp64Context):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#string.
    def enterString(self, ctx:SubstraitTypeParser.StringContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#string.
    def exitString(self, ctx:SubstraitTypeParser.StringContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#binary.
    def enterBinary(self, ctx:SubstraitTypeParser.BinaryContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#binary.
    def exitBinary(self, ctx:SubstraitTypeParser.BinaryContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#timestamp.
    def enterTimestamp(self, ctx:SubstraitTypeParser.TimestampContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#timestamp.
    def exitTimestamp(self, ctx:SubstraitTypeParser.TimestampContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#timestampTz.
    def enterTimestampTz(self, ctx:SubstraitTypeParser.TimestampTzContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#timestampTz.
    def exitTimestampTz(self, ctx:SubstraitTypeParser.TimestampTzContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#date.
    def enterDate(self, ctx:SubstraitTypeParser.DateContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#date.
    def exitDate(self, ctx:SubstraitTypeParser.DateContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#time.
    def enterTime(self, ctx:SubstraitTypeParser.TimeContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#time.
    def exitTime(self, ctx:SubstraitTypeParser.TimeContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#intervalYear.
    def enterIntervalYear(self, ctx:SubstraitTypeParser.IntervalYearContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#intervalYear.
    def exitIntervalYear(self, ctx:SubstraitTypeParser.IntervalYearContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#uuid.
    def enterUuid(self, ctx:SubstraitTypeParser.UuidContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#uuid.
    def exitUuid(self, ctx:SubstraitTypeParser.UuidContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#fixedChar.
    def enterFixedChar(self, ctx:SubstraitTypeParser.FixedCharContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#fixedChar.
    def exitFixedChar(self, ctx:SubstraitTypeParser.FixedCharContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#varChar.
    def enterVarChar(self, ctx:SubstraitTypeParser.VarCharContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#varChar.
    def exitVarChar(self, ctx:SubstraitTypeParser.VarCharContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#fixedBinary.
    def enterFixedBinary(self, ctx:SubstraitTypeParser.FixedBinaryContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#fixedBinary.
    def exitFixedBinary(self, ctx:SubstraitTypeParser.FixedBinaryContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#decimal.
    def enterDecimal(self, ctx:SubstraitTypeParser.DecimalContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#decimal.
    def exitDecimal(self, ctx:SubstraitTypeParser.DecimalContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#precisionIntervalDay.
    def enterPrecisionIntervalDay(self, ctx:SubstraitTypeParser.PrecisionIntervalDayContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#precisionIntervalDay.
    def exitPrecisionIntervalDay(self, ctx:SubstraitTypeParser.PrecisionIntervalDayContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#precisionTime.
    def enterPrecisionTime(self, ctx:SubstraitTypeParser.PrecisionTimeContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#precisionTime.
    def exitPrecisionTime(self, ctx:SubstraitTypeParser.PrecisionTimeContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#precisionTimestamp.
    def enterPrecisionTimestamp(self, ctx:SubstraitTypeParser.PrecisionTimestampContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#precisionTimestamp.
    def exitPrecisionTimestamp(self, ctx:SubstraitTypeParser.PrecisionTimestampContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#precisionTimestampTZ.
    def enterPrecisionTimestampTZ(self, ctx:SubstraitTypeParser.PrecisionTimestampTZContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#precisionTimestampTZ.
    def exitPrecisionTimestampTZ(self, ctx:SubstraitTypeParser.PrecisionTimestampTZContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#struct.
    def enterStruct(self, ctx:SubstraitTypeParser.StructContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#struct.
    def exitStruct(self, ctx:SubstraitTypeParser.StructContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#nStruct.
    def enterNStruct(self, ctx:SubstraitTypeParser.NStructContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#nStruct.
    def exitNStruct(self, ctx:SubstraitTypeParser.NStructContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#list.
    def enterList(self, ctx:SubstraitTypeParser.ListContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#list.
    def exitList(self, ctx:SubstraitTypeParser.ListContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#map.
    def enterMap(self, ctx:SubstraitTypeParser.MapContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#map.
    def exitMap(self, ctx:SubstraitTypeParser.MapContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#userDefined.
    def enterUserDefined(self, ctx:SubstraitTypeParser.UserDefinedContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#userDefined.
    def exitUserDefined(self, ctx:SubstraitTypeParser.UserDefinedContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#numericLiteral.
    def enterNumericLiteral(self, ctx:SubstraitTypeParser.NumericLiteralContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#numericLiteral.
    def exitNumericLiteral(self, ctx:SubstraitTypeParser.NumericLiteralContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#numericParameterName.
    def enterNumericParameterName(self, ctx:SubstraitTypeParser.NumericParameterNameContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#numericParameterName.
    def exitNumericParameterName(self, ctx:SubstraitTypeParser.NumericParameterNameContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#numericExpression.
    def enterNumericExpression(self, ctx:SubstraitTypeParser.NumericExpressionContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#numericExpression.
    def exitNumericExpression(self, ctx:SubstraitTypeParser.NumericExpressionContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#anyType.
    def enterAnyType(self, ctx:SubstraitTypeParser.AnyTypeContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#anyType.
    def exitAnyType(self, ctx:SubstraitTypeParser.AnyTypeContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#typeDef.
    def enterTypeDef(self, ctx:SubstraitTypeParser.TypeDefContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#typeDef.
    def exitTypeDef(self, ctx:SubstraitTypeParser.TypeDefContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#IfExpr.
    def enterIfExpr(self, ctx:SubstraitTypeParser.IfExprContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#IfExpr.
    def exitIfExpr(self, ctx:SubstraitTypeParser.IfExprContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#TypeLiteral.
    def enterTypeLiteral(self, ctx:SubstraitTypeParser.TypeLiteralContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#TypeLiteral.
    def exitTypeLiteral(self, ctx:SubstraitTypeParser.TypeLiteralContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#MultilineDefinition.
    def enterMultilineDefinition(self, ctx:SubstraitTypeParser.MultilineDefinitionContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#MultilineDefinition.
    def exitMultilineDefinition(self, ctx:SubstraitTypeParser.MultilineDefinitionContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#Ternary.
    def enterTernary(self, ctx:SubstraitTypeParser.TernaryContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#Ternary.
    def exitTernary(self, ctx:SubstraitTypeParser.TernaryContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#BinaryExpr.
    def enterBinaryExpr(self, ctx:SubstraitTypeParser.BinaryExprContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#BinaryExpr.
    def exitBinaryExpr(self, ctx:SubstraitTypeParser.BinaryExprContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#ParenExpression.
    def enterParenExpression(self, ctx:SubstraitTypeParser.ParenExpressionContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#ParenExpression.
    def exitParenExpression(self, ctx:SubstraitTypeParser.ParenExpressionContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#ParameterName.
    def enterParameterName(self, ctx:SubstraitTypeParser.ParameterNameContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#ParameterName.
    def exitParameterName(self, ctx:SubstraitTypeParser.ParameterNameContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#FunctionCall.
    def enterFunctionCall(self, ctx:SubstraitTypeParser.FunctionCallContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#FunctionCall.
    def exitFunctionCall(self, ctx:SubstraitTypeParser.FunctionCallContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#NotExpr.
    def enterNotExpr(self, ctx:SubstraitTypeParser.NotExprContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#NotExpr.
    def exitNotExpr(self, ctx:SubstraitTypeParser.NotExprContext):
        pass


    # Enter a parse tree produced by SubstraitTypeParser#LiteralNumber.
    def enterLiteralNumber(self, ctx:SubstraitTypeParser.LiteralNumberContext):
        pass

    # Exit a parse tree produced by SubstraitTypeParser#LiteralNumber.
    def exitLiteralNumber(self, ctx:SubstraitTypeParser.LiteralNumberContext):
        pass



del SubstraitTypeParser