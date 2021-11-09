package io.substrait.type.parser;

import io.substrait.type.SubstraitTypeParser;
import io.substrait.type.SubstraitTypeVisitor;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import io.substrait.function.ParameterizedType;
import io.substrait.function.ParameterizedTypeCreator;
import io.substrait.function.TypeExpression;
import io.substrait.function.TypeExpressionCreator;
import org.antlr.v4.runtime.tree.ErrorNode;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.RuleNode;
import org.antlr.v4.runtime.tree.TerminalNode;

import java.util.Locale;
import java.util.function.Function;
import java.util.function.IntFunction;

public class ParseToPojo {

  public static Type type(SubstraitTypeParser.StartContext ctx) {
    return (Type) ctx.accept(Visitor.SIMPLE);
  }

  public static ParameterizedType parameterizedType(SubstraitTypeParser.StartContext ctx) {
    return (ParameterizedType) ctx.accept(Visitor.PARAMETERIZED);
  }

  public static TypeExpression typeExpression(SubstraitTypeParser.StartContext ctx) {
    return ctx.accept(Visitor.EXPRESSION);
  }
  
  public static enum Visitor implements SubstraitTypeVisitor<TypeExpression> {
    SIMPLE, PARAMETERIZED, EXPRESSION;

    private void checkParameterizedOrExpression() {
      if (this != EXPRESSION && this != PARAMETERIZED) {
        throw new UnsupportedOperationException(
            "This construct can only be used in Parameterized Types or Type Expressions.");
      }
    }

    private void checkExpression() {
      if (this != EXPRESSION) {
        throw new UnsupportedOperationException(
            "This construct can only be used in Type Expressions.");
      }
    }

    @Override public TypeExpression visitStart(final SubstraitTypeParser.StartContext ctx) {
      return ctx.expr().accept(this);
    }

    @Override public Type visitBoolean(final SubstraitTypeParser.BooleanContext ctx) {
      return withNull(ctx).BOOLEAN;
    }

    @Override public Type visitI8(final SubstraitTypeParser.I8Context ctx) {
      return withNull(ctx).I8;
    }

    @Override public Type visitI16(final SubstraitTypeParser.I16Context ctx) {
      return withNull(ctx).I16;
    }

    @Override public Type visitI32(final SubstraitTypeParser.I32Context ctx) {
      return withNull(ctx).I32;
    }

    @Override public Type visitI64(final SubstraitTypeParser.I64Context ctx) {
      return withNull(ctx).I64;
    }

    @Override public TypeExpression visitTypeLiteral(final SubstraitTypeParser.TypeLiteralContext ctx) {
      return ctx.type().accept(this);
    }

    @Override public Type visitFp32(final SubstraitTypeParser.Fp32Context ctx) {
      return withNull(ctx).FP32;
    }

    @Override public Type visitFp64(final SubstraitTypeParser.Fp64Context ctx) {
      return withNull(ctx).FP64;
    }

    @Override public Type visitString(final SubstraitTypeParser.StringContext ctx) {
      return withNull(ctx).STRING;
    }

    @Override public Type visitBinary(final SubstraitTypeParser.BinaryContext ctx) {
      return withNull(ctx).BINARY;
    }

    @Override public Type visitTimestamp(
        final SubstraitTypeParser.TimestampContext ctx) {
      return withNull(ctx).TIMESTAMP;
    }

    @Override public Type visitTimestampTz(final SubstraitTypeParser.TimestampTzContext ctx) {
      return withNull(ctx).TIMESTAMP_TZ;
    }

    @Override public Type visitDate(final SubstraitTypeParser.DateContext ctx) {
      return withNull(ctx).DATE;
    }

    @Override public Type visitTime(final SubstraitTypeParser.TimeContext ctx) {
      return withNull(ctx).TIME;
    }

    @Override public Type visitIntervalDay(final SubstraitTypeParser.IntervalDayContext ctx) {
      return withNull(ctx).INTERVAL_DAY;
    }

    @Override public Type visitIntervalYear(final SubstraitTypeParser.IntervalYearContext ctx) {
      return withNull(ctx).INTERVAL_YEAR;
    }

    @Override public Type visitUuid(final SubstraitTypeParser.UuidContext ctx) {
      return withNull(ctx).UUID;
    }

    @Override public TypeExpression visitFixedChar(final SubstraitTypeParser.FixedCharContext ctx) {
      return of(ctx.len,
          withNull(ctx)::fixedChar,
          withNullP(ctx)::fixedCharE,
          withNullE(ctx)::fixedCharE
      );
    }

    private TypeExpression of(SubstraitTypeParser.NumericParameterContext ctx, IntFunction<TypeExpression> intFunc, Function<String, TypeExpression> strFunc, Function<TypeExpression, TypeExpression> exprFunc) {
      TypeExpression type = ctx.accept(this);
      if (type instanceof TypeExpression.IntegerLiteral) {
        return intFunc.apply(((TypeExpression.IntegerLiteral) type).value());
      }
      if (type instanceof ParameterizedType.StringLiteral) {
        checkParameterizedOrExpression();
        return strFunc.apply(((ParameterizedType.StringLiteral) type).value());
      }
      checkExpression();
      return exprFunc.apply(type);
    }

    @Override public TypeExpression visitVarChar(final SubstraitTypeParser.VarCharContext ctx) {
      return of(ctx.len,
          withNull(ctx)::varChar,
          withNullP(ctx)::varCharE,
          withNullE(ctx)::varCharE
      );
    }

    @Override public TypeExpression visitFixedBinary(
        final SubstraitTypeParser.FixedBinaryContext ctx) {
      return of(ctx.len,
          withNull(ctx)::fixedBinary,
          withNullP(ctx)::fixedBinaryE,
          withNullE(ctx)::fixedBinaryE
      );
    }

    @Override public TypeExpression visitDecimal(final SubstraitTypeParser.DecimalContext ctx) {
      Object precision = i(ctx.precision);
      Object scale = i(ctx.scale);

      if (precision instanceof Integer && scale instanceof Integer) {
        return withNull(ctx).decimal((int) precision, (int) scale);
      }

      if (precision instanceof String && scale instanceof String) {
        checkParameterizedOrExpression();
        return withNullP(ctx).decimalE((String) precision, (String) scale);
      }

      checkExpression();
      return withNullE(ctx).decimalE(ctx.precision.accept(this), ctx.scale.accept(this));

    }

    private Object i(SubstraitTypeParser.NumericParameterContext ctx) {
      TypeExpression type = ctx.accept(this);
      if (type instanceof TypeExpression.IntegerLiteral) {
        return ((TypeExpression.IntegerLiteral) type).value();
      } else if (type instanceof ParameterizedType.StringLiteral) {
        checkParameterizedOrExpression();
        return ((ParameterizedType.StringLiteral) type).value();
      } else {
        checkExpression();
        return type;
      }
    }

    @Override public TypeExpression visitStruct(final SubstraitTypeParser.StructContext ctx) {
      var types = ctx.expr().stream().map(t -> t.accept(this)).toList();
      if (types.stream().allMatch(t -> t instanceof Type)) {
        return withNull(ctx).struct(types.stream().map(t -> ((Type) t)).toList());
      }

      if (types.stream().allMatch(t -> t instanceof ParameterizedType)) {
        checkParameterizedOrExpression();
        return withNullP(ctx).structE(types.stream().map(t -> ((ParameterizedType) t)).toList());
      }

      checkExpression();
      return withNullE(ctx).structE(types);
    }

    @Override public TypeExpression visitNStruct(final SubstraitTypeParser.NStructContext ctx) {
      throw new UnsupportedOperationException();
    }

    @Override public TypeExpression visitList(final SubstraitTypeParser.ListContext ctx) {
      TypeExpression element = ctx.expr().accept(this);
      if (element instanceof Type) {
        return withNull(ctx).list((Type) element);
      }

      if (element instanceof ParameterizedType) {
        checkParameterizedOrExpression();
        return withNullP(ctx).listE((ParameterizedType) element);
      }

      checkExpression();
      return withNullE(ctx).listE(element);
    }

    @Override public TypeExpression visitMap(final SubstraitTypeParser.MapContext ctx) {
      TypeExpression key = ctx.key.accept(this);
      TypeExpression value = ctx.value.accept(this);
      if (key instanceof Type && value instanceof Type) {
        return withNull(ctx).map((Type) key, (Type) value);
      }

      if (key instanceof ParameterizedType && value instanceof ParameterizedType) {
        checkParameterizedOrExpression();
        return withNullP(ctx).mapE((ParameterizedType) key, (ParameterizedType) value);
      }
      checkExpression();
      return withNullE(ctx).mapE(key, value);
    }

    private TypeCreator withNull(SubstraitTypeParser.RequiredTypeContext required) {
      return Type.withNullability(((SubstraitTypeParser.TypeContext) required.parent).isnull != null);
    }

    private TypeExpressionCreator withNullE(SubstraitTypeParser.RequiredTypeContext required) {
      return TypeExpression.withNullability(((SubstraitTypeParser.TypeContext) required.parent).isnull != null);
    }

    private ParameterizedTypeCreator withNullP(SubstraitTypeParser.RequiredTypeContext required) {
      return ParameterizedType.withNullability(((SubstraitTypeParser.TypeContext) required.parent).isnull != null);
    }

    @Override public TypeExpression visitType(final SubstraitTypeParser.TypeContext ctx) {
      return ctx.requiredType().accept(this);
    }

    @Override public TypeExpression visitTypeParam(final SubstraitTypeParser.TypeParamContext ctx) {
      checkParameterizedOrExpression();
      return ParameterizedType.StringLiteral.builder().value(ctx.getText()).build();
    }

    @Override public TypeExpression visitParenExpression(
        final SubstraitTypeParser.ParenExpressionContext ctx) {
      return ctx.expr().accept(this);
    }

    @Override public TypeExpression visitIfExpr(final SubstraitTypeParser.IfExprContext ctx) {
      checkExpression();
      return TypeExpression.IfOperation.builder()
          .ifCondition(ctx.ifExpr.accept(this))
          .thenExpr(ctx.thenExpr.accept(this))
          .elseExpr(ctx.elseExpr.accept(this))
          .build();
    }

    @Override public TypeExpression visitTernary(final SubstraitTypeParser.TernaryContext ctx) {
      checkExpression();
      return TypeExpression.IfOperation.builder()
          .ifCondition(ctx.ifExpr.accept(this))
          .thenExpr(ctx.thenExpr.accept(this))
          .elseExpr(ctx.elseExpr.accept(this))
          .build();
    }

    @Override public TypeExpression visitMultilineDefinition(
        final SubstraitTypeParser.MultilineDefinitionContext ctx) {
      checkExpression();
      var exprs = ctx.expr().stream().map(t -> t.accept(this)).toList();
      var identifiers = ctx.Identifier().stream().map(t -> t.getText()).toList();
      var finalExpr = ctx.finalType.accept(this);

      var bldr = TypeExpression.ReturnProgram.builder();
      for (int i = 0; i < exprs.size(); i++) {
        bldr.addAssignments(TypeExpression.ReturnProgram.Assignment.builder()
            .expr(exprs.get(i))
            .name(identifiers.get(i))
            .build()
        );
      }

      bldr.finalExpression(finalExpr);
      return bldr.build();
    }

    @Override public TypeExpression visitBinaryExpr(final SubstraitTypeParser.BinaryExprContext ctx) {
      checkExpression();
      TypeExpression.BinaryOperation.OpType type = switch (ctx.op.getText()
          .toUpperCase(Locale.ROOT)) {
        case "+" -> TypeExpression.BinaryOperation.OpType.ADD;
        case "-" -> TypeExpression.BinaryOperation.OpType.SUBTRACT;
        case "*" -> TypeExpression.BinaryOperation.OpType.MULTIPLY;
        case "/" -> TypeExpression.BinaryOperation.OpType.DIVIDE;
        case ">" -> TypeExpression.BinaryOperation.OpType.GT;
        case "<" -> TypeExpression.BinaryOperation.OpType.LT;
        case "AND" -> TypeExpression.BinaryOperation.OpType.AND;
        case "OR" -> TypeExpression.BinaryOperation.OpType.OR;
        case "=" -> TypeExpression.BinaryOperation.OpType.EQ;
        case ":=" -> TypeExpression.BinaryOperation.OpType.COVERS;
        default -> throw new IllegalStateException();
      };
      return TypeExpression.BinaryOperation.builder().opType(type)
          .left(ctx.left.accept(this))
          .right(ctx.right.accept(this))
          .build();

    }

    @Override public TypeExpression visitNumericLiteral(
        final SubstraitTypeParser.NumericLiteralContext ctx) {
      return TypeExpression.IntegerLiteral.builder().value(Integer.parseInt(ctx.getText())).build();
    }

    @Override public TypeExpression visitNumericParameterName(
        final SubstraitTypeParser.NumericParameterNameContext ctx) {
      checkParameterizedOrExpression();
      return ParameterizedType.StringLiteral.builder().value(ctx.getText()).build();
    }

    @Override public TypeExpression visitNumericExpression(
        final SubstraitTypeParser.NumericExpressionContext ctx) {
      return ctx.expr().accept(this);
    }

    @Override public TypeExpression visitFunctionCall(
        final SubstraitTypeParser.FunctionCallContext ctx) {
      checkExpression();
      if (ctx.expr().size() != 2) {
        throw new IllegalStateException("Only two argument functions exist for type expressions.");
      }
      var name = ctx.Identifier().getSymbol().getText().toUpperCase(Locale.ROOT);
      TypeExpression.BinaryOperation.OpType type = switch (name) {
        case "MIN" -> TypeExpression.BinaryOperation.OpType.MIN;
        case "MAX" -> TypeExpression.BinaryOperation.OpType.MAX;
        default -> throw new IllegalStateException("The following operation was unrecognized: " + name);
      };
      return TypeExpression.BinaryOperation.builder().opType(type)
          .left(ctx.expr(0).accept(this))
          .right(ctx.expr(1).accept(this))
          .build();
    }

    @Override public TypeExpression visitNotExpr(final SubstraitTypeParser.NotExprContext ctx) {
      return TypeExpression.NotOperation.builder().inner(ctx.expr().accept(this)).build();
    }

    @Override public TypeExpression visitLiteralNumber(
        final SubstraitTypeParser.LiteralNumberContext ctx) {
      return i(Integer.parseInt(ctx.getText()));
    }

    protected TypeExpression i(int val) {
      return TypeExpression.IntegerLiteral.builder().value(val).build();
    }

    @Override public Type visit(final ParseTree tree) {
      throw new UnsupportedOperationException();
    }

    @Override public Type visitChildren(final RuleNode node) {
      throw new UnsupportedOperationException();
    }

    @Override public Type visitTerminal(final TerminalNode node) {
      throw new UnsupportedOperationException();
    }

    @Override public Type visitErrorNode(final ErrorNode node) {
      throw new UnsupportedOperationException();
    }
  }
}

