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
