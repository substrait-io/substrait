package io.substrait.type;


import io.substrait.type.proto.TypeProtoConverter;
import org.immutables.value.Value;

import java.util.List;

@Value.Immutable
public interface NamedStruct {
  Type.Struct struct();
  List<String> names();

  public static NamedStruct of(List<String> names, Type.Struct type) {
    return ImmutableNamedStruct.builder().addAllNames(names).struct(type).build();
  }

  default io.substrait.proto.NamedStruct toProto() {
    var type = struct().accept(TypeProtoConverter.INSTANCE);
    return io.substrait.proto.NamedStruct.newBuilder()
        .setStruct(type.getStruct())
        .addAllNames(names())
        .build();
  }
}
