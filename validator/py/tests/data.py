# SPDX-License-Identifier: Apache-2.0

BASIC_PLAN = """
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

BASIC_YAML = """---
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
