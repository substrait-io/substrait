package io.substrait.isthmus.expression;

import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.isthmus.CallConverter;
import org.apache.calcite.rex.RexCall;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.sql.fun.SqlArrayValueConstructor;
import org.apache.calcite.sql.fun.SqlMapValueConstructor;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;

public class LiteralConstructorConverter implements CallConverter {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(LiteralConstructorConverter.class);

  @Override
  public Optional<Expression> convert(RexCall call, Function<RexNode, Expression> topLevelConverter) {
    return switch(call.getOperator()) {
      case SqlArrayValueConstructor array -> Optional.of(
          ExpressionCreator.list(false,
              call.operands.stream().map(t -> ((Expression.Literal) t.accept(new RexExpressionConverter()))).toList()));


      case SqlMapValueConstructor map -> {
        List<Expression.Literal> literals = call.operands.stream().map(t -> ((Expression.Literal) t.accept(new RexExpressionConverter()))).toList();
        Map<Expression.Literal, Expression.Literal> items = new HashMap<>();
        assert literals.size() % 2 == 0;
        for (int i = 0; i < literals.size(); i+=2) {
          items.put(literals.get(i), literals.get(i + 1));
        }
        yield Optional.of(ExpressionCreator.map(false, items));
      }
      default -> Optional.empty();
    };
  }
}
