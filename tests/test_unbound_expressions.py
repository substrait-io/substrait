# SPDX-License-Identifier: Apache-2.0
from pathlib import Path

from antlr4 import CommonTokenStream, InputStream
from antlr4.error.ErrorListener import ErrorListener
from google.protobuf import text_format

from substrait import algebra_pb2, extended_expression_pb2, type_pb2
from tests.coverage.extensions import Extension
from tests.type.antlr_parser.SubstraitLexer import SubstraitLexer
from tests.type.antlr_parser.SubstraitTypeParser import SubstraitTypeParser


class ParseErrorListener(ErrorListener):
    def __init__(self):
        super().__init__()
        self.errors = []

    def syntaxError(self, recognizer, offending_symbol, line, column, msg, e):
        self.errors.append(f"Syntax error at line {line}, column {column}: {msg}")


def parse_textproto(path: str, message):
    text_format.Parse(Path(path).read_text(), message, allow_unknown_field=False)
    return message


def parse_type_statement(text: str):
    lexer = SubstraitLexer(InputStream(text))
    parser = SubstraitTypeParser(CommonTokenStream(lexer))
    error_listener = ParseErrorListener()
    parser.removeErrorListeners()
    parser.addErrorListener(error_listener)
    tree = parser.typeStatement()
    assert parser.getNumberOfSyntaxErrors() == 0, error_listener.errors
    return tree


def test_unknown_type_parser_accepts_nullable_and_non_nullable_forms():
    assert parse_type_statement("unknown").typeDef().getText() == "unknown"
    assert parse_type_statement("unknown?").typeDef().getText() == "unknown?"


def test_unknown_short_type_is_stable():
    assert Extension.get_short_type("unknown") == "unknown"
    assert Extension.get_short_type("unknown?") == "unknown"


def test_named_expression_example_contract():
    expr = parse_textproto(
        "site/examples/proto-textformat/unbound_expression/named_expression.textproto",
        algebra_pb2.Expression(),
    )

    assert expr.WhichOneof("rex_type") == "named_expression"
    assert list(expr.named_expression.names) == ["foo"]


def test_scalar_function_unknown_example_contract():
    expr = parse_textproto(
        "site/examples/proto-textformat/unbound_expression/scalar_function_unknown.textproto",
        algebra_pb2.Expression(),
    )

    assert expr.WhichOneof("rex_type") == "scalar_function"
    assert expr.scalar_function.function_reference == 1
    assert len(expr.scalar_function.arguments) == 2
    assert list(expr.scalar_function.arguments[0].value.named_expression.names) == ["a"]
    assert list(expr.scalar_function.arguments[1].value.named_expression.names) == ["b"]
    assert expr.scalar_function.output_type.WhichOneof("kind") == "unknown"
    assert (
        expr.scalar_function.output_type.unknown.nullability
        == type_pb2.Type.NULLABILITY_UNSPECIFIED
    )


def test_unbound_extended_expression_example_contract():
    expr = parse_textproto(
        "site/examples/proto-textformat/extended_expression/unbound_named_projection.textproto",
        extended_expression_pb2.ExtendedExpression(),
    )

    assert expr.version.producer == "substrait"
    assert len(expr.extension_urns) == 1
    assert expr.extension_urns[0].extension_urn_anchor == 1
    assert expr.extension_urns[0].urn == "extension:io.substrait:unknown"

    assert len(expr.extensions) == 1
    assert expr.extensions[0].WhichOneof("mapping_type") == "extension_function"
    assert expr.extensions[0].extension_function.extension_urn_reference == 1
    assert expr.extensions[0].extension_function.function_anchor == 1
    assert expr.extensions[0].extension_function.name == "add:unknown_unknown"

    assert list(expr.base_schema.names) == ["a", "b"]
    assert expr.base_schema.struct.nullability == type_pb2.Type.NULLABILITY_REQUIRED
    assert [
        field_type.WhichOneof("kind") for field_type in expr.base_schema.struct.types
    ] == [
        "unknown",
        "unknown",
    ]

    assert len(expr.referred_expr) == 1
    assert list(expr.referred_expr[0].output_names) == ["sum"]
    assert expr.referred_expr[0].expression.WhichOneof("rex_type") == "scalar_function"

    fn = expr.referred_expr[0].expression.scalar_function
    assert fn.function_reference == 1
    assert len(fn.arguments) == 2
    assert list(fn.arguments[0].value.named_expression.names) == ["a"]
    assert list(fn.arguments[1].value.named_expression.names) == ["b"]
    assert fn.output_type.WhichOneof("kind") == "unknown"
