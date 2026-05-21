# SPDX-License-Identifier: Apache-2.0

from pathlib import Path

import yaml
from antlr4 import CommonTokenStream, InputStream
from antlr4.error.ErrorListener import ErrorListener

from antlr_parser.SubstraitTypeLexer import SubstraitTypeLexer
from antlr_parser.SubstraitTypeParser import SubstraitTypeParser


class ErrorCollector(ErrorListener):
    def __init__(self):
        super().__init__()
        self.errors = []

    def syntaxError(self, recognizer, offendingSymbol, line, column, msg, e):
        self.errors.append((line, column, msg))


def parse_type_expression(value: str):
    """Parse a single Substrait type expression with the generated grammar."""
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


def iter_type_expressions(extension):
    """Yield type expression strings from a simple extension YAML document."""

    def walk_structure(structure):
        if isinstance(structure, str):
            yield structure
        elif isinstance(structure, dict):
            for value in structure.values():
                yield from walk_structure(value)

    for typ in extension.get("types", []):
        yield from walk_structure(typ.get("structure"))

    for functions in (
        extension.get("scalar_functions"),
        extension.get("aggregate_functions"),
        extension.get("window_functions"),
    ):
        for function in functions or []:
            for impl in function.get("impls", []):
                for arg in impl.get("args", []):
                    if "value" in arg:
                        yield arg["value"]
                    if "type" in arg:
                        yield arg["type"]
                if "return" in impl:
                    yield impl["return"]
                if "intermediate" in impl:
                    yield impl["intermediate"]


def extension_yaml_files():
    """Yield extension YAML files whose type strings should match the grammar."""
    repo_root = Path(__file__).parents[2]
    yield from sorted((repo_root / "extensions").glob("*.yaml"))
    yield from sorted((repo_root / "site" / "examples").glob("**/*.yaml"))


def test_parse_valid_type_expressions():
    """Type grammar accepts representative valid type expressions."""
    valid_cases = [
        "u!point",
        "u!point?",
        "u!wrapper<i32>",
        "ext.u!point",
        "ext.u!point?",
        "list<ext.u!point>",
        "map<string, ext.u!point>",
        "struct<ext.u!point, i32>",
        "func<ext.u!point -> fp64>",
        "func<(ext.u!point, other.u!point) -> fp64>",
        "ext.u!wrapper<other.u!point>",
        "ext.u!wrapper<list<other.u!point>, decimal<P,S>>",
        "$ext.u!point",
    ]

    for case in valid_cases:
        parse_type_expression(case)


def test_extension_yaml_type_expressions_are_grammar_compliant():
    """All type expressions in checked-in extension YAML parse successfully."""
    failures = []
    for path in extension_yaml_files():
        with path.open() as f:
            extension = yaml.load(f, Loader=yaml.FullLoader)

        for expression in iter_type_expressions(extension):
            try:
                parse_type_expression(expression)
            except AssertionError as err:
                failures.append(f"{path}: {expression}: {err}")

    assert failures == []
