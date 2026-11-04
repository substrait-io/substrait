# SPDX-License-Identifier: Apache-2.0
import pytest

from tests.helpers import (
    make_aggregate_test_header,
    make_scalar_header,
    make_window_test_header,
    parse_string,
)
from tests.parser import ParseError
from tests.parser.nodes import AggregateArgument, CaseLiteral

UUID_TEXT = "00112233-4455-6677-8899-aabbccddeeff"


@pytest.mark.parametrize(
    "value",
    [
        UUID_TEXT,
        UUID_TEXT.upper(),
        "00112233-4455-6677-8899-AaBbCcDdEeFf",
        "00000000-0000-0000-0000-000000000000",
        "ffffffff-ffff-ffff-ffff-ffffffffffff",
        "01234567-89ab-cdef-0123-456789abcdef",
    ],
)
@pytest.mark.parametrize("datatype", ["uuid", "uuid?", "UUID", "UUID?"])
def test_uuid_scalar_literals(value, datatype):
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    test_file = parse_string(
        header + f"coalesce({value}::{datatype}) = {value}::{datatype}\n"
    )
    expected = CaseLiteral(value, datatype.lower(), nullable=datatype.endswith("?"))
    assert len(test_file.testcases) == 1
    assert test_file.testcases[0].args == [expected]
    assert test_file.testcases[0].result == expected


def test_uuid_null_literal():
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    test_file = parse_string(header + "coalesce(null::uuid?) = null::uuid?\n")
    expected = CaseLiteral(None, "uuid?", nullable=True)
    assert test_file.testcases[0].args == [expected]
    assert test_file.testcases[0].result == expected


@pytest.mark.parametrize(
    "value",
    [
        "00112233445566778899aabbccddeeff",
        "0011223-4455-6677-8899-aabbccddeeff",
        "001122334-4455-6677-8899-aabbccddeeff",
        "00112233-4455-6677-8899-aabbccddeef",
        "00112233-4455-6677-8899-aabbccddeeff0",
        "00112233-4455-6677-8899-aabbccddeefg",
        "00112233-4455-6677-8899-aabbccddeef@f",
        f"{UUID_TEXT}@",
        "00112233-4455-6677-8899-aabbcc ddee ff",
        f"'{UUID_TEXT}'",
        f"{{{UUID_TEXT}}}",
        f"urn:uuid:{UUID_TEXT}",
    ],
)
def test_invalid_uuid_literals(value):
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    with pytest.raises(ParseError, match="Syntax errors"):
        parse_string(header + f"coalesce({value}::uuid) = {UUID_TEXT}::uuid\n")


def test_uuid_null_requires_nullable_type():
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    with pytest.raises(ParseError, match="Null literal must have a nullable type"):
        parse_string(header + "coalesce(null::uuid) = null::uuid?\n")


@pytest.mark.parametrize(
    "literal, datatype, expected_value",
    [
        (f"[{UUID_TEXT}]", "list<uuid>", [UUID_TEXT]),
        (f"({UUID_TEXT})", "struct<uuid>", [UUID_TEXT]),
        (
            f"{{{UUID_TEXT}: 1}}",
            "map<uuid,i32>",
            [{"key": UUID_TEXT, "value": "1"}],
        ),
        (
            f"([{UUID_TEXT}])",
            "struct<list<uuid>>",
            [[UUID_TEXT]],
        ),
    ],
)
def test_uuid_compound_literals(literal, datatype, expected_value):
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    test_file = parse_string(
        header + f"coalesce({literal}::{datatype}) = {literal}::{datatype}\n"
    )
    expected = CaseLiteral(expected_value, datatype)
    assert test_file.testcases[0].args == [expected]
    assert test_file.testcases[0].result == expected


def test_uuid_aggregate_literals():
    header = make_aggregate_test_header(
        "v1.0", "extension:io.substrait:functions_aggregate_generic"
    )
    test_file = parse_string(header + f"count(({UUID_TEXT}, null)::uuid?) = 1::i64\n")
    assert test_file.testcases[0].args == [
        CaseLiteral([UUID_TEXT, "null"], "uuid?", nullable=True)
    ]
    assert test_file.testcases[0].result == CaseLiteral("1", "i64")


def test_uuid_window_literals():
    header = make_window_test_header("v1.0", "extension:io.substrait:functions_window")
    test_file = parse_string(
        header
        + f"DEFINE frame(uuid?) = (({UUID_TEXT}), (null))\n"
        + f"lag(col0) OVER frame = (null, {UUID_TEXT})::uuid?\n"
    )
    testcase = test_file.testcases[0]
    assert testcase.rows == [[UUID_TEXT], ["null"]]
    assert testcase.args == [AggregateArgument("col0", "uuid?", "", None)]
    assert testcase.result == CaseLiteral(["null", UUID_TEXT], "uuid?", nullable=True)


def test_uuid_shaped_string_remains_a_string():
    header = make_scalar_header("v1.0", "extension:io.substrait:functions_comparison")
    test_file = parse_string(
        header + f"coalesce('{UUID_TEXT}'::str) = '{UUID_TEXT}'::str\n"
    )
    expected = CaseLiteral(f"'{UUID_TEXT}'", "str")
    assert test_file.testcases[0].args == [expected]
    assert test_file.testcases[0].result == expected
