# SPDX-License-Identifier: Apache-2.0

import substrait_validator as sv


_BASIC_PLAN = """
{
 "extensionUris": [],
 "extensions": [],
 "relations": [
  {
   "rel": {
    "project": {
     "input": {
      "read": {
       "common": {
        "direct": {}
       },
       "projection": {
        "select": {
         "structItems": [
          {
           "field": 0
          },
          {
           "field": 1
          }
         ]
        },
        "maintainSingularStruct": false
       },
       "namedTable": {
        "names": [
         "person"
        ]
       }
      }
     },
     "expressions": [
      {
       "selection": {
        "directReference": {
         "structField": {
          "field": 0
         }
        }
       }
      },
      {
       "selection": {
        "directReference": {
         "structField": {
          "field": 1
         }
        }
       }
      }
     ]
    }
   }
  }
 ],
 "expectedTypeUrls": []
}
"""

_YAML = """---
types:
  - name: point
    structure:
      latitude: i32
      longitude: i32
  - name: line
    structure:
      start: point
      end: point
"""

def test_proto_roundtrip():
    """Round-trip test a basic Plan using the protobuf wrapper functions."""
    original_plan = sv.load_plan(_BASIC_PLAN)
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

    # Check identity.
    round_tripped_plan = sv.load_plan(original_plan)
    assert round_tripped_plan == original_plan


def test_parsing():
    """Test the parsing function."""
    result = sv.plan_to_parse_result(_BASIC_PLAN)
    assert type(result) == sv.ParseResult

    root = sv.parse_plan(_BASIC_PLAN)
    assert type(root) == sv.Node

    root = sv.plan_to_parse_tree(_BASIC_PLAN)
    assert type(root) == sv.Node


def test_export_html():
    """Test the HTML export function."""
    html = sv.plan_to_html(_BASIC_PLAN)
    assert type(html) == str
    lines = list(filter(bool, html.split('\n')))
    assert lines[0] == '<!DOCTYPE html>'
    assert lines[-1] == '</html>'


def test_export_diags():
    """Test the diagnostics export functions."""
    diags = sv.plan_to_diagnostics_str(_BASIC_PLAN)
    assert type(diags) == str
    lines = list(filter(bool, diags.split('\n')))
    assert lines[0] == 'Warning at plan: not yet implemented: the following child nodes were not recognized by the validator: relations[0] (1)'

    diags = list(sv.plan_to_diagnostics(_BASIC_PLAN))
    for diag in diags:
        assert type(diag) == sv.Diagnostic
    assert diags[0].msg == 'not yet implemented: the following child nodes were not recognized by the validator: relations[0] (1)'


def test_resolver_callback():
    """Tests whether the YAML URI resolver callback works."""

    def resolver(s):
        if s == 'test':
            return _YAML.encode('utf-8')
        raise ValueError('unknown URI')

    config = sv.Config()
    config.add_uri_resolver(resolver)

    diags = list(sv.plan_to_diagnostics({
        'extensionUris': [{
            'extension_uri_anchor': 1,
            'uri': 'test',
        }]
    }, config))
    for diag in diags:
        print(diag.msg)
    assert diags[0].msg == 'not yet implemented: the following child nodes were not recognized by the validator: types (1)'

    diags = list(sv.plan_to_diagnostics({
        'extensionUris': [{
            'extension_uri_anchor': 1,
            'uri': 'not-test',
        }]
    }, config))
    for diag in diags:
        print(diag.msg)
    assert diags[0].msg == 'failed to resolve YAML: ValueError: unknown URI (2002)'


# TODO: check_plan_valid()/check_plan_not_invalid()
