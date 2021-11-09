package io.substrait.isthmus.expression;

import com.google.common.collect.ImmutableList;
import io.substrait.expression.Expression;
import io.substrait.function.SimpleExtension;
import io.substrait.isthmus.CallConverter;
import io.substrait.type.Type;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexCall;
import org.apache.calcite.rex.RexNode;

import java.util.List;
import java.util.Optional;
import java.util.function.Function;
import java.util.stream.Stream;

public class ScalarFunctionConverter
    extends FunctionConverter<SimpleExtension.ScalarFunctionVariant, Expression, ScalarFunctionConverter.WrappedScalarCall>
    implements CallConverter {

  public ScalarFunctionConverter(List<SimpleExtension.ScalarFunctionVariant> functions, RelDataTypeFactory typeFactory) {
    super(functions, typeFactory);
  }

  public ScalarFunctionConverter(List<SimpleExtension.ScalarFunctionVariant> functions, List<FunctionMappings.Sig> additionalSignatures, RelDataTypeFactory typeFactory) {
    super(functions, additionalSignatures, typeFactory);
  }

  @Override
  protected ImmutableList<FunctionMappings.Sig> getSigs() {
    return FunctionMappings.SCALAR_SIGS;
  }

  @Override
  public Optional<Expression> convert(RexCall call, Function<RexNode, Expression> topLevelConverter) {
    FunctionFinder m = signatures.get(call.op);
    if (m == null) {
      return Optional.empty();
    }
    if (!m.allowedArgCount(call.getOperands().size())) {
      return Optional.empty();
    }

    var wrapped = new WrappedScalarCall(call);
    return m.attemptMatch(wrapped, topLevelConverter);
  }

  @Override
  protected Expression generateBinding(
      WrappedScalarCall call,
      SimpleExtension.ScalarFunctionVariant function,
      List<Expression> arguments, Type outputType) {
    return Expression.ScalarFunctionInvocation.builder()
        .outputType(outputType)
        .declaration(function)
        .addAllArguments(arguments)
        .build();
  }

  static class WrappedScalarCall implements GenericCall {

    private final RexCall delegate;

    private WrappedScalarCall(RexCall delegate) {
      this.delegate = delegate;
    }

    @Override
    public Stream<RexNode> getOperands() {
      return delegate.getOperands().stream();
    }

    @Override
    public RelDataType getType() {
      return delegate.getType();
    }
  }
}
