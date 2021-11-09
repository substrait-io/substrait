package io.substrait.type;

import java.util.stream.Collectors;
import java.util.stream.Stream;

public class TypeCreator {

  public static final TypeCreator REQUIRED = new TypeCreator(false);
  public static final TypeCreator NULLABLE = new TypeCreator(true);

  protected final boolean nullable;
  public final Type BOOLEAN;
  public final Type I8;
  public final Type I16;
  public final Type I32;
  public final Type I64;
  public final Type FP32;
  public final Type FP64;
  public final Type STRING;
  public final Type BINARY;
  public final Type TIMESTAMP;
  public final Type TIMESTAMP_TZ;
  public final Type DATE;
  public final Type TIME;
  public final Type INTERVAL_DAY;
  public final Type INTERVAL_YEAR;
  public final Type UUID;

  protected TypeCreator(boolean nullable) {
    this.nullable = nullable;
    BOOLEAN = Type.Bool.builder().nullable(nullable).build();
    I8 = Type.I8.builder().nullable(nullable).build();
    I16 = Type.I16.builder().nullable(nullable).build();
    I32 = Type.I32.builder().nullable(nullable).build();
    I64 = Type.I64.builder().nullable(nullable).build();
    FP32 = Type.FP32.builder().nullable(nullable).build();
    FP64 = Type.FP64.builder().nullable(nullable).build();
    STRING = Type.Str.builder().nullable(nullable).build();
    BINARY = Type.Binary.builder().nullable(nullable).build();
    TIMESTAMP = Type.Timestamp.builder().nullable(nullable).build();
    TIMESTAMP_TZ = Type.TimestampTZ.builder().nullable(nullable).build();
    DATE = Type.Date.builder().nullable(nullable).build();
    TIME = Type.Time.builder().nullable(nullable).build();
    INTERVAL_DAY = Type.IntervalDay.builder().nullable(nullable).build();
    INTERVAL_YEAR = Type.IntervalYear.builder().nullable(nullable).build();
    UUID = Type.UUID.builder().nullable(nullable).build();
  }

  public Type fixedChar(int len) {
    return Type.FixedChar.builder().nullable(nullable).length(len).build();
  }

  public final Type varChar(int len) {
    return Type.VarChar.builder().nullable(nullable).length(len).build();
  }

  public final Type fixedBinary(int len) {
    return Type.FixedBinary.builder().nullable(nullable).length(len).build();
  }

  public final Type decimal(int precision, int scale) {
    return Type.Decimal.builder().nullable(nullable).precision(precision).scale(scale).build();
  }

  public final Type.Struct struct(Type... types) {
    return Type.Struct.builder().nullable(nullable).addFields(types).build();
  }

  public Type.Struct struct(Iterable<? extends Type> types) {
    return Type.Struct.builder().nullable(nullable).addAllFields(types).build();
  }

  public Type.Struct struct(Stream<? extends Type> types) {
    return Type.Struct.builder().nullable(nullable).addAllFields(types.collect(Collectors.toList())).build();
  }

  public Type list(Type type) {
    return Type.ListType.builder().nullable(nullable).elementType(type).build();
  }

  public Type map(Type key, Type value) {
    return Type.Map.builder().nullable(nullable).key(key).value(value).build();
  }

  public static TypeCreator of(boolean nullability) {
    return nullability ? NULLABLE : REQUIRED;
  }
}
