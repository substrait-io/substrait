# SPDX-License-Identifier: Apache-2.0
import json
import os
from dataclasses import asdict

from tests.baseline import read_baseline_file, generate_baseline
from tests.coverage.case_file_parser import load_all_testcases
from tests.coverage.coverage import get_test_coverage
from tests.coverage.extensions import build_type_to_short_type
from tests.coverage.extensions import Extension, FunctionRegistry


# NOTE: this test is run as part of pre-commit hook
def test_substrait_extension_coverage():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    baseline = read_baseline_file(os.path.join(script_dir, "baseline.json"))
    extensions_path = os.path.join(script_dir, "../extensions")
    registry = Extension.read_substrait_extensions(extensions_path)

    test_case_dir = os.path.join(script_dir, "./cases")
    all_test_files = load_all_testcases(test_case_dir)
    coverage = get_test_coverage(all_test_files, registry)

    assert (
        coverage.num_tests_with_no_matching_function == 0
    ), f"{coverage.num_tests_with_no_matching_function} tests with no matching function"

    actual_baseline = generate_baseline(registry, coverage)
    errors = actual_baseline.validate_against(baseline)
    assert not errors, (
        "\n".join(errors)
        + f"The baseline file does not match the current test coverage. "
        f"Please update the file at tests/baseline.json to align with the current baseline"
        f"{json.dumps(asdict(actual_baseline), indent=2)}"
    )

    if baseline != actual_baseline:
        print("\nBaseline has changed, updating tests/baseline.json")
        print(json.dumps(asdict(actual_baseline), indent=2))


def test_build_type_to_short_type():
    long_to_short = build_type_to_short_type()
    assert long_to_short["i64"] == "i64"
    assert long_to_short["fp64"] == "fp64"
    assert long_to_short["timestamp"] == "ts"
    assert long_to_short["timestamp_tz"] == "tstz"
    assert long_to_short["precision_timestamp"] == "pts"
    assert long_to_short["precision_timestamp_tz"] == "ptstz"
    assert long_to_short["interval_year"] == "iyear"
    assert long_to_short["interval_day"] == "iday"
    assert long_to_short["decimal"] == "dec"
    assert long_to_short["boolean"] == "bool"
    assert long_to_short["string"] == "str"
    assert long_to_short["binary"] == "vbin"
    assert long_to_short["fixedbinary"] == "fbin"
    assert long_to_short["fixedchar"] == "fchar"
    assert long_to_short["varchar"] == "vchar"
    assert long_to_short["list"] == "list"
    assert long_to_short["map"] == "map"
    assert long_to_short["struct"] == "struct"


def test_is_type_variable():
    """Test the is_type_variable helper method"""
    assert Extension.is_type_variable("T")
    assert Extension.is_type_variable("U")
    assert Extension.is_type_variable("V")
    assert Extension.is_type_variable("W")
    assert Extension.is_type_variable("X")

    assert not Extension.is_type_variable("t")  # lowercase
    assert not Extension.is_type_variable("i32")  # multi-char
    assert not Extension.is_type_variable("str")  # multi-char
    assert not Extension.is_type_variable("TT")  # multi-char


def test_type_variables_recognized_by_get_short_type():
    """Test that single uppercase letters are preserved as type variables"""
    assert Extension.get_short_type("T") == "T"
    assert Extension.get_short_type("U") == "U"
    assert Extension.get_short_type("V") == "V"
    assert Extension.get_short_type("W") == "W"
    assert Extension.get_short_type("X") == "X"


def test_type_variables_match_concrete_types():
    """Test that type variables (single uppercase letters) match any concrete type"""
    assert FunctionRegistry.is_same_type("T", "i32")
    assert FunctionRegistry.is_same_type("U", "str")
    assert FunctionRegistry.is_same_type("V", "fp64")
    assert FunctionRegistry.is_same_type("T", "list<i32>")
    assert FunctionRegistry.is_same_type("U", "lambda<i32->boolean>")

    assert FunctionRegistry.is_same_type("i32", "i32")
    assert FunctionRegistry.is_same_type("str", "str")

    assert not FunctionRegistry.is_same_type("i32", "str")
    assert not FunctionRegistry.is_same_type("fp32", "fp64")


def test_lambda_types_match_regardless_of_parameters():
    """Test that lambda types match other lambda types"""
    assert FunctionRegistry.is_same_type("lambda<i32->i32>", "lambda<i32->i32>")
    assert FunctionRegistry.is_same_type("lambda<T->U>", "lambda<i32->boolean>")
    assert FunctionRegistry.is_same_type(
        "lambda<struct<i32,i32>->i32>",
        "lambda<struct<i32,i32>->i32>"
    )
