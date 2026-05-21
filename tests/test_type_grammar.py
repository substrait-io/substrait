# SPDX-License-Identifier: Apache-2.0

from antlr4 import CommonTokenStream, InputStream
from antlr4.error.ErrorListener import ErrorListener

from tests.type.antlr_parser.SubstraitTypeLexer import SubstraitTypeLexer
from tests.type.antlr_parser.SubstraitTypeParser import SubstraitTypeParser


class ErrorCollector(ErrorListener):
    def __init__(self):
        super().__init__()
        self.errors = []

    def syntaxError(self, recognizer, offendingSymbol, line, column, msg, e):
        self.errors.append((line, column, msg))


def parse_type_expression(value: str):
    lexer = SubstraitTypeLexer(InputStream(value))
    token_stream = CommonTokenStream(lexer)
    parser = SubstraitTypeParser(token_stream)
    error_listener = ErrorCollector()
    parser.removeErrorListeners()
    parser.addErrorListener(error_listener)

    tree = parser.startRule()

    assert error_listener.errors == []
    assert parser.getNumberOfSyntaxErrors() == 0
    return tree


def test_dependency_qualified_udt_reference():
    parse_type_expression("ext.u!point")


def test_dependency_qualified_udt_reference_in_parameterized_type():
    parse_type_expression("list<ext.u!point>")


def test_dependency_qualified_udt_reference_with_parameters():
    parse_type_expression("ext.u!wrapper<other.u!point>")
