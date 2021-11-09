package io.substrait.type.proto;

import io.substrait.function.TypeExpressionVisitor;
import io.substrait.proto.Type;

public class TypeProtoConverter extends BaseProtoConverter<Type, Integer> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(TypeProtoConverter.class);

  public static TypeExpressionVisitor<Type, RuntimeException> INSTANCE = new TypeProtoConverter();

  public TypeProtoConverter() {
    super("Type literals cannot contain parameters or expressions.");
  }

  private static final BaseProtoTypes<Type, Integer> NULLABLE = new Types(Type.Nullability.NULLABILITY_NULLABLE);
  private static final BaseProtoTypes<Type, Integer> REQUIRED = new Types(Type.Nullability.NULLABILITY_REQUIRED);

  @Override public BaseProtoTypes<Type, Integer> typeContainer(final boolean nullable) {
    return nullable ? NULLABLE : REQUIRED;
  }

  private static class Types extends BaseProtoTypes<Type, Integer> {

    public Types(final Type.Nullability nullability) {
      super(nullability);
    }

    public Type fixedChar(Integer len) {
      return wrap(Type.FixedChar.newBuilder().setLength(len).setNullability(nullability).build());
    }

    @Override public Type typeParam(final String name) {
      throw new UnsupportedOperationException("It is not possible to use parameters in basic types.");
    }

    @Override public Integer integerParam(final String name) {
      throw new UnsupportedOperationException("It is not possible to use parameters in basic types.");
    }

    public Type varChar(Integer len) {
      return wrap(Type.VarChar.newBuilder().setLength(len).setNullability(nullability).build());
    }

    public Type fixedBinary(Integer len) {
      return wrap(Type.FixedBinary.newBuilder().setLength(len).setNullability(nullability).build());
    }

    public Type decimal(Integer scale, Integer precision) {
      return wrap(Type.Decimal.newBuilder().setScale(scale).setPrecision(precision).setNullability(nullability).build());
    }

    public Type struct(Iterable<Type> types) {
      return wrap(Type.Struct.newBuilder().addAllTypes(types).setNullability(nullability).build());
    }

    public Type list(Type type) {
      return wrap(Type.List.newBuilder().setType(type).setNullability(nullability).build());
    }

    public Type map(Type key, Type value) {
      return wrap(Type.Map.newBuilder().setKey(key).setValue(value).setNullability(nullability).build());
    }

    @Override protected Type wrap(final Object o) {
      var bldr = Type.newBuilder();
      return switch(o) {
        case Type.Boolean t -> bldr.setBool(t).build();
        case Type.I8 t -> bldr.setI8(t).build();
        case Type.I16 t -> bldr.setI16(t).build();
        case Type.I32 t -> bldr.setI32(t).build();
        case Type.I64 t -> bldr.setI64(t).build();
        case Type.FP32 t -> bldr.setFp32(t).build();
        case Type.FP64 t -> bldr.setFp64(t).build();
        case Type.String t -> bldr.setString(t).build();
        case Type.Binary t -> bldr.setBinary(t).build();
        case Type.Timestamp t -> bldr.setTimestamp(t).build();
        case Type.Date t -> bldr.setDate(t).build();
        case Type.Time t -> bldr.setTime(t).build();
        case Type.TimestampTZ t -> bldr.setTimestampTz(t).build();
        case Type.IntervalYear t -> bldr.setIntervalYear(t).build();
        case Type.IntervalDay t -> bldr.setIntervalDay(t).build();
        case Type.FixedChar t -> bldr.setFixedChar(t).build();
        case Type.VarChar t -> bldr.setVarchar(t).build();
        case Type.FixedBinary t -> bldr.setFixedBinary(t).build();
        case Type.Decimal t -> bldr.setDecimal(t).build();
        case Type.Struct t -> bldr.setStruct(t).build();
        case Type.List t -> bldr.setList(t).build();
        case Type.Map t -> bldr.setMap(t).build();
        case Type.UUID t -> bldr.setUuid(t).build();
        default -> throw new UnsupportedOperationException("Unable to wrap type of " + o.getClass());
      };
    }

    @Override protected Integer i(final int integerValue) {
      return integerValue;
    }
  }

}
