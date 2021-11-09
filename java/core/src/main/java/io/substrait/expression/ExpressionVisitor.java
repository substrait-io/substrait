package io.substrait.expression;

public interface ExpressionVisitor<R, E extends Throwable> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ExpressionVisitor.class);

  R visit(Expression.NullLiteral expr) throws E;
  R visit(Expression.BoolLiteral expr) throws E;
  R visit(Expression.I8Literal expr) throws E;
  R visit(Expression.I16Literal expr) throws E;
  R visit(Expression.I32Literal expr) throws E;
  R visit(Expression.I64Literal expr) throws E;
  R visit(Expression.FP32Literal expr) throws E;
  R visit(Expression.FP64Literal expr) throws E;
  R visit(Expression.StrLiteral expr) throws E;
  R visit(Expression.BinaryLiteral expr) throws E;
  R visit(Expression.TimeLiteral expr) throws E;
  R visit(Expression.DateLiteral expr) throws E;
  R visit(Expression.TimestampLiteral expr) throws E;
  R visit(Expression.TimestampTZLiteral expr) throws E;
  R visit(Expression.IntervalYearLiteral expr) throws E;
  R visit(Expression.IntervalDayLiteral expr) throws E;
  R visit(Expression.UUIDLiteral expr) throws E;
  R visit(Expression.FixedCharLiteral expr) throws E;
  R visit(Expression.VarCharLiteral expr) throws E;
  R visit(Expression.FixedBinaryLiteral expr) throws E;
  R visit(Expression.DecimalLiteral expr) throws E;
  R visit(Expression.MapLiteral expr) throws E;
  R visit(Expression.ListLiteral expr) throws E;
  R visit(Expression.StructLiteral expr) throws E;
  R visit(Expression.Switch expr) throws E;
  R visit(Expression.IfThen expr) throws E;
  R visit(Expression.ScalarFunctionInvocation expr) throws E;
  R visit(Expression.Cast expr) throws E;
  R visit(Expression.SingleOrList expr) throws E;
  R visit(Expression.MultiOrList expr) throws E;
  R visit(FieldReference expr) throws E;

}
