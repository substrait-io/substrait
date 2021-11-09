package io.substrait.type.proto;

import io.substrait.type.Type;
import io.substrait.function.NullableType;
import io.substrait.function.TypeExpressionVisitor;

abstract class BaseProtoConverter<T, I> extends TypeExpressionVisitor.TypeExpressionThrowsVisitor<T, RuntimeException> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(BaseProtoConverter.class);

  public abstract BaseProtoTypes<T, I> typeContainer(boolean nullable);

  public BaseProtoConverter(String unsupportedMessage) {
    super(unsupportedMessage);
  }

  public final BaseProtoTypes<T, I> typeContainer(NullableType literal) {
    return typeContainer(literal.nullable());
  }

  @Override public final T visit(final Type.Bool expr) {
    return typeContainer(expr).BOOLEAN;
  }

  @Override public final T visit(final Type.I8 expr) {
    return typeContainer(expr).I8;
  }

  @Override public final T visit(final Type.I16 expr) {
    return typeContainer(expr).I16;
  }

  @Override public final T visit(final Type.I32 expr) {
    return typeContainer(expr).I32;
  }

  @Override public final T visit(final Type.I64 expr) {
    return typeContainer(expr).I64;
  }

  @Override public final T visit(final Type.FP32 expr) {
    return typeContainer(expr).FP32;
  }

  @Override public final T visit(final Type.FP64 expr) {
    return typeContainer(expr).FP64;
  }

  @Override public final T visit(final Type.Str expr) {
    return typeContainer(expr).STRING;
  }

  @Override public final T visit(final Type.Binary expr) {
    return typeContainer(expr).BINARY;
  }

  @Override public final T visit(final Type.Date expr) {
    return typeContainer(expr).DATE;
  }

  @Override public final T visit(final Type.Time expr) {
    return typeContainer(expr).TIME;
  }

  @Override public final T visit(final Type.TimestampTZ expr) {
    return typeContainer(expr).TIMESTAMP_TZ;
  }

  @Override public final T visit(final Type.Timestamp expr) {
    return typeContainer(expr).TIMESTAMP;
  }

  @Override public final T visit(final Type.IntervalYear expr) {
    return typeContainer(expr).INTERVAL_YEAR;
  }

  @Override public final T visit(final Type.IntervalDay expr) {
    return typeContainer(expr).INTERVAL_DAY;
  }

  @Override public final T visit(final Type.UUID expr) {
    return typeContainer(expr).UUID;
  }

  @Override public final T visit(final Type.FixedChar expr) {
    return typeContainer(expr).fixedChar(expr.length());
  }

  @Override public final T visit(final Type.VarChar expr) {
    return typeContainer(expr).varChar(expr.length());
  }

  @Override public final T visit(final Type.FixedBinary expr) {
    return typeContainer(expr).fixedBinary(expr.length());
  }

  @Override public final T visit(final Type.Decimal expr) {
    return typeContainer(expr).decimal(expr.scale(), expr.precision());
  }

  @Override public final T visit(final Type.Struct expr) {
    return typeContainer(expr).struct(expr.fields().stream().map(t -> t.accept(this)).toList());
  }

  @Override public final T visit(final Type.ListType expr) {
    return typeContainer(expr).list(expr.elementType().accept(this));
  }

  @Override public final T visit(final Type.Map expr) {
    return typeContainer(expr).map(expr.key().accept(this), expr.value().accept(this));
  }
  
}
