package io.substrait.relation;

import io.substrait.expression.Expression;
import io.substrait.proto.JoinRel;
import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.immutables.value.Value;

import java.util.Optional;
import java.util.stream.Stream;

@Value.Immutable
public abstract class Join extends BiRel {

  public abstract Optional<Expression> getCondition();

  public abstract Optional<Expression> getPostJoinFilter();

  public abstract JoinType getJoinType();

  public static enum JoinType {
    UNKNOWN(JoinRel.JoinType.JOIN_TYPE_UNSPECIFIED),
    INNER(JoinRel.JoinType.JOIN_TYPE_INNER),
    OUTER(JoinRel.JoinType.JOIN_TYPE_OUTER),
    LEFT(JoinRel.JoinType.JOIN_TYPE_LEFT),
    RIGHT(JoinRel.JoinType.JOIN_TYPE_RIGHT),
    SEMI(JoinRel.JoinType.JOIN_TYPE_SEMI),
    ANTI(JoinRel.JoinType.JOIN_TYPE_ANTI);

    private JoinRel.JoinType proto;

    JoinType(JoinRel.JoinType proto) {
      this.proto = proto;
    }

    public JoinRel.JoinType toProto() {
      return proto;
    }
  }

  @Override
  protected Type.Struct deriveRecordType() {
    return TypeCreator.REQUIRED.struct(Stream.concat(
        getLeft().getRecordType().fields().stream(),
        getRight().getRecordType().fields().stream()
    ));
  }

  @Override
  public <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E {
    return visitor.visit(this);
  }

  public static ImmutableJoin.Builder builder() {
    return ImmutableJoin.builder();
  }
}
