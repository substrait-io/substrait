# SPDX-License-Identifier: Apache-2.0
import os

import pytest
from antlr4 import InputStream
from tests.coverage.case_file_parser import parse_stream, parse_one_file
from tests.coverage.extensions import Extension
from tests.coverage.visitor import ParseError
from tests.coverage.nodes import CaseLiteral


def parse_string(input_string):
    return parse_stream(InputStream(input_string), "test_string")


def make_header(version, include):
    return f"""### SUBSTRAIT_SCALAR_TEST: {version}
### SUBSTRAIT_INCLUDE: '{include}'

"""


def make_aggregate_test_header(version, include):
    return f"""### SUBSTRAIT_AGGREGATE_TEST: {version}
### SUBSTRAIT_INCLUDE: '{include}'

"""


def test_parse_basic_example():
    header = make_header("v1.0", "/extensions/functions_arithmetic.yaml")
    tests = """# 'Basic examples without any special cases'
add(120::i8, 5::i8) = 125::i8
add(100::i16, 100::i16) = 200::i16
add(1::i8?, 2::i8?) = 3::i8?

# Overflow examples demonstrating overflow behavior
add(120::i8, 10::i8) [overflow:ERROR] = <!ERROR>
"""

    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 4


def test_parse_date_time_example():
    header = make_header("v1.0", "/extensions/functions_datetime.yaml")
    tests = """# timestamp examples using the timestamp type
lt('2016-12-31T13:30:15'::ts, '2017-12-31T13:30:15'::ts) = true::bool
"""

    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "lt"
    assert test_file.testcases[0].base_uri == "/extensions/functions_datetime.yaml"
    assert (
        test_file.testcases[0].group.name
        == "timestamp examples using the timestamp type"
    )
    assert test_file.testcases[0].result == CaseLiteral("true", "bool")
    assert test_file.testcases[0].args[0] == CaseLiteral("2016-12-31T13:30:15", "ts")
    assert test_file.testcases[0].args[1] == CaseLiteral("2017-12-31T13:30:15", "ts")


def test_parse_decimal_example():
    header = make_header("v1.0", "extensions/functions_arithmetic_decimal.yaml")
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
        == "extensions/functions_arithmetic_decimal.yaml"
    )
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("64", "fp64")
    assert test_file.testcases[0].args[0] == CaseLiteral("8", "dec<38,0>")
    assert test_file.testcases[0].args[1] == CaseLiteral("2", "dec<38,0>")


def test_parse_decimal_example_with_nan():
    header = make_header("v1.0", "extensions/functions_arithmetic_decimal.yaml")
    tests = """# basic
power(-1::dec, 0.5::dec<38,1>) [complex_number_result:NAN] = nan::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "power"
    assert (
        test_file.testcases[0].base_uri
        == "extensions/functions_arithmetic_decimal.yaml"
    )
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("nan", "fp64")
    assert test_file.testcases[0].args[0] == CaseLiteral("-1", "dec")
    assert test_file.testcases[0].args[1] == CaseLiteral("0.5", "dec<38,1>")


def test_parse_string_example():
    header = make_header("v1.0", "extensions/functions_string.yaml")
    tests = """# basic
concat('abc'::str, 'def'::str) = 'abcdef'::str
regexp_string_split('HHHelloooo'::str, 'Hel+'::str) = ['HH', 'oooo']::List<str>
octet_length('à'::str) = 2::i64
octet_length('😄'::str) = 4::i64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 4
    assert test_file.testcases[0].func_name == "concat"
    assert test_file.testcases[0].base_uri == "extensions/functions_string.yaml"
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("'abcdef'", "str")

    assert test_file.testcases[1].func_name == "regexp_string_split"
    assert test_file.testcases[1].base_uri == "extensions/functions_string.yaml"
    assert test_file.testcases[1].group.name == "basic"
    assert test_file.testcases[1].result == CaseLiteral(["'HH'", "'oooo'"], "List<str>")
    assert test_file.testcases[1].args[0] == CaseLiteral("'HHHelloooo'", "str")
    assert test_file.testcases[1].args[1] == CaseLiteral("'Hel+'", "str")

    assert test_file.testcases[2].func_name == "octet_length"
    assert test_file.testcases[2].base_uri == "extensions/functions_string.yaml"
    assert test_file.testcases[2].group.name == "basic"
    assert test_file.testcases[2].result == CaseLiteral("2", "i64")
    assert test_file.testcases[2].args[0] == CaseLiteral("'à'", "str")

    assert test_file.testcases[3].func_name == "octet_length"
    assert test_file.testcases[3].base_uri == "extensions/functions_string.yaml"
    assert test_file.testcases[3].group.name == "basic"
    assert test_file.testcases[3].result == CaseLiteral("4", "i64")
    assert test_file.testcases[3].args[0] == CaseLiteral("'😄'", "str")


def test_parse_string_list_example():
    header = make_header("v1.0", "extensions/functions_string.yaml")
    tests = """# basic
some_func('abc'::str, 'def'::str) = [1, 2, 3, 4, 5, 6]::List<i8>
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "some_func"
    assert test_file.testcases[0].base_uri == "extensions/functions_string.yaml"
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral(
        ["1", "2", "3", "4", "5", "6"], "List<i8>"
    )
    assert test_file.testcases[0].args[0] == CaseLiteral("'abc'", "str")
    assert test_file.testcases[0].args[1] == CaseLiteral("'def'", "str")


def test_parse_aggregate_func_test():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
avg((1,2,3)::fp32) = 2::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1


def test_parse_aggregate_func_test_compact():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(col0::fp32, col1::fp32) = 1::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1


def test_parse_aggregate_func_test_multiple_args():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
DEFINE t1(fp32, fp32) = ((20, 20), (-3, -3), (1, 1), (10,10), (5,5))
corr(t1.col0, t1.col1) = 1::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1


def test_parse_aggregate_func_test_compact_mixed_args():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
((20), (-3), (1), (10)) LIST_AGG(col0::fp32, ','::string) = 1::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1


def test_parse_aggregate_func_test_compact_string_agg():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
(('ant'), ('bat'), ('cat')) string_agg(col0::str, ','::str) = 1::fp64
(('ant'), ('bat'), ('cat')) string_agg(col0::string, ','::string) = 1::fp64
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 2


def test_parse_aggregate_func_max():
    header = make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
    tests = """# basic
max((2.5, 0, 5.0, -2.5, -7.5)::fp32) = 5.0::fp32
"""
    test_file = parse_string(header + tests)
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].func_name == "max"
    assert test_file.testcases[0].base_uri == "extensions/functions_arithmetic.yaml"
    assert test_file.testcases[0].group.name == "basic"
    assert test_file.testcases[0].result == CaseLiteral("5.0", "fp32")
    assert test_file.testcases[0].args == [
        CaseLiteral(value=["2.5", "0", "5.0", "-2.5", "-7.5"], type="fp32")
    ]


def get_absolute_path(relative_path):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, relative_path)


def test_parse_file_add():
    test_file = parse_one_file(get_absolute_path("../cases/arithmetic/add.test"))
    assert len(test_file.testcases) == 15
    assert test_file.testcases[0].func_name == "add"
    assert test_file.testcases[0].base_uri == "/extensions/functions_arithmetic.yaml"
    assert test_file.include == "/extensions/functions_arithmetic.yaml"


def test_parse_file_max():
    test_file = parse_one_file(get_absolute_path("../cases/arithmetic/max.test"))
    assert len(test_file.testcases) == 12
    assert test_file.testcases[0].func_name == "max"
    assert test_file.testcases[0].base_uri == "/extensions/functions_arithmetic.yaml"
    assert test_file.include == "/extensions/functions_arithmetic.yaml"


def test_parse_file_lt_datetime():
    test_file = parse_one_file(get_absolute_path("../cases/datetime/lt_datetime.test"))
    assert len(test_file.testcases) == 13
    assert test_file.testcases[0].func_name == "lt"
    assert test_file.testcases[0].base_uri == "/extensions/functions_datetime.yaml"


def test_parse_file_power_decimal():
    test_file = parse_one_file(
        get_absolute_path("../cases/arithmetic_decimal/power.test")
    )
    assert len(test_file.testcases) == 9
    assert test_file.testcases[0].func_name == "power"
    assert (
        test_file.testcases[0].base_uri
        == "/extensions/functions_arithmetic_decimal.yaml"
    )


@pytest.mark.parametrize(
    "input_func_test, position, expected_message",
    [
        (
            "add(-12::i8, +5::i8) = -7.0::i8",
            29,
            "no viable alternative at input '-7.0::i8'",
        ),
        (
            "add(123.5::i8, 5::i8) = 125::i8",
            11,
            "no viable alternative at input '123.5::i8'",
        ),
        (
            "add(123.5::i16, 5.5::i16) = 125::i16",
            11,
            "no viable alternative at input '123.5::i16'",
        ),
        (
            "add(123.5::i32, 5.5::i32) = 125::i32",
            21,
            "no viable alternative at input '5.5::i32'",
        ),
        (
            "add(123f::i64, 5.5::i64) = 125::i64",
            7,
            "no viable alternative at input '123f'",
        ),
        (
            "add(123::i64, 5_000::i64) = 5123::i64",
            15,
            "no viable alternative at input '5_000'",
        ),
        (
            "add(123::dec<38,10>, 5.0E::dec<38,10>) = 123::dec<38,10>",
            24,
            "no viable alternative at input '5.0E'",
        ),
        (
            "add(123::dec<38,10>, 1a.2::dec<38,10>) = 123::fp32",
            22,
            "no viable alternative at input '1a'",
        ),
        (
            "add(123::dec<38,10>, 1.2.3::dec<38,10>) = 123::fp32",
            24,
            "no viable alternative at input '1.2.'",
        ),
        (
            "add(123::dec<38,10>, +-12.3::dec<38,10>) = 123::i64",
            21,
            "extraneous input '+'",
        ),
        ("add(123::fp32, .5E2::fp32) = 123::fp32", 15, "extraneous input '.'"),
        ("add(123::fp32, 4.1::fp32) = ++123::fp32", 28, "extraneous input '+'"),
        (
            "add(123::fp32, 2.5E::fp32) = 123::fp32",
            18,
            "no viable alternative at input '2.5E'",
        ),
        (
            "add(123::fp32, 1.4E+::fp32) = 123::fp32",
            18,
            "no viable alternative at input '1.4E'",
        ),
        (
            "add(123::fp32, 3.E.5::fp32) = 123::fp32",
            17,
            "no viable alternative at input '3.E'",
        ),
    ],
)
def test_parse_errors_with_bad_scalar_testcases(
    input_func_test, position, expected_message
):
    header = make_header("v1.0", "extensions/functions_arithmetic.yaml") + "# basic\n"
    with pytest.raises(ParseError) as pm:
        parse_string(header + input_func_test + "\n")
    assert f"Syntax error at line 5, column {position}: {expected_message}" in str(
        pm.value
    )


@pytest.mark.parametrize(
    "input_func_test, expected_message",
    [
        (
            "max((-12, +5)::i8) = -7.0::i8",
            "no viable alternative at input '-7.0::i8'",
        ),
        (
            "max((-12, 'arg')::str) = -7::i8",
            "All values in a column must have the same type",
        ),
        (
            """DEFINE t1(fp32, fp32) = ((20, 20), (-3, -3), (1, 1), (10,10), (5,5))
                corr(t1.col0, t2.col1) = 1::fp64""",
            "Table name in argument does not match the table name in the function call",
        ),
        (
            "((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(my_col::fp32, col0::fp32) = 1::fp64",
            "mismatched input 'my_col'",
        ),
        (
            "((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(col0::fp32, column1::fp32) = 1::fp64",
            "mismatched input 'column1'",
        ),
    ],
)
def test_parse_errors_with_bad_aggregate_testcases(input_func_test, expected_message):
    header = (
        make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
        + "# basic\n"
    )
    with pytest.raises(ParseError) as pm:
        parse_string(header + input_func_test + "\n")
    assert expected_message in str(pm.value)


@pytest.mark.parametrize(
    "input_func_test",
    [
        "f1(1::i8, 2::i16, 3::i32, 4::i64) = -7.0::fp32",
        "f2(1.0::fp32, 2.0::fp64) = -7.0::fp32",
        "f3('a'::str, 'b'::string) = 'c'::str",
        "f4(false::bool, true::boolean) = false::bool",
        "f5(1.1::dec, 2.2::decimal) = 3.3::dec",
        "f6(1.1::dec<38,10>, 2.2::dec<38,10>) = 3.3::dec<38,10>",
        "f7(1.1::dec<38,10>, 2.2::decimal<38,10>) = 3.3::decimal<38,10>",
        "f8('1991-01-01'::date) = '2001-01-01'::date",
        "f9('13:01:01.2345678'::time) = '23:59:59.999'::time",
        "f10('1991-01-01T01:02:03.456'::ts, '1991-01-01T00:00:00'::timestamp) = '1991-01-01T22:33:44'::ts",
        "f11('1991-01-01T01:02:03.456+05:30'::tstz, '1991-01-01T00:00:00+15:30'::timestamp_tz) = 23::i32",
        "f12('1991-01-01'::date, 5::i64) = '1991-01-01T00:00:00+15:30'::timestamp_tz",
        "f13('P10Y5M'::interval_year, 5::i64) = 'P15Y5M'::interval_year",
        "f14('P10Y5M'::iyear, 5::i64) = 'P15Y5M'::iyear",
        "f15('P10DT5H6M7.2000S'::interval_day<6>, 5::i64) = 'P10DT10H6M7.2000S'::interval_day<6>",
        "f16('P10DT6M7.200S'::interval_day<3>, 5::i64) = 'P10DT11M7.200S'::interval_day<3>",
        "f16('P10DT6M0.2000S'::iday<4>, 5::i64) = 'P10DT11M5.2000S'::iday<4>",
        "f16('P10DT6M7S'::interval_day, 5::i64) = 'P10DT11M7S'::interval_day",
        "ltrim('abcabcdef'::str, 'abc'::str) [spaces_only:FALSE] = 'def'::str",
        "concat('abcd'::str, Null::str) [null_handling:ACCEPT_NULLS] = Null::str",
        "concat('abcd'::str, Null::str) [null_handling:IGNORE_NULLS] = 'abcd'::str",
        "concat(Null::str) [null_handling:ACCEPT_NULLS] = Null::str",
        "regexp_string_split('Hello'::str, 'Hel+?'::str) = ['', 'lo']::List<str>",
        "regexp_replace('USD100'::str, '(?<=USD)\\d{3}'::str, '999'::str) [lookaround:TRUE] = 'USD999'::str",
        "divide(5::i64, 0::i64) [on_division_by_zero:LIMIT] = inf::fp64",
        "modulus(5::i8, 0::i8) [on_domain_error:Null] = Null::i8",
        "modulus(8::i8, -3::i8) [division_type:TRUNCATE] = 2::i8",
        "and(true::bool, false::bool) = false::bool",
        "or(true::bool, false::boolean) = true::bool",
        "not(true::bool) = false::bool",
        "is_null(Null::str) = true::bool",
        "logb(2.0::fp64, 0.0::fp64) [on_log_zero:MINUS_INFINITY] = -inf::fp64",
        "logb(10::fp64, -inf::fp64) [on_domain_error:NONE] = Null::fp64",
        "regexp_string_split('HHHelloooo'::str, 'Hel+'::str) = ['HH', 'oooo']::List<str>",
        "octet_length(''::str) = 0::i64",
        "octet_length(' '::str) = 1::i64",
        "octet_length('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'::str) = 48::i64",
        "concat('abcd'::varchar<9>, Null::str) [null_handling:ACCEPT_NULLS] = Null::str",
        "concat('abcd'::vchar<9>, 'ef'::varchar<9>) = Null::vchar<9>",
        "concat('abcd'::vchar<9>, 'ef'::fixedchar<9>) = Null::fchar<9>",
        "concat('abcd'::fbin<9>, 'ef'::fixedbinary<9>) = Null::fbin<9>",
        "f35('1991-01-01T01:02:03.456'::pts<3>) = '1991-01-01T01:02:30.123123'::precision_timestamp<3>",
        "f36('1991-01-01T01:02:03.456'::pts<3>, '1991-01-01T01:02:30.123123'::precision_timestamp<3>) = 123456::i64",
        "f37('1991-01-01T01:02:03.123456'::pts<6>, '1991-01-01T04:05:06.456'::precision_timestamp<6>) = 123456::i64",
        "f38('1991-01-01T01:02:03.456+05:30'::ptstz<3>) = '1991-01-01T00:00:00+15:30'::precision_timestamp_tz<3>",
        "f39('1991-01-01T01:02:03.123456+05:30'::ptstz<6>) = '1991-01-01T00:00:00+15:30'::precision_timestamp_tz<6>",
        "logb(10::fp64, -inf::fp64) [on_domain_error:ERROR] = <!ERROR>",
        "bitwise_and(-31766::dec<5, 0>, 900::dec<3, 0>) = 896::dec<5, 0>",
        "or(true::bool, true::bool) = true::bool",
        "between(5::i8, 0::i8, 127::i8) = true::bool",
    ],
)
def test_parse_various_scalar_func_argument_types(input_func_test):
    header = make_header("v1.0", "extensions/functions_arithmetic.yaml") + "# basic\n"
    test_file = parse_string(header + input_func_test + "\n")
    assert len(test_file.testcases) == 1


@pytest.mark.parametrize(
    "input_func_test",
    [
        "f1((1, 2, 3, 4)::i64) = -7.0::fp32",
        "((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) count_star() = 1::fp64",
        "((20), (3), (1), (10), (5)) count_star() = 1::fp64",
        """DEFINE t1(fp32, fp32) = ((20, 20), (-3, -3), (1, 1), (10,10), (5,5))
            count_star() = 1::fp64""",
        "((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(col0::fp32, col1::fp32) = 1::fp64",
        """DEFINE t1(fp32, fp32) = ((20, -20), (-3, 3), (1, -1), (10, -10), (5, -5))
corr(t1.col0, t1.col1) = -11::fp64"
        """,
    ],
)
def test_parse_various_aggregate_scalar_func_argument_types(input_func_test):
    header = (
        make_aggregate_test_header("v1.0", "extensions/functions_arithmetic.yaml")
        + "# basic\n"
    )
    test_file = parse_string(header + input_func_test + "\n")
    assert len(test_file.testcases) == 1


@pytest.mark.parametrize(
    "func_name, func_args, func_ret, func_uri, expected_failure",
    [
        # lt for i8 with correct uri
        ("lt", ["i8", "i8"], "bool", "/extensions/functions_comparison.yaml", False),
        ("add", ["i8", "i8"], "i8", "/extensions/functions_arithmetic.yaml", False),
        (
            "add",
            ["dec", "dec"],
            "dec",
            "/extensions/functions_arithmetic_decimal.yaml",
            False,
        ),
        (
            "bitwise_xor",
            ["dec", "dec"],
            "dec",
            "/extensions/functions_arithmetic_decimal.yaml",
            False,
        ),
        # negative case, lt for i8 with wrong uri
        ("lt", ["i8", "i8"], "bool", "/extensions/functions_datetime.yaml", True),
        (
            "add",
            ["i8", "i8"],
            "i8",
            "/extensions/functions_arithmetic_decimal.yaml",
            True,
        ),
        ("add", ["dec", "dec"], "dec", "/extensions/functions_arithmetic.yaml", True),
        ("max", ["dec", "dec"], "dec", "/extensions/functions_arithmetic.yaml", True),
    ],
)
def test_uri_match_in_get_function(
    func_name, func_args, func_ret, func_uri, expected_failure
):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    extensions_path = os.path.join(script_dir, "../../extensions")
    registry = Extension.read_substrait_extensions(extensions_path)

    function = registry.get_function(func_name, func_uri, func_args, func_ret)
    assert (function is None) == expected_failure
