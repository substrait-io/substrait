package io.substrait.relation;

import io.substrait.expression.Expression;
import io.substrait.type.NamedStruct;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.immutables.value.Value;

import java.util.List;

@Value.Immutable
public abstract class VirtualTableScan extends AbstractReadRel {

  public abstract List<String> getDfsNames();
  public abstract List<Expression.StructLiteral> getRows();

  @Override
  public final NamedStruct getInitialSchema() {
    Type.Struct struct = TypeCreator.REQUIRED.struct(
        getRows().stream().map(Expression::getType));

    return NamedStruct.of(getDfsNames(), struct);
  }

  @Override
  public <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E {
    return visitor.visit(this);
  }

  public static ImmutableVirtualTableScan.Builder builder() {
    return ImmutableVirtualTableScan.builder();
  }
}
