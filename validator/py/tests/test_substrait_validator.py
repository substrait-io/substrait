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
    assert lines[0] == 'Warning at plan: encountered values for protobuf field(s) not yet understood by the validator: .relations[0] (1003)'

    diags = list(sv.plan_to_diagnostics(_BASIC_PLAN))
    for diag in diags:
        assert type(diag) == sv.Diagnostic
    assert diags[0].msg == 'encountered values for protobuf field(s) not yet understood by the validator: .relations[0] (1003)'


# TODO: check_plan_valid()/check_plan_not_invalid()
