package io.substrait.relation;

import io.substrait.type.Type;
import io.substrait.util.Util;

import java.util.function.Supplier;

public abstract class AbstractRel implements Rel {

  private Supplier<Type.Struct> recordType = Util.memoize(() -> {
    Type.Struct s = deriveRecordType();
    return getRemap().map(r -> r.remap(s)).orElse(s);
  });

  protected abstract Type.Struct deriveRecordType();

  @Override
  public final Type.Struct getRecordType() {
    return recordType.get();
  }

}
