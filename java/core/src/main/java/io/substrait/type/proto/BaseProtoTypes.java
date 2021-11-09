package io.substrait.type.proto;

import io.substrait.proto.Type;

import java.util.Arrays;

abstract class BaseProtoTypes<T, I> {
  
  protected final Type.Nullability nullability;
  public final T BOOLEAN;
  public final T I8;
  public final T I16;
  public final T I32;
  public final T I64;
  public final T FP32;
  public final T FP64;
  public final T STRING;
  public final T BINARY;
  public final T TIMESTAMP;
  public final T TIMESTAMP_TZ;
  public final T DATE;
  public final T TIME;
  public final T INTERVAL_DAY;
  public final T INTERVAL_YEAR;
  public final T UUID;

  public BaseProtoTypes(Type.Nullability nullability) {
    this.nullability = nullability;
    BOOLEAN = wrap(Type.Boolean.newBuilder().setNullability(nullability).build());
    I8 = wrap(Type.I8.newBuilder().setNullability(nullability).build());
    I16 = wrap(Type.I16.newBuilder().setNullability(nullability).build());
    I32 = wrap(Type.I32.newBuilder().setNullability(nullability).build());
    I64 = wrap(Type.I64.newBuilder().setNullability(nullability).build());
    FP32 = wrap(Type.FP32.newBuilder().setNullability(nullability).build());
    FP64 = wrap(Type.FP64.newBuilder().setNullability(nullability).build());
    STRING = wrap(Type.String.newBuilder().setNullability(nullability).build());
    BINARY = wrap(Type.Binary.newBuilder().setNullability(nullability).build());
    TIMESTAMP = wrap(Type.Timestamp.newBuilder().setNullability(nullability).build());
    TIMESTAMP_TZ = wrap(Type.TimestampTZ.newBuilder().setNullability(nullability).build());
    DATE = wrap(Type.Date.newBuilder().setNullability(nullability).build());
    TIME = wrap(Type.Time.newBuilder().setNullability(nullability).build());
    INTERVAL_DAY = wrap(Type.IntervalDay.newBuilder().setNullability(nullability).build());
    INTERVAL_YEAR = wrap(Type.IntervalYear.newBuilder().setNullability(nullability).build());
    UUID = wrap(Type.UUID.newBuilder().setNullability(nullability).build());
  }

  public abstract T fixedChar(I len);

  public final T fixedChar(int len) {
    return fixedChar(i(len));
  }

  public final T fixedChar(String len) {
    return fixedChar(integerParam(len));
  }

  public final T varChar(int len) {
    return varChar(i(len));
  }

  public final T varChar(String len) {
    return varChar(integerParam(len));
  }

  public final T fixedBinary(int len) {
    return fixedBinary(i(len));
  }

  public final T fixedBinary(String len) {
    return fixedBinary(integerParam(len));
  }

  public final T decimal(int scale, int precision) {
    return decimal(i(scale), i(precision));
  }

  public final T decimal(I scale, int precision) {
    return decimal(scale, i(precision));
  }

  public final T decimal(int scale, I precision) {
    return decimal(i(scale), precision);
  }

  public abstract T typeParam(String name);

  public abstract I integerParam(String name);

  public abstract T varChar(I len);

  public abstract T fixedBinary(I len);

  public abstract T decimal(I scale, I precision);

  public final T struct(T... types) {
    return struct(Arrays.asList(types));
  }

  public abstract T struct(Iterable<T> types);

  public abstract T list(T type);

  public abstract T map(T key, T value);

  protected abstract T wrap(Object o);

  protected abstract I i(int integerValue);
}
