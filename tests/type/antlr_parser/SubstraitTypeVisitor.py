# SPDX-License-Identifier: Apache-2.0
# Generated from SubstraitType.g4 by ANTLR 4.13.2
from antlr4 import *
if "." in __name__:
    from .SubstraitTypeParser import SubstraitTypeParser
else:
    from SubstraitTypeParser import SubstraitTypeParser

# This class defines a complete generic visitor for a parse tree produced by SubstraitTypeParser.

class SubstraitTypeVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by SubstraitTypeParser#startRule.
    def visitStartRule(self, ctx:SubstraitTypeParser.StartRuleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#typeStatement.
    def visitTypeStatement(self, ctx:SubstraitTypeParser.TypeStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#boolean.
    def visitBoolean(self, ctx:SubstraitTypeParser.BooleanContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#i8.
    def visitI8(self, ctx:SubstraitTypeParser.I8Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#i16.
    def visitI16(self, ctx:SubstraitTypeParser.I16Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#i32.
    def visitI32(self, ctx:SubstraitTypeParser.I32Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#i64.
    def visitI64(self, ctx:SubstraitTypeParser.I64Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#fp32.
    def visitFp32(self, ctx:SubstraitTypeParser.Fp32Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#fp64.
    def visitFp64(self, ctx:SubstraitTypeParser.Fp64Context):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#string.
    def visitString(self, ctx:SubstraitTypeParser.StringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#binary.
    def visitBinary(self, ctx:SubstraitTypeParser.BinaryContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#timestamp.
    def visitTimestamp(self, ctx:SubstraitTypeParser.TimestampContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#timestampTz.
    def visitTimestampTz(self, ctx:SubstraitTypeParser.TimestampTzContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#date.
    def visitDate(self, ctx:SubstraitTypeParser.DateContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#time.
    def visitTime(self, ctx:SubstraitTypeParser.TimeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#intervalYear.
    def visitIntervalYear(self, ctx:SubstraitTypeParser.IntervalYearContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#uuid.
    def visitUuid(self, ctx:SubstraitTypeParser.UuidContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#fixedChar.
    def visitFixedChar(self, ctx:SubstraitTypeParser.FixedCharContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#varChar.
    def visitVarChar(self, ctx:SubstraitTypeParser.VarCharContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#fixedBinary.
    def visitFixedBinary(self, ctx:SubstraitTypeParser.FixedBinaryContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#decimal.
    def visitDecimal(self, ctx:SubstraitTypeParser.DecimalContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#precisionIntervalDay.
    def visitPrecisionIntervalDay(self, ctx:SubstraitTypeParser.PrecisionIntervalDayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#precisionTime.
    def visitPrecisionTime(self, ctx:SubstraitTypeParser.PrecisionTimeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#precisionTimestamp.
    def visitPrecisionTimestamp(self, ctx:SubstraitTypeParser.PrecisionTimestampContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#precisionTimestampTZ.
    def visitPrecisionTimestampTZ(self, ctx:SubstraitTypeParser.PrecisionTimestampTZContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#struct.
    def visitStruct(self, ctx:SubstraitTypeParser.StructContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#nStruct.
    def visitNStruct(self, ctx:SubstraitTypeParser.NStructContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#list.
    def visitList(self, ctx:SubstraitTypeParser.ListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#map.
    def visitMap(self, ctx:SubstraitTypeParser.MapContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#userDefined.
    def visitUserDefined(self, ctx:SubstraitTypeParser.UserDefinedContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#numericLiteral.
    def visitNumericLiteral(self, ctx:SubstraitTypeParser.NumericLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#numericParameterName.
    def visitNumericParameterName(self, ctx:SubstraitTypeParser.NumericParameterNameContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#numericExpression.
    def visitNumericExpression(self, ctx:SubstraitTypeParser.NumericExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#anyType.
    def visitAnyType(self, ctx:SubstraitTypeParser.AnyTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#typeDef.
    def visitTypeDef(self, ctx:SubstraitTypeParser.TypeDefContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#IfExpr.
    def visitIfExpr(self, ctx:SubstraitTypeParser.IfExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#TypeLiteral.
    def visitTypeLiteral(self, ctx:SubstraitTypeParser.TypeLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#MultilineDefinition.
    def visitMultilineDefinition(self, ctx:SubstraitTypeParser.MultilineDefinitionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#Ternary.
    def visitTernary(self, ctx:SubstraitTypeParser.TernaryContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#BinaryExpr.
    def visitBinaryExpr(self, ctx:SubstraitTypeParser.BinaryExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#ParenExpression.
    def visitParenExpression(self, ctx:SubstraitTypeParser.ParenExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#ParameterName.
    def visitParameterName(self, ctx:SubstraitTypeParser.ParameterNameContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#FunctionCall.
    def visitFunctionCall(self, ctx:SubstraitTypeParser.FunctionCallContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#NotExpr.
    def visitNotExpr(self, ctx:SubstraitTypeParser.NotExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by SubstraitTypeParser#LiteralNumber.
    def visitLiteralNumber(self, ctx:SubstraitTypeParser.LiteralNumberContext):
        return self.visitChildren(ctx)



del SubstraitTypeParser