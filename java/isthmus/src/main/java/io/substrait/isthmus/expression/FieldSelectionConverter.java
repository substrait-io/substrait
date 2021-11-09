package io.substrait.isthmus.expression;

import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.expression.FieldReference;
import io.substrait.isthmus.CallConverter;
import org.apache.calcite.rex.*;
import org.apache.calcite.sql.SqlKind;

import java.util.Optional;
import java.util.function.Function;

/**
 * Converts field selections from Calcite representation.
 */
public class FieldSelectionConverter implements CallConverter {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(FieldSelectionConverter.class);

  public FieldSelectionConverter() {
    super();
  }

  @Override
  public Optional<Expression> convert(RexCall call, Function<RexNode, Expression> topLevelConverter) {
    if (!(call.getKind() == SqlKind.ITEM)) {
      return Optional.empty();
    }

    var toDereference = call.getOperands().get(0);
    var reference = call.getOperands().get(1);

    if (reference.getKind() != SqlKind.LITERAL || !(reference instanceof RexLiteral)) {
      logger.warn("Found item operator without literal kind/type. This isn't handled well. Reference was {} with toString {}.", reference.getKind().name(), reference);
      return Optional.empty();
    }

    var literal = LiteralConverter.convert((RexLiteral) reference);

    var input = topLevelConverter.apply(toDereference);

    switch(toDereference.getType().getSqlTypeName()) {
      case ROW: {
        var index = toInt(literal);
        if (index.isEmpty()) {
          return Optional.empty();
        }
        if (input instanceof FieldReference) {
          return Optional.of(((FieldReference) input).dereferenceStruct(index.get()));
        } else {
          return Optional.of(FieldReference.newStructReference(index.get(), input));
        }
      }
      case ARRAY: {
        var index = toInt(literal);
        if (index.isEmpty()) {
          return Optional.empty();
        }

        if(input instanceof FieldReference) {
          return Optional.of(((FieldReference) input).dereferenceList(index.get()));
        } else {
          return Optional.of(FieldReference.newListReference(index.get(), input));
        }

      }

      case MAP:  {
        var mapKey = toString(literal);
        if(mapKey.isEmpty()) {
          return Optional.empty();
        }

        Expression.Literal keyLiteral = ExpressionCreator.string(false, mapKey.get());
        if (input instanceof FieldReference) {
          return Optional.of(((FieldReference) input).dereferenceMap(keyLiteral));
        } else {
          return Optional.of(FieldReference.newMapReference(keyLiteral, input));
        }
      }
    }

    return Optional.empty();
  }

  private Optional<Integer> toInt(Expression.Literal l) {
    return switch(l) {
      case Expression.I8Literal i8 -> Optional.of(i8.value());
      case Expression.I16Literal i16 -> Optional.of(i16.value());
      case Expression.I32Literal i32 -> Optional.of(i32.value());
      case Expression.I64Literal i64 -> Optional.of((int) i64.value());
      default -> {
        logger.warn("Literal expected to be int type but was not. {}.", l);
        yield Optional.empty();
      }
    };
  }

  public Optional<String> toString(Expression.Literal l) {
    if (!(l instanceof Expression.FixedCharLiteral)) {
      logger.warn("Literal expected to be char type but was not. {}", l);
      return Optional.empty();
    }

    return Optional.of(((Expression.FixedCharLiteral) l).value());
  }

}
