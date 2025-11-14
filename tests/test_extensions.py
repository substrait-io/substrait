# SPDX-License-Identifier: Apache-2.0
import json
import os
from dataclasses import asdict

from tests.baseline import read_baseline_file, generate_baseline
from tests.coverage.case_file_parser import load_all_testcases
from tests.coverage.coverage import get_test_coverage
from tests.coverage.extensions import build_type_to_short_type
from tests.coverage.extensions import Extension, FunctionRegistry
from tests.coverage.extensions import (
    parse_type_string,
    TypeVariable,
    SimpleType,
    LambdaType,
    ListType,
    StructType,
    ParameterizedType,
    types_match,
)


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
    assert FunctionRegistry.is_same_type(
        TypeVariable("T"), SimpleType("i32")
    )
    assert FunctionRegistry.is_same_type(
        TypeVariable("U"), SimpleType("str")
    )
    assert FunctionRegistry.is_same_type(
        TypeVariable("V"), SimpleType("fp64")
    )
    assert FunctionRegistry.is_same_type(
        TypeVariable("T"), ListType(SimpleType("i32"))
    )
    assert FunctionRegistry.is_same_type(
        TypeVariable("U"),
        LambdaType(SimpleType("i32"), SimpleType("bool"))
    )

    assert FunctionRegistry.is_same_type(
        SimpleType("i32"), SimpleType("i32")
    )
    assert FunctionRegistry.is_same_type(
        SimpleType("str"), SimpleType("str")
    )

    assert not FunctionRegistry.is_same_type(
        SimpleType("i32"), SimpleType("str")
    )
    assert not FunctionRegistry.is_same_type(
        SimpleType("fp32"), SimpleType("fp64")
    )


def test_lambda_type_variable_matching():
    """Test proper type variable matching in lambda signatures"""
    # Different type variables (T -> U) should match different concrete types
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("U")),
        LambdaType(SimpleType("i32"), SimpleType("str"))
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("U")),
        LambdaType(SimpleType("i32"), SimpleType("bool"))
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("U")),
        LambdaType(SimpleType("str"), SimpleType("fp64"))
    )

    # Same type variable (T -> T) should match when input and output are same
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("i32"), SimpleType("i32"))
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("str"), SimpleType("str"))
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("fp64"), SimpleType("fp64"))
    )

    # Same type variable should NOT match when input and output differ
    assert not FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("i32"), SimpleType("str"))
    )
    assert not FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("i32"), SimpleType("bool"))
    )
    assert not FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("str"), SimpleType("i32"))
    )


def test_lambda_struct_type_variable_matching():
    """Test type variable matching with struct parameters"""
    # struct<T, T> means both elements must be same type
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("i32")]),
            SimpleType("i32")
        )
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("str"), SimpleType("str")]),
            SimpleType("i32")
        )
    )

    # struct<T, T> should NOT match when struct elements differ
    assert not FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("str")]),
            SimpleType("i32")
        )
    )
    assert not FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("fp64")]),
            SimpleType("i32")
        )
    )

    # struct<T, U> means elements can be different types
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("U")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("str")]),
            SimpleType("i32")
        )
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("U")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("i32")]),
            SimpleType("i32")
        )
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("U")]),
            SimpleType("i32")
        ),
        LambdaType(
            StructType([SimpleType("str"), SimpleType("fp64")]),
            SimpleType("i32")
        )
    )


def test_lambda_return_type_variable_matching():
    """Test type variable matching in return types"""
    # Return type with type variable from input
    assert FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("i32"), SimpleType("i32"))
    )
    assert FunctionRegistry.is_same_type(
        LambdaType(
            ListType(TypeVariable("T")),
            TypeVariable("T")
        ),
        LambdaType(
            ListType(SimpleType("i32")),
            SimpleType("i32")
        )
    )

    # Return type should match when type variable is consistent
    assert FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            TypeVariable("T")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("i32")]),
            SimpleType("i32")
        )
    )

    # Return type should NOT match when type variable is inconsistent
    assert not FunctionRegistry.is_same_type(
        LambdaType(
            StructType([TypeVariable("T"), TypeVariable("T")]),
            TypeVariable("T")
        ),
        LambdaType(
            StructType([SimpleType("i32"), SimpleType("i32")]),
            SimpleType("str")
        )
    )
    assert not FunctionRegistry.is_same_type(
        LambdaType(TypeVariable("T"), TypeVariable("T")),
        LambdaType(SimpleType("i32"), SimpleType("str"))
    )


def test_parse_type_variable():
    """Test parsing type variables"""
    assert parse_type_string("T") == TypeVariable("T")


def test_parse_simple_type():
    """Test parsing simple types"""
    assert parse_type_string("i32") == SimpleType("i32")
    # Note: ANTLR parser normalizes "boolean" to "bool"
    assert parse_type_string("boolean") == SimpleType("bool")


def test_parse_parameterized_type():
    """Test parsing parameterized types like list<i32>"""
    result = parse_type_string("list<i32>")
    # ANTLR parser should give us ListType, not ParameterizedType
    from tests.coverage.extensions import ListType
    assert result == ListType(SimpleType("i32"))


def test_parse_struct_type():
    """Test parsing struct types with multiple parameters"""
    result = parse_type_string("struct<i32,str>")
    # ANTLR parser should give us StructType, not ParameterizedType
    from tests.coverage.extensions import StructType
    assert result == StructType([SimpleType("i32"), SimpleType("str")])


def test_parse_lambda_type():
    """Test parsing lambda types"""
    assert parse_type_string("lambda<i32->str>") == LambdaType(
        SimpleType("i32"),
        SimpleType("str")
    )


def test_parse_lambda_with_type_variables():
    """Test parsing lambda with type variables"""
    assert parse_type_string("lambda<T->U>") == LambdaType(
        TypeVariable("T"),
        TypeVariable("U")
    )


def test_parse_nested_lambda_type():
    """Test parsing lambda with struct parameter"""
    from tests.coverage.extensions import StructType
    assert parse_type_string("lambda<struct<i32,i32>->i32>") == LambdaType(
        StructType([SimpleType("i32"), SimpleType("i32")]),
        SimpleType("i32")
    )


def test_types_match_with_bindings():
    """Test type matching with variable bindings"""
    # T should bind to i32
    pattern = parse_type_string("T")
    concrete = parse_type_string("i32")
    bindings = {}
    assert types_match(pattern, concrete, bindings)
    assert bindings["T"] == concrete

    # Second T should match the same binding
    pattern2 = parse_type_string("T")
    assert types_match(pattern2, concrete, bindings)

    # T should NOT match str if already bound to i32
    concrete2 = parse_type_string("str")
    assert not types_match(pattern2, concrete2, bindings)
