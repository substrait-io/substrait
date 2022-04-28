# Field References

In Substrait, all fields are dealt with on a positional basis. Field names are only used at the edge of a plan, for the purposes of naming fields for the outside world. Each operation returns a simple or compound data type. Additional operations can refer to data within that initial operation using field references. To reference a field, you use a reference based on the type of field position you want to reference.

| Reference Type            | Properties                                                   | Type Applicability | Type return                |
| ------------------------- | ------------------------------------------------------------ | ------------------ | -------------------------- |
| Struct Field              | Ordinal position. Zero-based. Only legal within the range of possible fields within a struct. Selecting an ordinal outside the applicable field range results in an invalid plan. | struct             | Type of field referenced   |
| Array Value               | Array offset. Zero-based. Negative numbers can be used to describe an offset relative to the end of the array. For example, -1 means the last element in an array. Negative and positive overflows return null values (no wrapping). | list               | type of list               |
| Array Slice               | Array offset and element count. Zero-based. Negative numbers can be used to describe an offset relative to the end of the array. For example, -1 means the last element in an array. Position does not wrap, nor does length. | list               | Same type as original list |
| Map Key                   | A map value that is matched exactly against available map keys and returned. [TBD, can multiple matches occur?] | map                | Value type of map          |
| Map KeyExpression         | A wildcard string that is matched against a simplified form of regular expressions. Requires the key type of the map to be a character type. [Format detail needed, intention to include basic regex concepts such as greedy/non-greedy.] | map                | List of map value type     |
| Masked Complex Expression | An expression that provides a mask over a schema declaring which portions of the schema should be presented. This allows a user to select a portion of a complex object but mask certain subsections of that same object. | any                | any                        |

#### Compound References

References are typically constructed as a sequence. For example: [struct position 0, struct position 1, array offset 2, array slice 1..3].

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

Given this schema, you could declare a mask in pseudocode, such as:

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



## Discussion Points

* Should we support column reordering/positioning using a masked complex expression? (Right now, you can only mask things out.)





