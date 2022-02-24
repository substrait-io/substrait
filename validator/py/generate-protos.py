"""
Generates Substrait's protobuf files for use by substrait-validator.
"""

import os
import shutil
import subprocess
import protobuf_gen

# Base directory to which all protobuf import paths are relative.
PROTO_ROOT = os.path.join("..", "..", "proto")

# Protobuf package name.
PROTO_PACKAGE = "substrait"

# Base directory to which all Python import paths are relative.
PY_PACKAGE_ROOT = "."

# Python package/module names to be prefixed to the protobuf package path,
# dot-separated.
PY_PACKAGE_PREFIX = "substrait_validator"

# ------------------------------------------------------------------------------

# Delete the previously generated files to make sure the tree is clean.
proto_module_dir = os.path.join(
    PY_PACKAGE_ROOT, *PY_PACKAGE_PREFIX.split("."), PROTO_PACKAGE
)
if os.path.isdir(proto_module_dir):
    shutil.rmtree(proto_module_dir)

# Get the paths of all .proto files.
proto_files = []
for path, _, files in os.walk(os.path.join(PROTO_ROOT, PROTO_PACKAGE)):
    for fname in files:
        if fname.endswith(".proto"):
            proto_files.append(os.path.relpath(os.path.join(path, fname), PROTO_ROOT))

# Use protobuf_gen to generate the Python files using protoc and patch them
# with the Python import path prefix.
protobuf_gen.remap(PY_PACKAGE_ROOT, PY_PACKAGE_PREFIX, [PROTO_ROOT], proto_files)

# Remove the _grpc.py files that protobuf_gen generates but we don't use.
for path, _, files in os.walk(proto_module_dir):
    for fname in files:
        if fname.endswith("_grpc.py"):
            os.unlink(os.path.join(path, fname))
