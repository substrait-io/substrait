# SPDX-License-Identifier: Apache-2.0
import json
from dataclasses import dataclass
from typing import List, Dict

from tests.coverage.coverage import TestCoverage
from tests.coverage.extensions import FunctionRegistry


@dataclass
class Registry:
    extension_count: int
    dependency_count: int
    function_count: int
    num_aggregate_functions: int
    num_scalar_functions: int
    num_window_functions: int
    num_function_overloads: int


@dataclass
class Coverage:
    total_test_count: int
    num_function_variants: int
    num_covered_function_variants: int

    def num_function_variants_without_coverage(self):
        return self.num_function_variants - self.num_covered_function_variants


@dataclass
class Baseline:
    registry: Registry
    coverage: Coverage

    @classmethod
    def from_dict(cls, data: Dict):
        registry_data = data["registry"]
        test_coverage_data = data["coverage"]
        registry = Registry(**registry_data)
        coverage = Coverage(**test_coverage_data)
        return cls(registry, coverage)

    def num_function_variants_without_coverage(self):
        return self.coverage.num_function_variants_without_coverage()

    def validate_against(self, expected):
        errors = []

        if self.registry.extension_count < expected.registry.extension_count:
            errors.append(
                f"Extension count mismatch: expected {expected.registry.extension_count}, got {self.registry.extension_count}"
            )
        if self.registry.dependency_count < expected.registry.dependency_count:
            errors.append(
                f"Dependency count mismatch: expected {expected.registry.dependency_count}, got {self.registry.dependency_count}"
            )
        if self.registry.function_count < expected.registry.function_count:
            errors.append(
                f"Function count mismatch: expected {expected.registry.function_count}, got {self.registry.function_count}"
            )
        if (
            self.registry.num_aggregate_functions
            < expected.registry.num_aggregate_functions
        ):
            errors.append(
                f"Aggregate function count mismatch: expected {expected.registry.num_aggregate_functions}, got {self.registry.num_aggregate_functions}"
            )
        if self.registry.num_scalar_functions < expected.registry.num_scalar_functions:
            errors.append(
                f"Scalar function count mismatch: expected {expected.registry.num_scalar_functions}, got {self.registry.num_scalar_functions}"
            )
        if self.registry.num_window_functions < expected.registry.num_window_functions:
            errors.append(
                f"Window function count mismatch: expected {expected.registry.num_window_functions}, got {self.registry.num_window_functions}"
            )
        if (
            self.registry.num_function_overloads
            < expected.registry.num_function_overloads
        ):
            errors.append(
                f"Function overload count mismatch: expected {expected.registry.num_function_overloads}, got {self.registry.num_function_overloads}"
            )

        if self.coverage.total_test_count < expected.coverage.total_test_count:
            errors.append(
                f"Total test count mismatch: expected {expected.coverage.total_test_count}, got {self.coverage.total_test_count}"
            )
        if (
            self.coverage.num_function_variants
            < expected.coverage.num_function_variants
        ):
            errors.append(
                f"Total function variants mismatch: expected {expected.coverage.num_function_variants}, got {self.coverage.num_function_variants}"
            )
        if (
            self.coverage.num_covered_function_variants
            < expected.coverage.num_covered_function_variants
        ):
            errors.append(
                f"Covered function variants mismatch: expected {expected.coverage.num_covered_function_variants}, got {self.coverage.num_covered_function_variants}"
            )

        expected_coverage_gap = expected.num_function_variants_without_coverage()
        actual_coverage_gap = self.num_function_variants_without_coverage()
        if actual_coverage_gap > expected_coverage_gap:
            errors.append(
                f"Coverage gap too large: {actual_coverage_gap} function variants with no tests, "
                f"out of {self.coverage.num_function_variants} total function variants. "
                f"New functions should be added along with test cases that illustrate their behavior."
            )

        return errors


def read_baseline_file(file_path: str) -> Baseline:
    with open(file_path, "r") as file:
        data = json.load(file)
    return Baseline.from_dict(data)


def generate_baseline(registry: FunctionRegistry, coverage: TestCoverage):
    registry_data = Registry(
        extension_count=len(registry.extensions),
        dependency_count=len(registry.dependencies),
        function_count=len(registry.registry),
        num_aggregate_functions=len(registry.aggregate_functions),
        num_scalar_functions=len(registry.scalar_functions),
        num_window_functions=len(registry.window_functions),
        num_function_overloads=sum([len(f) for f in registry.registry.values()]),
    )
    test_coverage_data = Coverage(
        total_test_count=coverage.test_count,
        num_function_variants=coverage.total_function_variants,
        num_covered_function_variants=coverage.num_covered_function_variants,
    )
    return Baseline(registry=registry_data, coverage=test_coverage_data)
