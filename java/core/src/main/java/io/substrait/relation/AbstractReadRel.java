package io.substrait.relation;

import io.substrait.expression.Expression;
import io.substrait.io.substrait.extension.AdvancedExtension;
import io.substrait.type.NamedStruct;
import io.substrait.type.Type;

import java.util.Optional;

public abstract class AbstractReadRel extends ZeroInputRel {

  public abstract NamedStruct getInitialSchema();

  public abstract Optional<Expression> getFilter();

  //TODO:
  //public abstract Optional<MaskExpression>

  public abstract Optional<AdvancedExtension> getGeneralExtension();

  @Override
  protected final Type.Struct deriveRecordType() {
    return getInitialSchema().struct();
  }
}
