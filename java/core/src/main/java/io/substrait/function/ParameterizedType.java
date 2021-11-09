package io.substrait.function;

import io.substrait.type.TypeVisitor;
import org.immutables.value.Value;

import java.util.Locale;

/**
 * Types used in function argument declarations. Can utilize strings for integer or type parameters.
 */
@Value.Enclosing
public interface ParameterizedType extends TypeExpression {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ParameterizedType.class);

  static class RequiredParameterizedVisitorException extends RuntimeException {
    @Override
    public synchronized Throwable fillInStackTrace() {
      return this;
    }
  }

  <R, E extends Throwable> R accept(final TypeVisitor<R, E> typeVisitor) throws E;


  public static ParameterizedTypeCreator withNullability(boolean nullable) {
    return nullable ? ParameterizedTypeCreator.NULLABLE : ParameterizedTypeCreator.REQUIRED;
  }

  interface NullableParameterizedType extends ParameterizedType {
    boolean nullable();
  }

  default boolean isWildcard() {
    return false;
  }

  static abstract class BaseParameterizedType implements ParameterizedType {
    public final <R, E extends Throwable> R accept(final TypeVisitor<R, E> typeVisitor) throws E {
      if (typeVisitor instanceof ParameterizedTypeVisitor) {
        return accept((ParameterizedTypeVisitor<R, E>) typeVisitor);
      }
      throw new RequiredParameterizedVisitorException();
    }
    abstract <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E;
  }

  @Value.Immutable
  static abstract class FixedChar extends BaseParameterizedType implements NullableType {
    public abstract StringLiteral length();

    public static ImmutableParameterizedType.FixedChar.Builder builder() {
      return ImmutableParameterizedType.FixedChar.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class VarChar extends BaseParameterizedType implements NullableType {
    public abstract StringLiteral length();

    public static ImmutableParameterizedType.VarChar.Builder builder() {
      return ImmutableParameterizedType.VarChar.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class FixedBinary extends BaseParameterizedType implements NullableType {
    public abstract StringLiteral length();

    public static ImmutableParameterizedType.FixedBinary.Builder builder() {
      return ImmutableParameterizedType.FixedBinary.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class Decimal extends BaseParameterizedType implements NullableType {
    public abstract StringLiteral scale();

    public abstract StringLiteral precision();

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
    public static ImmutableParameterizedType.Decimal.Builder builder() {
      return ImmutableParameterizedType.Decimal.builder();
    }
  }

  @Value.Immutable
  static abstract class Struct extends BaseParameterizedType implements NullableType {
    public abstract java.util.List<ParameterizedType> fields();

    public static ImmutableParameterizedType.Struct.Builder builder() {
      return ImmutableParameterizedType.Struct.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class ListType extends BaseParameterizedType implements NullableType {
    public abstract ParameterizedType name();

    public static ImmutableParameterizedType.ListType.Builder builder() {
      return ImmutableParameterizedType.ListType.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class Map extends BaseParameterizedType implements NullableType {
    public abstract ParameterizedType key();

    public abstract ParameterizedType value();

    public static ImmutableParameterizedType.Map.Builder builder() {
      return ImmutableParameterizedType.Map.builder();
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

  @Value.Immutable
  static abstract class StringLiteral extends BaseParameterizedType {
    public abstract String value();

    public static ImmutableParameterizedType.StringLiteral.Builder builder() {
      return ImmutableParameterizedType.StringLiteral.builder();
    }

    @Override
    public boolean isWildcard() {
      return value().toLowerCase(Locale.ROOT).startsWith("any");
    }

    @Override
    <R, E extends Throwable> R accept(final ParameterizedTypeVisitor<R, E> parameterizedTypeVisitor) throws E {
      return parameterizedTypeVisitor.visit(this);
    }
  }

}
