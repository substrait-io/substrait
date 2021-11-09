package io.substrait.relation;

import java.util.Collections;
import java.util.List;

public abstract class ZeroInputRel extends AbstractRel {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ZeroInputRel.class);

  @Override
  public final List<Rel> getInputs() {
    return Collections.emptyList();
  }
}
