# SPDX-License-Identifier: Apache-2.0
import json
import os
from dataclasses import asdict

from tests.baseline import read_baseline_file, generate_baseline
from tests.coverage.case_file_parser import load_all_testcases
from tests.coverage.coverage import get_test_coverage
from tests.coverage.extensions import build_type_to_short_type
from tests.coverage.extensions import Extension


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
