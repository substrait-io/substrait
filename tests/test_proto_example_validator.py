# SPDX-License-Identifier: Apache-2.0
"""Proto Example Validator: Validates protobuf textformat examples.

Ensures examples are valid and use no unknown fields.
"""
from pathlib import Path
from google.protobuf import text_format
from google.protobuf.message import Message
import pytest

try:
    from substrait import algebra_pb2
except ImportError:
    raise ImportError(
        "Protobuf bindings not found. Run 'buf generate' to generate them."
    )


def validate_example(textproto: str, message_class: type[Message]) -> None:
    """Parse and validate a textproto string with strict field checking."""
    message = message_class()
    text_format.Parse(textproto, message, allow_unknown_field=False)
    assert message.ListFields(), "Message has no fields populated"


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
