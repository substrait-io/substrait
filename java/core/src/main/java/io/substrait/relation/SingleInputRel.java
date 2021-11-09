package io.substrait.relation;

import java.util.Collections;
import java.util.List;

public abstract class SingleInputRel extends AbstractRel {

  public abstract Rel getInput();

  @Override
  public final List<Rel> getInputs() {
    return Collections.singletonList(getInput());
  }

}
