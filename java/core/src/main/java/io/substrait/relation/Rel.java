package io.substrait.relation;

import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.immutables.value.Value;

import java.util.List;
import java.util.Optional;
import java.util.stream.IntStream;

public interface Rel {
  Optional<Remap> getRemap();
  Type.Struct getRecordType();
  List<Rel> getInputs();

  @Value.Immutable
  public static abstract class Remap {
    public abstract List<Integer> indices();

    public Type.Struct remap(Type.Struct initial) {
      List<Type> types = initial.fields();
      return TypeCreator.of(initial.nullable())
          .struct(indices().stream().map(i -> types.get(i)));
    }

    public static Remap of(Iterable<Integer> fields) {
      return ImmutableRemap.builder().addAllIndices(fields).build();
    }

    public static Remap offset(int start, int length) {
      return of(IntStream.range(start, start + length).mapToObj(i -> i).toList());
    }
  }

  <O, E extends Exception> O accept(RelVisitor<O, E> visitor) throws E;
}
