package io.substrait.function;

import io.substrait.type.TypeCreator;

import java.util.Arrays;

public class TypeExpressionCreator extends TypeCreator implements ExtendedTypeCreator<TypeExpression, TypeExpression> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(TypeExpressionCreator.class);

  public static final TypeExpressionCreator REQUIRED = new TypeExpressionCreator(false);
  public static final TypeExpressionCreator NULLABLE = new TypeExpressionCreator(true);

  protected TypeExpressionCreator(boolean nullable) {
    super(nullable);
  }

  public TypeExpression fixedCharE(TypeExpression len) {
    return TypeExpression.FixedChar.builder().nullable(nullable).length(len).build();
  }

  public TypeExpression varCharE(TypeExpression len) {
    return TypeExpression.VarChar.builder().nullable(nullable).length(len).build();
  }

  public TypeExpression fixedBinaryE(TypeExpression len) {
    return TypeExpression.FixedBinary.builder().nullable(nullable).length(len).build();
  }

  public TypeExpression decimalE(TypeExpression precision, TypeExpression scale) {
    return TypeExpression.Decimal.builder().nullable(nullable).scale(scale).precision(precision).build();
  }

  public TypeExpression structE(TypeExpression... types) {
    return TypeExpression.Struct.builder().nullable(nullable).addFields(types).build();
  }

  public TypeExpression structE(Iterable<? extends TypeExpression> types) {
    return TypeExpression.Struct.builder().nullable(nullable).addAllFields(types).build();
  }

  public TypeExpression listE(TypeExpression type) {
    return TypeExpression.ListType.builder().nullable(nullable).elementType(type).build();
  }

  public TypeExpression mapE(TypeExpression key, TypeExpression value) {
    return TypeExpression.Map.builder().nullable(nullable).key(key).value(value).build();
  }

  public record Assign(String name, TypeExpression expr) {};

  public static TypeExpression program(TypeExpression finalExpr, Assign... assignments) {
    return TypeExpression.ReturnProgram.builder()
        .finalExpression(finalExpr)
        .addAllAssignments(Arrays.stream(assignments)
            .map(a -> TypeExpression.ReturnProgram.Assignment.builder().name(a.name()).expr(a.expr()).build())
            .toList()).build();
  }

  public static TypeExpression plus(TypeExpression left, TypeExpression right) {
    return binary(TypeExpression.BinaryOperation.OpType.ADD, left, right);
  }

  public static TypeExpression minus(TypeExpression left, TypeExpression right) {
    return binary(TypeExpression.BinaryOperation.OpType.SUBTRACT, left, right);
  }

  public static TypeExpression binary(TypeExpression.BinaryOperation.OpType op, TypeExpression left, TypeExpression right) {
    return TypeExpression.BinaryOperation.builder().opType(op).left(left).right(right).build();
  }

  public static TypeExpression.IntegerLiteral i(int i) {
    return TypeExpression.IntegerLiteral.builder().value(i).build();
  }
}
