# SPDX-License-Identifier: Apache-2.0
import json
import os
from dataclasses import asdict

from tests.baseline import read_baseline_file, generate_baseline
from tests.coverage.case_file_parser import load_all_testcases
from tests.coverage.coverage import get_test_coverage
from tests.coverage.extensions import build_type_to_short_type
from tests.coverage.extensions import Extension


def compare_baselines(expected, actual):
    errors = []

    if actual.registry.extension_count < expected.registry.extension_count:
        errors.append(
            f"Extension count mismatch: expected {expected.registry.extension_count}, got {actual.registry.extension_count}"
        )
    if actual.registry.dependency_count < expected.registry.dependency_count:
        errors.append(
            f"Dependency count mismatch: expected {expected.registry.dependency_count}, got {actual.registry.dependency_count}"
        )
    if actual.registry.function_count < expected.registry.function_count:
        errors.append(
            f"Function count mismatch: expected {expected.registry.function_count}, got {actual.registry.function_count}"
        )
    if (
        actual.registry.num_aggregate_functions
        < expected.registry.num_aggregate_functions
    ):
        errors.append(
            f"Aggregate function count mismatch: expected {expected.registry.num_aggregate_functions}, got {actual.registry.num_aggregate_functions}"
        )
    if actual.registry.num_scalar_functions < expected.registry.num_scalar_functions:
        errors.append(
            f"Scalar function count mismatch: expected {expected.registry.num_scalar_functions}, got {actual.registry.num_scalar_functions}"
        )
    if actual.registry.num_window_functions < expected.registry.num_window_functions:
        errors.append(
            f"Window function count mismatch: expected {expected.registry.num_window_functions}, got {actual.registry.num_window_functions}"
        )
    if (
        actual.registry.num_function_overloads
        < expected.registry.num_function_overloads
    ):
        errors.append(
            f"Function overload count mismatch: expected {expected.registry.num_function_overloads}, got {actual.registry.num_function_overloads}"
        )

    if actual.coverage.total_test_count < expected.coverage.total_test_count:
        errors.append(
            f"Total test count mismatch: expected {expected.coverage.total_test_count}, got {actual.coverage.total_test_count}"
        )
    if actual.coverage.num_function_variants < expected.coverage.num_function_variants:
        errors.append(
            f"Total function variants mismatch: expected {expected.coverage.num_function_variants}, got {actual.coverage.num_function_variants}"
        )
    if (
        actual.coverage.num_covered_function_variants
        < expected.coverage.num_covered_function_variants
    ):
        errors.append(
            f"Covered function variants mismatch: expected {expected.coverage.num_covered_function_variants}, got {actual.coverage.num_covered_function_variants}"
        )

    expected_coverage_gap = expected.num_function_variants_without_coverage()
    actual_coverage_gap = actual.num_function_variants_without_coverage()
    if actual_coverage_gap > expected_coverage_gap:
        errors.append(
            f"Coverage gap too large: {actual_coverage_gap} function variants with no tests, "
            f"out of {actual.coverage.num_function_variants} total function variants. "
            f"New functions should be added along with test cases that illustrate their behavior."
        )

    return errors


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
    errors = compare_baselines(baseline, actual_baseline)
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
