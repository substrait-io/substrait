# SPDX-License-Identifier: Apache-2.0

import substrait_validator as sv
import pytest
from data import BASIC_PLAN, BASIC_YAML


def test_proto_roundtrip():
    """Round-trip test a basic Plan using the protobuf wrapper functions."""
    original_plan = sv.load_plan(BASIC_PLAN)
    assert type(original_plan) is sv.Plan

    # Round-trip via binary representation.
    data = sv.plan_to_proto(original_plan)
    assert type(data) is bytes
    round_tripped_plan = sv.load_plan(data)
    assert round_tripped_plan == original_plan

    # Round-trip via JSON string.
    data = sv.plan_to_json(original_plan)
    assert type(data) is str
    round_tripped_plan = sv.load_plan(data)
    assert round_tripped_plan == original_plan

    # Round-trip via JSON dict.
    data = sv.plan_to_dict(original_plan)
    assert type(data) is dict
    round_tripped_plan = sv.load_plan(data)
    assert round_tripped_plan == original_plan

    # Round-trip via YAML.
    data = sv.plan_to_yaml(original_plan)
    assert type(data) is str
    round_tripped_plan = sv.load_plan_from_yaml(data)
    assert round_tripped_plan == original_plan

    # Round-trip via JSOM.
    data = sv.plan_to_jsom(original_plan)
    assert type(data) is str
    # TODO: disabled due to bugs in JSOM
    #round_tripped_plan = sv.load_plan_from_jsom(data)
    #assert round_tripped_plan == original_plan

    # Check identity.
    round_tripped_plan = sv.load_plan(original_plan)
    assert round_tripped_plan == original_plan


def test_parsing():
    """Test the parsing function."""
    result = sv.plan_to_parse_result(BASIC_PLAN)
    assert type(result) == sv.ParseResult

    root = sv.parse_plan(BASIC_PLAN)
    assert type(root) == sv.ParseResult

    root = sv.plan_to_parse_result(BASIC_PLAN)
    assert type(root) == sv.ParseResult


def test_export_html():
    """Test the HTML export function."""
    html = sv.plan_to_html(BASIC_PLAN)
    assert type(html) == str
    lines = list(filter(bool, html.split('\n')))
    assert lines[0] == '<!DOCTYPE html>'
    assert lines[-1] == '</html>'


def test_export_diags():
    """Test the diagnostics export functions."""
    diags = sv.plan_to_diagnostics_str(BASIC_PLAN)
    assert type(diags) == str

    diags = list(sv.plan_to_diagnostics(BASIC_PLAN))
    for diag in diags:
        assert type(diag) == sv.Diagnostic


def test_valid_invalid():
    """Test the plan validity functions."""
    # Override all diagnostics to info, so the plan is considered valid.
    config = sv.Config()
    config.override_diagnostic_level(0, 'info', 'info')
    plan = sv.plan_to_result_handle(BASIC_PLAN, config)
    assert sv.check_plan(plan) == 1
    sv.check_plan_valid(plan)
    sv.check_plan_not_invalid(plan)

    # Override all diagnostics to warning, so the validity is considered to be
    # unknown.
    config = sv.Config()
    config.override_diagnostic_level(0, 'warning', 'warning')
    plan = sv.plan_to_result_handle(BASIC_PLAN, config)
    assert sv.check_plan(plan) == 0
    with pytest.raises(ValueError):
        sv.check_plan_valid(plan)
    sv.check_plan_not_invalid(plan)

    # Override all diagnostics to error, so the plan is considered to be
    # invalid.
    config = sv.Config()
    config.override_diagnostic_level(0, 'error', 'error')
    plan = sv.plan_to_result_handle(BASIC_PLAN, config)
    assert sv.check_plan(plan) == -1
    with pytest.raises(ValueError):
        sv.check_plan_valid(plan)
    with pytest.raises(ValueError):
        sv.check_plan_not_invalid(plan)


def test_resolver_callback():
    """Tests whether the YAML URI resolver callback works."""

    def resolver(s):
        if s == 'test:hello':
            return BASIC_YAML.encode('utf-8')
        raise ValueError('unknown URI')

    config = sv.Config()

    # Disable "not yet implemented" warnings.
    config.override_diagnostic_level(1, 'info', 'info')

    # Disable missing root relation error, so we don't have to supply one.
    config.override_diagnostic_level(5001, 'info', 'info')

    # Add the resolver.
    config.add_uri_resolver(resolver)

    sv.check_plan_valid({
        'extensionUris': [{
            'extension_uri_anchor': 1,
            'uri': 'test:hello',
        }]
    }, config)

    with pytest.raises(ValueError, match=r'failed to resolve YAML: ValueError: unknown URI \(code 2002\)'):
        sv.check_plan_valid({
            'extensionUris': [{
                'extension_uri_anchor': 1,
                'uri': 'test:bye',
            }]
        }, config)
