package io.substrait.function;

import io.substrait.type.TypeCreator;

public class ParameterizedTypeCreator extends TypeCreator implements ExtendedTypeCreator<ParameterizedType, String> {

  public static final ParameterizedTypeCreator REQUIRED = new ParameterizedTypeCreator(false);
  public static final ParameterizedTypeCreator NULLABLE = new ParameterizedTypeCreator(true);

  protected ParameterizedTypeCreator(boolean nullable) {
    super(nullable);
  }

  public ParameterizedType.StringLiteral parameter(String literal) {
    return ParameterizedType.StringLiteral.builder().value(literal).build();
  }

  public ParameterizedType fixedCharE(String len) {
    return ParameterizedType.FixedChar.builder().nullable(nullable).length(parameter(len)).build();
  }

  public ParameterizedType varCharE(String len) {
    return ParameterizedType.VarChar.builder().nullable(nullable).length(parameter(len)).build();
  }

  public ParameterizedType fixedBinaryE(String len) {
    return ParameterizedType.FixedBinary.builder().nullable(nullable).length(parameter(len)).build();
  }

  public ParameterizedType decimalE(String precision, String scale) {
    return ParameterizedType.Decimal.builder().nullable(nullable).precision(parameter(precision)).scale(parameter(scale)).build();
  }

  public ParameterizedType structE(ParameterizedType... types) {
    return ParameterizedType.Struct.builder().nullable(nullable).addFields(types).build();
  }

  public ParameterizedType structE(Iterable<? extends ParameterizedType> types) {
    return ParameterizedType.Struct.builder().nullable(nullable).addAllFields(types).build();
  }

  public ParameterizedType listE(ParameterizedType type) {
    return ParameterizedType.ListType.builder().nullable(nullable).name(type).build();
  }

  public ParameterizedType mapE(ParameterizedType key, ParameterizedType value) {
    return ParameterizedType.Map.builder().nullable(nullable).key(key).value(value).build();
  }
}
