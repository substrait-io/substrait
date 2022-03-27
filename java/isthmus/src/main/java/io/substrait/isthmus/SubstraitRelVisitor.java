package io.substrait.isthmus;

import io.substrait.expression.Expression;
import io.substrait.expression.ExpressionCreator;
import io.substrait.expression.FieldReference;
import io.substrait.function.SimpleExtension;
import io.substrait.isthmus.expression.*;
import io.substrait.relation.*;
import io.substrait.type.Type;
import org.apache.calcite.rel.RelFieldCollation;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.RelRoot;
import org.apache.calcite.rel.core.AggregateCall;
import org.apache.calcite.rel.core.TableFunctionScan;
import org.apache.calcite.rel.core.TableScan;
import org.apache.calcite.rel.logical.*;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.rex.RexVisitor;
import org.apache.calcite.util.ImmutableBitSet;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class SubstraitRelVisitor extends RelNodeVisitor<Rel, RuntimeException> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(SubstraitRelVisitor.class);

  private final SimpleExtension.ExtensionCollection extensions;
  private final RexVisitor<Expression> converter;
  private final AggregateFunctionConverter aggregateFunctionConverter;

  public SubstraitRelVisitor(RelDataTypeFactory typeFactory, SimpleExtension.ExtensionCollection extensions) {
    this.extensions = extensions;
    var converters = new ArrayList<CallConverter>();
    converters.addAll(CallConverters.DEFAULTS);
    converters.add(new ScalarFunctionConverter(extensions.scalarFunctions(), typeFactory));
    this.converter = new RexExpressionConverter(converters);
    this.aggregateFunctionConverter = new AggregateFunctionConverter(extensions.aggregateFunctions(), typeFactory);
  }

  private Expression toExpression(RexNode node) {
    return node.accept(converter);
  }

  @Override
  public Rel visit(TableScan scan) {
    var type = TypeConverter.toNamedStruct(scan.getRowType());
    return NamedScan.builder().initialSchema(type)
        .addAllNames(scan.getTable().getQualifiedName())
        .build();
  }

  @Override
  public Rel visit(TableFunctionScan scan) {
    return super.visit(scan);
  }

  @Override
  public Rel visit(LogicalValues values) {
    var type = TypeConverter.toNamedStruct(values.getRowType());
    if (values.getTuples().isEmpty()) {
      return EmptyScan.builder().initialSchema(type).build();
    }

    List<Expression.StructLiteral> structs = values.getTuples().stream().map(list -> {
      var fields =  list.stream().map(l -> LiteralConverter.convert(l)).collect(Collectors.toUnmodifiableList());
      return ExpressionCreator.struct(false, fields);
    }).collect(Collectors.toUnmodifiableList());
    return VirtualTableScan.builder().addAllDfsNames(type.names()).addAllRows(structs).build();
  }

  @Override
  public Rel visit(LogicalFilter filter) {
    var condition = toExpression(filter.getCondition());
    return Filter.builder().condition(condition).input(apply(filter.getInput())).build();
  }

  @Override
  public Rel visit(LogicalCalc calc) {
    return super.visit(calc);
  }

  @Override
  public Rel visit(LogicalProject project) {
    var expressions = project.getProjects().stream().map(this::toExpression).toList();

    // todo: eliminate excessive projects. This should be done by converting rexinputrefs to remaps.
    return Project.builder()
        .remap(Rel.Remap.offset(project.getInput().getRowType().getFieldCount(), expressions.size()))
        .expressions(expressions)
        .input(apply(project.getInput()))
        .build();
  }

  @Override
  public Rel visit(LogicalJoin join) {
    var left = apply(join.getLeft());
    var right = apply(join.getRight());
    var condition = toExpression(join.getCondition());
    var joinType = switch(join.getJoinType()) {
      case INNER -> Join.JoinType.INNER;
      case LEFT -> Join.JoinType.LEFT;
      case RIGHT -> Join.JoinType.RIGHT;
      case FULL -> Join.JoinType.OUTER;
      case SEMI -> Join.JoinType.SEMI;
      case ANTI -> Join.JoinType.ANTI;
    };

    return Join.builder().condition(condition).joinType(joinType).left(left).right(right).build();
  }

  @Override
  public Rel visit(LogicalCorrelate correlate) {
    return super.visit(correlate);
  }

  @Override
  public Rel visit(LogicalUnion union) {
    return super.visit(union);
  }

  @Override
  public Rel visit(LogicalIntersect intersect) {
    return super.visit(intersect);
  }

  @Override
  public Rel visit(LogicalMinus minus) {
    return super.visit(minus);
  }

  @Override
  public Rel visit(LogicalAggregate aggregate) {
    var input = apply(aggregate.getInput());
    Stream<ImmutableBitSet> sets;
    if (aggregate.groupSets != null) {
      sets = aggregate.groupSets.stream();
    } else {
      sets = Stream.of(aggregate.getGroupSet());
    }

    var groupings = sets.filter(s -> s != null)
        .map(s -> fromGroupSet(s, input))
        .collect(Collectors.toList());

    var aggCalls = aggregate.getAggCallList()
        .stream()
        .map(c -> fromAggCall(aggregate.getInput(), input.getRecordType(), c))
        .collect(Collectors.toList());

    return Aggregate.builder().input(input).addAllGroupings(groupings).addAllMeasures(aggCalls).build();
  }

  Aggregate.Grouping fromGroupSet(ImmutableBitSet bitSet, Rel input) {
    List<Expression> references = bitSet.asList()
        .stream().map(i -> FieldReference.newInputRelReference(i, input))
        .collect(Collectors.toList());
    return Aggregate.Grouping.builder().addAllExpressions(references).build();
  }

  Aggregate.Measure fromAggCall(RelNode input, Type.Struct inputType, AggregateCall call) {
    var invocation = aggregateFunctionConverter.convert(input, inputType, call, t -> t.accept(converter));
    if(invocation.isEmpty()) {
      throw new UnsupportedOperationException("Unable to find binding for call " + call);
    }
    var builder = Aggregate.Measure.builder().function(invocation.get());
    if (call.filterArg != -1) {
       builder.preMeasureFilter(FieldReference.newRootStructReference(call.filterArg, inputType));
    }
    return builder.build();
  }

  @Override
  public Rel visit(LogicalMatch match) {
    return super.visit(match);
  }

  @Override
  public Rel visit(LogicalSort sort) {
    var input = apply(sort.getInput());
    var fields = sort.getCollation().getFieldCollations().stream().map(t -> toSortField(t, input.getRecordType())).toList();
    var convertedSort = Sort.builder().addAllSortFields(fields).input(input).build();
    if (sort.fetch == null && sort.offset == null) {
      return convertedSort;
    }
    var offset = Optional.ofNullable(sort.offset).map(r -> asLong(r)).orElse(0L);
    var builder = Fetch.builder().input(convertedSort).offset(offset);
    if (sort.fetch == null) {
      return builder.build();
    }

    return builder.count(asLong(sort.fetch)).build();
  }

  private long asLong(RexNode rex) {
    var expr = toExpression(rex);
    return switch(expr) {
      case Expression.I64Literal i -> i.value();
      case Expression.I32Literal i -> i.value();
      default -> throw new UnsupportedOperationException("Unknown type: " + rex);
    };
  }

  public static Expression.SortField toSortField(RelFieldCollation collation, Type.Struct inputType) {
    Expression.SortDirection direction = switch(collation.direction) {
      case STRICTLY_ASCENDING,ASCENDING -> collation.nullDirection == RelFieldCollation.NullDirection.LAST ? Expression.SortDirection.ASC_NULLS_LAST : Expression.SortDirection.ASC_NULLS_FIRST;
      case STRICTLY_DESCENDING,DESCENDING -> collation.nullDirection == RelFieldCollation.NullDirection.LAST ? Expression.SortDirection.DESC_NULLS_LAST : Expression.SortDirection.DESC_NULLS_FIRST;
      case CLUSTERED -> Expression.SortDirection.CLUSTERED;
    };

    return Expression.SortField.builder().expr(FieldReference.newRootStructReference(collation.getFieldIndex(), inputType))
        .direction(direction).build();
  }

  @Override
  public Rel visit(LogicalExchange exchange) {
    return super.visit(exchange);
  }

  @Override
  public Rel visit(LogicalTableModify modify) {
    return super.visit(modify);
  }

  @Override
  public Rel visitOther(RelNode other) {
    throw new UnsupportedOperationException("Unable to handle node: " + other);
  }

  public Rel apply(RelNode r) {
    return reverseAccept(r);
  }

  public static Rel convert(RelRoot root, SimpleExtension.ExtensionCollection extensions) {
    SubstraitRelVisitor visitor = new SubstraitRelVisitor(root.rel.getCluster().getTypeFactory(), extensions);
    return visitor.apply(root.rel);
  }
}
