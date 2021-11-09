package io.substrait.relation;

import io.substrait.expression.Expression;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.immutables.value.Value;

import java.util.List;
import java.util.stream.Stream;

@Value.Immutable
public abstract class Filter extends SingleInputRel {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(Filter.class);

  public abstract Expression getCondition();

  @Override
  protected Type.Struct deriveRecordType() {
    return getInput().getRecordType();
  }

  @Override
  public <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E {
    return visitor.visit(this);
  }

  public static ImmutableFilter.Builder builder() {
    return ImmutableFilter.builder();
  }
}
