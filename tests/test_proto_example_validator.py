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
    """Named callees use function-valued references without a legacy callee."""
    example = Path(
        "site/examples/proto-textformat/named_lambda/reused_identity.textproto"
    )
    plan = text_format.Parse(example.read_text(), plan_pb2.Plan())
    expressions = plan.relations[0].root.input.project.expressions
    assert len(expressions) == 2
    for expression in expressions:
        invocation = expression.lambda_invocation
        assert invocation.function.WhichOneof("rex_type") == "named_lambda_reference"
        assert not invocation.HasField("lambda")
        assert (
            invocation.function.named_lambda_reference.lambda_reference
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
        function=algebra_pb2.Expression(
            named_lambda_reference=algebra_pb2.Expression.NamedLambdaReference(
                lambda_reference=anchor
            )
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
        assert not restored.HasField("lambda")
        assert restored.function.WhichOneof("rex_type") == "named_lambda_reference"
        assert restored.function.named_lambda_reference.lambda_reference == anchor


def test_unified_invocation_descriptor():
    descriptor = algebra_pb2.Expression.LambdaInvocation.DESCRIPTOR
    function = descriptor.fields_by_name["function"]
    assert function.number == 3
    assert function.message_type == algebra_pb2.Expression.DESCRIPTOR
    assert function.containing_oneof is None
    legacy = descriptor.fields_by_name["lambda"]
    assert legacy.number == 1
    assert legacy.GetOptions().deprecated
    assert legacy.containing_oneof is None
    assert not descriptor.oneofs
    assert set(descriptor.fields_by_name) == {"lambda", "arguments", "function"}
    assert descriptor.fields_by_name["arguments"].number == 2


@pytest.mark.parametrize("dual_write", [False, True])
def test_inline_unified_invocation_round_trip(dual_write: bool):
    invocation = algebra_pb2.Expression.LambdaInvocation()
    inline = getattr(invocation.function, "lambda")
    inline.parameters.nullability = type_pb2.Type.NULLABILITY_REQUIRED
    inline.body.literal.i32 = 42
    if dual_write:
        getattr(invocation, "lambda").CopyFrom(inline)
    invocation.arguments.SetInParent()

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
        assert restored.HasField("function")
        assert restored.HasField("arguments")
        assert restored.function.WhichOneof("rex_type") == "lambda"
        assert restored.HasField("lambda") == dual_write


def test_inline_invocation_dual_write():
    example = Path(
        "site/examples/proto-textformat/lambda_invocation/inline_invocation.textproto"
    )
    invocation = text_format.Parse(
        example.read_text(), algebra_pb2.Expression.LambdaInvocation()
    )
    assert invocation.function.WhichOneof("rex_type") == "lambda"
    assert getattr(invocation.function, "lambda") == getattr(invocation, "lambda")

    legacy = algebra_pb2.Expression.LambdaInvocation()
    legacy.CopyFrom(invocation)
    legacy.ClearField("function")
    restored = algebra_pb2.Expression.LambdaInvocation.FromString(
        legacy.SerializeToString()
    )
    assert restored == legacy
    assert restored.HasField("lambda")
    assert not restored.HasField("function")


def test_apply_twice_parameter_invocations():
    example = Path("site/examples/proto-textformat/lambda/apply_twice.textproto")
    definition = text_format.Parse(
        example.read_text(), algebra_pb2.Expression.Lambda()
    )
    assert definition.parameters.nullability == type_pb2.Type.NULLABILITY_REQUIRED
    assert len(definition.parameters.types) == 2
    function_type, argument_type = definition.parameters.types
    assert function_type.WhichOneof("kind") == "func"
    assert function_type.func.nullability == type_pb2.Type.NULLABILITY_REQUIRED
    assert argument_type.WhichOneof("kind") == "i32"
    assert argument_type.i32.nullability == type_pb2.Type.NULLABILITY_REQUIRED
    assert list(function_type.func.parameter_types) == [argument_type]
    assert function_type.func.return_type == argument_type

    outer = definition.body.lambda_invocation
    inner = outer.arguments.fields[0].lambda_invocation
    for invocation in [outer, inner]:
        assert not invocation.HasField("lambda")
        assert invocation.function.WhichOneof("rex_type") == "selection"
        assert len(invocation.arguments.fields) == 1
        reference = invocation.function.selection
        assert reference.WhichOneof("root_type") == "lambda_parameter_reference"
        assert reference.lambda_parameter_reference.steps_out == 0
        assert reference.direct_reference.struct_field.field == 0
    argument = inner.arguments.fields[0]
    assert argument.WhichOneof("rex_type") == "selection"
    assert argument.selection.WhichOneof("root_type") == "lambda_parameter_reference"
    assert argument.selection.lambda_parameter_reference.steps_out == 0
    assert argument.selection.direct_reference.struct_field.field == 1

    for restored in [
        algebra_pb2.Expression.Lambda.FromString(definition.SerializeToString()),
        json_format.Parse(
            json_format.MessageToJson(definition), algebra_pb2.Expression.Lambda()
        ),
    ]:
        assert restored == definition


def test_returned_function_invocation():
    example = Path(
        "site/examples/proto-textformat/lambda_invocation/returned_function.textproto"
    )
    invocation = text_format.Parse(
        example.read_text(), algebra_pb2.Expression.LambdaInvocation()
    )
    assert not invocation.HasField("lambda")
    assert invocation.HasField("arguments")
    assert not invocation.arguments.fields
    assert invocation.function.WhichOneof("rex_type") == "lambda_invocation"
    factory_call = invocation.function.lambda_invocation
    assert factory_call.function.WhichOneof("rex_type") == "lambda"
    factory = getattr(factory_call.function, "lambda")
    assert factory == getattr(factory_call, "lambda")
    assert len(factory.parameters.types) == 1
    assert len(factory_call.arguments.fields) == 1
    assert factory_call.arguments.fields[0].literal.i32 == 42
    assert factory.body.WhichOneof("rex_type") == "lambda"
    returned = getattr(factory.body, "lambda")
    assert returned.parameters.nullability == type_pb2.Type.NULLABILITY_REQUIRED
    assert not returned.parameters.types
    capture = returned.body.selection
    assert capture.WhichOneof("root_type") == "lambda_parameter_reference"
    assert capture.lambda_parameter_reference.steps_out == 1
    assert capture.direct_reference.struct_field.field == 0
    assert json_format.Parse(
        json_format.MessageToJson(invocation),
        algebra_pb2.Expression.LambdaInvocation(),
    ) == invocation


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
    for textproto_file in example_files:
        plan_rel = validate_example(textproto_file.read_text(), plan_pb2.PlanRel)
        assert isinstance(plan_rel, plan_pb2.PlanRel)
        project = plan_rel.root.input.project
        assert (
            project.expressions[0].WhichOneof("rex_type")
            == "detached_expression_ordinal"
        )
        ordinal = project.expressions[0].detached_expression_ordinal
        assert ordinal < len(plan_rel.detached_expressions)
        assert plan_rel.detached_expressions[ordinal].literal.i64 == 42
        assert list(project.input.read.base_schema.names) == ["value"]
        assert len(project.input.read.base_schema.struct.types) == 1
        assert list(plan_rel.root.names) == ["value", "answer"]


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
