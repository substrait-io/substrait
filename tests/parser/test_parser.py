# SPDX-License-Identifier: Apache-2.0
import os

import pytest
from antlr4 import InputStream
from tests.parser import ParseError, parse_one_file, parse_stream
from tests.parser.nodes import AggregateArgument, CaseLiteral, FuncCallArg


def parse_string(input_string):
    return parse_stream(InputStream(input_string), "test_string")


def make_header(version, include):
    return f"""### SUBSTRAIT_SCALAR_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def make_aggregate_test_header(version, include):
    return f"""### SUBSTRAIT_AGGREGATE_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def make_window_test_header(version, include):
    return f"""### SUBSTRAIT_WINDOW_TEST: {version}
### SUBSTRAIT_INCLUDE: {include}

"""


def get_absolute_path(relative_path):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, relative_path)


def test_parse_basic_example():
    header = make_header("v1.0", "extension:io.substrait:functions_arithmetic")
    tests = """# 'Basic examples without any special cases'
add(120::i8, 5::i8) = 125::i8
add(100::i16, 100::i16) = 200::i16
add(1::i8?, 2::i8?) = 3::i8?

# Overflow examples demonstrating overflow behavior
add(120::i8, 10::i8) [overflow:ERROR] = <!ERROR>
"""

    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 4
