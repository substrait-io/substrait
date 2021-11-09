package io.substrait.relation;

import java.util.Arrays;
import java.util.List;

public abstract class BiRel extends AbstractRel {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(BiRel.class);

  public abstract Rel getLeft();
  public abstract Rel getRight();

  @Override
  public final List<Rel> getInputs() {
    return Arrays.asList(getLeft(), getRight());
  }
}
