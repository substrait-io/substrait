package io.substrait.examples;

import static io.substrait.examples.SparkHelper.ROOT_DIR;
import static io.substrait.examples.SparkHelper.TESTS_CSV;
import static io.substrait.examples.SparkHelper.TESTS_TABLE;
import static io.substrait.examples.SparkHelper.VEHICLES_CSV;
import static io.substrait.examples.SparkHelper.VEHICLE_TABLE;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import org.apache.spark.sql.SparkSession;
import org.apache.spark.sql.catalyst.plans.logical.LogicalPlan;

import io.substrait.plan.PlanProtoConverter;
import io.substrait.spark.logical.ToSubstraitRel;

/** Minimal Spark application */
public class SparkSQL implements App.Action {

  public SparkSQL() {

  }

  @Override
  public void run(String arg) {

    // Connect to a local in-process Spark instance
    try (SparkSession spark = SparkHelper.connectLocalSpark()) {
      spark.catalog().listDatabases().show();

      // load from CSV files
      String vehiclesFile = Paths.get(ROOT_DIR, VEHICLES_CSV).toString();
      String testsFile = Paths.get(ROOT_DIR, TESTS_CSV).toString();

      System.out.println("Reading " + vehiclesFile);
      System.out.println("Reading " + testsFile);

      spark.read().option("delimiter", ",").option("header", "true").csv(vehiclesFile)
          .createOrReplaceTempView(VEHICLE_TABLE);
      spark.read().option("delimiter", ",").option("header", "true").csv(testsFile)
        .createOrReplaceTempView(TESTS_TABLE);  

      String sqlQuery = """
           SELECT vehicles.colour, count(*) as colourcount
           FROM vehicles
           INNER JOIN tests ON vehicles.vehicle_id=tests.vehicle_id
           WHERE tests.test_result = 'P'
           GROUP BY vehicles.colour
           ORDER BY count(*)
          """;

          var result = spark.sql(sqlQuery);
      result.show();

      LogicalPlan logical = result.logicalPlan();
      System.out.println(logical);

      LogicalPlan optimised = result.queryExecution().optimizedPlan();
      System.out.println(optimised);

      createSubstrait(optimised);
      spark.stop();
    } catch (Exception e) {
      e.printStackTrace(System.out);
    }
  }

  public void createSubstrait(LogicalPlan enginePlan) {
    ToSubstraitRel toSubstrait = new ToSubstraitRel();
    io.substrait.plan.Plan plan = toSubstrait.convert(enginePlan);
    System.out.println(plan);

    PlanProtoConverter planToProto = new PlanProtoConverter();
    byte[] buffer = planToProto.toProto(plan).toByteArray();
    try {
      Files.write(Paths.get(ROOT_DIR,"spark_sql_substrait.plan"), buffer);
      System.out.println("File written to "+Paths.get(ROOT_DIR,"spark_sql_substrait.plan"));

    } catch (IOException e) {
      e.printStackTrace();
    }
  }

}
