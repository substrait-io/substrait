import json
from typing import Iterable
from google.protobuf import json_format

from .substrait_validator import ParseResult
from .substrait.plan_pb2 import Plan
from .substrait.validator.validator_pb2 import Node, Diagnostic


def load_plan_from_proto(data: bytes) -> Plan:
    """Load a Substrait plan from its protobuf serialization."""
    if not isinstance(data, bytes):
        raise TypeError("unsupported type: {}".format(type(data)))
    plan = Plan()
    plan.ParseFromString(data)
    return plan


def load_plan_from_json(data: str) -> Plan:
    """Load a Substrait plan from its JSON string representation."""
    if not isinstance(data, str):
        raise TypeError("unsupported type: {}".format(type(data)))
    return json_format.Parse(data, Plan())


def load_plan_from_dict(data: dict) -> Plan:
    """Load a Substrait plan from its Python object JSON representation."""
    if not isinstance(data, dict):
        raise TypeError("unsupported type: {}".format(type(data)))
    return load_plan_from_json(json.dumps(data))


def load_plan(data) -> Plan:
    """Loads a plan from its binary protobuf serialization (bytes input),
    a JSON string (string input), or a dictionary representation of such a
    JSON string (dict input). If data is already a Plan, this function is
    no-op and simply returns its input."""
    if isinstance(data, Plan):
        return data
    elif isinstance(data, bytes):
        return load_plan_from_proto(data)
    elif isinstance(data, str):
        return load_plan_from_json(data)
    elif isinstance(data, dict):
        return load_plan_from_dict(data)
    else:
        raise TypeError("unsupported type: {}".format(type(data)))


def parse_plan(plan) -> Node:
    """Parses the given plan with the validator, and returns its parse tree.
    plan can be anything supported by load_plan(), a Plan object, or a
    ParseResult object. Alternate name for plan_to_parse_tree()."""
    return plan_to_parse_tree(plan)


def plan_to_proto(plan) -> bytes:
    """Converts a plan to its binary protobuf serialization. plan can be
    anything supported by load_plan()."""
    return load_plan(plan).SerializeToString()


def plan_to_json(plan) -> str:
    """Converts a plan to its JSON serialization, returned as a string. plan
    can be anything supported by load_plan()."""
    return json_format.MessageToJson(load_plan(plan))


def plan_to_dict(plan) -> dict:
    """Converts a plan to its JSON serialization, returned as a dict. plan can
    be anything supported by load_plan()."""
    return json_format.MessageToDict(load_plan(plan))


def plan_to_parse_result(plan) -> ParseResult:
    """Parses a Substrait plan using the validator, and returns its result
    handle object. plan can be anything supported by load_plan(). If the
    input is already a ParseResult, it is returned as-is."""
    if isinstance(plan, ParseResult):
        return plan
    if isinstance(plan, bytes):
        data = plan
    else:
        data = plan_to_proto(plan)
    return ParseResult(data)


def plan_to_parse_tree(plan) -> Node:
    """Parses the given plan with the validator, and returns its parse tree.
    plan can be anything supported by load_plan(), a Plan object, or a
    ParseResult object."""
    root = Node()
    root.ParseFromString(plan_to_parse_tree_proto(plan))
    return root


def plan_to_parse_tree_proto(plan) -> str:
    """Same as parse_plan(), but returns the binary serialization of the
    parse tree. This is faster, if you don't plan to use the serialization from
    python."""
    return plan_to_parse_result(plan).export_proto()


def plan_to_diagnostics(plan) -> Iterable[Diagnostic]:
    """Converts a plan to an iterable of Diagnostics. plan can be anything
    supported by plan_to_parse_result()."""
    def walk(node):
        for data in node.data:
            if data.HasField('child'):
                walk(data.child.node)
            elif data.HasField('diagnostic'):
                yield data.diagnostic
    return walk(plan_to_parse_tree(plan))


def plan_to_diagnostics_str(plan) -> str:
    """Converts a plan to a multiline string representing the diagnostic
    messages returned by the validator for that plan. plan can be anything
    supported by plan_to_parse_result()."""
    return plan_to_parse_result(plan).export_diagnostics()


def plan_to_html(plan) -> str:
    """Generates a HTML page for the given plan to serve as documentation
    while debugging. plan can be anything supported by
    plan_to_parse_result()."""
    return plan_to_parse_result(plan).export_html()


def check_plan(plan) -> int:
    """Returns 1 if the given plan is valid, -1 if it is invalid, or 0 if the
    validator cannot determine validity. plan can be anything supported by
    load_plan(), a Plan object, or a ParseResult object."""
    return plan_to_parse_result(plan).check()


def check_plan_valid(plan):
    """Throws a ValueError exception containing the first error or warning
    encountered in the plan if the validator cannot prove correctness of
    the given plan. plan can be anything supported by load_plan(), a Plan
    object, or a ParseResult object."""
    plan_to_parse_result(plan).check_valid()


def check_plan_not_invalid(plan):
    """Throws a ValueError exception containing the first error encountered in
    the plan if the validator can prove that the given plan is invalid. plan
    can be anything supported by load_plan(), a Plan object, or a ParseResult
    object."""
    plan_to_parse_result(plan).check_not_invalid()
