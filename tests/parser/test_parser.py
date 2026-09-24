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


def test_parse_func_call_arg():
    header = make_header("v1.0", "extension:io.substrait:functions_arithmetic")
    tests = """# associativity
add(1::i32, add(2::i32, 3::i32)) = add(add(1::i32, 2::i32), 3::i32)
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    tc = test_file.testcases[0]
    assert tc.func_name == "add"
    assert tc.args[0] == CaseLiteral("1", "i32")
    assert tc.args[1] == FuncCallArg(
        func_name="add",
        args=[CaseLiteral("2", "i32"), CaseLiteral("3", "i32")],
    )
    assert tc.result == FuncCallArg(
        func_name="add",
        args=[
            FuncCallArg(
                func_name="add",
                args=[CaseLiteral("1", "i32"), CaseLiteral("2", "i32")],
            ),
            CaseLiteral("3", "i32"),
        ],
    )


def test_parse_date_time_example():
    header = make_header("v1.0", "extension:io.substrait:functions_datetime")
    tests = """# timestamp examples using the precision_timestamp type
lt(2016-12-31T13:30:15::pts<6>, 2017-12-31T13:30:15::pts<6>) = true::bool
"""

    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "lt"
    assert (
        test_file.testcases[0].base_uri == "extension:io.substrait:functions_datetime"
    )
    assert (
        test_file.testcases[0].group.name
        == "timestamp examples using the precision_timestamp type"
    )
    assert test_file.testcases[0].result == CaseLiteral("true", "bool")
    assert test_file.testcases[0].args[0] == CaseLiteral(
        "2016-12-31T13:30:15", "pts<6>"
    )
    assert test_file.testcases[0].args[1] == CaseLiteral(
        "2017-12-31T13:30:15", "pts<6>"
    )
