package io.substrait.type.proto;

import io.substrait.proto.ParameterizedType;
import io.substrait.function.TypeExpression;
import io.substrait.function.TypeExpressionVisitor;
import io.substrait.proto.Type;

public class ParameterizedProtoConverter extends BaseProtoConverter<ParameterizedType, ParameterizedType.IntegerOption> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(
      ParameterizedProtoConverter.class);

  public ParameterizedProtoConverter() {
    super("Parameterized types cannot include return type expressions.");
  }

  @Override public BaseProtoTypes<ParameterizedType, ParameterizedType.IntegerOption> typeContainer(
      final boolean nullable) {
    return nullable ? PARAMETERIZED_NULLABLE : PARAMETERIZED_REQUIRED;
  }

  private static final BaseProtoTypes<ParameterizedType, ParameterizedType.IntegerOption> PARAMETERIZED_NULLABLE = new ParameterizedTypes(Type.Nullability.NULLABILITY_NULLABLE);
  private static final BaseProtoTypes<ParameterizedType, ParameterizedType.IntegerOption> PARAMETERIZED_REQUIRED = new ParameterizedTypes(Type.Nullability.NULLABILITY_REQUIRED);

  public ParameterizedType.IntegerOption i(final TypeExpression num) {
    return num.accept(new IntegerVisitor());
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.FixedChar expr) throws RuntimeException {
    return typeContainer(expr).fixedChar(expr.length().value());
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.VarChar expr) throws RuntimeException {
    return  typeContainer(expr).varChar(expr.length().value());
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.FixedBinary expr) throws RuntimeException {
    return  typeContainer(expr).fixedBinary(expr.length().value());
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.Decimal expr) throws RuntimeException {
    return  typeContainer(expr).decimal(i(expr.precision()), i(expr.scale()));
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.Struct expr) throws RuntimeException {
    return typeContainer(expr).struct(expr.fields().stream().map(f -> f.accept(this)).toList());
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.ListType expr) throws RuntimeException {
    return typeContainer(expr).list(expr.name().accept(this));
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.Map expr) throws RuntimeException {
    return typeContainer(expr).map(expr.key().accept(this), expr.value().accept(this));
  }

  @Override
  public ParameterizedType visit(io.substrait.function.ParameterizedType.StringLiteral stringLiteral) throws RuntimeException {
    return ParameterizedType.newBuilder().setTypeParameter(ParameterizedType.TypeParameter.newBuilder().setName(stringLiteral.value())).build();
  }

  private static class IntegerVisitor extends TypeExpressionVisitor.TypeExpressionThrowsVisitor<ParameterizedType.IntegerOption, RuntimeException> {

    public IntegerVisitor() {
      super("Parameterized integer locations should only include integer names or literals.");
    }

    @Override public ParameterizedType.IntegerOption visit(final TypeExpression.IntegerLiteral literal) {
      return ParameterizedType.IntegerOption.newBuilder().setLiteral(literal.value()).build();
    }

    @Override
    public ParameterizedType.IntegerOption visit(io.substrait.function.ParameterizedType.StringLiteral stringLiteral) throws RuntimeException {
      return ParameterizedType.IntegerOption.newBuilder()
          .setParameter(ParameterizedType.IntegerParameter.newBuilder().setName(stringLiteral.value()))
          .build();
    }

  }

  public static class ParameterizedTypes  extends BaseProtoTypes<ParameterizedType, ParameterizedType.IntegerOption> {

    public ParameterizedTypes(final Type.Nullability nullability) {
      super(nullability);
    }

    public ParameterizedType fixedChar(ParameterizedType.IntegerOption len) {
      return wrap(
          ParameterizedType.ParameterizedFixedChar.newBuilder().setLength(len).setNullability(nullability).build());
    }

    @Override public ParameterizedType typeParam(final String name) {
      return ParameterizedType.newBuilder().setTypeParameter(ParameterizedType.TypeParameter.newBuilder().setName(name)).build();
    }

    @Override public ParameterizedType.IntegerOption integerParam(final String name) {
      return ParameterizedType.IntegerOption.newBuilder().setParameter(
          ParameterizedType.IntegerParameter.newBuilder().setName(name)).build();
    }

    protected ParameterizedType.IntegerOption i(int len) {
      return ParameterizedType.IntegerOption.newBuilder().setLiteral(len).build();
    }

    private static ParameterizedType.IntegerOption i(String param) {
      return ParameterizedType.IntegerOption.newBuilder().setParameter(
          ParameterizedType.IntegerParameter.newBuilder().setName(param)).build();
    }

    public ParameterizedType varChar(ParameterizedType.IntegerOption len) {
      return wrap(ParameterizedType.ParameterizedVarChar.newBuilder().setLength(len).setNullability(nullability).build());
    }

    public ParameterizedType fixedBinary(ParameterizedType.IntegerOption len) {
      return wrap(ParameterizedType.ParameterizedFixedBinary.newBuilder().setLength(len).setNullability(nullability).build());
    }

    public ParameterizedType decimal(ParameterizedType.IntegerOption scale, ParameterizedType.IntegerOption precision) {
      return wrap(ParameterizedType.ParameterizedDecimal.newBuilder().setScale(scale).setPrecision(precision).setNullability(nullability).build());
    }

    public ParameterizedType struct(Iterable<ParameterizedType> types) {
      return wrap(ParameterizedType.ParameterizedStruct.newBuilder().addAllTypes(types).setNullability(nullability).build());
    }

    public ParameterizedType list(ParameterizedType type) {
      return wrap(ParameterizedType.ParameterizedList.newBuilder().setType(type).setNullability(Type.Nullability.NULLABILITY_NULLABLE).build());
    }

    public ParameterizedType map(ParameterizedType key, ParameterizedType value) {
      return wrap(ParameterizedType.ParameterizedMap.newBuilder().setKey(key).setValue(value).setNullability(Type.Nullability.NULLABILITY_NULLABLE).build());
    }

    @Override protected ParameterizedType wrap(final Object o) {
      var bldr = ParameterizedType.newBuilder();
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
        case ParameterizedType.ParameterizedFixedChar t -> bldr.setFixedChar(t).build();
        case ParameterizedType.ParameterizedVarChar t -> bldr.setVarchar(t).build();
        case ParameterizedType.ParameterizedFixedBinary t -> bldr.setFixedBinary(t).build();
        case ParameterizedType.ParameterizedDecimal t -> bldr.setDecimal(t).build();
        case ParameterizedType.ParameterizedStruct t -> bldr.setStruct(t).build();
        case ParameterizedType.ParameterizedList t -> bldr.setList(t).build();
        case ParameterizedType.ParameterizedMap t -> bldr.setMap(t).build();
        case Type.UUID t -> bldr.setUuid(t).build();
        default -> throw new UnsupportedOperationException("Unable to wrap type of " + o.getClass());
      };
    }
  }
}
