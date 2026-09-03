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


def undefined_type_names(expression: str):
    """Identifiers in ``expression`` that sit where a type name belongs.

    The grammar has to accept an arbitrary identifier in a type position,
    because that is how a signature names a parameter (``decimal<P, S>``) or a
    derivation expression names an intermediate value.  A misspelled built-in
    therefore parses cleanly: ``f64`` is not a Substrait type, but it is a
    perfectly good identifier, so it reaches the ``ParameterName`` alternative
    instead of ``scalarType``.

    Numeric parameter slots reduce to ``NumericParameterName``, a different
    node, so a ``ParameterName`` means an identifier landed where the grammar
    expected a *type*.  Outside a derivation expression nothing legitimately
    introduces such a name, so it is a type that does not exist.

    Derivation expressions are exempt: ``MultilineDefinition`` binds its own
    identifiers by assignment and freely references signature parameters, and
    those are resolved by the type system rather than by this grammar.
    """
    tree = parse_type_expression(expression)
    if isinstance(tree.getChild(0), SubstraitTypeParser.MultilineDefinitionContext):
        return []

    names = []
    stack = [tree]
    while stack:
        node = stack.pop()
        if isinstance(node, SubstraitTypeParser.ParameterNameContext):
            names.append(node.getText())
        for index in range(node.getChildCount()):
            stack.append(node.getChild(index))
    return names


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
        "null?",
        "list<null?>",
        "map<null?, null?>",
        "struct<null?, i32>",
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


def test_undefined_type_names():
    """Identifiers standing in for a type are reported, parameters are not."""
    assert undefined_type_names("f64") == ["f64"]
    assert undefined_type_names("list<f64>") == ["f64"]
    assert undefined_type_names("int64") == ["int64"]
    assert undefined_type_names("func<i32 -> int>") == ["int"]

    for valid in [
        "i64",
        "fp64",
        "any1",
        "u!point",
        "ext.u!point",
        "list<i32>",
        "decimal<P, S>",
        "varchar<L>",
        "precision_timestamp<P>",
        # A derivation expression binds its own names and reads signature
        # parameters, so its identifiers are not type names.
        "init_scale = max(S1, S2)\nDECIMAL<init_scale, 0>",
    ]:
        assert undefined_type_names(valid) == [], valid


def test_extension_yaml_type_names_are_defined():
    """No checked-in extension YAML names a type that does not exist.

    ``simple_extensions_schema.yaml`` types every argument and return as an
    opaque string, and the grammar accepts any identifier in a type position,
    so a misspelled built-in passes both the schema check and the grammar walk
    above.  Five such names (``f64`` for ``fp64``, ``int64`` and ``int`` for
    ``i64``) shipped in example files embedded in the published documentation
    before this check existed.
    """
    repo_root = Path(__file__).parents[2]
    failures = []
    for path in extension_yaml_files():
        with path.open() as f:
            extension = yaml.load(f, Loader=yaml.FullLoader)

        for expression in iter_type_expressions(extension):
            for name in undefined_type_names(expression):
                failures.append(
                    f"{path.relative_to(repo_root)}: {expression!r} names "
                    f"'{name}', which is not a Substrait type"
                )

    assert failures == [], "\n".join(failures)
