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


def iter_structure_type_expressions(structure):
    """Yield type expressions from structure's NSTRUCT syntactic sugar form."""
    if isinstance(structure, str):
        yield structure
    elif isinstance(structure, dict):
        for value in structure.values():
            yield from iter_structure_type_expressions(value)


def iter_type_expressions(extension):
    """Yield type expression strings from a simple extension YAML document."""
    for typ in extension.get("types", []):
        yield from iter_structure_type_expressions(typ.get("structure"))

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


def test_iter_structure_type_expressions():
    """Structure syntactic sugar is reduced to the type strings it contains."""
    cases = [
        (
            "NSTRUCT<longitude: i32, latitude: i32>",
            ["NSTRUCT<longitude: i32, latitude: i32>"],
        ),
        ({"longitude": "i32", "latitude": "i32"}, ["i32", "i32"]),
        (
            {"start": {"x": "fp64", "y": "fp64"}, "end": "u!point"},
            ["fp64", "fp64", "u!point"],
        ),
        (None, []),
    ]

    for structure, expected in cases:
        assert list(iter_structure_type_expressions(structure)) == expected


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


def render_precedence(ctx):
    """Render an expr parse tree as a fully-parenthesized string.

    Binary operators all expose ``left``/``op``/``right``; parenthesized
    expressions pass through to their inner expr so the rendered structure
    reflects grammar-derived precedence rather than the source parentheses.
    """
    paren = SubstraitTypeParser.ParenExpressionContext
    if isinstance(ctx, paren):
        return render_precedence(ctx.expr())
    if getattr(ctx, "op", None) is not None:
        left = render_precedence(ctx.left)
        right = render_precedence(ctx.right)
        return f"({left} {ctx.op.text} {right})"
    return ctx.getText()


def test_operator_precedence():
    """Binary operators bind with conventional precedence."""
    cases = [
        ("1 + 2 * 3", "(1 + (2 * 3))"),
        ("1 + 2 * 3 - 4 / 5", "((1 + (2 * 3)) - (4 / 5))"),
        ("1 * 2 + 3", "((1 * 2) + 3)"),
        ("(1 + 2) * 3", "((1 + 2) * 3)"),
        ("1 + 2 < 3 * 4", "((1 + 2) < (3 * 4))"),
        ("a and b or c", "((a and b) or c)"),
    ]

    for expression, expected in cases:
        tree = parse_type_expression(expression)
        assert render_precedence(tree.expr()) == expected, expression


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
