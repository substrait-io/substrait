package io.substrait.relation;

import io.substrait.expression.AggregateFunctionInvocation;
import io.substrait.expression.Expression;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.immutables.value.Value;

import java.util.LinkedHashSet;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

@Value.Immutable
public abstract class Aggregate extends SingleInputRel {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(Aggregate.class);

  public abstract List<Grouping> getGroupings();

  public abstract List<Measure> getMeasures();

  @Override
  protected Type.Struct deriveRecordType() {
    return TypeCreator.REQUIRED.struct(
      Stream.concat(
          // unique grouping expressions
          getGroupings().stream()
              .flatMap(g -> g.getExpressions().stream())
              .collect(Collectors.toCollection(LinkedHashSet::new))
              .stream().map(Expression::getType),

          // measures
          getMeasures().stream().map(t -> t.getFunction().getType())
          ));
  }

  @Override
  public <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E {
    return visitor.visit(this);
  }

  @Value.Immutable
  public static abstract class Grouping {
    public abstract List<Expression> getExpressions();

    public static ImmutableGrouping.Builder builder() {
      return ImmutableGrouping.builder();
    }
  }

  @Value.Immutable
  public static abstract class Measure {
    public abstract AggregateFunctionInvocation getFunction();
    public abstract Optional<Expression> getPreMeasureFilter();

    public static ImmutableMeasure.Builder builder() {
      return ImmutableMeasure.builder();
    }
  }

  public static ImmutableAggregate.Builder builder() {
    return ImmutableAggregate.builder();
  }
}
