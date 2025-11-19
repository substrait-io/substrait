#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
set -e

mkdir -p proto_gen
find proto -name "*.proto" -exec protoc --python_out=proto_gen --proto_path=proto {} +
find proto_gen -type d -exec touch {}/__init__.py \;
