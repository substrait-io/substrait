package io.substrait.expression.proto;

import io.substrait.expression.AggregateFunctionInvocation;
import io.substrait.expression.ExpressionVisitor;
import io.substrait.expression.FieldReference;
import io.substrait.proto.AggregateFunction;
import io.substrait.proto.Expression;
import io.substrait.type.proto.TypeProtoConverter;

import java.util.List;
import java.util.function.Consumer;

public class ExpressionProtoConverter implements ExpressionVisitor<Expression, RuntimeException> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ExpressionProtoConverter.class);

  private final FunctionLookup lookup;

  public ExpressionProtoConverter(FunctionLookup lookup) {
    this.lookup = lookup;
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.NullLiteral expr) {
    return lit(bldr -> bldr.setNull(expr.type().accept(TypeProtoConverter.INSTANCE)));
  }

  private Expression lit(Consumer<Expression.Literal.Builder> consumer) {
    var builder = Expression.Literal.newBuilder();
    consumer.accept(builder);
    return Expression.newBuilder().setLiteral(builder).build();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.BoolLiteral expr) {
    return lit(bldr -> bldr.setBoolean(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.I8Literal expr) {
    return lit(bldr -> bldr.setI8(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.I16Literal expr) {
    return lit(bldr -> bldr.setI16(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.I32Literal expr) {
    return lit(bldr -> bldr.setI32(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.I64Literal expr) {
    return lit(bldr -> bldr.setI64(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.FP32Literal expr) {
    return lit(bldr -> bldr.setFp32(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.FP64Literal expr) {
    return lit(bldr -> bldr.setFp64(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.StrLiteral expr) {
    return lit(bldr -> bldr.setString(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.BinaryLiteral expr) {
    return lit(bldr -> bldr.setBinary(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.TimeLiteral expr) {
    return lit(bldr -> bldr.setTime(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.DateLiteral expr) {
    return lit(bldr -> bldr.setDate(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.TimestampLiteral expr) {
    return lit(bldr -> bldr.setTimestamp(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.TimestampTZLiteral expr) {
    return lit(bldr -> bldr.setTimestampTz(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.IntervalYearLiteral expr) {
    return lit(bldr -> bldr.setIntervalYearToMonth(
        Expression.Literal.IntervalYearToMonth.newBuilder().setYears(expr.years()).setMonths(expr.months())));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.IntervalDayLiteral expr) {
    return lit(bldr -> bldr.setIntervalDayToSecond(
        Expression.Literal.IntervalDayToSecond.newBuilder().setDays(expr.days()).setSeconds(expr.seconds())));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.UUIDLiteral expr) {
    return lit(bldr -> bldr.setUuid(expr.toBytes()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.FixedCharLiteral expr) {
    return lit(bldr -> bldr.setFixedChar(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.VarCharLiteral expr) {
    return lit(bldr -> bldr.setVarChar(Expression.Literal.VarChar.newBuilder().setValue(expr.value()).setLength(expr.length())));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.FixedBinaryLiteral expr) {
    return lit(bldr -> bldr.setFixedBinary(expr.value()));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.DecimalLiteral expr) {
    return lit(bldr -> bldr.setDecimal(
        Expression.Literal.Decimal.newBuilder()
            .setValue(expr.value())
            .setPrecision(expr.precision())
            .setScale(expr.scale())));
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.MapLiteral expr) {
    return lit(bldr -> {
      var keyValues = expr.values().entrySet().stream().map(e -> {
        var key = toLiteral(e.getKey());
        var value = toLiteral(e.getValue());
        return Expression.Literal.Map.KeyValue.newBuilder().setKey(key).setValue(value).build();
      }).toList();
      bldr.setMap(Expression.Literal.Map.newBuilder().addAllKeyValues(keyValues));
    });
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.ListLiteral expr) {
    return lit(bldr -> {
      var values = expr.values().stream().map(this::toLiteral).toList();
      bldr.setList(Expression.Literal.List.newBuilder().addAllValues(values));
    });
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.StructLiteral expr) {
    return lit(bldr -> {
      var values = expr.fields().stream().map(this::toLiteral).toList();
      bldr.setStruct(Expression.Literal.Struct.newBuilder().addAllFields(values));
    });
  }

  private Expression.Literal toLiteral(io.substrait.expression.Expression expression) {
    var e = expression.accept(this);
    assert e.getRexTypeCase() == Expression.RexTypeCase.LITERAL;
    return e.getLiteral();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.Switch expr) {
    var clauses = expr.switchClauses().stream().map(
        s -> Expression.SwitchExpression.IfValue.newBuilder().setIf(toLiteral(s.condition())).setThen(s.then().accept(this)).build()).toList();
    return Expression.newBuilder().setSwitchExpression(Expression.SwitchExpression.newBuilder().addAllIfs(clauses)
            .setElse(expr.defaultClause().accept(this))
        ).build();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.IfThen expr) {
    var clauses = expr.ifClauses().stream().map(
        s -> Expression.IfThen.IfClause.newBuilder()
            .setIf(s.condition().accept(this))
            .setThen(s.then().accept(this)).build())
        .toList();
    return Expression.newBuilder().setIfThen(Expression.IfThen.newBuilder().addAllIfs(clauses)
        .setElse(expr.elseClause().accept(this))
    ).build();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.ScalarFunctionInvocation expr) {
    return Expression.newBuilder().setScalarFunction(Expression.ScalarFunction.newBuilder()
        .setOutputType(expr.getType().accept(TypeProtoConverter.INSTANCE))
        .setFunctionReference(lookup.getFunctionReference(expr.declaration()))
        .addAllArgs(expr.arguments().stream().map(a -> a.accept(this)).toList())
    ).build();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.Cast expr) {
    return Expression.newBuilder()
        .setCast(Expression.Cast.newBuilder()
            .setInput(expr.input().accept(this))
            .setType(expr.getType().accept(TypeProtoConverter.INSTANCE)))
        .build();
  }

  private Expression from(io.substrait.expression.Expression expr) {
    return expr.accept(this);
  }

  private List<Expression> from(List<io.substrait.expression.Expression> expr) {
    return expr.stream().map(this::from).toList();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.SingleOrList expr) throws RuntimeException {
    return Expression.newBuilder()
        .setSingularOrList(
            Expression.SingularOrList.newBuilder()
                .setValue(expr.condition().accept(this))
                .addAllOptions(from(expr.options())))
        .build();
  }

  @Override
  public Expression visit(io.substrait.expression.Expression.MultiOrList expr) throws RuntimeException {
    return Expression.newBuilder()
        .setMultiOrList(
            Expression.MultiOrList.newBuilder()
                .addAllValue(from(expr.conditions()))
                .addAllOptions(expr.optionCombinations()
                    .stream()
                    .map(r -> Expression.MultiOrList.Record.newBuilder().addAllFields(from(r.values())).build()).toList())
        )
        .build();
  }

  @Override
  public Expression visit(FieldReference expr) {
    Expression.ReferenceSegment top = null;
    Expression.ReferenceSegment seg = null;
    for (var segment : expr.segments()) {
      var protoSegment = switch (segment) {
        case FieldReference.StructField f -> {
          var bldr = Expression.ReferenceSegment.StructField.newBuilder().setField(f.offset());
          if (seg != null) {
            bldr.setChild(seg);
          }
          yield Expression.ReferenceSegment.newBuilder().setStructField(bldr);
        }
        case FieldReference.ListElement f -> {
          var bldr = Expression.ReferenceSegment.ListElement.newBuilder().setOffset(f.offset());
          if (seg != null) {
            bldr.setChild(seg);
          }
          yield Expression.ReferenceSegment.newBuilder().setListElement(bldr);
        }
        case FieldReference.MapKey f -> {
          var bldr = Expression.ReferenceSegment.MapKey.newBuilder().setMapKey(toLiteral(f.key()));
          if (seg != null) {
            bldr.setChild(seg);
          }
          yield Expression.ReferenceSegment.newBuilder().setMapKey(bldr);
        }
        default -> throw new IllegalArgumentException("Unhandled type: " + segment);
      };
      var builtSegment = protoSegment.build();
      if(top == null) {
        top = builtSegment;
      }
      seg = builtSegment;
    }

    var out = Expression.FieldReference.newBuilder().setDirectReference(top);
    if (expr.inputExpression().isPresent()) {
     out.setExpression(from(expr.inputExpression().get()));
    } else {
      out.setRootReference(Expression.FieldReference.RootReference.getDefaultInstance());
    }

    return Expression.newBuilder().setSelection(out).build();
  }


}
