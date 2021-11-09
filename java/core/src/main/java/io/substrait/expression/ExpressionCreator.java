package io.substrait.expression;

import com.google.protobuf.ByteString;
import io.substrait.function.SimpleExtension;
import io.substrait.type.Type;

import java.math.BigDecimal;
import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.util.*;
import java.util.concurrent.TimeUnit;

public class ExpressionCreator {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ExpressionCreator.class);

  private ExpressionCreator() {
  }

  public static Expression.NullLiteral typedNull(Type t) {
    return Expression.NullLiteral.builder().type(t).build();
  }

  public static Expression.BoolLiteral bool(boolean nullable, boolean value) {
    return Expression.BoolLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.I8Literal i8(boolean nullable, int value) {
    return Expression.I8Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.I16Literal i16(boolean nullable, int value) {
    return Expression.I16Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.I32Literal i32(boolean nullable, int value) {
    return Expression.I32Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.I64Literal i64(boolean nullable, long value) {
    return Expression.I64Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.FP32Literal fp32(boolean nullable, float value) {
    return Expression.FP32Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.FP64Literal fp64(boolean nullable, double value) {
    return Expression.FP64Literal.builder().nullable(nullable).value(value).build();
  }

  public static Expression.StrLiteral string(boolean nullable, String value) {
    return Expression.StrLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.BinaryLiteral binary(boolean nullable, ByteString value) {
    return Expression.BinaryLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.BinaryLiteral binary(boolean nullable, byte[] value) {
    return Expression.BinaryLiteral.builder().nullable(nullable).value(ByteString.copyFrom(value)).build();
  }

  public static Expression.DateLiteral date(boolean nullable, int value) {
    return Expression.DateLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.TimeLiteral time(boolean nullable, long value) {
    return Expression.TimeLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.TimestampLiteral timestamp(boolean nullable, long value) {
    return Expression.TimestampLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.TimestampLiteral timestamp(boolean nullable, LocalDateTime value) {
    var epochMicro = TimeUnit.SECONDS.toMicros(value.toEpochSecond(ZoneOffset.UTC)) +
        TimeUnit.NANOSECONDS.toMicros(value.toLocalTime().getNano());
    return timestamp(nullable, epochMicro);
  }

  public static Expression.TimestampLiteral timestamp(boolean nullable, int year, int month, int dayOfMonth, int hour, int minute, int second, int micros) {
    return timestamp(nullable,
        LocalDateTime.of(year, month, dayOfMonth, hour, minute, second)
            .withNano((int) TimeUnit.MICROSECONDS.toNanos(micros)));
  }

  public static Expression.TimestampTZLiteral timestampTZ(boolean nullable, long value) {
    return Expression.TimestampTZLiteral.builder().nullable(nullable).value(value).build();
  }

  public static Expression.TimestampTZLiteral timestampTZ(boolean nullable, Instant value) {
    var epochMicro = TimeUnit.SECONDS.toMicros(value.getEpochSecond()) + TimeUnit.NANOSECONDS.toMicros(value.getNano());
    return timestampTZ(nullable, epochMicro);
  }

  public static Expression.IntervalYearLiteral intervalYear(boolean nullable, int years, int months) {
    return Expression.IntervalYearLiteral.builder().nullable(nullable).years(years).months(months).build();
  }

  public static Expression.IntervalDayLiteral intervalDay(boolean nullable, int days, int seconds) {
    return Expression.IntervalDayLiteral.builder().nullable(nullable).days(days).seconds(seconds).build();
  }

  public static Expression.UUIDLiteral uuid(boolean nullable, ByteString uuid) {
    var bb = uuid.asReadOnlyByteBuffer();
    return Expression.UUIDLiteral.builder().nullable(nullable).value(new UUID(bb.getLong(), bb.getLong())).build();
  }

  public static Expression.UUIDLiteral uuid(boolean nullable, UUID uuid) {
    return Expression.UUIDLiteral.builder().nullable(nullable).value(uuid).build();
  }

  public static Expression.FixedCharLiteral fixedChar(boolean nullable, String str) {
    return Expression.FixedCharLiteral.builder().nullable(nullable).value(str).build();
  }

  public static Expression.VarCharLiteral varChar(boolean nullable, String str, int len) {
    return Expression.VarCharLiteral.builder().nullable(nullable).value(str).length(len).build();
  }

  public static Expression.FixedBinaryLiteral fixedBinary(boolean nullable, ByteString bytes) {
    return Expression.FixedBinaryLiteral.builder().nullable(nullable).value(bytes).build();
  }

  public static Expression.FixedBinaryLiteral fixedBinary(boolean nullable, byte[] bytes) {
    return Expression.FixedBinaryLiteral.builder().nullable(nullable).value(ByteString.copyFrom(bytes)).build();
  }

  public static Expression.DecimalLiteral decimal(boolean nullable, ByteString value, int precision, int scale) {
    return Expression.DecimalLiteral.builder().nullable(nullable).value(value).precision(precision).scale(scale).build();
  }

  public static Expression.DecimalLiteral decimal(boolean nullable, BigDecimal value, int precision, int scale) {
    var twosComplement = padLeftIfNeeded(value.multiply(BigDecimal.valueOf(scale * 10L)).toBigInteger().toByteArray(), 38);
    return Expression.DecimalLiteral.builder().nullable(nullable).value(twosComplement).precision(precision).scale(scale).build();
  }

  public static Expression.MapLiteral map(boolean nullable, Map<Expression.Literal, Expression.Literal> values) {
    return Expression.MapLiteral.builder().nullable(nullable).putAllValues(values).build();
  }

  public static Expression.ListLiteral list(boolean nullable, Expression.Literal... values) {
    return Expression.ListLiteral.builder().nullable(nullable).addValues(values).build();
  }

  public static Expression.ListLiteral list(boolean nullable, Iterable<? extends Expression.Literal> values) {
    return Expression.ListLiteral.builder().nullable(nullable).addAllValues(values).build();
  }

  public static Expression.StructLiteral struct(boolean nullable, Expression.Literal... values) {
    return Expression.StructLiteral.builder().nullable(nullable).addFields(values).build();
  }

  public static Expression.StructLiteral struct(boolean nullable, Iterable<? extends Expression.Literal> values) {
    return Expression.StructLiteral.builder().nullable(nullable).addAllFields(values).build();
  }

  public static Expression.Switch switchStatement(Expression defaultExpression, Expression.SwitchClause... conditionClauses) {
    return Expression.Switch.builder().defaultClause(defaultExpression).addSwitchClauses(conditionClauses).build();
  }

  public static Expression.Switch switchStatement(Expression defaultExpression, Iterable<? extends Expression.SwitchClause> conditionClauses) {
    return Expression.Switch.builder().defaultClause(defaultExpression).addAllSwitchClauses(conditionClauses).build();
  }

  public static Expression.SwitchClause switchClause(Expression.Literal expectedValue, Expression resultExpression) {
    return Expression.SwitchClause.builder().condition(expectedValue).then(resultExpression).build();
  }

  public static Expression.IfThen ifThenStatement(Expression elseExpression, Expression.IfClause... conditionClauses) {
    return Expression.IfThen.builder().elseClause(elseExpression).addIfClauses(conditionClauses).build();
  }

  public static Expression.IfThen ifThenStatement(Expression elseExpression, Iterable<? extends Expression.IfClause> conditionClauses) {
    return Expression.IfThen.builder().elseClause(elseExpression).addAllIfClauses(conditionClauses).build();
  }

  public static Expression.IfClause ifThenClause(Expression conditionExpression, Expression resultExpression) {
    return Expression.IfClause.builder().condition(conditionExpression).then(resultExpression).build();
  }

  public static Expression.ScalarFunctionInvocation scalarFunction(SimpleExtension.ScalarFunctionVariant declaration, Type outputType, Expression...arguments) {
    return Expression.ScalarFunctionInvocation.builder().declaration(declaration).outputType(outputType).addArguments(arguments).build();
  }

  public static Expression.ScalarFunctionInvocation scalarFunction(SimpleExtension.ScalarFunctionVariant declaration, Type outputType, Iterable<? extends Expression> arguments) {
    return Expression.ScalarFunctionInvocation.builder().declaration(declaration).outputType(outputType).addAllArguments(arguments).build();
  }

  public static AggregateFunctionInvocation aggregateFunction(
      SimpleExtension.AggregateFunctionVariant declaration,
      Type outputType,
      Expression.AggregationPhase phase,
      List<Expression.SortField> sort,
      Iterable<? extends Expression> arguments) {
    return AggregateFunctionInvocation.builder()
        .declaration(declaration)
        .outputType(outputType)
        .aggregationPhase(phase)
        .sort(sort)
        .addAllArguments(arguments)
        .build();
  }

  public static AggregateFunctionInvocation aggregateFunction(
      SimpleExtension.AggregateFunctionVariant declaration,
      Type outputType,
      Expression.AggregationPhase phase,
      List<Expression.SortField> sort,
      Expression... arguments) {
    return AggregateFunctionInvocation.builder()
        .declaration(declaration)
        .outputType(outputType)
        .aggregationPhase(phase)
        .sort(sort)
        .addArguments(arguments)
        .build();
  }

  public static Expression cast(Type type, Expression expression) {
    return Expression.Cast.builder().type(type).input(expression).build();
  }

  private static ByteString padLeftIfNeeded(byte[] value, int length) {

    if (length < value.length) {
      throw new IllegalArgumentException("Byte values should either be at or below the expected length.");
    }

    if(length == value.length) {
      return ByteString.copyFrom(value);
    }

    byte[] newArray = new byte[length];
    System.arraycopy(value, 0, newArray, length - value.length, value.length);
    return ByteString.copyFrom(newArray);
  }



}
