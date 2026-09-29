# SPDX-License-Identifier: Apache-2.0
"""Proto Example Validator: Validates protobuf textformat examples.

Ensures examples are valid and use no unknown fields.
"""

from pathlib import Path
from google.protobuf import json_format, text_format
from google.protobuf.message import Message
import pytest

try:
    from substrait import algebra_pb2, plan_pb2
except ImportError as err:
    raise ImportError(
        "Protobuf bindings not found. Run 'buf generate' to generate them."
    ) from err


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


def test_validate_named_lambdas():
    """Validate named lambda examples."""
    examples_dir = Path("site/examples/proto-textformat/named_lambda")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(textproto_file.read_text(), plan_pb2.Plan)


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
    assert [
        value.i32 for value in function.arguments[0].value.literal.list.values
    ] == [1, 2, 3]
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


def test_validate_field_references():
    """Validate field reference examples."""
    examples_dir = Path("site/examples/proto-textformat/field_reference")
    for textproto_file in examples_dir.glob("*.textproto"):
        validate_example(
            textproto_file.read_text(), algebra_pb2.Expression.FieldReference
        )
