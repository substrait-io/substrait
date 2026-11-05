# SPDX-License-Identifier: Apache-2.0
"""Proto Example Validator: Validates protobuf textformat examples.

Ensures examples are valid and use no unknown fields.
"""

from pathlib import Path
from google.protobuf import json_format, text_format
from google.protobuf.message import Message
import pytest

try:
    from substrait import algebra_pb2, extended_expression_pb2, plan_pb2, type_pb2
except ImportError as err:
    raise ImportError(
        "Protobuf bindings not found. Run 'buf generate' to generate them."
    ) from err


def validate_example(textproto: str, message_class: type[Message]) -> Message:
    """Parse and validate a textproto string with strict field checking."""
    message = message_class()
    text_format.Parse(textproto, message, allow_unknown_field=False)
    assert message.ListFields(), "Message has no fields populated"
    return message


def test_validation_rejects_unknown_fields():
    """Test that validation rejects proto text with unknown fields."""
    invalid_textproto = """
parameters: {types: [{i32: {nullability: NULLABILITY_REQUIRED}}]}
body: {literal: {i32: 42}}
unknown_field: "should fail"
"""
    with pytest.raises(text_format.ParseError, match="unknown_field"):
        validate_example(invalid_textproto, algebra_pb2.Expression.Lambda)


def test_validation_rejects_empty_messages():
    """Test that validation rejects empty messages."""
    with pytest.raises(AssertionError, match="no fields populated"):
        validate_example("", algebra_pb2.Expression.Lambda)


def test_validate_lambdas():
    """Validate lambda expression examples."""
    examples_dir = Path("site/examples/proto-textformat/lambda")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(textproto_file.read_text(), algebra_pb2.Expression.Lambda)


def test_validate_lambda_invocations():
    """Validate lambda invocation examples."""
    examples_dir = Path("site/examples/proto-textformat/lambda_invocation")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(
            textproto_file.read_text(), algebra_pb2.Expression.LambdaInvocation
        )


def test_validate_named_lambdas():
    """Validate named lambda examples."""
    examples_dir = Path("site/examples/proto-textformat/named_lambda")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(textproto_file.read_text(), plan_pb2.Plan)


def test_named_lambda_invocations_use_reference_messages():
    """Named lambda invocations use the shared reference message."""
    example = Path(
        "site/examples/proto-textformat/named_lambda/reused_identity.textproto"
    )
    plan = text_format.Parse(example.read_text(), plan_pb2.Plan())
    expressions = plan.relations[0].root.input.project.expressions
    assert len(expressions) == 2
    for expression in expressions:
        invocation = expression.lambda_invocation
        assert invocation.WhichOneof("lambda_type") == "named_lambda"
        assert (
            invocation.named_lambda.lambda_reference
            == plan.named_lambdas[0].lambda_anchor
        )


def test_named_lambda_reference_as_function_argument():
    """The transform example passes a function reference, not a call result."""
    example = Path(
        "site/examples/proto-textformat/named_lambda/transform_identity.textproto"
    )
    plan = text_format.Parse(example.read_text(), plan_pb2.Plan())
    function = plan.relations[0].root.input.project.expressions[0].scalar_function
    declaration = plan.extensions[0].extension_function
    assert declaration.name == "transform:list_func"
    assert function.function_reference == declaration.function_anchor
    assert len(function.arguments) == 2
    assert [value.i32 for value in function.arguments[0].value.literal.list.values] == [
        1,
        2,
        3,
    ]
    argument = function.arguments[1]
    assert argument.WhichOneof("arg_type") == "value"
    assert argument.value.WhichOneof("rex_type") == "named_lambda_reference"
    assert (
        argument.value.named_lambda_reference.lambda_reference
        == plan.named_lambdas[0].lambda_anchor
        == declaration.function_anchor
    )


@pytest.mark.parametrize("anchor", [0, 1, 4294967295])
def test_named_lambda_reference_round_trip(anchor):
    """Preserve function-valued references, including anchor zero, on the wire."""
    argument = algebra_pb2.FunctionArgument(
        value=algebra_pb2.Expression(
            named_lambda_reference=algebra_pb2.Expression.NamedLambdaReference(
                lambda_reference=anchor
            )
        )
    )
    restored_arguments = [
        algebra_pb2.FunctionArgument.FromString(argument.SerializeToString()),
        json_format.Parse(
            json_format.MessageToJson(argument), algebra_pb2.FunctionArgument()
        ),
    ]
    for restored in restored_arguments:
        assert restored == argument
        assert restored.value.WhichOneof("rex_type") == "named_lambda_reference"
        assert restored.value.named_lambda_reference.lambda_reference == anchor


@pytest.mark.parametrize("anchor", [0, 1, 4294967295])
def test_named_lambda_invocation_round_trip(anchor):
    """Preserve invocation references, including anchor zero, on the wire."""
    invocation = algebra_pb2.Expression.LambdaInvocation(
        named_lambda=algebra_pb2.Expression.NamedLambdaReference(
            lambda_reference=anchor
        )
    )
    restored_invocations = [
        algebra_pb2.Expression.LambdaInvocation.FromString(
            invocation.SerializeToString()
        ),
        json_format.Parse(
            json_format.MessageToJson(invocation),
            algebra_pb2.Expression.LambdaInvocation(),
        ),
    ]
    for restored in restored_invocations:
        assert restored == invocation
        assert restored.WhichOneof("lambda_type") == "named_lambda"
        assert restored.named_lambda.lambda_reference == anchor


def test_validate_table_functions():
    """Calls declare positional output types, including producer-resolved schemas."""
    examples_dir = Path("site/examples/proto-textformat/table_function")
    example_files = list(examples_dir.glob("*.textproto"))
    assert example_files, "No table function examples found"
    for example in example_files:
        function = validate_example(example.read_text(), algebra_pb2.TableFunction)
        assert function.HasField("output_type")
        assert function.output_type.nullability == type_pb2.Type.NULLABILITY_REQUIRED


def test_validate_table_function_relations():
    """Standalone relations directly contain the function call."""
    examples_dir = Path("site/examples/proto-textformat/table_function_rel")
    example_files = list(examples_dir.glob("*.textproto"))
    assert example_files, "No table function relation examples found"
    for example in example_files:
        relation = validate_example(example.read_text(), algebra_pb2.TableFunctionRel)
        assert relation.HasField("function")
        assert relation.function.HasField("output_type")
        assert (
            relation.function.output_type.nullability
            == type_pb2.Type.NULLABILITY_REQUIRED
        )


def test_validate_field_references():
    """Validate field reference examples."""
    examples_dir = Path("site/examples/proto-textformat/field_reference")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(
            textproto_file.read_text(), algebra_pb2.Expression.FieldReference
        )


def test_validate_plan_rels():
    """Validate plan relation examples."""
    examples_dir = Path("site/examples/proto-textformat/plan_rel")
    example_files = list(examples_dir.glob("*.textproto"))
    assert example_files, "No plan relation examples found"
    plan_rels = {}
    for textproto_file in example_files:
        plan_rel = validate_example(textproto_file.read_text(), plan_pb2.PlanRel)
        assert isinstance(plan_rel, plan_pb2.PlanRel)
        plan_rels[textproto_file.name] = plan_rel

    expression_plan_rel = plan_rels["detached_expressions.textproto"]
    project = expression_plan_rel.root.input.project
    assert project.expressions[0].WhichOneof("rex_type") == (
        "detached_expression_ordinal"
    )
    expression_ordinal = project.expressions[0].detached_expression_ordinal
    assert expression_ordinal < len(expression_plan_rel.detached_expressions)
    assert (
        expression_plan_rel.detached_expressions[expression_ordinal].literal.i64 == 42
    )
    assert list(project.input.read.base_schema.names) == ["value"]
    assert len(project.input.read.base_schema.struct.types) == 1
    assert list(expression_plan_rel.root.names) == ["value", "answer"]

    relation_plan_rel = plan_rels["detached_rels.textproto"]
    assert relation_plan_rel.root.input.WhichOneof("rel_type") == (
        "detached_rel_ordinal"
    )
    relation_ordinal = relation_plan_rel.root.input.detached_rel_ordinal
    assert relation_ordinal == 0
    assert len(relation_plan_rel.detached_rels) == 1
    relation = relation_plan_rel.detached_rels[relation_ordinal]
    assert relation.WhichOneof("rel_type") == "read"
    read = relation.read
    assert list(read.named_table.names) == ["example"]
    assert read.common.WhichOneof("emit_kind") == "direct"
    assert list(read.base_schema.names) == ["value"]
    assert len(read.base_schema.struct.types) == 1
    assert read.base_schema.struct.types[0].WhichOneof("kind") == "i64"
    assert list(relation_plan_rel.root.names) == ["value"]


def test_validate_extended_expressions():
    """Validate detached expression and relation examples."""
    examples_dir = Path("site/examples/proto-textformat/extended_expression")
    example_files = list(examples_dir.glob("*.textproto"))
    assert example_files, "No extended expression examples found"
    examples = {}
    for textproto_file in example_files:
        extended = validate_example(
            textproto_file.read_text(), extended_expression_pb2.ExtendedExpression
        )
        assert isinstance(extended, extended_expression_pb2.ExtendedExpression)
        examples[textproto_file.name] = extended

    extended = examples["detached_expressions.textproto"]
    expression = extended.referred_expr[0].expression
    assert expression.WhichOneof("rex_type") == "detached_expression_ordinal"
    ordinal = expression.detached_expression_ordinal
    assert ordinal < len(extended.detached_expressions)
    nested = extended.detached_expressions[ordinal]
    assert nested.WhichOneof("rex_type") == "nested"
    child = nested.nested.struct.fields[0]
    assert child.WhichOneof("rex_type") == "detached_expression_ordinal"
    ordinal = child.detached_expression_ordinal
    assert ordinal < len(extended.detached_expressions)
    assert extended.detached_expressions[ordinal].literal.i64 == 42
    assert list(extended.referred_expr[0].output_names) == ["result", "answer"]

    extended = examples["detached_rels.textproto"]
    assert len(extended.detached_rels) == 2
    assert len(extended.detached_expressions) == 1
    assert list(extended.referred_expr[0].output_names) == ["answer"]
    expression = extended.referred_expr[0].expression
    assert expression.WhichOneof("rex_type") == "subquery"
    assert expression.subquery.WhichOneof("subquery_type") == "scalar"
    relation = expression.subquery.scalar.input
    assert relation.WhichOneof("rel_type") == "detached_rel_ordinal"
    assert relation.detached_rel_ordinal == 0
    relation = extended.detached_rels[relation.detached_rel_ordinal]
    assert relation.WhichOneof("rel_type") == "filter"
    filter_rel = relation.filter
    assert filter_rel.common.WhichOneof("emit_kind") == "direct"
    condition = filter_rel.condition
    assert condition.WhichOneof("rex_type") == "detached_expression_ordinal"
    assert condition.detached_expression_ordinal == 0
    condition = extended.detached_expressions[condition.detached_expression_ordinal]
    assert condition.WhichOneof("rex_type") == "literal"
    assert condition.literal.WhichOneof("literal_type") == "boolean"
    assert condition.literal.boolean
    assert filter_rel.input.WhichOneof("rel_type") == "detached_rel_ordinal"
    assert filter_rel.input.detached_rel_ordinal == 1
    relation = extended.detached_rels[filter_rel.input.detached_rel_ordinal]
    assert relation.WhichOneof("rel_type") == "read"
    read = relation.read
    assert read.common.WhichOneof("emit_kind") == "direct"
    assert list(read.base_schema.names) == ["value"]
    assert len(read.base_schema.struct.types) == 1
    assert read.base_schema.struct.types[0].WhichOneof("kind") == "i64"
    assert read.WhichOneof("read_type") == "virtual_table"
    assert len(read.virtual_table.expressions) == 1
    assert len(read.virtual_table.expressions[0].fields) == 1
    value = read.virtual_table.expressions[0].fields[0]
    assert value.WhichOneof("rex_type") == "literal"
    assert value.literal.WhichOneof("literal_type") == "i64"
    assert value.literal.i64 == 42


@pytest.mark.parametrize(
    ("subquery_text", "relation_field"),
    [
        ("scalar { input { detached_rel_ordinal: 0 } }", "input"),
        (
            "in_predicate { needles { literal { i64: 42 } } "
            "haystack { detached_rel_ordinal: 0 } }",
            "haystack",
        ),
        (
            "set_predicate { predicate_op: PREDICATE_OP_EXISTS "
            "tuples { detached_rel_ordinal: 0 } }",
            "tuples",
        ),
        (
            "set_comparison { reduction_op: REDUCTION_OP_ANY "
            "comparison_op: COMPARISON_OP_EQ left { literal { i64: 42 } } "
            "right { detached_rel_ordinal: 0 } }",
            "right",
        ),
    ],
    ids=["scalar", "in-predicate", "set-predicate", "set-comparison"],
)
def test_extended_expression_subquery_detachment_roundtrip(
    subquery_text, relation_field
):
    example = Path(
        "site/examples/proto-textformat/extended_expression/detached_rels.textproto"
    )
    extended = extended_expression_pb2.ExtendedExpression()
    text_format.Parse(example.read_text(), extended)
    subquery = extended.referred_expr[0].expression.subquery
    subquery.Clear()
    text_format.Parse(subquery_text, subquery)

    decoded = extended_expression_pb2.ExtendedExpression.FromString(
        extended.SerializeToString()
    )
    assert decoded == extended
    subquery = decoded.referred_expr[0].expression.subquery
    kind = subquery.WhichOneof("subquery_type")
    assert kind is not None
    relation = getattr(getattr(subquery, kind), relation_field)
    assert relation.WhichOneof("rel_type") == "detached_rel_ordinal"
    assert relation.detached_rel_ordinal == 0
    filter_rel = decoded.detached_rels[relation.detached_rel_ordinal].filter
    condition = decoded.detached_expressions[
        filter_rel.condition.detached_expression_ordinal
    ]
    assert condition.literal.boolean
    read = decoded.detached_rels[filter_rel.input.detached_rel_ordinal].read
    assert read.virtual_table.expressions[0].fields[0].literal.i64 == 42
