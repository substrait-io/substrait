#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

import os
import shutil
import pathlib

script_path = os.path.dirname(os.path.realpath(__file__))

# Remove generated protobuf files.
proto_output_path = os.path.join(script_path, 'substrait')
if os.path.isdir(proto_output_path):
    shutil.rmtree(proto_output_path)

# Remove compiled test files and test results.
suite_path = os.path.join(script_path, 'tests')
for fname in pathlib.Path(suite_path).rglob('*.test*'):
    os.remove(fname)
