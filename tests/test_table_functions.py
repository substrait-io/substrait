# SPDX-License-Identifier: Apache-2.0

import pytest
import yaml
from tests.coverage.extensions import validate_nullability_markers
from tests.helpers import iter_function_impls, load_extension
from tests.type.antlr_parser.SubstraitTypeParser import SubstraitTypeParser
from tests.type.test_type_grammar import extension_yaml_files, parse_type_expression


def validate_table_function_schema(schema):
    """Require a schema whose resulting type is a non-nullable struct.

    Parentheses are unwrapped, and a multiline derivation is checked by its
    final type, since neither needs the intermediate assignments evaluated.
    """
    if isinstance(schema, dict):
        return
    expression = parse_type_expression(schema).expr()
    while isinstance(expression, SubstraitTypeParser.ParenExpressionContext):
        expression = expression.expr()
    if isinstance(expression, SubstraitTypeParser.MultilineDefinitionContext):
        output_type = expression.finalType
    elif isinstance(expression, SubstraitTypeParser.TypeLiteralContext):
        output_type = expression.typeDef()
    else:
        raise AssertionError("Table-function schema must be a non-nullable struct")
    output = output_type.parameterizedType()
    assert (
        isinstance(
            output,
            (SubstraitTypeParser.StructContext, SubstraitTypeParser.NStructContext),
        )
        and output.isnull is None
    ), "Table-function schema must be a non-nullable struct"


@pytest.mark.parametrize(
    "schema",
    [
        "i64",
        "struct?<i64>",
        "(struct?<i64>)",
        "width = 10\ni64",
        "width = 10\nstruct?<i64>",
        "any1",
        "any1?",
        "42",
        "if P > 0 then struct<i64> else struct<string>",
    ],
)
def test_reject_invalid_table_function_schema(schema):
    with pytest.raises(AssertionError, match="non-nullable struct"):
        validate_table_function_schema(schema)


@pytest.mark.parametrize(
    "schema",
    [
        "struct<i64?>",
        "(struct<i64>)",
        {"value": "i64"},
        "width = 10\nstruct<varchar<width>>",
    ],
)
def test_accept_valid_table_function_schema(schema):
    validate_table_function_schema(schema)


def test_extension_table_function_schemas():
    for path in extension_yaml_files():
        for function, impl in iter_function_impls(
            load_extension(path), sections=("table_functions",)
        ):
            if "schema" in impl:
                try:
                    validate_table_function_schema(impl["schema"])
                except AssertionError as error:
                    pytest.fail(f"{path}: {function['name']}: {error}")


@pytest.mark.parametrize(
    "argument_type, expected_error_count",
    [("list?<i64>", 2), ("list<i64?>", 0)],
)
def test_table_function_declarations_use_declared_output_validation(
    tmp_path, argument_type, expected_error_count
):
    extension = {
        "urn": "extension:org.example:table_functions",
        "table_functions": [
            {
                "name": "generate",
                "impls": [
                    {
                        "args": [{"name": "input", "value": argument_type}],
                        "schema": "struct<i64?>",
                    },
                    {"args": [{"name": "input", "value": argument_type}]},
                ],
            }
        ],
    }
    (tmp_path / "functions_table.yaml").write_text(yaml.safe_dump(extension))
    errors = validate_nullability_markers(tmp_path)
    assert len(errors) == expected_error_count
    assert all("DECLARED_OUTPUT" in error for error in errors)
