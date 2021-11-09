package io.substrait.type;

import io.substrait.function.SimpleExtension;
import io.substrait.function.TypeExpression;

import java.util.List;

public class TypeExpressionEvaluator {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(
      TypeExpressionEvaluator.class);

  public static Type evaluateExpression(
      TypeExpression returnExpression,
      List<SimpleExtension.Argument> parameterizedTypeList,
      List<Type> actualTypes) {

    if (returnExpression instanceof Type) {
      return (Type) returnExpression;
    }
    throw new UnsupportedOperationException("NYI");
  }
}
