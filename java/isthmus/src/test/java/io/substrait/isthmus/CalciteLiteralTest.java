package io.substrait.isthmus;

import com.google.common.collect.ImmutableMap;
import io.substrait.expression.Expression;
import io.substrait.isthmus.expression.RexExpressionConverter;
import io.substrait.type.Type;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.sql.type.SqlTypeName;
import org.apache.calcite.util.DateString;
import org.apache.calcite.util.TimeString;
import org.apache.calcite.util.TimestampString;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.nio.charset.StandardCharsets;
import java.time.LocalDate;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;

import static io.substrait.expression.ExpressionCreator.*;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class CalciteLiteralTest extends CalciteObjs {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(CalciteLiteralTest.class);


  @Test
  void nullLiteral() {
    test(typedNull(Type.NULLABLE.varChar(10)), rex.makeNullLiteral(tN(SqlTypeName.VARCHAR, 10)));
  }

  @Test
  void tI8() {
    test(i8(false, 4), c(4, SqlTypeName.TINYINT));
  }

  @Test
  void tI16() {
    test(i16(false, 4), c(4, SqlTypeName.SMALLINT));
  }

  @Test
  void tI32() {
    test(i32(false, 4), c(4, SqlTypeName.INTEGER));
  }

  @Test
  void tI64() {
    test(i64(false, 1234L), c(1234L, SqlTypeName.BIGINT));
  }

  @Test
  void tFP32() {
    test(fp32(false, 4.44F), c(4.44F, SqlTypeName.FLOAT));
  }

  @Test
  void tFP64() {
    test(fp64(false, 4.45F), c(4.45F, SqlTypeName.DOUBLE));
  }

  @Test
  void tStr() {
    test(string(false, "my test"), c("my test", SqlTypeName.VARCHAR));
  }

  @Test
  void tBinary() {
    var val = "my test".getBytes(StandardCharsets.UTF_8);
    test(binary(false, val), c(new org.apache.calcite.avatica.util.ByteString(val), SqlTypeName.VARBINARY));
  }

  @Test
  void tTime() {
    test(time(false, (14L * 60 * 60 + 22 * 60 + 47) * 1000 * 1000), rex.makeTimeLiteral(new TimeString(14,22,47), 6));
  }

  @Test
  void tDate() {
    test(date(false, (int) LocalDate.of(2002, 2, 14).toEpochDay()), rex.makeDateLiteral(new DateString(2002,2,14)));
  }

  @Test
  void tTimestamp() {
    var ts = timestamp(false, 2002,2,14,16,20,47, 123);
    var nano = (int) TimeUnit.MICROSECONDS.toNanos(123);
    var tsx = new TimestampString(2002,2,14, 16, 20, 47).withNanos(nano);
    test(ts, rex.makeTimestampLiteral(tsx, 6));
  }


  @Disabled("Not clear what the right literal mapping is.")
  @Test
  void tTimestampTZ() {
    // Calcite has TimestampWithTimeZoneString but it doesn't appear to be available as a literal or data type. (Doesn't exist in SqlTypeName.)
  }

  @Disabled("NYI")
  @Test
  void tIntervalYear() {

  }

  @Disabled("NYI")
  @Test
  void tIntervalDay() {

  }

  @Test
  void tFixedChar() {
    test(fixedChar(false, "hello "), c("hello ", SqlTypeName.CHAR));
  }

  @Test
  void tVarChar() {
    test(varChar(false, "hello ", 10), c("hello ", SqlTypeName.VARCHAR, 10));
  }

  @Test
  void tDecimalLiteral() {
    BigDecimal bd = BigDecimal.valueOf(-123.45789);
    test(decimal(false, bd, 32, 10), c(bd, SqlTypeName.DECIMAL, 32, 10));
  }

  @Test
  void tMap() {
    var ss = ImmutableMap.<Expression.Literal, Expression.Literal>of(string(false, "foo"), i32(false, 4), string(false, "bar"), i32(false, -1));
    var calcite = rex.makeLiteral(ImmutableMap.of("foo", 4, "bar", -1), type.createMapType(t(SqlTypeName.VARCHAR), t(SqlTypeName.INTEGER)), true, false);
    test(map(false, ss), calcite);
  }

  @Test
  void tList() {
    test(list(false, i32(false, 4), i32(false, -1)),
        rex.makeLiteral(Arrays.asList(4, -1), type.createArrayType(t(SqlTypeName.INTEGER), -1), false, false));
  }

  @Test
  void tStruct() {
    test(struct(false, i32(false, 4), i32(false, -1)),
        rex.makeLiteral(Arrays.asList(4, -1), type.createStructType(Arrays.asList(t(SqlTypeName.INTEGER), t(SqlTypeName.INTEGER)), Arrays.asList("c1", "c2")), false, false));

  }
  @Test
  void tFixedBinary() {
    var val = "my test".getBytes(StandardCharsets.UTF_8);
    test(fixedBinary(false, val), c(new org.apache.calcite.avatica.util.ByteString(val), SqlTypeName.BINARY));
  }

  public void test(Expression expression, RexNode rex) {
    assertEquals(expression, rex.accept(new RexExpressionConverter()));
  }

}
