package io.substrait.isthmus;

import com.google.protobuf.util.JsonFormat;
import io.substrait.proto.Plan;
import picocli.CommandLine;

import java.util.List;
import java.util.concurrent.Callable;
import static picocli.CommandLine.*;

@Command(
    name = "isthmus",
    version = "isthmus 0.1",
    description = "Converts a SQL query to a Substrait Plan")
public class PlanEntryPoint implements Callable<Integer> {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(PlanEntryPoint.class);

  @Parameters(index = "0", description = "The sql we should parse.")
  private String sql;

  @Option(names = {"-c", "--create"}, description = "Create table statements e.g. CREATE TABLE T1(foo int, bar bigint)")
  private List<String> createStatements;

  @Override
  public Integer call() throws Exception {
    SqlToSubstrait converter = new SqlToSubstrait();
    Plan plan = converter.execute(sql, createStatements);
    System.out.println(JsonFormat.printer().includingDefaultValueFields().print(plan));
    return 0;
  }

  // this example implements Callable, so parsing, error handling and handling user
  // requests for usage help or version help can be done with one line of code.
  public static void main(String... args) {
    int exitCode = new CommandLine(new PlanEntryPoint()).execute(args);
    System.exit(exitCode);
  }
}
