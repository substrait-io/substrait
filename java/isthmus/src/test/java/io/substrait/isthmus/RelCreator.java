package io.substrait.isthmus;

import org.apache.calcite.config.CalciteConnectionConfig;
import org.apache.calcite.config.CalciteConnectionProperty;
import org.apache.calcite.jdbc.CalciteSchema;
import org.apache.calcite.jdbc.JavaTypeFactoryImpl;
import org.apache.calcite.plan.Contexts;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelOptCostImpl;
import org.apache.calcite.plan.volcano.VolcanoPlanner;
import org.apache.calcite.prepare.CalciteCatalogReader;
import org.apache.calcite.rel.RelRoot;
import org.apache.calcite.rel.metadata.DefaultRelMetadataProvider;
import org.apache.calcite.rel.metadata.ProxyingMetadataHandlerProvider;
import org.apache.calcite.rel.metadata.RelMetadataQuery;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.rex.RexBuilder;
import org.apache.calcite.sql.fun.SqlStdOperatorTable;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.sql.parser.SqlParser;
import org.apache.calcite.sql.validate.SqlValidator;
import org.apache.calcite.sql.validate.SqlValidatorCatalogReader;
import org.apache.calcite.sql.validate.SqlValidatorImpl;
import org.apache.calcite.sql2rel.SqlToRelConverter;
import org.apache.calcite.sql2rel.StandardConvertletTable;
import org.apache.calcite.tools.RelBuilder;

import java.util.Arrays;

public class RelCreator {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(RelCreator.class);

  private RelOptCluster cluster;
  private CalciteCatalogReader catalog;

  public RelCreator() {
    CalciteSchema schema = CalciteSchema.createRootSchema(false);
    RelDataTypeFactory factory = new JavaTypeFactoryImpl(SubstraitTypeSystem.TYPE_SYSTEM);
    CalciteConnectionConfig config = CalciteConnectionConfig.DEFAULT.set(CalciteConnectionProperty.CASE_SENSITIVE, "false");
    catalog = new CalciteCatalogReader(schema, Arrays.asList(), factory, config);
    VolcanoPlanner planner = new VolcanoPlanner(RelOptCostImpl.FACTORY, Contexts.EMPTY_CONTEXT);
    cluster = RelOptCluster.create(planner, new RexBuilder(factory));
  }

  public RelRoot parse(String sql) {

    try {
      SqlParser parser = SqlParser.create(sql, SqlParser.Config.DEFAULT);
      var parsed = parser.parseQuery();
      cluster.setMetadataQuerySupplier(() -> new RelMetadataQuery(new ProxyingMetadataHandlerProvider(DefaultRelMetadataProvider.INSTANCE)));
      SqlValidator validator = new Validator(catalog, cluster.getTypeFactory(), SqlValidator.Config.DEFAULT);

      SqlToRelConverter.Config converterConfig = SqlToRelConverter.config()
          .withTrimUnusedFields(true)
          .withExpand(false);
      SqlToRelConverter converter = new SqlToRelConverter(null, validator, catalog, cluster,
          StandardConvertletTable.INSTANCE, converterConfig);
      RelRoot root = converter.convertQuery(parsed, true, true);
      return root;
    } catch (SqlParseException e) {
      throw new RuntimeException(e);
    }
  }

  public RelBuilder createRelBuilder() {
    return RelBuilder.proto(Contexts.EMPTY_CONTEXT).create(cluster, catalog);
  }

  public RexBuilder rex() {
    return cluster.getRexBuilder();
  }

  public RelDataTypeFactory type() {
    return cluster.getTypeFactory();
  }

  private static final class Validator extends SqlValidatorImpl {

    public Validator(SqlValidatorCatalogReader catalogReader, RelDataTypeFactory typeFactory, Config config) {
      super(SqlStdOperatorTable.instance(), catalogReader, typeFactory, config);
    }

  }


}
