# SPDX-License-Identifier: Apache-2.0
"""Dialect Schema Validator: keeps the dialect schema in sync with the protos.

Dialects declare which relations, expressions and types they support, and which
enum values (join types, set operations, ...) they accept for them.  Every one
of those declarations mirrors a member of the protobuf definitions, so
`text/dialect_schema.yaml` has to be updated whenever a relation, expression,
type or enum value is added, removed or renamed.

These tests fail when the two drift apart and point at the entries that need
attention.  Discrepancies that are intentional are recorded here, together with
the reason for them.
"""

import re
from dataclasses import dataclass, field
from pathlib import Path

import pytest
import yaml
from google.protobuf.descriptor import Descriptor, EnumDescriptor

try:
    from substrait import algebra_pb2, plan_pb2, type_pb2
except ImportError as err:
    raise ImportError(
        "Protobuf bindings not found. Run 'buf generate' to generate them."
    ) from err

DIALECT_SCHEMA_PATH = Path(__file__).parent.parent / "text" / "dialect_schema.yaml"

# Path to a node of the dialect schema, indexing into mappings by key and into
# sequences by position.
SchemaPath = tuple[str | int, ...]

# Keys that structure the dialect schema rather than name something in it.
STRUCTURAL_KEYS = frozenset(
    {"properties", "definitions", "items", "oneOf", "enum", "const"}
)


def screaming_snake_case(name: str) -> str:
    """Convert a protobuf message or enum name to SCREAMING_SNAKE_CASE."""
    name = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name).upper()


def label(path: SchemaPath) -> str:
    """Return a readable name for the node at `path`."""
    return ".".join(
        str(step)
        for step in path
        if not isinstance(step, int) and step not in STRUCTURAL_KEYS
    )


def location(path: SchemaPath) -> str:
    """Return the full location of the node at `path`."""
    return ".".join(map(str, path))


def resolve(node, path: SchemaPath):
    """Return the node at `path`, or None if the path does not exist."""
    for step in path:
        if isinstance(step, int):
            if not isinstance(node, list) or step >= len(node):
                return None
        elif not isinstance(node, dict) or step not in node:
            return None
        node = node[step]
    return node


def enum_paths(node, path: SchemaPath = ()) -> list[SchemaPath]:
    """Return the paths of every `enum` list at or below `node`."""
    if isinstance(node, dict):
        paths = []
        for key, value in node.items():
            if key == "enum":
                paths.append((*path, key))
            else:
                paths.extend(enum_paths(value, (*path, key)))
        return paths
    if isinstance(node, list):
        return [
            found
            for index, value in enumerate(node)
            for found in enum_paths(value, (*path, index))
        ]
    return []


def by_dialect_name(members, dialect_name, origin=None) -> dict[str, str]:
    """Key protobuf members by their dialect name, rejecting ambiguous ones."""
    origin = origin or (lambda member: member.full_name)
    options = {}
    for member in members:
        name = dialect_name(member)
        if name in options:
            raise AssertionError(
                f"{origin(member)} and {options[name]} are both named {name}"
            )
        options[name] = origin(member)
    return options


def oneof_options(
    descriptor: Descriptor, oneof: str, renames: dict[str, str] | None = None
) -> dict[str, str]:
    """Return the dialect names of a oneof's members, keyed to their origin.

    `renames` maps the name of a member to the name dialects declare it as.
    """
    renames = renames or {}
    return by_dialect_name(
        descriptor.oneofs_by_name[oneof].fields,
        lambda member: renames.get(member.name, member.name.upper()),
    )


def enum_options(descriptor: EnumDescriptor) -> dict[str, str]:
    """Return the dialect names of an enum's values, keyed to their origin.

    Protobuf requires enum values to carry the name of their enum as prefix and
    to declare an unspecified zero value.  Dialects use neither.
    """
    prefix = f"{screaming_snake_case(descriptor.name)}_"
    for value in descriptor.values:
        if not value.name.startswith(prefix):
            raise AssertionError(
                f"{descriptor.full_name}.{value.name} is not prefixed with {prefix}"
            )
    return by_dialect_name(
        [value for value in descriptor.values if value.name != f"{prefix}UNSPECIFIED"],
        lambda value: value.name.removeprefix(prefix),
        lambda value: f"{descriptor.full_name}.{value.name}",
    )


def relation_options() -> dict[str, str]:
    """Return the dialect names of all relations, keyed to their origin.

    Relations are named after their message without the `Rel` suffix, so
    `ConsistentPartitionWindowRel` is declared as `CONSISTENT_PARTITION_WINDOW`.
    """
    return by_dialect_name(
        algebra_pb2.Rel.DESCRIPTOR.oneofs_by_name["rel_type"].fields,
        lambda member: screaming_snake_case(
            member.message_type.name.removesuffix("Rel")
        ),
    )


def expression_options() -> dict[str, str]:
    """Return the dialect names of all expressions, keyed to their origin.

    Expressions are named after their `Expression.rex_type` member, except for
    `switch_expression`, which dialects declare as `SWITCH`.
    """
    return oneof_options(
        algebra_pb2.Expression.DESCRIPTOR,
        "rex_type",
        renames={"switch_expression": "SWITCH"},
    )


@dataclass(frozen=True)
class Declaration:
    """A `supported_*` property listing what a dialect supports."""

    # Name of the property in the dialect schema.
    property: str
    # Key naming the declaration in the definitions of its long form.
    discriminator: str
    # Dialect name of every protobuf member the property covers, keyed to the
    # member it originates from.
    members: dict[str, str]
    # Members that dialects deliberately cannot declare support for, keyed to
    # the reason why.
    not_declarable: dict[str, str] = field(default_factory=dict)
    # Declarations that deliberately have no short form, keyed to the reason.
    long_form_only: dict[str, str] = field(default_factory=dict)

    @property
    def path(self) -> SchemaPath:
        return ("properties", self.property)

    @property
    def options(self) -> dict[str, str]:
        """Return what a dialect can declare, keyed to the protobuf origin."""
        return {
            name: origin
            for name, origin in self.members.items()
            if name not in self.not_declarable
        }


DECLARATIONS = [
    Declaration(
        property="supported_relations",
        discriminator="relation",
        members=relation_options(),
        long_form_only=dict.fromkeys(
            ("EXTENSION_SINGLE", "EXTENSION_MULTI", "EXTENSION_LEAF"),
            "the supported extension messages have to be named",
        ),
    ),
    Declaration(
        property="supported_expressions",
        discriminator="expression",
        members=expression_options(),
    ),
    Declaration(
        property="supported_types",
        discriminator="type",
        members=oneof_options(type_pb2.Type.DESCRIPTOR, "kind"),
        not_declarable={
            "ALIAS": "Type.alias points at a type declared in Plan.type_aliases, "
            "so support for it follows from support for the aliased type",
        },
        long_form_only={
            "USER_DEFINED": "the extension declaring the type has to be named",
        },
    ),
]


@dataclass(frozen=True)
class EnumList:
    """A list of enum values a dialect can restrict a declaration to."""

    # Path of the enum list in the dialect schema.
    path: SchemaPath
    # Dialect name of every value the list covers, keyed to the protobuf value
    # it originates from.
    values: dict[str, str]
    # Values the list deliberately does not offer, keyed to the reason why.
    excluded: dict[str, str] = field(default_factory=dict)


ENUM_LISTS = [
    EnumList(
        path=("definitions", "join_types", "items", "enum"),
        values=enum_options(algebra_pb2.JoinRel.JoinType.DESCRIPTOR),
    ),
    EnumList(
        path=("definitions", "lateral_join_types", "items", "enum"),
        values=enum_options(algebra_pb2.JoinRel.JoinType.DESCRIPTOR),
        excluded=dict.fromkeys(
            (
                "OUTER",
                "RIGHT",
                "RIGHT_SEMI",
                "RIGHT_ANTI",
                "RIGHT_SINGLE",
                "RIGHT_MARK",
            ),
            "a lateral join iterates over the left input, so the right handed "
            "variants do not apply to it",
        ),
    ),
    EnumList(
        path=(
            "definitions",
            "read_relation",
            "properties",
            "read_types",
            "items",
            "enum",
        ),
        values=oneof_options(algebra_pb2.ReadRel.DESCRIPTOR, "read_type"),
    ),
    EnumList(
        path=(
            "definitions",
            "set_relation",
            "properties",
            "operations",
            "items",
            "enum",
        ),
        values=enum_options(algebra_pb2.SetRel.SetOp.DESCRIPTOR),
    ),
    EnumList(
        path=(
            "definitions",
            "write_relation",
            "properties",
            "write_types",
            "items",
            "enum",
        ),
        values=oneof_options(algebra_pb2.WriteRel.DESCRIPTOR, "write_type"),
    ),
    EnumList(
        path=(
            "definitions",
            "ddl_relation",
            "properties",
            "write_types",
            "items",
            "enum",
        ),
        values=oneof_options(algebra_pb2.DdlRel.DESCRIPTOR, "write_type"),
    ),
    EnumList(
        path=(
            "definitions",
            "exchange_relation",
            "properties",
            "kinds",
            "items",
            "enum",
        ),
        values=oneof_options(algebra_pb2.ExchangeRel.DESCRIPTOR, "exchange_kind"),
    ),
    EnumList(
        path=(
            "definitions",
            "expand_relation",
            "properties",
            "field_types",
            "items",
            "enum",
        ),
        values=oneof_options(
            algebra_pb2.ExpandRel.ExpandField.DESCRIPTOR, "field_type"
        ),
    ),
    EnumList(
        path=(
            "definitions",
            "cast_expression",
            "properties",
            "failure_options",
            "items",
            "enum",
        ),
        values=enum_options(algebra_pb2.Expression.Cast.FailureBehavior.DESCRIPTOR),
    ),
    EnumList(
        path=(
            "definitions",
            "subquery_expression",
            "properties",
            "subquery_types",
            "items",
            "enum",
        ),
        values=oneof_options(
            algebra_pb2.Expression.Subquery.DESCRIPTOR, "subquery_type"
        ),
    ),
    EnumList(
        path=(
            "definitions",
            "nested_expression",
            "properties",
            "nested_types",
            "items",
            "enum",
        ),
        values=oneof_options(algebra_pb2.Expression.Nested.DESCRIPTOR, "nested_type"),
    ),
    EnumList(
        path=(
            "definitions",
            "execution_context_variable",
            "properties",
            "variable_types",
            "items",
            "oneOf",
            0,
            "enum",
        ),
        values=oneof_options(
            algebra_pb2.Expression.ExecutionContextVariable.DESCRIPTOR,
            "execution_context_variable_type",
        ),
    ),
    EnumList(
        path=(
            "properties",
            "supported_execution_behavior",
            "properties",
            "supported_variable_evaluation_mode",
            "items",
            "oneOf",
            0,
            "enum",
        ),
        values=enum_options(
            plan_pb2.ExecutionBehavior.VariableEvaluationMode.DESCRIPTOR
        ),
    ),
]

# Enum lists that describe the dialect itself rather than the protos, keyed to
# what they describe.
DIALECT_ONLY_ENUM_LISTS = {
    (
        "definitions",
        "system_function_metadata",
        "properties",
        "notation",
        "enum",
    ): "how the system modelled by the dialect spells function invocations",
}

FIX_HINT = (
    f"Update {DIALECT_SCHEMA_PATH.name} to match the protobuf definitions, or "
    f"record the discrepancy as intentional in {Path(__file__).name}."
)


@pytest.fixture(scope="module")
def schema() -> dict:
    with DIALECT_SCHEMA_PATH.open() as schema_file:
        return yaml.safe_load(schema_file)


def alternatives(schema: dict, declaration: Declaration) -> list[dict]:
    """Return the alternative forms a declaration can be written in."""
    forms = resolve(schema, (*declaration.path, "items", "oneOf"))
    assert forms is not None, (
        f"{location(declaration.path)} does not list alternative forms in "
        f"{DIALECT_SCHEMA_PATH.name}"
    )
    return forms


def short_form_path(schema: dict, declaration: Declaration) -> SchemaPath:
    """Return the path of the names a declaration can be written as."""
    paths = [
        (*declaration.path, "items", "oneOf", index, "enum")
        for index, alternative in enumerate(alternatives(schema, declaration))
        if "enum" in alternative
    ]
    assert len(paths) == 1, (
        f"expected {declaration.property} to offer a single short form, "
        f"found {len(paths)}"
    )
    return paths[0]


def short_form(schema: dict, declaration: Declaration) -> list[str]:
    """Return the names a declaration can be written as without properties."""
    return resolve(schema, short_form_path(schema, declaration))


def long_form(schema: dict, declaration: Declaration) -> dict[str, str]:
    """Return the definition names of a declaration, keyed by declared name."""
    definitions = {}
    for alternative in alternatives(schema, declaration):
        if "$ref" not in alternative:
            continue
        name = alternative["$ref"].removeprefix("#/definitions/")
        definition = resolve(schema, ("definitions", name))
        assert definition is not None, (
            f"{declaration.property} references undefined {alternative['$ref']}"
        )
        declared = resolve(
            definition, ("properties", declaration.discriminator, "const")
        )
        assert declared is not None, (
            f"definition {name} referenced by {declaration.property} does not "
            f"name the {declaration.discriminator} it describes"
        )
        assert declared not in definitions, (
            f"definitions {definitions[declared]} and {name} both describe the "
            f"{declaration.discriminator} {declared}, so a dialect declaring it "
            f"matches {declaration.property} twice"
        )
        definitions[declared] = name
    return definitions


@pytest.mark.parametrize(
    "declaration", DECLARATIONS, ids=lambda declaration: declaration.property
)
def test_declarations_are_defined(schema: dict, declaration: Declaration):
    """Every relation, expression and type is declarable with properties."""
    defined = long_form(schema, declaration)
    errors = [
        f"{name} ({origin}) has no definition"
        for name, origin in declaration.options.items()
        if name not in defined
    ]
    errors += [
        f"{name} is defined as {definition} but is not part of the protos"
        for name, definition in defined.items()
        if name not in declaration.members
    ]
    errors += [
        f"{name} is defined as {defined[name]} even though {reason}"
        for name, reason in declaration.not_declarable.items()
        if name in defined
    ]
    assert not errors, (
        f"the definitions of {declaration.property} do not match the protos:\n  "
        + "\n  ".join(errors)
        + f"\n{FIX_HINT}"
    )


@pytest.mark.parametrize(
    "declaration", DECLARATIONS, ids=lambda declaration: declaration.property
)
def test_declarations_have_short_form(schema: dict, declaration: Declaration):
    """Every relation, expression and type is declarable by name alone."""
    declarable = short_form(schema, declaration)
    errors = [
        f"{name} ({origin}) cannot be declared by name"
        for name, origin in declaration.options.items()
        if name not in declarable and name not in declaration.long_form_only
    ]
    errors += [
        f"{name} can be declared by name but is not part of the protos"
        for name in declarable
        if name not in declaration.members
    ]
    errors += [
        f"{name} can be declared by name even though {reason}"
        for name, reason in (
            declaration.long_form_only | declaration.not_declarable
        ).items()
        if name in declarable
    ]
    assert not errors, (
        f"the names accepted by {declaration.property} do not match the protos:\n  "
        + "\n  ".join(errors)
        + f"\n{FIX_HINT}"
    )


@pytest.mark.parametrize(
    "declaration", DECLARATIONS, ids=lambda declaration: declaration.property
)
def test_long_form_only_declarations_require_detail(
    schema: dict, declaration: Declaration
):
    """A declaration without a short form requires the detail it exists for."""
    defined = long_form(schema, declaration)
    errors = []
    for name, reason in declaration.long_form_only.items():
        definition = resolve(schema, ("definitions", defined[name]))
        required = set(definition.get("required", ())) - {declaration.discriminator}
        if not required:
            errors.append(
                f"{defined[name]} requires nothing beyond "
                f"{declaration.discriminator} even though {reason}"
            )
    assert not errors, (
        f"{declaration.property} accepts declarations without detail:\n  "
        + "\n  ".join(errors)
        + f"\nRequire the detail in {DIALECT_SCHEMA_PATH.name}, or give the "
        f"declaration a short form in {Path(__file__).name}."
    )


@pytest.mark.parametrize(
    "declaration", DECLARATIONS, ids=lambda declaration: declaration.property
)
def test_intentional_exceptions_still_apply(declaration: Declaration):
    """The exceptions recorded here are about members that still exist."""
    stale = [
        f"{name} is recorded as {kind} but is not part of the protos"
        for kind, exceptions in (
            ("not declarable", declaration.not_declarable),
            ("long form only", declaration.long_form_only),
        )
        for name in exceptions
        if name not in declaration.members
    ]
    assert not stale, (
        f"the exceptions recorded for {declaration.property} are stale:\n  "
        + "\n  ".join(stale)
        + f"\nRemove them from {Path(__file__).name}."
    )


def test_definitions_are_reachable(schema: dict):
    """Definitions of relations, expressions and types are referenced."""
    unreachable = []
    for declaration in DECLARATIONS:
        referenced = set(long_form(schema, declaration).values())
        unreachable += [
            f"{name} is not referenced by {declaration.property}"
            for name, definition in schema["definitions"].items()
            if name not in referenced
            and resolve(definition, ("properties", declaration.discriminator, "const"))
            is not None
        ]
    assert not unreachable, (
        f"these definitions of {DIALECT_SCHEMA_PATH.name} are unreachable:\n  "
        + "\n  ".join(unreachable)
    )


@pytest.mark.parametrize(
    "enum_list", ENUM_LISTS, ids=lambda enum_list: label(enum_list.path)
)
def test_enum_lists_match_protobuf(schema: dict, enum_list: EnumList):
    """The enum values a dialect can pick from are the protobuf values."""
    listed = resolve(schema, enum_list.path)
    assert listed is not None, (
        f"{location(enum_list.path)} does not exist in {DIALECT_SCHEMA_PATH.name}"
    )
    errors = [
        f"{value} is not part of the protos"
        for value in listed
        if value not in enum_list.values
    ]
    errors += [
        f"{value} ({origin}) is missing"
        for value, origin in enum_list.values.items()
        if value not in listed and value not in enum_list.excluded
    ]
    errors += [
        f"{value} is offered even though {reason}"
        for value, reason in enum_list.excluded.items()
        if value in listed
    ]
    assert not errors, (
        f"{location(enum_list.path)} does not match the protos:\n  "
        + "\n  ".join(errors)
        + f"\n{FIX_HINT}"
    )


@pytest.mark.parametrize(
    "enum_list", ENUM_LISTS, ids=lambda enum_list: label(enum_list.path)
)
def test_excluded_enum_values_still_apply(enum_list: EnumList):
    """The values recorded as excluded here still exist in the protos."""
    stale = [
        f"{value} is recorded as excluded but is not part of the protos"
        for value in enum_list.excluded
        if value not in enum_list.values
    ]
    assert not stale, (
        f"the exclusions recorded for {location(enum_list.path)} are stale:\n  "
        + "\n  ".join(stale)
        + f"\nRemove them from {Path(__file__).name}."
    )


def test_all_enum_lists_are_checked(schema: dict):
    """Every enum list of the dialect schema is checked against the protos."""
    checked = {enum_list.path for enum_list in ENUM_LISTS}
    checked |= set(DIALECT_ONLY_ENUM_LISTS)
    # The short forms are checked by test_declarations_have_short_form.
    checked |= {short_form_path(schema, declaration) for declaration in DECLARATIONS}
    unchecked = [location(path) for path in enum_paths(schema) if path not in checked]
    assert not unchecked, (
        "these enum lists are not checked against the protos:\n  "
        + "\n  ".join(unchecked)
        + f"\nAdd them to ENUM_LISTS or DIALECT_ONLY_ENUM_LISTS in "
        f"{Path(__file__).name}."
    )


def test_dialect_only_enum_lists_exist(schema: dict):
    """The enums recorded as describing the dialect itself still exist."""
    stale = [
        f"{location(path)}, recorded as {described}, does not exist"
        for path, described in DIALECT_ONLY_ENUM_LISTS.items()
        if resolve(schema, path) is None
    ]
    assert not stale, (
        f"these records of {Path(__file__).name} are stale:\n  "
        + "\n  ".join(stale)
        + "\nRemove them or point them at the enum list that replaced them."
    )


def test_join_type_enums_agree():
    """All join relations offer the join types the dialect schema lists once."""
    join_types = set(enum_options(algebra_pb2.JoinRel.JoinType.DESCRIPTOR))
    for relation in (
        algebra_pb2.HashJoinRel,
        algebra_pb2.MergeJoinRel,
        algebra_pb2.NestedLoopJoinRel,
    ):
        assert set(enum_options(relation.JoinType.DESCRIPTOR)) == join_types, (
            f"{relation.DESCRIPTOR.name}.JoinType no longer offers the same join "
            f"types as JoinRel.JoinType, so dialects can no longer share the "
            f"join_types definition of {DIALECT_SCHEMA_PATH.name}"
        )
