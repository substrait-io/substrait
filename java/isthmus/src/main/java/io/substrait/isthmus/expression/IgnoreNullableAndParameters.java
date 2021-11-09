package io.substrait.isthmus.expression;

import io.substrait.function.ParameterizedType;
import io.substrait.function.ParameterizedTypeVisitor;
import io.substrait.type.Type;

public class IgnoreNullableAndParameters implements ParameterizedTypeVisitor<Boolean, RuntimeException> {

  private final ParameterizedType typeToMatch;

  public IgnoreNullableAndParameters(ParameterizedType typeToMatch) {
    this.typeToMatch = typeToMatch;
  }

  @Override
  public Boolean visit(Type.Bool type) {
    return typeToMatch instanceof Type.Bool;
  }

  @Override
  public Boolean visit(Type.I8 type) {
    return typeToMatch instanceof Type.I8;
  }

  @Override
  public Boolean visit(Type.I16 type) {
    return typeToMatch instanceof Type.I16;
  }

  @Override
  public Boolean visit(Type.I32 type) {
    return typeToMatch instanceof Type.I32;
  }

  @Override
  public Boolean visit(Type.I64 type) {
    return typeToMatch instanceof Type.I64;
  }

  @Override
  public Boolean visit(Type.FP32 type) {
    return typeToMatch instanceof Type.FP32;
  }

  @Override
  public Boolean visit(Type.FP64 type) {
    return typeToMatch instanceof Type.FP64;
  }

  @Override
  public Boolean visit(Type.Str type) {
    return typeToMatch instanceof Type.Str;
  }

  @Override
  public Boolean visit(Type.Binary type) {
    return typeToMatch instanceof Type.Binary;
  }

  @Override
  public Boolean visit(Type.Date type) {
    return typeToMatch instanceof Type.Date;
  }

  @Override
  public Boolean visit(Type.Time type) {
    return typeToMatch instanceof Type.Time;
  }

  @Override
  public Boolean visit(Type.TimestampTZ type) {
    return typeToMatch instanceof Type.TimestampTZ;
  }

  @Override
  public Boolean visit(Type.Timestamp type) {
    return typeToMatch instanceof Type.Timestamp;
  }

  @Override
  public Boolean visit(Type.IntervalYear type) {
    return typeToMatch instanceof Type.IntervalYear;
  }

  @Override
  public Boolean visit(Type.IntervalDay type) {
    return typeToMatch instanceof Type.IntervalDay;
  }

  @Override
  public Boolean visit(Type.UUID type) {
    return typeToMatch instanceof Type.UUID;
  }

  @Override
  public Boolean visit(Type.FixedChar type) {
    return typeToMatch instanceof Type.FixedChar || typeToMatch instanceof ParameterizedType.FixedChar;
  }

  @Override
  public Boolean visit(Type.VarChar type) {
    return typeToMatch instanceof Type.VarChar || typeToMatch instanceof ParameterizedType.VarChar;
  }

  @Override
  public Boolean visit(Type.FixedBinary type) {
    return typeToMatch instanceof Type.FixedBinary || typeToMatch instanceof ParameterizedType.FixedBinary;
  }

  @Override
  public Boolean visit(Type.Decimal type) {
    return typeToMatch instanceof Type.Decimal || typeToMatch instanceof ParameterizedType.Decimal;
  }

  @Override
  public Boolean visit(Type.Struct type) {
    return typeToMatch instanceof Type.Struct || typeToMatch instanceof ParameterizedType.Struct;
  }

  @Override
  public Boolean visit(Type.ListType type) {
    return typeToMatch instanceof Type.ListType || typeToMatch instanceof ParameterizedType.ListType;
  }

  @Override
  public Boolean visit(Type.Map type) {
    return typeToMatch instanceof Type.Map || typeToMatch instanceof ParameterizedType.Map;
  }

  @Override
  public Boolean visit(ParameterizedType.FixedChar expr) throws RuntimeException {
    return typeToMatch instanceof Type.FixedChar || typeToMatch instanceof ParameterizedType.FixedChar;
  }

  @Override
  public Boolean visit(ParameterizedType.VarChar expr) throws RuntimeException {
    return typeToMatch instanceof Type.VarChar || typeToMatch instanceof ParameterizedType.VarChar;
  }

  @Override
  public Boolean visit(ParameterizedType.FixedBinary expr) throws RuntimeException {
    return typeToMatch instanceof Type.FixedBinary || typeToMatch instanceof ParameterizedType.FixedBinary;
  }

  @Override
  public Boolean visit(ParameterizedType.Decimal expr) throws RuntimeException {
    return typeToMatch instanceof Type.Decimal || typeToMatch instanceof ParameterizedType.Decimal;
  }

  @Override
  public Boolean visit(ParameterizedType.Struct expr) throws RuntimeException {
    return typeToMatch instanceof Type.Struct || typeToMatch instanceof ParameterizedType.Struct;
  }

  @Override
  public Boolean visit(ParameterizedType.ListType expr) throws RuntimeException {
    return typeToMatch instanceof Type.ListType || typeToMatch instanceof ParameterizedType.ListType;
  }

  @Override
  public Boolean visit(ParameterizedType.Map expr) throws RuntimeException {
    return typeToMatch instanceof Type.Map || typeToMatch instanceof ParameterizedType.Map;
  }

  @Override
  public Boolean visit(ParameterizedType.StringLiteral stringLiteral) throws RuntimeException {
    return false;
  }
}
