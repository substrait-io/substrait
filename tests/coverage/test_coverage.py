# SPDX-License-Identifier: Apache-2.0
import os

import pytest
from antlr4 import InputStream
from tests.parser import parse_stream
from tests.coverage.coverage import get_test_coverage, validate_nullability
from tests.coverage.extensions import Extension, validate_impl_nullability_markers


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


def get_absolute_path(relative_path):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(script_dir, relative_path)


def test_coverage_accepts_multiple_known_dependencies():
    header = """### SUBSTRAIT_SCALAR_TEST: v1.0
### SUBSTRAIT_INCLUDE: extension:io.substrait:functions_arithmetic
### SUBSTRAIT_DEPENDENCY: extension:io.substrait:functions_comparison
### SUBSTRAIT_DEPENDENCY: extension:io.substrait:functions_boolean

"""
    test_file = parse_string(header + "# basic\nadd(1::i8, 2::i8) = 3::i8\n")
    registry = Extension.read_substrait_extensions(
        get_absolute_path("../../extensions")
    )

    coverage = get_test_coverage([test_file], registry)

    assert coverage.num_tests_with_no_matching_function == 0


def test_coverage_rejects_unknown_dependency():
    header = """### SUBSTRAIT_SCALAR_TEST: v1.0
### SUBSTRAIT_INCLUDE: extension:io.substrait:functions_arithmetic
### SUBSTRAIT_DEPENDENCY: extension:io.substrait:functions_does_not_exist

"""
    test_file = parse_string(header + "# basic\nadd(1::i8, 2::i8) = 3::i8\n")
    registry = Extension.read_substrait_extensions(
        get_absolute_path("../../extensions")
    )

    with pytest.raises(
        ValueError,
        match="Unknown extension URN: extension:io.substrait:functions_does_not_exist",
    ):
        get_test_coverage([test_file], registry)


@pytest.mark.parametrize(
    "func_name, func_args, func_ret, func_urn, expected_failure",
    [
        # lt for i8 with correct urn
        (
            "lt",
            ["i8", "i8"],
            "bool",
            "extension:io.substrait:functions_comparison",
            False,
        ),
        (
            "add",
            ["i8", "i8"],
            "i8",
            "extension:io.substrait:functions_arithmetic",
            False,
        ),
        (
            "add",
            ["dec", "dec"],
            "dec",
            "extension:io.substrait:functions_arithmetic_decimal",
            False,
        ),
        (
            "bitwise_xor",
            ["dec", "dec"],
            "dec",
            "extension:io.substrait:functions_arithmetic_decimal",
            False,
        ),
        # negative case, lt for i8 with wrong urn
        ("lt", ["i8", "i8"], "bool", "extension:io.substrait:functions_datetime", True),
        (
            "add",
            ["i8", "i8"],
            "i8",
            "extension:io.substrait:functions_arithmetic_decimal",
            True,
        ),
        (
            "add",
            ["dec", "dec"],
            "dec",
            "extension:io.substrait:functions_arithmetic",
            True,
        ),
        (
            "max",
            ["dec", "dec"],
            "dec",
            "extension:io.substrait:functions_arithmetic",
            True,
        ),
    ],
)
def test_urn_match_in_get_function(
    func_name, func_args, func_ret, func_urn, expected_failure
):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    extensions_path = os.path.join(script_dir, "../../extensions")
    registry = Extension.read_substrait_extensions(extensions_path)

    function = registry.get_function(func_name, func_urn, func_args, func_ret)
    assert (function is None) == expected_failure


class TestNullabilityValidation:
    """Tests for validate_nullability covering MIRROR, DECLARED_OUTPUT, and DISCRETE rules."""

    @staticmethod
    def _registry():
        script_dir = os.path.dirname(os.path.abspath(__file__))
        extensions_path = os.path.join(script_dir, "../../extensions")
        return Extension.read_substrait_extensions(extensions_path)

    def test_mirror_nullable_input_requires_nullable_output(self):
        """MIRROR: if any arg is nullable, the output must be nullable."""
        header = make_header("v1.0", "extension:io.substrait:functions_boolean")
        test_file = parse_string(
            header
            + """\
# basic
and(true::bool, null::bool?) = false::bool
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert len(errors) == 1
        assert "MIRROR" in errors[0]
        assert "should be nullable" in errors[0]

    def test_mirror_nullable_input_with_nullable_output_ok(self):
        """MIRROR: nullable input + nullable output is correct."""
        header = make_header("v1.0", "extension:io.substrait:functions_boolean")
        test_file = parse_string(
            header
            + """\
# basic
and(true::bool, null::bool?) = false::bool?
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert errors == []

    def test_mirror_non_nullable_input_non_nullable_output_ok(self):
        """MIRROR: all non-nullable inputs + non-nullable output is correct."""
        header = make_header("v1.0", "extension:io.substrait:functions_boolean")
        test_file = parse_string(
            header
            + """\
# basic
and(true::bool, false::bool) = false::bool
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert errors == []

    def test_declared_output_requires_nullable_when_declared(self):
        """DECLARED_OUTPUT: bool_and declares boolean? return — output must be nullable."""
        header = make_aggregate_test_header(
            "v1.0", "extension:io.substrait:functions_boolean"
        )
        test_file = parse_string(
            header
            + """\
# basic
bool_and((true, false)::bool) = false::bool
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert len(errors) == 1
        assert "DECLARED_OUTPUT" in errors[0]

    def test_declared_output_nullable_return_ok(self):
        """DECLARED_OUTPUT: bool_and with nullable output is correct."""
        header = make_aggregate_test_header(
            "v1.0", "extension:io.substrait:functions_boolean"
        )
        test_file = parse_string(
            header
            + """\
# basic
bool_and((true, false)::bool) = false::bool?
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert errors == []

    def test_declared_output_non_nullable_when_declared_non_nullable(self):
        """DECLARED_OUTPUT: is_null declares non-nullable boolean return — nullable output is wrong."""
        header = make_header("v1.0", "extension:io.substrait:functions_comparison")
        test_file = parse_string(
            header
            + """\
# basic
is_null(null::i8?) = true::bool?
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert len(errors) == 1
        assert "should not be nullable" in errors[0]

    def test_error_results_are_skipped(self):
        """Error results (<!ERROR>) should not be checked for nullability."""
        header = make_header("v1.0", "extension:io.substrait:functions_arithmetic")
        test_file = parse_string(
            header
            + """\
# basic
add(120::i8, 10::i8) [overflow:ERROR] = <!ERROR>
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert errors == []

    def test_mirror_options_skip_false_positive(self):
        """MIRROR with function options: nullable output with non-nullable args is allowed
        when options are present (e.g. on_domain_error:NONE can produce null)."""
        header = make_header("v1.0", "extension:io.substrait:functions_arithmetic")
        test_file = parse_string(
            header
            + """\
# basic
divide(5::i8, 0::i8) [on_division_by_zero:NAN] = null::i8?
"""
        )
        errors = validate_nullability(test_file, self._registry())
        assert errors == []


class TestDeclarationNullabilityMarkers:
    """Tests for validate_impl_nullability_markers, which rejects nullability
    markers that the declared nullability handling would ignore."""

    def test_mirror_rejects_return_marker(self):
        """MIRROR: the return marker is ignored, so it must not be declared."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean", "name": "a"}],
                "nullability": "MIRROR",
                "return": "boolean?",
            },
            "functions_boolean.yaml: not",
        )
        assert len(errors) == 1
        assert "boolean?" in errors[0]

    def test_mirror_rejects_argument_marker(self):
        """MIRROR: argument nullability is stripped before binding."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean?", "name": "a"}],
                "nullability": "MIRROR",
                "return": "boolean",
            },
            "functions_boolean.yaml: not",
        )
        assert len(errors) == 1
        assert "argument 'a'" in errors[0]

    def test_mirror_without_markers_ok(self):
        """MIRROR: a declaration with no markers is what the spec requires."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean", "name": "a"}],
                "nullability": "MIRROR",
                "return": "boolean",
            },
            "functions_boolean.yaml: not",
        )
        assert errors == []

    def test_nullability_defaults_to_mirror(self):
        """An impl with no nullability key is MIRROR, so the return marker is ignored."""
        errors = validate_impl_nullability_markers(
            {"args": [{"value": "i8", "name": "x"}], "return": "i8?"},
            "functions_test.yaml: f",
        )
        assert len(errors) == 1
        assert "MIRROR" in errors[0]

    def test_declared_output_allows_return_marker(self):
        """DECLARED_OUTPUT: the return marker is authoritative, not ignored."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean", "name": "a"}],
                "nullability": "DECLARED_OUTPUT",
                "return": "boolean?",
            },
            "functions_boolean.yaml: bool_and",
        )
        assert errors == []

    def test_declared_output_rejects_argument_marker(self):
        """DECLARED_OUTPUT: argument nullability is still stripped before binding."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean?", "name": "a"}],
                "nullability": "DECLARED_OUTPUT",
                "return": "boolean?",
            },
            "functions_boolean.yaml: bool_and",
        )
        assert len(errors) == 1
        assert "DECLARED_OUTPUT" in errors[0]

    def test_discrete_is_exempt(self):
        """DISCRETE: markers are meaningful on both sides."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "boolean?", "name": "a"}],
                "nullability": "DISCRETE",
                "return": "boolean?",
            },
            "functions_test.yaml: f",
        )
        assert errors == []

    def test_ternary_in_derivation_expression_is_not_a_marker(self):
        """A '?' in a derivation expression's intermediate lines is a ternary
        operator, not a nullability marker."""
        errors = validate_impl_nullability_markers(
            {
                "args": [
                    {"value": "decimal<P1,S1>", "name": "x"},
                    {"value": "decimal<P2,S2>", "name": "y"},
                ],
                "return": (
                    "init_scale = max(S1,S2)\n"
                    "init_prec = init_scale + max(P1 - S1, P2 - S2) + 1\n"
                    "delta = init_prec - 38\n"
                    "scale = init_prec > 38 ? scale_after_borrow : init_scale\n"
                    "DECIMAL<prec, scale>"
                ),
            },
            "functions_arithmetic_decimal.yaml: add",
        )
        assert errors == []

    def test_derivation_expression_return_marker_is_rejected(self):
        """The final line of a derivation expression is still checked."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "decimal<P,S>", "name": "x"}],
                "return": ("precision = min(P - S + 1, 38)\ndecimal?<precision, 0>"),
            },
            "functions_rounding_decimal.yaml: ceil",
        )
        assert len(errors) == 1
        assert "decimal?<precision, 0>" in errors[0]

    def test_nested_markers_are_allowed(self):
        """Only outermost nullability is ignored; nested markers are meaningful."""
        errors = validate_impl_nullability_markers(
            {
                "args": [
                    {"value": "list<any1>", "name": "x"},
                    {"value": "func<any1 -> boolean?>", "name": "p"},
                ],
                "return": "list<any1>",
            },
            "functions_list.yaml: filter",
        )
        assert errors == []

    def test_option_arguments_are_skipped(self):
        """An option (enum) argument has no 'value' key."""
        errors = validate_impl_nullability_markers(
            {
                "args": [{"value": "i8", "name": "x"}, {"options": ["A", "B"]}],
                "return": "i8",
            },
            "functions_test.yaml: f",
        )
        assert errors == []

    def test_zero_argument_mirror_rejects_return_marker(self):
        """MIRROR with no arguments derives a non-nullable return type, so a
        return marker is ignored and must not be declared."""
        errors = validate_impl_nullability_markers(
            {"args": [], "nullability": "MIRROR", "return": "i64?"},
            "functions_test.yaml: count",
        )
        assert len(errors) == 1
        assert "i64?" in errors[0]

    def test_zero_argument_without_args_key(self):
        """An impl that declares no arguments may omit the 'args' key."""
        errors = validate_impl_nullability_markers(
            {"nullability": "MIRROR", "return": "i64"},
            "functions_test.yaml: count",
        )
        assert errors == []
