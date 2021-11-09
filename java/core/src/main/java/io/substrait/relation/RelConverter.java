package io.substrait.relation;

import io.substrait.expression.Expression;
import io.substrait.expression.proto.ExpressionProtoConverter;
import io.substrait.expression.proto.FunctionLookup;
import io.substrait.proto.Rel;
import io.substrait.proto.*;
import io.substrait.type.proto.TypeProtoConverter;

import java.util.Collection;
import java.util.List;

public class RelConverter implements RelVisitor<Rel, RuntimeException> {

  private final ExpressionProtoConverter protoConverter;
  private final FunctionLookup functionLookup;

  public RelConverter(FunctionLookup functionLookup) {
    this.functionLookup = functionLookup;
    this.protoConverter = new ExpressionProtoConverter(functionLookup);
  }

  private List<io.substrait.proto.Expression> toProto(Collection<Expression> expressions) {
    return expressions.stream().map(this::toProto).toList();
  }

  private io.substrait.proto.Expression toProto(Expression expression) {
    return expression.accept(protoConverter);
  }

  private io.substrait.proto.Rel toProto(io.substrait.relation.Rel rel) {
    return rel.accept(this);
  }

  private io.substrait.proto.Type toProto(io.substrait.type.Type type) {
    return type.accept(TypeProtoConverter.INSTANCE);
  }

  private List<SortField> toProtoS(Collection<Expression.SortField> sorts) {
    return sorts.stream()
        .map(s -> {
          return SortField.newBuilder().setDirection(s.direction().toProto())
              .setExpr(toProto(s.expr()))
              .build();
        })
        .toList();
  }

  @Override
  public Rel visit(Aggregate aggregate) throws RuntimeException {
    var builder = AggregateRel.newBuilder()
        .setInput(toProto(aggregate.getInput()))
        .setCommon(common(aggregate))
        .addAllGroupings(aggregate.getGroupings().stream().map(this::toProto).toList())
        .addAllMeasures(aggregate.getMeasures().stream().map(this::toProto).toList())
        ;

    return Rel.newBuilder().setAggregate(builder).build();
  }

  private AggregateRel.Measure toProto(Aggregate.Measure measure) {
    var func = AggregateFunction.newBuilder()
        .setPhase(measure.getFunction().aggregationPhase().toProto())
        .setOutputType(toProto(measure.getFunction().getType()))
        .addAllArgs(toProto(measure.getFunction().arguments()))
        .addAllSorts(toProtoS(measure.getFunction().sort()))
        .setFunctionReference(functionLookup.getFunctionReference(measure.getFunction().declaration()));

    var builder = AggregateRel.Measure.newBuilder()
        .setMeasure(func);

    measure.getPreMeasureFilter().ifPresent(f -> builder.setFilter(toProto(f)));
    return builder.build();
  }

  private AggregateRel.Grouping toProto(Aggregate.Grouping grouping) {
    return AggregateRel.Grouping.newBuilder().addAllGroupingExpressions(toProto(grouping.getExpressions())).build();
  }


  @Override
  public Rel visit(EmptyScan emptyScan) throws RuntimeException {
    return Rel.newBuilder().setRead(ReadRel.newBuilder()
            .setCommon(common(emptyScan))
            .setVirtualTable(ReadRel.VirtualTable.newBuilder().build())
            .setBaseSchema(emptyScan.getInitialSchema().toProto())
            .build())
        .build();
  }

  @Override
  public Rel visit(Fetch fetch) throws RuntimeException {
    var builder = FetchRel.newBuilder()
        .setCommon(common(fetch))
        .setInput(toProto(fetch.getInput()))
        .setOffset(fetch.getOffset());

    fetch.getCount().ifPresent(f -> builder.setCount(f));
    return Rel.newBuilder().setFetch(builder).build();
  }

  @Override
  public Rel visit(Filter filter) throws RuntimeException {
    var builder = FilterRel.newBuilder()
        .setCommon(common(filter))
        .setInput(toProto(filter.getInput()))
        .setCondition(filter.getCondition().accept(protoConverter));

    return Rel.newBuilder().setFilter(builder).build();
  }

  @Override
  public Rel visit(Join join) throws RuntimeException {
    var builder = JoinRel.newBuilder()
        .setCommon(common(join))
        .setLeft(toProto(join.getLeft()))
        .setRight(toProto(join.getRight()))
        .setType(join.getJoinType().toProto());

    join.getCondition().ifPresent(t -> builder.setExpression(toProto(t)));

    return Rel.newBuilder().setJoin(builder).build();
  }

  @Override
  public Rel visit(NamedScan namedScan) throws RuntimeException {
    return Rel.newBuilder().setRead(ReadRel.newBuilder()
            .setCommon(common(namedScan))
            .setNamedTable(ReadRel.NamedTable.newBuilder().addAllNames(namedScan.getNames()))
            .setBaseSchema(namedScan.getInitialSchema().toProto())
            .build())
        .build();
  }

  @Override
  public Rel visit(Project project) throws RuntimeException {
    var builder = ProjectRel.newBuilder()
        .setCommon(common(project))
        .setInput(toProto(project.getInput()))
        .addAllExpressions(project.getExpressions().stream().map(this::toProto).toList());

    return Rel.newBuilder().setProject(builder).build();
  }

  @Override
  public Rel visit(Sort sort) throws RuntimeException {
    var builder = SortRel.newBuilder()
        .setCommon(common(sort))
        .setInput(toProto(sort.getInput()))
        .addAllSorts(toProtoS(sort.getSortFields()));
    return Rel.newBuilder().setSort(builder).build();
  }


  @Override
  public Rel visit(VirtualTableScan virtualTableScan) throws RuntimeException {
    return Rel.newBuilder().setRead(ReadRel.newBuilder()
            .setCommon(common(virtualTableScan))
            .setVirtualTable(ReadRel.VirtualTable.newBuilder()
                .addAllValues(virtualTableScan.getRows().stream().map(this::toProto).map(t -> t.getLiteral().getStruct()).toList())
                .build())
            .setBaseSchema(virtualTableScan.getInitialSchema().toProto())
            .build())
        .build();
  }

  private RelCommon common(io.substrait.relation.Rel rel) {
    var builder = RelCommon.newBuilder();
    rel.getRemap().ifPresentOrElse(
        r -> builder.setEmit(RelCommon.Emit.newBuilder().addAllOutputMapping(r.indices())),
        () -> builder.setDirect(RelCommon.Direct.getDefaultInstance())
    );
    return builder.build();
  }
}
