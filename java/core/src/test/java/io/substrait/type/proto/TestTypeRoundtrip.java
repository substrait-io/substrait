package io.substrait.type.proto;

import io.substrait.type.Type;
import io.substrait.type.TypeCreator;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class TestTypeRoundtrip {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(TestTypeRoundtrip.class);

  @ParameterizedTest
  @ValueSource(booleans = {true, false})
  public void roundtrip(boolean n) {
    t(creator(n).BOOLEAN);
    t(creator(n).I8);
    t(creator(n).I16);
    t(creator(n).I32);
    t(creator(n).I64);
    t(creator(n).FP32);
    t(creator(n).FP64);
    t(creator(n).STRING);
    t(creator(n).BINARY);
    t(creator(n).TIME);
    t(creator(n).DATE);
    t(creator(n).TIMESTAMP);
    t(creator(n).TIMESTAMP_TZ);
    t(creator(n).INTERVAL_YEAR);
    t(creator(n).INTERVAL_DAY);
    t(creator(n).UUID);
    t(creator(n).fixedChar(25));
    t(creator(n).varChar(35));
    t(creator(n).fixedBinary(45));
    t(creator(n).decimal(34,3));
    t(creator(n).map(creator(n).I8, creator(n).I16));
    t(creator(n).list(creator(n).TIME));
    t(creator(n).struct(creator(n).TIME, creator(n).TIMESTAMP, creator(n).TIMESTAMP_TZ));
  }

  /**
   * Test a type pojo -> proto -> pojo roundtrip.
   * @param type
   */
  private void t(Type type) {
    var converted = type.accept(new TypeProtoConverter());
    assertEquals(type, FromProto.from(converted));
  }

  private TypeCreator creator(boolean nullable) {
    return nullable ? TypeCreator.NULLABLE : TypeCreator.REQUIRED;
  }
}
