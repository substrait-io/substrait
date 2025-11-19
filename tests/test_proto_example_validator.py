# SPDX-License-Identifier: Apache-2.0
"""Proto Example Validator: Validates protobuf textformat examples.

Ensures examples are valid and use no unknown fields.
Tests are skipped if protobuf bindings aren't available.
To generate bindings: ./scripts/generate_python_protos.sh
"""
from pathlib import Path
from google.protobuf import text_format
from google.protobuf.message import Message
import pytest
import warnings

# Try to import algebra_pb2, but warn if it's not available
try:
    from substrait import algebra_pb2
except ImportError:
    warnings.warn(
        "Protobuf bindings not generated. Proto example validator tests will be skipped. "
        "Run './scripts/generate_python_protos.sh' to generate them.",
        UserWarning,
    )
    algebra_pb2 = None

pytestmark = pytest.mark.skipif(
    algebra_pb2 is None, reason="Protobuf bindings not available"
)


def validate_example(textproto: str, message_class: type[Message]) -> None:
    """Parse and validate a textproto string with strict field checking."""
    message = message_class()
    text_format.Parse(textproto, message, allow_unknown_field=False)
    assert message.ListFields(), "Message has no fields populated"


def test_validation_rejects_unknown_fields():
    """Test that validation rejects proto text with unknown fields."""
    invalid_textproto = """
parameter_types: [{i32: {nullability: NULLABILITY_REQUIRED}}]
return_type: {i32: {nullability: NULLABILITY_REQUIRED}}
body: {literal: {i32: 42}}
unknown_field: "should fail"
"""
    with pytest.raises(text_format.ParseError, match="unknown_field"):
        validate_example(invalid_textproto, algebra_pb2.Expression.Lambda)


def test_validation_rejects_empty_messages():
    """Test that validation rejects empty messages."""
    with pytest.raises(AssertionError, match="no fields populated"):
        validate_example("", algebra_pb2.Expression.Lambda)


def test_validate_examples():
    """Validate all protobuf textformat examples."""
    examples = [
        ("lambdas", algebra_pb2.Expression.Lambda),
        ("lambda_invocations", algebra_pb2.Expression.LambdaInvocation),
        ("field_references", algebra_pb2.Expression.FieldReference),
    ]

    for directory, message_class in examples:
        examples_dir = Path("site/examples/proto-textformat") / directory

        if not examples_dir.exists():
            continue

        for textproto_file in examples_dir.glob("*.textproto"):
            validate_example(textproto_file.read_text(), message_class)
