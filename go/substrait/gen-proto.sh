#!/bin/sh

TOPLEVEL=$(git rev-parse --show-toplevel)
protoc --go_out=$TOPLEVEL/go --go_opt=module=github.com/substrait-io/substrait/go \
      --proto_path=$TOPLEVEL/proto $TOPLEVEL/proto/substrait/*.proto $TOPLEVEL/proto/substrait/extensions/*.proto
