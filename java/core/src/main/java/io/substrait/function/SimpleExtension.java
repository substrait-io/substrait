package io.substrait.function;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import io.substrait.expression.Expression;
import io.substrait.type.Deserializers;
import io.substrait.type.Type;
import io.substrait.type.TypeExpressionEvaluator;
import io.substrait.util.Util;
import org.immutables.value.Value;

import javax.annotation.Nullable;
import java.io.IOException;
import java.io.InputStream;
import java.util.*;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * Classes used to deserialize YAML extension files. Currently, constrained to Function deserialization.
 */
@Value.Enclosing
public class SimpleExtension {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(SimpleExtension.class);

  private static final ObjectMapper MAPPER = new ObjectMapper(new YAMLFactory())
      .enable(DeserializationFeature.ACCEPT_SINGLE_VALUE_AS_ARRAY)
      .registerModule(new Jdk8Module())
      .registerModule(Deserializers.MODULE);

  enum Nullability {MIRROR, DECLARED_OUTPUT, DISCRETE}
  enum Decomposability {NONE, ONE, MANY}

  private SimpleExtension(){}

  @JsonTypeInfo(use = JsonTypeInfo.Id.DEDUCTION)
  @JsonSubTypes({@JsonSubTypes.Type(ValueArgument.class), @JsonSubTypes.Type(TypeArgument.class), @JsonSubTypes.Type(EnumArgument.class)})
  public interface Argument {
    String toTypeString();
    boolean required();

  }
  public record ValueArgument(@JsonProperty(required = true) ParameterizedType value, String name, boolean constant) implements Argument {
    public String toTypeString() {
      return value.accept(ToTypeString.INSTANCE);
    }

    public boolean required() {
      return true;
    }
  }
  public record TypeArgument(ParameterizedType type, String name) implements Argument {
    public String toTypeString() {
      return "type";
    }

    public boolean required() {
      return true;
    }

  }

  public record EnumArgument(List<String> options, String name, boolean required) implements Argument {
    public String toTypeString() {
      return required ? "req" : "opt";
    }
  }

  @Value.Immutable
  public interface FunctionAnchor {
    String namespace();
    String key();

    static FunctionAnchor of(String namespace, String key) {
      return ImmutableSimpleExtension.FunctionAnchor.builder().namespace(namespace).key(key).build();
    }
  }

  @JsonDeserialize(as = ImmutableSimpleExtension.VariadicBehavior.class)
  @JsonSerialize(as = ImmutableSimpleExtension.VariadicBehavior.class)
  @Value.Immutable
  public interface VariadicBehavior {
    int getMin();
    OptionalInt getMax();
    enum ParameterConsistency {CONSISTENT, INCONSISTENT}

    default ParameterConsistency parameterConsistency() {
      return ParameterConsistency.CONSISTENT;
    }
  }

  public static abstract class Function {
    @Value.Default
    public String name() {
      // we can't use null detection here since we initially construct this with a parent name.
      return "";
    }

    @Value.Default
    public String uri() {
      // we can't use null detection here since we initially construct this without a uri, then resolve later.
      return "";
    }

    public abstract Optional<VariadicBehavior> variadic();

    @Value.Default
    @Nullable public String description() {
      return "";
    }

    public abstract List<Argument> args();

    public List<Argument> requiredArguments() {
      return requiredArgsSupplier.get();
    }

    @Override
    public String toString() {
      return key();
    }

    @Value.Default
    public Nullability nullability() {
      return Nullability.MIRROR;
    }

    public FunctionAnchor getAnchor() {
      return anchorSupplier.get();
    }

    @JsonProperty(value = "return")
    public abstract TypeExpression returnType();

    private final Supplier<FunctionAnchor> anchorSupplier = Util.memoize(() -> FunctionAnchor.of(uri(), key()));
    private final Supplier<String> keySupplier = Util.memoize(() -> constructKey(name(), args()));
    private final Supplier<List<Argument>> requiredArgsSupplier = Util.memoize(() -> {
      return args().stream().filter(Argument::required).toList();
    });

    public static String constructKeyFromTypes(String name, List<Type> arguments) {
      try {
        return name + ":" + arguments.stream()
            .map(t -> t.accept(ToTypeString.INSTANCE))
            .collect(Collectors.joining("_"));
      } catch (UnsupportedOperationException ex) {
        throw new UnsupportedOperationException(String.format("Failure converting types of function %s.", name), ex);
      }
    }
    public static String constructKey(String name, List<Argument> arguments) {
      try {
        return name + ":" + arguments.stream()
            .map(Argument::toTypeString)
            .collect(Collectors.joining("_"));
      } catch (UnsupportedOperationException ex) {
        throw new UnsupportedOperationException(String.format("Failure converting types of function %s.", name), ex);
      }
    }

    public Util.IntRange getRange() {
      // end range is exclusive so add one to size.

      long optionalCount = args().stream().filter(t -> !t.required()).count();
      int max = variadic()
          .map(t -> t.getMax().stream().map(x -> args().size() - 1 + x + 1)
          .findFirst()
              .orElse(Integer.MAX_VALUE)).orElse(args().size() + 1);
      int min = variadic()
          .map(t -> args().size() - 1 + t.getMin()).orElse(requiredArguments().size());
      return Util.IntRange.of(min, max);
    }

    public void validateOutputType(List<Expression> argumentExpressions, Type outputType) {
      // TODO: support advanced output type validation using return expressions, parameters, etc.
      // The code below was too restrictive in the case of nullability conversion.
      return;
//      boolean makeNullable = nullability() == Nullability.MIRROR &&
//          argumentExpressions.stream().filter(e -> e.getType().nullable()).findFirst().isPresent();
//      if (returnType() instanceof Type && !outputType.equals(returnType())) {
//
//        throw new IllegalArgumentException(String.format("Output type of %s doesn't match expected output type of %s for %s.", outputType, returnType(), this.key()));
//      }
    }

    public String key() {
      return keySupplier.get();
    }

    public Type resolveType(List<Type> argumentTypes) {
      return TypeExpressionEvaluator.evaluateExpression(returnType(), args(), argumentTypes);
    }

  }

  @JsonDeserialize(as = ImmutableSimpleExtension.ScalarFunction.class)
  @JsonSerialize(as = ImmutableSimpleExtension.ScalarFunction.class)
  @Value.Immutable
  public static abstract class ScalarFunction {
    public abstract String name();
    @Nullable public abstract String description();
    public abstract List<ScalarFunctionVariant> impls();

    Stream<ScalarFunctionVariant> resolve(String uri) {
      return impls().stream().map(f -> f.resolve(uri, name(), description()));
    }
  }

  @JsonDeserialize(as = ImmutableSimpleExtension.ScalarFunctionVariant.class)
  @JsonSerialize(as = ImmutableSimpleExtension.ScalarFunctionVariant.class)
  @Value.Immutable
  public static abstract class ScalarFunctionVariant extends Function {
    public ScalarFunctionVariant resolve(String uri, String name, String description) {
      return ImmutableSimpleExtension.ScalarFunctionVariant.builder()
          .uri(uri)
          .name(name)
          .description(description)
          .nullability(nullability())
          .args(args())
          .variadic(variadic())
          .returnType(returnType())
          .build();
    }

  }

  @JsonDeserialize(as = ImmutableSimpleExtension.AggregateFunction.class)
  @JsonSerialize(as = ImmutableSimpleExtension.AggregateFunction.class)
  @Value.Immutable
  public static abstract class AggregateFunction {
    @Nullable public abstract String name();

    @Nullable public abstract String description();
    public abstract List<AggregateFunctionVariant> impls();

    public Stream<AggregateFunctionVariant> resolve(String uri) {
      return impls().stream().map(f -> f.resolve(uri, name(), description()));
    }
  }

  @JsonDeserialize(as = ImmutableSimpleExtension.AggregateFunctionVariant.class)
  @JsonSerialize(as = ImmutableSimpleExtension.AggregateFunctionVariant.class)
  @Value.Immutable
  public static abstract class AggregateFunctionVariant extends Function {
    @Value.Default
    @JsonProperty("decomposable")
    public Decomposability decomposability() {
      return Decomposability.NONE;
    }

    @Override
    public String toString() {
      return super.toString();
    }

    public abstract TypeExpression intermediate();

    AggregateFunctionVariant resolve(String uri, String name, String description) {
      return ImmutableSimpleExtension.AggregateFunctionVariant.builder()
          .uri(uri)
          .name(name)
          .description(description)
          .nullability(nullability())
          .args(args())
          .variadic(variadic())
          .decomposability(decomposability())
          .intermediate(intermediate())
          .returnType(returnType())
          .build();
    }
  }

  @JsonDeserialize(as = ImmutableSimpleExtension.FunctionSignatures.class)
  @JsonSerialize(as = ImmutableSimpleExtension.FunctionSignatures.class)
  @Value.Immutable
  public static abstract class FunctionSignatures {
    @JsonProperty("scalar_functions") public abstract List<ScalarFunction> scalars();
    @JsonProperty("aggregate_functions") public abstract List<AggregateFunction> aggregates();

    public int size() {
      return (scalars() == null ? 0 : scalars().size()) +
          (aggregates() == null ? 0 : aggregates().size());
    }

    public Stream<SimpleExtension.Function> resolve(String uri) {
      return Stream.concat(
          scalars() == null ? Stream.of() : scalars().stream().flatMap(f -> f.resolve(uri)),
          aggregates() == null ? Stream.of() : aggregates().stream().flatMap(f -> f.resolve(uri))
          );
    }

  }

  @Value.Immutable
  public abstract static class ExtensionCollection {
    public abstract List<ScalarFunctionVariant> scalarFunctions();
    public abstract List<AggregateFunctionVariant> aggregateFunctions();

    private final Supplier<Set<String>> namespaceSupplier = Util.memoize(() -> {
      return Stream.concat(scalarFunctions().stream().map(Function::uri),
          aggregateFunctions().stream().map(Function::uri)).collect(Collectors.toSet());
    });
    private final Supplier<Map<FunctionAnchor, ScalarFunctionVariant>> scalarFunctionsLookup = Util.memoize(() -> {
      return scalarFunctions().stream().collect(Collectors.toMap(Function::getAnchor, java.util.function.Function.identity()));
    });

    private final Supplier<Map<FunctionAnchor, AggregateFunctionVariant>> aggregateFunctionsLookup = Util.memoize(() -> {
      return aggregateFunctions().stream().collect(Collectors.toMap(Function::getAnchor, java.util.function.Function.identity()));
    });

    public ScalarFunctionVariant getScalarFunction(FunctionAnchor anchor) {
       ScalarFunctionVariant variant = scalarFunctionsLookup.get().get(anchor);
       if (variant != null) {
         return variant;
       }
      checkNamespace(anchor.namespace());
      throw new IllegalArgumentException(String.format("Unexpected scalar function with key %s. The namespace %s is loaded " +
          "but no scalar function with this key found.", anchor.key(), anchor.namespace()));
    }

    private void checkNamespace(String name) {
      if (namespaceSupplier.get().contains(name)) {
        return;
      }

      throw new IllegalArgumentException(String.format("Received a reference for extension %s " +
          "but that extension is not currently loaded.", name));
    }

    public AggregateFunctionVariant getAggregateFunction(FunctionAnchor anchor) {
      var variant = aggregateFunctionsLookup.get().get(anchor);
      if (variant != null) {
        return variant;
      }

      checkNamespace(anchor.namespace());
      throw new IllegalArgumentException(String.format("Unexpected aggregate function with key %s. The namespace %s is loaded " +
          "but no aggregate function with this key was found.", anchor.key(), anchor.namespace()));
    }

    public ExtensionCollection merge(ExtensionCollection extensionCollection) {
      return ImmutableSimpleExtension.ExtensionCollection.builder()
          .addAllAggregateFunctions(aggregateFunctions()).addAllAggregateFunctions(extensionCollection.aggregateFunctions())
          .addAllScalarFunctions(scalarFunctions()).addAllScalarFunctions(extensionCollection.scalarFunctions())
          .build();
    }
  }

  public static ExtensionCollection loadDefaults() throws IOException {
    var defaultFiles = Arrays.asList(
        "boolean", "aggregate_generic", "arithmetic_decimal", "arithmetic", "comparison", "datetime", "string"
    ).stream().map(c -> String.format("/functions_%s.yaml", c)).toList();

    return load(defaultFiles);
  }

  public static ExtensionCollection load(List<String> resourcePaths) throws IOException {
    if (resourcePaths.isEmpty()) {
      throw new IllegalArgumentException("Require at least one resource path.");
    }

    var extensions = resourcePaths.stream().map(path -> {
      try(var stream = ExtensionCollection.class.getResourceAsStream(path)) {
        return load(path, stream);
      } catch (IOException e) {
        throw new RuntimeException(e);
      }
    }).toList();
    ExtensionCollection complete = extensions.get(0);
    for (int i = 1; i < extensions.size(); i++) {
      complete = complete.merge(extensions.get(i));
    }
    return complete;
  }

  private static ExtensionCollection load(String namespace, InputStream stream) {
    try {
      var doc = MAPPER.readValue(stream, SimpleExtension.FunctionSignatures.class);
      var collection = ImmutableSimpleExtension.ExtensionCollection.builder()
          .addAllAggregateFunctions(doc.aggregates().stream().flatMap(t -> t.resolve(namespace)).toList())
          .addAllScalarFunctions(doc.scalars().stream().flatMap(t -> t.resolve(namespace)).toList())
          .build();
      logger.debug("Loaded {} aggregate functions and {} scalar functions from {}.", collection.aggregateFunctions().size(), collection.scalarFunctions().size(), namespace);
      return collection;
    } catch (RuntimeException ex) {
      throw ex;
    } catch (Exception ex) {
      throw new RuntimeException("Failure while parsing " + namespace, ex);
    }
  }
}
