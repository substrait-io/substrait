# SPDX-License-Identifier: Apache-2.0

import pytest
import yaml
from tests.coverage.extensions import validate_nullability_markers
from tests.helpers import iter_function_impls, load_extension
from tests.type.antlr_parser.SubstraitTypeParser import SubstraitTypeParser
from tests.type.test_type_grammar import extension_yaml_files, parse_type_expression


def validate_table_function_schema(expression):
    """Check concrete schema outputs, leaving binding-dependent expressions to consumers."""
    output = parse_type_expression(expression).expr()
    while isinstance(output, SubstraitTypeParser.ParenExpressionContext):
        output = output.expr()
    if isinstance(output, SubstraitTypeParser.MultilineDefinitionContext):
        output = output.finalType
    elif isinstance(output, SubstraitTypeParser.TypeLiteralContext):
        output = output.typeDef()
    else:
        return

    if output.anyType() is not None:
        return
    struct = output.parameterizedType()
    assert (
        isinstance(
            struct,
            (SubstraitTypeParser.StructContext, SubstraitTypeParser.NStructContext),
        )
        and struct.isnull is None
    ), "Table-function schema must be a non-nullable struct"


@pytest.mark.parametrize(
    "expression, is_valid",
    [
        ("i64", False),
        ("struct?<i64>", False),
        ("list<i64>", False),
        ("struct<i64?>", True),
        ("(struct<any1>)", True),
        ("NSTRUCT<value: i64>", True),
        ("width = 10\nstruct<varchar<width>>", True),
        ("width = 10\nvarchar<width>", False),
        ("any1", True),
        ("if P > 0 then struct<i64> else struct<string>", True),
    ],
)
def test_table_function_schema_types(expression, is_valid):
    if is_valid:
        validate_table_function_schema(expression)
    else:
        with pytest.raises(AssertionError, match="non-nullable struct"):
            validate_table_function_schema(expression)


def test_extension_table_function_schemas():
    """Check declared schemas in extension catalogs and documentation examples."""
    for path in extension_yaml_files():
        extension = load_extension(path)
        for function, impl in iter_function_impls(
            extension, sections=("table_functions",)
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
