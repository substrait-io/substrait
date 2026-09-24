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


def test_parse_decimal_example():
    header = make_header("v1.0", "extension:io.substrait:functions_arithmetic_decimal")
    tests = """# basic
power(8::dec<38,0>, 2::dec<38, 0>) = 64::fp64
power(1.0::dec<38, 0>, -1.0::dec<38, 0>) = 1.0::fp64
power(-1::dec, 0.5::dec<38,1>) [complex_number_result:NAN] = nan::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 3
    assert test_file.testcases[0].func_name == "power"
    assert (
        test_file.testcases[0].base_uri
        == "extension:io.substrait:functions_arithmetic_decimal"
    )
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("64", "fp64")
    assert test_file.testcases[0].args[0] == CaseLiteral("8", "dec<38,0>")
    assert test_file.testcases[0].args[1] == CaseLiteral("2", "dec<38,0>")


def test_parse_decimal_example_with_nan():
    header = make_header("v1.0", "extension:io.substrait:functions_arithmetic_decimal")
    tests = """# basic
power(-1::dec, 0.5::dec<38,1>) [complex_number_result:NAN] = nan::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "power"
    assert (
        test_file.testcases[0].base_uri
        == "extension:io.substrait:functions_arithmetic_decimal"
    )
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("nan", "fp64")
    assert test_file.testcases[0].args[0] == CaseLiteral("-1", "dec")
    assert test_file.testcases[0].args[1] == CaseLiteral("0.5", "dec<38,1>")


def test_parse_string_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
concat('abc'::str, 'def'::str) = 'abcdef'::str
regexp_string_split('HHHelloooo'::str, 'Hel+'::str) = ['HH', 'oooo']::List<str>
octet_length('à'::str) = 2::i64
octet_length('😄'::str) = 4::i64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 4
    assert test_file.testcases[0].func_name == "concat"
    assert test_file.testcases[0].base_uri == "extension:io.substrait:functions_string"
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("'abcdef'", "str")

    assert test_file.testcases[1].func_name == "regexp_string_split"
    assert test_file.testcases[1].base_uri == "extension:io.substrait:functions_string"
    assert test_file.testcases[1].group.name == "basic"
    assert test_file.testcases[1].result == CaseLiteral(["'HH'", "'oooo'"], "List<str>")
    assert test_file.testcases[1].args[0] == CaseLiteral("'HHHelloooo'", "str")
    assert test_file.testcases[1].args[1] == CaseLiteral("'Hel+'", "str")

    assert test_file.testcases[2].func_name == "octet_length"
    assert test_file.testcases[2].base_uri == "extension:io.substrait:functions_string"
    assert test_file.testcases[2].group.name == "basic"
    assert test_file.testcases[2].result == CaseLiteral("2", "i64")
    assert test_file.testcases[2].args[0] == CaseLiteral("'à'", "str")

    assert test_file.testcases[3].func_name == "octet_length"
    assert test_file.testcases[3].base_uri == "extension:io.substrait:functions_string"
    assert test_file.testcases[3].group.name == "basic"
    assert test_file.testcases[3].result == CaseLiteral("4", "i64")
    assert test_file.testcases[3].args[0] == CaseLiteral("'😄'", "str")


def test_parse_type_shaped_string_literals():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
identity('2020-05-10'::str) = '2020-05-10'::str
identity('12:00:00'::str) = '12:00:00'::str
identity('2020-05-10T01:02:03'::str) = '2020-05-10T01:02:03'::str
identity('2020-05-10T01:02:03.123'::str) = '2020-05-10T01:02:03.123'::str
identity('2020-05-10T01:02:03+01:00'::str) = '2020-05-10T01:02:03+01:00'::str
identity('2020-05-10T01:02:03Z'::str) = '2020-05-10T01:02:03Z'::str
identity('P1Y'::str) = 'P1Y'::str
identity('P1D'::str) = 'P1D'::str
identity('P1Y2M3DT4H5M6S'::str) = 'P1Y2M3DT4H5M6S'::str
"""

    test_file = parse_string(header + tests)

    values = [
        "'2020-05-10'",
        "'12:00:00'",
        "'2020-05-10T01:02:03'",
        "'2020-05-10T01:02:03.123'",
        "'2020-05-10T01:02:03+01:00'",
        "'2020-05-10T01:02:03Z'",
        "'P1Y'",
        "'P1D'",
        "'P1Y2M3DT4H5M6S'",
    ]
    assert len(test_file.testcases) == len(values)
    for test_case, value in zip(test_file.testcases, values, strict=True):
        assert test_case.args[0] == CaseLiteral(value, "str")
        assert test_case.result == CaseLiteral(value, "str")


def test_parse_string_list_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func('abc'::str, 'def'::str) = [1, 2, 3, 4, 5, 6]::List<i8>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "some_func"
    assert test_file.testcases[0].base_uri == "extension:io.substrait:functions_string"
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral(
        ["1", "2", "3", "4", "5", "6"], "List<i8>"
    )
    assert test_file.testcases[0].args[0] == CaseLiteral("'abc'", "str")
    assert test_file.testcases[0].args[1] == CaseLiteral("'def'", "str")


def test_parse_nested_list_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func([[1, 2], [3, 4]]::List<List<i32>>) = [[5, 6]]::List<List<i32>>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        [["1", "2"], ["3", "4"]], "List<List<i32>>"
    )
    assert test_file.testcases[0].result == CaseLiteral([["5", "6"]], "List<List<i32>>")


def test_parse_triply_nested_list_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func([[[1, 2], [3, 4]], [[5, 6]]]::List<List<List<i32>>>) = [[[7]]]::List<List<List<i32>>>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        [[["1", "2"], ["3", "4"]], [["5", "6"]]], "List<List<List<i32>>>"
    )
    assert test_file.testcases[0].result == CaseLiteral(
        [[["7"]]], "List<List<List<i32>>>"
    )


def test_parse_null_list_arg():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func(null::List?<i32>) = null::List?<i32>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        None, "List?<i32>", nullable=True
    )
    assert test_file.testcases[0].result == CaseLiteral(
        None, "List?<i32>", nullable=True
    )


def test_parse_struct_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func((1, 'abc', true)::struct<i32, str, bool>) = (2, 'def')::struct<i32, str>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        ["1", "'abc'", "true"], "struct<i32,str,bool>"
    )
    assert test_file.testcases[0].result == CaseLiteral(
        ["2", "'def'"], "struct<i32,str>"
    )


def test_parse_empty_struct_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func(()::struct<>) = ()::struct<>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral([], "struct<>")
    assert test_file.testcases[0].result == CaseLiteral([], "struct<>")


def test_parse_map_example():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func({'a': 1, 'b': 2}::map<str, i32>) = {}::map<str, i32>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        [
            {"key": "'a'", "value": "1"},
            {"key": "'b'", "value": "2"},
        ],
        "map<str,i32>",
    )
    assert test_file.testcases[0].result == CaseLiteral([], "map<str,i32>")


def test_parse_nested_complex_literals():
    header = make_header("v1.0", "extension:io.substrait:functions_string")
    tests = """# basic
some_func(([1, 2], {'x': (3, null)})::struct<list<i32>, map<str, struct<i32, str?>>>) = null::map?<str, i32>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        [
            ["1", "2"],
            [{"key": "'x'", "value": ["3", "null"]}],
        ],
        "struct<list<i32>,map<str,struct<i32,str?>>>",
    )
    assert test_file.testcases[0].result == CaseLiteral(
        None, "map?<str,i32>", nullable=True
    )


def test_parse_user_defined_type_literal():
    header = make_header("v1.0", "extension:org.example:extension_types")
    tests = """# basic
some_func((4, 2)::u!point) = (1, 1)::u!point
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(["4", "2"], "u!point")
    assert test_file.testcases[0].result == CaseLiteral(["1", "1"], "u!point")


def test_parse_nullable_user_defined_type_literal():
    header = make_header("v1.0", "extension:org.example:extension_types")
    tests = """# basic
some_func((4, 2)::u!point?) = (1, 1)::u!point?
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args[0] == CaseLiteral(
        ["4", "2"], "u!point?", nullable=True
    )
    assert test_file.testcases[0].result == CaseLiteral(
        ["1", "1"], "u!point?", nullable=True
    )
