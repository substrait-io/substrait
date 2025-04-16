#!/bin/sh

mkdir -p substraitpb

protoc \
  --proto_path=proto \
  --go_out substraitpb \
  --go_opt=module=github.com/substrait-io/substrait-protobuf/go/substraitpb \
  substrait/algebra.proto \
  substrait/capabilities.proto \
  substrait/extended_expression.proto \
  substrait/function.proto \
  substrait/parameterized_types.proto \
  substrait/plan.proto \
  substrait/type_expressions.proto \
  substrait/extensions/extensions.proto