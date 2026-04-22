# Field References

In Substrait, all fields are dealt with on a positional basis. Field names are only used at the edge of a plan, for the purposes of naming fields for the outside world. Each operation returns a simple or compound data type. Additional operations can refer to data within that initial operation using field references. To reference a field, you use a reference based on the type of field position you want to reference.

Field references can originate from different root types:

- **RootReference**: References the incoming record from the relation
- **OuterReference**: References outer query records in correlated subqueries, supporting either offset-based (`steps_out`) or id-based (`id_reference`) resolution (see [Outer References](#outer-references))
- **Expression**: References the result of evaluating an expression
- **LambdaParameterReference**: References lambda parameters within lambda body expressions (see [Lambda Expressions](lambda_expressions.md))

| Reference Type            | Properties                                                   | Type Applicability | Type return                |
| ------------------------- | ------------------------------------------------------------ | ------------------ | -------------------------- |
| Struct Field              | Ordinal position. Zero-based. Only legal within the range of possible fields within a struct. Selecting an ordinal outside the applicable field range results in an invalid plan. | struct             | Type of field referenced   |
| Array Value               | Array offset. Zero-based. Negative numbers can be used to describe an offset relative to the end of the array. For example, -1 means the last element in an array. Negative and positive overflows return null values (no wrapping). | list               | type of list               |
| Array Slice               | Array offset and element count. Zero-based. Negative numbers can be used to describe an offset relative to the end of the array. For example, -1 means the last element in an array. Position does not wrap, nor does length. | list               | Same type as original list |
| Map Key                   | A map value that is matched exactly against available map keys and returned. | map                | Value type of map          |
| Map KeyExpression         | A wildcard string that is matched against a simplified form of regular expressions. Requires the key type of the map to be a character type. [Format detail needed, intention to include basic regex concepts such as greedy/non-greedy.] | map                | List of map value type     |
| Masked Complex Expression | An expression that provides a mask over a schema declaring which portions of the schema should be presented. This allows a user to select a portion of a complex object but mask certain subsections of that same object. | any                | any                        |

#### Compound References

References are typically constructed as a sequence. For example: [struct position 0, struct position 1, array offset 2, array slice 1..3].

Field references are in the same order they are defined in their schema. For example, let's consider the following schema:

```
column a:
  struct<
    b: list<
      struct<
        c: map<string, 
          struct<
            x: i32>>>>>
```

If we want to represent the SQL expression:

```
a.b[2].c['my_map_key'].x
```

We will need to declare the nested field such that:

```
Struct field reference a
Struct field b
List offset 2
Struct field c
Map key my_map_key
Struct field x
```

Or more formally in Protobuf Text, we get:

```
selection {
  direct_reference {
    struct_field {
      field: 0 # .a
      child {
        struct_field {
          field: 0 # .b
          child {
            list_element {
              offset: 2
              child {
                struct_field {
                  field: 0 # .c
                  child {
                    map_key {
                      map_key {
                        string: "my_map_key" # ['my_map_key']
                      }
                      child {
                        struct_field {
                          field: 0 # .x
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  root_reference { }
}
```

#### Validation

References must validate against the schema of the record being referenced. If not, an error is expected.

### Masked Complex Expression 

A masked complex expression is used to do a subselection of a portion of a complex record. It allows a user to specify the portion of the complex object to consume. Imagine you have a schema of (note that structs are lists of fields here, as they are in general in Substrait as field names are not used internally in Substrait):

```
struct:
  - struct:
    - integer
    - list:
      struct:
        - i32
        - string
        - string
     - i32
  - i16
  - i32
  - i64
```

Given this schema, you could declare a mask of fields to include in pseudocode, such as:

```
0:[0,1:[..5:[0,2]]],2,3

OR

0:
  - 0
  - 1:
    ..5:
      -0
      -2
2
3
```

This mask states that we would like to include fields 0 2 and 3 at the top-level. Within field 0, we want to include subfields 0 and 1. For subfield 0.1, we want to include up to only the first 5 records in the array and only includes fields 0 and 2 within the struct within that array. The resulting schema would be:

```
struct:
  - struct:
    - integer
    - list:
      struct: 
        - i32
        - string
  - i32
  - i64
```

#### Unwrapping Behavior

By default, when only a single field is selected from a struct, that struct is removed. When only a single element is removed from a list, the list is removed. A user can also configure the mask to avoid unwrapping in these cases. [TBD how we express this in the serialization formats.]



???+ question "Discussion Points"

    * Should we support column reordering/positioning using a masked complex expression? (Right now, you can only mask things out.)

### Outer References

Outer references allow expressions inside a subquery to access records from an enclosing relation. The `OuterReference` root type supports two mutually exclusive resolution strategies:

#### `steps_out` (offset-based)

`steps_out` resolves the reference by counting subquery boundaries upward (`steps_out >= 1`). This works correctly whenever the plan is a **tree**, i.e., when each relation has exactly one parent, the path to the binding relation can be uniquely determined via `steps_out`.

#### `id_reference` (id-based)

`id_reference` resolves the reference by referring to the binding relation via its plan-wide unique `RelCommon.id`. The `id` on the referenced relation must be set (>= 1) and unique across all relations in the plan.

#### When to use `id_reference`

`id_reference` must be used instead of `steps_out` when a plan contains **shared relations** via `ReferenceRel` with unresolved outer references. In such plans, the binding relation (i.e., the relation providing the actual value of the outer reference) can be reached through multiple paths with different depths, making offset-based resolution ambiguous because `steps_out` depends on *which* path is followed.

For example, consider a plan with two nested scalar subqueries that share a common relation `x`. The outer reference to `tableA.a` lives inside `x`, which is reached via paths of different depth:

```
PlanRel.relations[0].rel:  # let's call it 'x'
FilterRel(a > outer_ref(tableA.a))
  └── ReadRel(tableB)

PlanRel.relations[1].root:
ProjectRel [id=1]
├── ReadRel(tableA)
└── Subquery.Scalar ── (1)
    └── SetRel(MINUS_PRIMARY)
        ├── Subquery.Scalar ── (2)
        │   └── ReferenceRel(0)
        └── ReferenceRel(0)
```

The shared relation `x` contains an outer reference to `tableA.a`. It is reached through two paths — one via subquery (2), one directly — so `steps_out` would need a different value depending on which path is followed. With `id_reference = 1`, the reference inside `x` unambiguously names `ProjectRel` to resolve the outer reference regardless of path.

