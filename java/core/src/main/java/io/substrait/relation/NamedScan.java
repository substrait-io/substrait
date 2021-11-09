package io.substrait.relation;

import io.substrait.io.substrait.extension.AdvancedExtension;
import org.immutables.value.Value;

import java.util.List;
import java.util.Optional;

@Value.Immutable
public abstract class NamedScan extends AbstractReadRel {

  public abstract List<String> getNames();

  public abstract Optional<AdvancedExtension> getExtension();

  @Override
  public <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E {
    return visitor.visit(this);
  }

  public static ImmutableNamedScan.Builder builder() {
    return ImmutableNamedScan.builder();
  }
}
