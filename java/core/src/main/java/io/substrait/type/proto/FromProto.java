package io.substrait.type.proto;

import io.substrait.type.Type;
import io.substrait.type.TypeCreator;


public class FromProto {

  public static Type from(io.substrait.proto.Type type) {
    return switch(type.getKindCase()) {
      case BOOL -> n(type.getBool().getNullability()).BOOLEAN;
      case I8 -> n(type.getI8().getNullability()).I8;
      case I16 -> n(type.getI16().getNullability()).I16;
      case I32 -> n(type.getI32().getNullability()).I32;
      case I64 -> n(type.getI64().getNullability()).I64;
      case FP32 -> n(type.getFp32().getNullability()).FP32;
      case FP64 -> n(type.getFp64().getNullability()).FP64;
      case STRING -> n(type.getString().getNullability()).STRING;
      case BINARY -> n(type.getBinary().getNullability()).BINARY;
      case TIMESTAMP -> n(type.getTimestamp().getNullability()).TIMESTAMP;
      case DATE -> n(type.getDate().getNullability()).DATE;
      case TIME -> n(type.getTime().getNullability()).TIME;
      case INTERVAL_YEAR -> n(type.getIntervalYear().getNullability()).INTERVAL_YEAR;
      case INTERVAL_DAY -> n(type.getIntervalDay().getNullability()).INTERVAL_DAY;
      case TIMESTAMP_TZ -> n(type.getTimestampTz().getNullability()).TIMESTAMP_TZ;
      case UUID -> n(type.getUuid().getNullability()).UUID;
      case FIXED_CHAR -> n(type.getFixedChar().getNullability()).fixedChar(type.getFixedChar().getLength());
      case VARCHAR -> n(type.getVarchar().getNullability()).varChar(type.getVarchar().getLength());
      case FIXED_BINARY -> n(type.getFixedBinary().getNullability()).fixedBinary(type.getFixedBinary().getLength());
      case DECIMAL -> n(type.getDecimal().getNullability()).decimal(type.getDecimal().getPrecision(), type.getDecimal().getScale());
      case STRUCT -> n(type.getStruct().getNullability()).struct(type.getStruct().getTypesList().stream().map(FromProto::from).toList());
      case LIST -> n(type.getList().getNullability()).list(from(type.getList().getType()));
      case MAP -> n(type.getMap().getNullability()).map(from(type.getMap().getKey()), from(type.getMap().getValue()));
      case USER_DEFINED_TYPE_REFERENCE, KIND_NOT_SET -> throw new UnsupportedOperationException();
    };
  }

  private static TypeCreator n(io.substrait.proto.Type.Nullability n) {
    return n == io.substrait.proto.Type.Nullability.NULLABILITY_NULLABLE ? TypeCreator.NULLABLE : TypeCreator.REQUIRED;
  }
}
