package io.substrait.isthmus;

import io.substrait.expression.proto.FunctionLookup;
import io.substrait.function.ImmutableSimpleExtension;
import io.substrait.function.SimpleExtension;
import io.substrait.proto.Plan;
import io.substrait.proto.PlanRel;
import io.substrait.relation.Rel;
import io.substrait.relation.RelConverter;
import org.apache.calcite.config.CalciteConnectionConfig;
import org.apache.calcite.config.CalciteConnectionProperty;
import org.apache.calcite.jdbc.CalciteSchema;
import org.apache.calcite.jdbc.JavaTypeFactoryImpl;
import org.apache.calcite.plan.Contexts;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelOptCostImpl;
import org.apache.calcite.plan.RelOptUtil;
import org.apache.calcite.plan.hep.HepPlanner;
import org.apache.calcite.plan.hep.HepProgram;
import org.apache.calcite.plan.volcano.VolcanoPlanner;
import org.apache.calcite.prepare.CalciteCatalogReader;
import org.apache.calcite.rel.RelRoot;
import org.apache.calcite.rel.metadata.DefaultRelMetadataProvider;
import org.apache.calcite.rel.metadata.ProxyingMetadataHandlerProvider;
import org.apache.calcite.rel.metadata.RelMetadataQuery;
import org.apache.calcite.rel.rules.AggregateExpandDistinctAggregatesRule;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexBuilder;
import org.apache.calcite.schema.impl.AbstractTable;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.SqlOperatorTable;
import org.apache.calcite.sql.ddl.SqlColumnDeclaration;
import org.apache.calcite.sql.ddl.SqlCreateTable;
import org.apache.calcite.sql.fun.SqlStdOperatorTable;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.sql.parser.SqlParser;
import org.apache.calcite.sql.parser.SqlParserPos;
import org.apache.calcite.sql.parser.ddl.SqlDdlParserImpl;
import org.apache.calcite.sql.validate.SqlValidator;
import org.apache.calcite.sql.validate.SqlValidatorCatalogReader;
import org.apache.calcite.sql.validate.SqlValidatorImpl;
import org.apache.calcite.sql2rel.SqlToRelConverter;
import org.apache.calcite.sql2rel.StandardConvertletTable;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Take a SQL statement and a set of table definitions and return a substrait plan.
 */
public class SqlToSubstrait {

  public Plan execute(String sql, List<String> tables) throws SqlParseException {
    CalciteSchema rootSchema = CalciteSchema.createRootSchema(false);
    SqlToRelConverter.Config converterConfig = SqlToRelConverter.config()
        .withTrimUnusedFields(true)
        .withExpand(false);

    RelDataTypeFactory factory = new JavaTypeFactoryImpl();
    CalciteConnectionConfig config = CalciteConnectionConfig.DEFAULT.set(CalciteConnectionProperty.CASE_SENSITIVE, "false");
    CalciteCatalogReader catalogReader = new CalciteCatalogReader(rootSchema, Arrays.asList(), factory, config);
    SqlValidator validator = Validator.create(factory, catalogReader, SqlValidator.Config.DEFAULT);

    if(tables != null) {
      for (String tableDef : tables) {
        DefinedTable t = parseCreateTable(factory, validator, tableDef);
        rootSchema.add(t.getName(), t);
      }
    }

    SqlParser parser = SqlParser.create(sql, SqlParser.Config.DEFAULT);
    var parsed = parser.parseQuery();

    VolcanoPlanner planner = new VolcanoPlanner(RelOptCostImpl.FACTORY, Contexts.of("hello"));
    RelOptCluster cluster = RelOptCluster.create(planner,new RexBuilder(factory));

    cluster.setMetadataQuerySupplier(() -> {
      ProxyingMetadataHandlerProvider handler = new ProxyingMetadataHandlerProvider(DefaultRelMetadataProvider.INSTANCE);
      return new RelMetadataQuery(handler);
    });

    SqlToRelConverter converter = new SqlToRelConverter(null, validator, catalogReader, cluster, StandardConvertletTable.INSTANCE, converterConfig);
    RelRoot root = converter.convertQuery(parsed, true, true);
    {
      var program = HepProgram.builder()
          .addRuleInstance(AggregateExpandDistinctAggregatesRule.Config.DEFAULT.toRule())
          .build();
      HepPlanner hepPlanner = new HepPlanner(program);
      hepPlanner.setRoot(root.rel);
      root = root.withRel(hepPlanner.findBestExp());
    }

    //System.out.println(RelOptUtil.toString(root.rel));
    Rel pojoRel = SubstraitRelVisitor.convert(root, EXTENSION_COLLECTION);
    FunctionLookup functionLookup = new FunctionLookup();
    RelConverter toProtoRel = new RelConverter(functionLookup);
    var protoRel = pojoRel.accept(toProtoRel);

    var planRel = PlanRel.newBuilder()
        .setRoot(
            io.substrait.proto.RelRoot.newBuilder()
                .setInput(protoRel)
                .addAllNames(TypeConverter.toNamedStruct(root.validatedRowType).names()));

    var plan = Plan.newBuilder();
    plan.addRelations(planRel);
    functionLookup.addFunctionsToPlan(plan);
    return plan.build();
  }

  private static final SimpleExtension.ExtensionCollection EXTENSION_COLLECTION;
  static {
    SimpleExtension.ExtensionCollection defaults = ImmutableSimpleExtension.ExtensionCollection.builder().build();
    try {
      defaults = SimpleExtension.loadDefaults();
    } catch (IOException e) {
      throw new RuntimeException("Failure while loading defaults.", e);
    }

    EXTENSION_COLLECTION = defaults;
  }

  private DefinedTable parseCreateTable(RelDataTypeFactory factory, SqlValidator validator, String sql) throws SqlParseException {
      SqlParser parser = SqlParser.create(sql, SqlParser.Config.DEFAULT.withParserFactory(SqlDdlParserImpl.FACTORY));
      var parsed = parser.parseQuery();
      //var validated = validator.validate(parsed);

      if (!(parsed instanceof SqlCreateTable)) {
        fail("Not a valid CREATE TABLE statement.");
      }

      SqlCreateTable create = (SqlCreateTable) parsed;
      if (create.name.names.size() > 1) {
        fail("Only simple table names are allowed.", create.name.getParserPosition());
      }

      if (create.query != null) {
        fail("CTAS not supported.", create.name.getParserPosition());
      }

      List<String> names = new ArrayList<>();
      List<RelDataType> columnTypes = new ArrayList<>();

      for (SqlNode node : create.columnList) {
        if (!(node instanceof SqlColumnDeclaration)) {
          fail("Unexpected column list construction.", node.getParserPosition());
        }

        SqlColumnDeclaration col = (SqlColumnDeclaration) node;
        if (col.name.names.size() != 1) {
          fail("Expected simple column names.", col.name.getParserPosition());
        }

        names.add(col.name.names.get(0));
        columnTypes.add(col.dataType.deriveType(validator));
      }

      return new DefinedTable(create.name.names.get(0), factory, factory.createStructType(columnTypes, names));
  }

  private static SqlParseException fail(String text, SqlParserPos pos) {
    return new SqlParseException(text, pos, null, null, new RuntimeException("fake lineage"));
  }

  private static SqlParseException fail(String text) {
    return fail(text, SqlParserPos.ZERO);
  }

  private static final class Validator extends SqlValidatorImpl {

    private Validator(SqlOperatorTable opTab, SqlValidatorCatalogReader catalogReader, RelDataTypeFactory typeFactory, Config config) {
      super(opTab, catalogReader, typeFactory, config);
    }

    public static Validator create(RelDataTypeFactory factory, CalciteCatalogReader catalog, SqlValidator.Config config) {
      return new Validator(SqlStdOperatorTable.instance(), catalog, factory, config);
    }

  }

  /**
   * A fully defined pre-specified table.
   */
  private static final class DefinedTable extends AbstractTable {

    private final String name;
    private final RelDataTypeFactory factory;
    private final RelDataType type;

    public DefinedTable(String name, RelDataTypeFactory factory, RelDataType type) {
      this.name = name;
      this.factory = factory;
      this.type = type;
    }

    @Override
    public RelDataType getRowType(RelDataTypeFactory typeFactory) {
      if (factory != typeFactory) {
        throw new IllegalStateException("Different type factory than previously used.");
      }
      return type;
    }

    public String getName() {
      return name;
    }
  }
}
