package io.substrait.examples;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.SparkSession;
import org.apache.spark.sql.catalyst.plans.logical.LogicalPlan;

import io.substrait.plan.Plan;
import io.substrait.plan.ProtoPlanConverter;
import io.substrait.spark.logical.ToLogicalPlan;

/** Minimal Spark application */
public class SparkConsumeSubstrait implements App.Action {

  public SparkConsumeSubstrait() {
  }

  @Override
  public void run(String arg) {

    // Connect to a local in-process Spark instance
    try (SparkSession spark = SparkHelper.connectLocalSpark()) {

      System.out.println("Reading from " + arg);
      byte[] buffer = Files.readAllBytes(Paths.get(arg));

      io.substrait.proto.Plan proto = io.substrait.proto.Plan.parseFrom(buffer);
      ProtoPlanConverter protoToPlan = new ProtoPlanConverter();
      Plan plan = protoToPlan.from(proto);

      ToLogicalPlan substraitConverter = new ToLogicalPlan(spark);
      LogicalPlan sparkPlan = substraitConverter.convert(plan);

      System.out.println(sparkPlan);

      Dataset.ofRows(spark, sparkPlan).show();

      spark.stop();
    } catch (IOException e) {
      e.printStackTrace();
    }
  }

}
