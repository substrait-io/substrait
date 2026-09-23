# SPDX-License-Identifier: Apache-2.0
"""Proto Example Validator: Validates protobuf textformat examples.

Ensures examples are valid and use no unknown fields.
"""

from pathlib import Path
from google.protobuf import text_format
from google.protobuf.message import Message
import pytest

try:
    from substrait import algebra_pb2, extended_expression_pb2, plan_pb2
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
    assert relation_ordinal < len(relation_plan_rel.detached_rels)
    assert relation_plan_rel.detached_rels[relation_ordinal].read.named_table.names == [
        "example"
    ]


def test_validate_extended_expressions():
    """Validate detached expressions in an expression-only message."""
    examples_dir = Path("site/examples/proto-textformat/extended_expression")
    example_files = list(examples_dir.glob("*.textproto"))
    assert example_files, "No extended expression examples found"
    for textproto_file in example_files:
        extended = validate_example(
            textproto_file.read_text(), extended_expression_pb2.ExtendedExpression
        )
        assert isinstance(extended, extended_expression_pb2.ExtendedExpression)
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
