package io.substrait.isthmus;

import com.google.protobuf.util.JsonFormat;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.Arrays;

public class TpchQueryNoValidation extends PlanTestBase {

  @ParameterizedTest
  //@ValueSource(ints = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22})
  @ValueSource(ints = {1,3,5,6,10,14,19})
  public void tpch(int query) throws Exception {
    SqlToSubstrait s = new SqlToSubstrait();
    String[] values = asString("tpch/schema.sql").split(";");
    var creates = Arrays.stream(values).filter(t -> !t.trim().isBlank()).toList();
    var plan = s.execute(asString(String.format("tpch/queries/%02d.sql", query)), creates);
    System.out.println(JsonFormat.printer().print(plan));
  }

}
