package io.substrait.isthmus;

import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.function.ImmutableSimpleExtension;
import io.substrait.function.SimpleExtension;
import io.substrait.isthmus.expression.RexExpressionConverter;
import io.substrait.isthmus.expression.ScalarFunctionConverter;
import io.substrait.type.Type;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.sql.type.SqlTypeName;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.util.function.Consumer;

import static org.apache.calcite.sql.fun.SqlStdOperatorTable.*;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class CalciteCallTest extends CalciteObjs {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(CalciteCallTest.class);

  private static final SimpleExtension.ExtensionCollection EXTENSION_COLLECTION;
  private final ScalarFunctionConverter functionConverter =
      new ScalarFunctionConverter(EXTENSION_COLLECTION.scalarFunctions(), type);
  private final RexExpressionConverter converter = new RexExpressionConverter(functionConverter);

  static {
    SimpleExtension.ExtensionCollection defaults = ImmutableSimpleExtension.ExtensionCollection.builder().build();
    try {
      defaults = SimpleExtension.loadDefaults();
    } catch (IOException e) {
      throw new RuntimeException("Failure while loading defaults.", e);
    }

    EXTENSION_COLLECTION = defaults;
  }

  @Test
  public void coerceNumericOp() {
    test("add:opt_i64_i64", rex.makeCall(PLUS, c(4, SqlTypeName.INTEGER), c(4, SqlTypeName.BIGINT)), func -> {
      // check that there is a cast for the incorrect argument type.
      assertEquals(ExpressionCreator.cast(Type.REQUIRED.I64, ExpressionCreator.i32(false, 4)), func.arguments().get(0));
    });
  }

  @Test
  public void directMatchPlus() {
    test("add:opt_i64_i64", rex.makeCall(PLUS, c(4, SqlTypeName.BIGINT), c(4, SqlTypeName.BIGINT)), func -> {

      // ensure both literals are included directly.
      assertTrue(func.arguments().get(0) instanceof Expression.I64Literal);
      assertTrue(func.arguments().get(1) instanceof Expression.I64Literal);
    });
  }

  @Test
  public void directMatchAnd() {
    test("and:bool", rex.makeCall(AND, c(true, SqlTypeName.BOOLEAN), c(true, SqlTypeName.BOOLEAN)));
  }

  @Test
  public void directMatchOr() {
    test("or:bool", rex.makeCall(OR, c(false, SqlTypeName.BOOLEAN), c(true, SqlTypeName.BOOLEAN)));
  }

  @Test
  public void not() {
    test("not:bool", rex.makeCall(NOT, c(false, SqlTypeName.BOOLEAN)));
  }


  private void test(String expectedName, RexNode call) {
    test(expectedName, call, c -> {});
  }

  private void test(String expectedName, RexNode call, Consumer<Expression.ScalarFunctionInvocation> consumer) {
    var expression = call.accept(converter);
    assertTrue(expression instanceof Expression.ScalarFunctionInvocation);
    Expression.ScalarFunctionInvocation func = (Expression.ScalarFunctionInvocation) expression;
    assertEquals(expectedName, func.declaration().key());
    consumer.accept(func);
  }
}
