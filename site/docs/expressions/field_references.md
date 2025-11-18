# Field References

In Substrait, all fields are dealt with on a positional basis. Field names are only used at the edge of a plan, for the purposes of naming fields for the outside world. Each operation returns a simple or compound data type. Additional operations can refer to data within that initial operation using field references. To reference a field, you use a reference based on the type of field position you want to reference.

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

### Reference Root Types

Field references can originate from different data sources depending on the evaluation context. The root type specifies where the referenced data comes from:

| Root Type                   | Description                                                                                      | Use Case                                    |
|-----------------------------|--------------------------------------------------------------------------------------------------|---------------------------------------------|
| RootReference               | References fields from the current input record being processed by the expression               | Standard field access in most operations    |
| OuterReference              | References fields from an outer query's record in subquery contexts                             | Correlated subqueries                        |
| LambdaParameterReference    | References parameters passed to a lambda function                                                | Lambda function bodies                       |
| Expression                  | References the output of another expression as the root                                          | Accessing fields from computed values        |

#### RootReference

The most common reference type. Refers to fields in the current input record.

**Example**: In `SELECT col1 + col2 FROM table`, both `col1` and `col2` use RootReference to access the current row.

```protobuf
selection {
  direct_reference { struct_field { field: 0 } }
  root_reference { }
}
```

#### OuterReference

Used in subqueries to reference fields from the outer query's record. The `steps_out` field indicates how many subquery boundaries to traverse (minimum value: 1).

**Example**: In `SELECT (SELECT COUNT(*) FROM orders WHERE orders.user_id = users.id) FROM users`, the `users.id` reference within the subquery uses OuterReference with `steps_out: 1`.

```protobuf
selection {
  direct_reference { struct_field { field: 0 } }
  outer_reference { steps_out: 1 }
}
```

#### LambdaParameterReference

Used within lambda function bodies to reference the lambda's parameters. This enables higher-order functions like `transform`, `filter`, and `reduce`.

**Fields**:
- `lambda_depth`: Number of lambda boundaries to traverse (0 = current lambda, 1 = outer lambda, etc.)
- `reference`: Zero-based index into the lambda's parameter list

**Example**: In `transform([1, 2, 3], (x) -> x * 2)`, the `x` in the body uses LambdaReference:

```protobuf
selection {
  direct_reference { struct_field { field: 0 } }
  lambda_reference {
    lambda_depth: 0      // Current lambda
    reference: 0         // First parameter
  }
}
```

**Nested Lambda Example**: In nested lambdas, `lambda_depth` allows inner lambdas to reference outer lambda parameters:

```
transform(matrix, (row) ->
  transform(row, (cell) ->
    cell + row.length  // 'cell' uses depth 0, 'row' uses depth 1
  )
)
```

**Closures**: Lambda bodies can reference data from multiple sources simultaneously:
- Lambda parameters via LambdaReference
- Input record fields via RootReference (closure over input)
- Outer query fields via OuterReference (closure over outer queries)

See [Lambda Functions](lambda_functions.md) for more details and examples.

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

