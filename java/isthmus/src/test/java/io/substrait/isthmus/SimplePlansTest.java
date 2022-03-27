package io.substrait.isthmus;

import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.util.JsonFormat;
import io.substrait.proto.Plan;
import org.apache.calcite.sql.parser.SqlParseException;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.util.Arrays;

public class SimplePlansTest extends PlanTestBase {

  @Test
  public void aggFilter() throws IOException, SqlParseException {
    SqlToSubstrait s = new SqlToSubstrait();
    String[] values = asString("tpch/schema.sql").split(";");
    var creates = Arrays.stream(values).filter(t -> !t.trim().isBlank()).toList();
    s.execute("select sum(L_ORDERKEY) filter(WHERE L_ORDERKEY > 10) from lineitem ", creates);
  }

  @Test
  public void cd() throws IOException, SqlParseException {
    SqlToSubstrait s = new SqlToSubstrait();
    String[] values = asString("tpch/schema.sql").split(";");
    var creates = Arrays.stream(values).filter(t -> !t.trim().isBlank()).toList();
    //creates.forEach(System.out::println);
    s.execute("select l_partkey, sum(distinct L_ORDERKEY) from lineitem group by l_partkey ", creates);
  }

  @Test
  public void filter() throws IOException, SqlParseException {
    SqlToSubstrait s = new SqlToSubstrait();
    String[] values = asString("tpch/schema.sql").split(";");
    var creates = Arrays.stream(values).filter(t -> !t.trim().isBlank()).toList();
    //creates.forEach(System.out::println);
    print(s.execute("select * from lineitem WHERE L_ORDERKEY > 10", creates));
  }


  private void print(Plan plan) {
    try {
      System.out.println(JsonFormat.printer().includingDefaultValueFields().print(plan));
    } catch (InvalidProtocolBufferException e) {
      throw new RuntimeException(e);
    }
  }


}
