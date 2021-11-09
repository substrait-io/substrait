package io.substrait.expression;

import io.substrait.relation.Rel;
import io.substrait.type.Type;
import io.substrait.type.TypeVisitor;
import org.immutables.value.Value;

import java.util.Collections;
import java.util.List;
import java.util.Optional;

@Value.Immutable
public abstract class FieldReference implements Expression {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(FieldReference.class);

  public abstract List<ReferenceSegment> segments();

  public abstract Type type();

  public abstract Optional<Expression> inputExpression();

  public Type getType() {return type();}

  public <R, E extends Throwable> R accept(ExpressionVisitor<R, E> visitor) throws E {
    return visitor.visit(this);
  }

  public boolean isSimpleRootReference() {
    return segments().size() == 1 && inputExpression().isEmpty();
  }

  public FieldReference dereferenceStruct(int index) {
    Type newType = StructFieldFinder.getReferencedType(type(), index);
    return dereference(newType, StructField.of(index));
  }

  private FieldReference dereference(Type newType, ReferenceSegment nextSegment) {
    return ImmutableFieldReference.builder()
        .type(newType)
        .addAllSegments(segments())
        .addSegments(nextSegment)
        .inputExpression(inputExpression())
        .build();
  }

  public FieldReference dereferenceList(int index) {
    Type newType = ListIndexFinder.getReferencedType(type(), index);
    return dereference(newType, ListElement.of(index));
  }

  public FieldReference dereferenceMap(Literal mapKey) {
    Type newType = MapKeyFinder.getReferencedType(type(), mapKey.getType());
    return dereference(newType, MapKey.of(mapKey));
  }

  public static FieldReference newMapReference(Literal mapKey, Expression expression) {
    return ImmutableFieldReference.builder()
        .addSegments(MapKey.of(mapKey))
        .inputExpression(expression)
        .type(MapKeyFinder.getReferencedType(expression.getType(), mapKey.getType()))
        .build();
  }

  public static FieldReference newListReference(int index, Expression expression) {
    return ImmutableFieldReference.builder()
        .addSegments(ListElement.of(index))
        .inputExpression(expression)
        .type(ListIndexFinder.getReferencedType(expression.getType(), index))
        .build();
  }

  public static FieldReference newStructReference(int index, Expression expression) {
    return ImmutableFieldReference.builder()
        .addSegments(StructField.of(index))
        .inputExpression(expression)
        .type(StructFieldFinder.getReferencedType(expression.getType(), index))
        .build();
  }

  public static FieldReference newRootStructReference(int index, Type knownType) {
    return ImmutableFieldReference.builder()
        .addSegments(StructField.of(index))
        .type(knownType)
        .build();
  }

  public static FieldReference newInputRelReference(int index, Rel rel) {
    return newInputRelReference(index, Collections.singletonList(rel));
  }

  public static FieldReference newInputRelReference(int index, List<Rel> rels) {
    int currentOffset = 0;
    for (Rel r : rels) {
      int relSize = r.getRecordType().fields().size();
      if (index < currentOffset + relSize) {
        Type referenceType = r.getRecordType().fields().get(index - currentOffset);
        return ImmutableFieldReference.builder()
            .addSegments(StructField.of(index))
            .type(referenceType)
            .build();
      }

      currentOffset += relSize;
    }

    throw new IllegalArgumentException(String.format("The current index %d wasn't found within the number of fields %d", index, currentOffset));
  }

  public interface ReferenceSegment {
    FieldReference apply(FieldReference reference);
    FieldReference constructOnExpression(Expression expr);
    FieldReference constructOnRoot(Type type);
  }

  @Value.Immutable public static abstract class StructField implements ReferenceSegment {
    public abstract int offset();

    public static StructField of(int index) {
      return ImmutableStructField.builder().offset(index).build();
    }

    @Override
    public FieldReference apply(FieldReference reference) {
      return reference.dereferenceStruct(offset());
    }

    @Override
    public FieldReference constructOnExpression(Expression expr) {
      return FieldReference.newStructReference(offset(), expr);
    }

    @Override
    public FieldReference constructOnRoot(Type type) {
      return FieldReference.newRootStructReference(offset(), type);
    }
  }

  @Value.Immutable public static abstract class ListElement implements ReferenceSegment {
    public abstract int offset();

    public static ListElement of(int index) {
      return ImmutableListElement.builder().offset(index).build();
    }

    @Override
    public FieldReference apply(FieldReference reference) {
      return reference.dereferenceList(offset());
    }

    @Override
    public FieldReference constructOnExpression(Expression expr) {
      return FieldReference.newListReference(offset(), expr);
    }

    @Override
    public FieldReference constructOnRoot(Type type) {
      throw new UnsupportedOperationException();
    }
  }

  @Value.Immutable public static abstract class MapKey implements ReferenceSegment {
    public abstract Expression.Literal key();

    public static MapKey of(Expression.Literal key) {
      return ImmutableMapKey.builder().key(key).build();
    }

    @Override
    public FieldReference apply(FieldReference reference) {
      return reference.dereferenceMap(key());
    }

    @Override
    public FieldReference constructOnExpression(Expression expr) {
      return FieldReference.newMapReference(key(), expr);
    }

    @Override
    public FieldReference constructOnRoot(Type type) {
      throw new UnsupportedOperationException();
    }
  }

  public static FieldReference ofExpression(Expression expression, List<ReferenceSegment> segments) {
    return of(null, expression, segments);
  }

  private static FieldReference of(Type type, Expression expression, List<ReferenceSegment> segments) {
    FieldReference reference = null;
    Collections.reverse(segments);
    for (int i =0; i < segments.size(); i++) {
      if (i == 0) {
        var last = segments.get(0);
        reference = type == null ? last.constructOnExpression(expression) : last.constructOnRoot(type);
      } else {
        reference = segments.get(i).apply(reference);
      }
    }

    return reference;
  }

  public static FieldReference ofRoot(Type type, List<ReferenceSegment> segments) {
    return of(type, null, segments);
  }

  private static class StructFieldFinder extends TypeVisitor.TypeThrowsVisitor<Type, RuntimeException> {

    private final int index;

    private StructFieldFinder(int index) {
      super("This visitor only supports retrieving struct types. Was applied to a non-struct type.");
      this.index = index;
    }

    @Override
    public Type visit(Type.Struct expr) throws RuntimeException {
      if (expr.fields().size() < index) {
        throw new IllegalArgumentException("Undefined struct type.");
      }
      return expr.fields().get(index);
    }

    public static Type getReferencedType(Type type, int index) {
      return type.accept(new StructFieldFinder(index));
    }

  }

  private static class ListIndexFinder extends TypeVisitor.TypeThrowsVisitor<Type, RuntimeException> {

    private final int index;

    private ListIndexFinder(int index) {
      super("This visitor only supports retrieving array index offsets. Was applied to a non-array type.");
      this.index = index;
    }

    @Override
    public Type visit(Type.ListType expr) throws RuntimeException {
      return expr.elementType();
    }

    public static Type getReferencedType(Type type, int index) {
      return type.accept(new ListIndexFinder(index));
    }

  }

  private static class MapKeyFinder extends TypeVisitor.TypeThrowsVisitor<Type, RuntimeException> {

    private final Type keyType;

    private MapKeyFinder(Type keyType) {
      super("This visitor only supports retrieving map values using map keys. Was applied to a non-map type.");
      this.keyType = keyType;
    }

    @Override
    public Type visit(Type.Map expr) throws RuntimeException {
      // TODO: decide whether to support inconsistent map key type literals. Inclined to not.
      if (!(expr.key().equals(keyType))) {
        throw new IllegalArgumentException(String.format("Key type %s of map does not matched expected type of %s.", expr.key(), keyType));
      }
      return expr.value();
    }

    public static Type getReferencedType(Type typeToDereference, Type keyType) {
      return typeToDereference.accept(new MapKeyFinder(keyType));
    }

  }

}
