# SPDX-License-Identifier: Apache-2.0
from antlr4 import CommonTokenStream, InputStream
from antlr4.error.ErrorListener import ErrorListener

from tests.type.antlr_parser.SubstraitLexer import SubstraitLexer
from tests.type.antlr_parser.SubstraitTypeParser import SubstraitTypeParser


class ParseErrorListener(ErrorListener):
    def __init__(self):
        super().__init__()
        self.errors = []

    def syntaxError(self, recognizer, offending_symbol, line, column, msg, e):
        self.errors.append(f"Syntax error at line {line}, column {column}: {msg}")


def parse_type_statement(text: str):
    lexer = SubstraitLexer(InputStream(text))
    parser = SubstraitTypeParser(CommonTokenStream(lexer))
    error_listener = ParseErrorListener()
    parser.removeErrorListeners()
    parser.addErrorListener(error_listener)
    tree = parser.typeStatement()
    return tree, parser.getNumberOfSyntaxErrors(), error_listener.errors


def test_unknown_type_parser_accepts_unknown():
    tree, num_errors, errors = parse_type_statement("unknown")
    assert num_errors == 0, errors
    assert tree.getText() == "unknown<EOF>"


def test_unknown_type_parser_rejects_nullable_unknown():
    _tree, num_errors, errors = parse_type_statement("unknown?")
    assert num_errors > 0, errors
