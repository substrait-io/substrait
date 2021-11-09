package io.substrait.isthmus.expression;

import com.google.common.collect.*;
import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.function.SimpleExtension;
import io.substrait.function.ParameterizedType;
import io.substrait.isthmus.TypeConverter;
import io.substrait.type.Type;
import io.substrait.util.Util;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexBuilder;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.sql.SqlOperator;


import java.util.*;
import java.util.function.Function;
import java.util.stream.Stream;

abstract class FunctionConverter<F extends SimpleExtension.Function, T, C extends FunctionConverter.GenericCall> {

  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(FunctionConverter.class);

  protected final Map<SqlOperator, FunctionFinder> signatures;
  protected final RelDataTypeFactory typeFactory;
  protected final RexBuilder rexBuilder;

  public FunctionConverter(List<F> functions, RelDataTypeFactory typeFactory) {
    this(functions, Collections.EMPTY_LIST, typeFactory);
  }

  public FunctionConverter(List<F> functions, List<FunctionMappings.Sig> additionalSignatures, RelDataTypeFactory typeFactory) {
    rexBuilder = new RexBuilder(typeFactory);
    var signatures = new ArrayList<FunctionMappings.Sig>(getSigs().size() + additionalSignatures.size());
    signatures.addAll(additionalSignatures);
    signatures.addAll(getSigs());
    this.typeFactory = typeFactory;

    var alm = ArrayListMultimap.<String, F>create();
    for (var f : functions) {
      alm.put(f.name().toLowerCase(Locale.ROOT), f);
    }

    ListMultimap<String, FunctionMappings.Sig> calciteOperators = signatures.stream().collect(Multimaps.toMultimap(FunctionMappings.Sig::name, f->f, () -> ArrayListMultimap.create()));
    var matcherMap = new IdentityHashMap<SqlOperator, FunctionFinder>();
    for (String key : alm.keySet()) {
      var sigs = calciteOperators.get(key);
      if (sigs == null) {
        logger.info("Dropping function due to no binding: {}", key);
        continue;
      }

      for(var sig : sigs) {
        var implList = alm.get(key);
        if (implList == null || implList.isEmpty()) {
          continue;
        }

        matcherMap.put(sig.operator(), new FunctionFinder(key, sig.operator(), implList));

      }
    }

    this.signatures = matcherMap;
  }



  protected abstract ImmutableList<FunctionMappings.Sig> getSigs();

  protected class FunctionFinder {
    private final String name;
    private final SqlOperator operator;
    private final List<F> functions;
    private final Map<String, F> directMap;
    private final SignatureMatcher<F> matcher;
    private final Optional<SingularArgumentMatcher<F>> singularInputType;
    private final Util.IntRange argRange;

    public FunctionFinder(String name, SqlOperator operator, List<F> functions) {
      this.name = name;
      this.operator = operator;
      this.functions = functions;
      this.argRange = Util.IntRange.of(
          functions.stream().mapToInt(t -> t.getRange().getStartInclusive()).min().getAsInt(),
          functions.stream().mapToInt(t -> t.getRange().getEndExclusive()).max().getAsInt());
      this.matcher = getSignatureMatcher(operator, functions);
      this.singularInputType = getSingularInputType(functions);
      var directMap = ImmutableMap.<String, F>builder();
      for (var func : functions) {
        String key = func.key();
        directMap.put(key, func);
        if (func.requiredArguments().size() != func.args().size()) {
          directMap.put(F.constructKey(name, func.requiredArguments()), func);
        }
      }
      this.directMap = directMap.build();
    }

    public boolean allowedArgCount(int count) {
      return argRange.within(count);
    }

    private static <F extends SimpleExtension.Function> SignatureMatcher<F> getSignatureMatcher(
        SqlOperator operator,
        List<F> functions) {
      // TODO: define up-converting matchers.
      return (a, b) -> Optional.empty();
    }

    /**
     * If some of the function variants for this function name have single, repeated argument type,
     * we will attempt to find matches using these patterns and least-restrictive casting.
     *
     * If this exists, the function finder will attempt to find a least-restrictive match using these.
     */
    private static <F extends SimpleExtension.Function> Optional<SingularArgumentMatcher<F>> getSingularInputType(List<F> functions) {
      List<SingularArgumentMatcher> matchers = new ArrayList<>();
      for (var f : functions) {

        // no need to do optional requirements since singular input only supports value arguments.
        if (f.getRange().getStartInclusive() < 2) {
          continue;
        }

        ParameterizedType firstType = null;

        // determine if all the required arguments are the of the same type. If so,
        for (var a : f.requiredArguments()) {
          if (!(a instanceof SimpleExtension.ValueArgument)) {
            firstType = null;
            break;
          }

          var pt = ((SimpleExtension.ValueArgument) a).value();

          if (firstType == null) {
            firstType = pt;
          } else {
            // TODO: decide if this is too lenient.
            if (!isMatch(firstType, pt)) {
              firstType = null;
              break;
            }
          }
        }


        if (firstType != null) {
          matchers.add(singular(f, firstType));
        }

      }

      return switch(matchers.size()) {
        case 0 -> Optional.empty();
        case 1 -> Optional.of(matchers.get(0));
        default -> Optional.of(chained(matchers));
      };
    }

    public static <F extends SimpleExtension.Function> SingularArgumentMatcher<F> singular(F function, ParameterizedType type) {
      return (inputType, outputType) -> {
        var check = isMatch(inputType, type);
        if (check) {
          return Optional.of(function);
        }
        return Optional.empty();
      };
    }

    public static SingularArgumentMatcher chained(List<SingularArgumentMatcher> matchers) {
      return (inputType, outputType) -> {
        for (var s : matchers) {
          var outcome = s.tryMatch(inputType, outputType);
          if (outcome.isPresent()) {
            return outcome;
          }
        }

        return Optional.empty();
      };
    }

    public Optional<T> attemptMatch(C call, Function<RexNode, Expression> topLevelConverter) {

      var operands = call.getOperands().map(topLevelConverter).toList();
      var opTypes = operands.stream().map(Expression::getType).toList();

      var outputType = TypeConverter.convert(call.getType());

      // try to do a direct match
      var directMatchkey = F.constructKeyFromTypes(name, opTypes);
      var variant = directMap.get(directMatchkey);
      if (variant != null) {
        variant.validateOutputType(operands, outputType);
        return Optional.of(generateBinding(call, variant, operands, outputType));
      }


      if (singularInputType.isPresent()) {
        RelDataType leastRestrictive = typeFactory.leastRestrictive(call.getOperands().map(RexNode::getType).toList());
        if (leastRestrictive == null) {
          return Optional.empty();
        }
        Type type = TypeConverter.convert(leastRestrictive);
        var out = singularInputType.get().tryMatch(type, outputType);

        if (out.isPresent()) {
          var declaration = out.get();
          var coercedArgs = coerceArguments(operands, type);
          declaration.validateOutputType(coercedArgs, outputType);
          return Optional.of(generateBinding(call, out.get(), coercedArgs, outputType));
        }
      }
      return Optional.empty();
    }
  }

  public interface GenericCall {
    Stream<RexNode> getOperands();
    RelDataType getType();
  }

  /**
   * Coerced types according to an expected output type. Coercion is only done for type mismatches, not for nullability or parameter mismatches.
   */
  private List<Expression> coerceArguments(List<Expression> arguments, Type type) {

    return arguments.stream().map(a -> {
      var typeMatches = isMatch(type, a.getType());
      if (!typeMatches) {
        return ExpressionCreator.cast(type, a);
      }
      return a;
    }).toList();
  }

  protected abstract T generateBinding(C call, F function, List<Expression> arguments, Type outputType);


  public interface SingularArgumentMatcher<F> {
    Optional<F> tryMatch(Type type, Type outputType);
  }

  public interface SignatureMatcher<F> {
    Optional<F> tryMatch(List<Type> types, Type outputType);
  }

  private static SignatureMatcher chainedSignature(SignatureMatcher... matchers) {
    return switch(matchers.length) {
      case 0 -> (types, outputType) -> Optional.empty();
      case 1 -> matchers[0];
      default -> (types, outputType) -> {
        for (SignatureMatcher m : matchers) {
          var t = m.tryMatch(types, outputType);
          if (t.isPresent()) {
            return t;
          }
        }
        return Optional.empty();
      };
    };
  }

  private static boolean isMatch(Type inputType, ParameterizedType type) {
    if(type.isWildcard()) {
      return true;
    }
    return inputType.accept(new IgnoreNullableAndParameters(type));
  }

  private static boolean isMatch(ParameterizedType inputType, ParameterizedType type) {
    if(type.isWildcard()) {
      return true;
    }
    return inputType.accept(new IgnoreNullableAndParameters(type));
  }

}
