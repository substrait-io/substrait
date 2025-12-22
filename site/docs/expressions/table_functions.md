# Table Functions

Table functions produce zero or more records for each input record. Unlike scalar functions that return a single value, table functions return a set of rows, making them useful for operations that "explode" or "generate" data from nested structures.

## Overview

Table functions are similar to scalar functions in their definition and reference mechanism, but differ fundamentally in their output:

- **Scalar functions:** Produce exactly one value per input row
- **Window functions:** Produce one value per input row (with access to window of rows)
- **Aggregate functions:** Produce one value per group of input rows
- **Table functions:** Produce zero or more rows per input record

## TableFunction Message

The `TableFunction` message is used to invoke table functions within relational operators (primarily `GenerateRel`).

| Field | Type | Description | Required |
|-------|------|-------------|----------|
| function_reference | uint32 | Points to a function_anchor in the plan that references a table function in the extension YAML. 0 is a valid anchor/reference. | Required |
| arguments | repeated FunctionArgument | Arguments bound to the table function (value, type, or enum arguments) | Required |
| options | repeated FunctionOption | Behavior options for corner cases | Optional |
| output_type | Type | The output type of the table function (must be a Struct type where each field is a generated column) | Required |

### Output Type

The `output_type` field must be a non-nullable `Struct` type where each field represents a column in the generated output rows. This explicitly defines the schema of rows produced by the table function.

**Examples:**
- `explode(array<i32>)` → `Struct{value: i32}`
- `posexplode(array<string>)` → `Struct{pos: i64, value: string}`
- `explode(map<string, i32>)` → `Struct{key: string, value: i32}`

### Arguments

Table function arguments work the same way as scalar function arguments:

- **Value arguments:** Use `FunctionArgument.value` with an expression
- **Type arguments:** Use `FunctionArgument.type`
- **Enum arguments:** Use `FunctionArgument.enum`

For table functions, value arguments typically reference fields containing nested data (arrays, maps, structs) that will be exploded or flattened.

## Common Table Functions

While specific table functions are defined in extension YAML files, common table functions include:

### Array Functions

| Function | Signature | Output | Description |
|----------|-----------|--------|-------------|
| `explode` | `array<T>` → `Struct{value: T}` | One row per array element | Flattens an array by producing one output row per element |
| `posexplode` | `array<T>` → `Struct{pos: i64, value: T}` | One row per array element with position | Like explode but includes 0-indexed position |
| `unnest` | `array<T>` → `Struct{value: T}` | One row per array element | Standard SQL synonym for explode |

### Map Functions

| Function | Signature | Output | Description |
|----------|-----------|--------|-------------|
| `explode` | `map<K,V>` → `Struct{key: K, value: V}` | One row per map entry | Flattens a map by producing one output row per key-value pair |

### Other Table Functions

Table functions can also be used for:

- **JSON parsing:** Extract multiple fields from JSON strings into structured rows
- **String splitting:** Split delimited strings into multiple rows
- **Sequence generation:** Generate arithmetic or date sequences
- **Regex extraction:** Extract multiple matches from text

### Examples

**Simple Array Explode:**
```protobuf
--8<-- "examples/proto-textformat/table_function/simple_explode.textproto"
```

**Position Explode:**
```protobuf
--8<-- "examples/proto-textformat/table_function/posexplode.textproto"
```

## Usage in Relations

Table functions are primarily used within the `GenerateRel` operator, which applies a table function to each row of an input relation.

**Example:**
```
Input Relation: users {user_id: i64, tags: array<string>}
Table Function: explode(tags) → Struct{tag: string}
Output Relation: {user_id: i64, tags: array<string>, tag: string}

Input data:
  {user_id: 1, tags: ['python', 'rust']}
  {user_id: 2, tags: ['java']}

Output data:
  {user_id: 1, tags: ['python', 'rust'], tag: 'python'}
  {user_id: 1, tags: ['python', 'rust'], tag: 'rust'}
  {user_id: 2, tags: ['java'], tag: 'java'}
```

**Protobuf Example:**
```protobuf
--8<-- "examples/proto-textformat/table_function/generate_rel_explode.textproto"
```

See the [Generate Operation](../relations/logical_relations.md#generate-operation) documentation for more details.

## Differences from Other Function Types

| Aspect | Scalar Function | Window Function | Aggregate Function | Table Function |
|--------|----------------|-----------------|-------------------|----------------|
| **Invocation context** | Expressions (ProjectRel) | Window expressions (WindowRel) | Aggregate expressions (AggregateRel) | Relational operators (GenerateRel) |
| **Input scope** | Single row | Window of rows | Group of rows | Single row |
| **Output cardinality** | Exactly 1 value | Exactly 1 value | Exactly 1 value | 0 or more rows |
| **Output type** | Scalar type | Scalar type | Scalar type | Struct type |
| **Can appear in Expression** | Yes | Yes (WindowFunction) | No | No |

## Extension Mechanism

Table functions are defined in extension YAML files using the same mechanism as scalar functions, but with special semantics:

- Must specify output type as a struct
- Must be marked as table-valued in function metadata
- Can only be referenced from relational operators (not expressions)

Example function registration (conceptual):
```yaml
table_functions:
  - name: explode
    impls:
      - args:
          - name: input
            value: list<T>
        return: struct<value: T>
    description: "Produces one row per array element"
```

=== "TableFunction Message"

    ```proto
%%% proto.algebra.TableExpression.TableFunction %%%
    ```

## Future Extensions

Potential future enhancements to table functions:

1. **Cardinality hints:** Allow functions to hint at expected output cardinality for optimization
2. **Ordering guarantees:** Define whether table functions preserve or define ordering of generated rows
3. **Parameterized behavior:** Support options for controlling generation behavior (e.g., include_nulls, deduplicate)
4. **User-defined table functions:** Support for user-defined table functions via embedded functions (Python, WebAssembly)

