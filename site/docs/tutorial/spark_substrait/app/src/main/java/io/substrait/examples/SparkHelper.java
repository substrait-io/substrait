package io.substrait.examples;

import org.apache.spark.sql.SparkSession;

public class SparkHelper {
  public static final String NAMESPACE = "demo_db";
  public static final String VEHICLE_TABLE = "vehicles";
  public static final String TESTS_TABLE = "tests";

  public static final String VEHICLES_PQ = "vehicles_subset_2023.parquet";
  public static final String TESTS_PQ = "tests_subset_2023.parquet";

  public static final String VEHICLES_CSV = "vehicles_subset_2023.csv";
  public static final String TESTS_CSV = "tests_subset_2023.csv";

  public static final String ROOT_DIR = "/opt/spark-data";

  // Connect to local spark for demo purposes
  public static SparkSession connectSpark(String spark_master) {   

    SparkSession spark = SparkSession.builder()
        // .config("spark.sql.warehouse.dir", "spark-warehouse")
        .config("spark.master", spark_master)
        .enableHiveSupport()
        .getOrCreate();

    spark.sparkContext().setLogLevel("ERROR");

    return spark;
  }

  public static SparkSession connectLocalSpark() {   

    SparkSession spark = SparkSession.builder()
        .enableHiveSupport()
        .getOrCreate();

    spark.sparkContext().setLogLevel("ERROR");

    return spark;
  }


}
