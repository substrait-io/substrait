# Field References

In CommonPlan, all fields are dealt with on a positional basis. Field names are only used at the edge of a plan for the purposes of naming field ids. Each operation returns a simple or compound data type. Additional operations can refer to data within that initial operation using field references. To reference a field, you use a reference based on the type of field position you want to reference.

| Reference Type    | Properties                                                   | Type Applicability | Type return                |
| ----------------- | ------------------------------------------------------------ | ------------------ | -------------------------- |
| Struct Field      | Ordinal position. Zero-based. Only legal within the range of possible fields within a struct. Selecting an ordinal outside the applicable field range results in an invalid plan. | struct             | Type of field referenced   |
| Array Value       | Array offset. Zero-based. Negative number can be used to describe a offset relative to the end of the array. For example, -1 means the last element in an array. Negative and positive overflows return null values (no wrapping). | list               | type of list               |
| Array Slice       | Array offset and element count. Zero-based. Negative number can be used to describe a offset relative to the end of the array. For example, -1 means the last element in an array. Position does not wrap, nor does length. | list               | Same type as original list |
| Map Key           | A map value that is matched exactly against available map keys and returned. [TBD, can multiple matches occur?] | map                | Value type of map          |
| Map KeyExpression | A wildcard string that is matched against a simplified form of regular expressions. Requires the key type of the map to be a character type. [Format detail needed, intention to include basic regex concepts such as greedy/non greedy.] | map                | List of map value type     |

#### Compound References

References are typically constructed as a sequence. For example: [struct position 0, struct position 1, array offset 2, array slice 1..3].

#### Validation

References must validate against the schema of the record being referenced. If not, an error is expected.

