# Table Functions

Table functions produce a relation containing zero or more rows per invocation.

Table functions are invoked using the [Table Function relation](../relations/logical_relations.md#table-function), which returns their generated rows.

## Function Definition Properties

Table functions are declared under `table_functions` in extension YAML files.

| Property | Description | Required |
| -------- | ----------- | -------- |
| Name | A case-sensitive UTF-8 name used to reference the function. | Required |
| List of arguments | Value, type, or enumeration arguments, following [scalar function argument types](scalar_functions.md#argument-types). | Optional, defaults to niladic. |
| Variadic Behavior | Whether the last argument can be repeated, with optional minimum and maximum counts. | Optional, defaults to a single argument. |
| Options | Named behavioral options declared per implementation, following the [scalar function option mechanism](scalar_functions.md#options). | Optional |
| Description | Human-readable description of the function or an individual implementation. | Optional |
| Schema | A type expression that resolves to a non-nullable struct describing the output row's columns. May be omitted when the output schema cannot be derived from argument types using a YAML type expression. | Optional, see [Schema Determination](#schema-determination). |

## Table Function Calls

| Field | Description | Required |
| ----- | ----------- | -------- |
| `function_reference` | A function anchor in the plan referring to a table function in an extension YAML file. Zero is valid, though non-zero anchors are preferred for ergonomics. | Required |
| `arguments` | Arguments matching the selected implementation's signature, including any variadic arguments. | Optional for niladic functions. |
| `options` | Named behavioral preferences using the implementation's declared options. | Optional |
| `output_type` | The concrete `Type.Struct` schema of every generated row, with `nullability` set to `NULLABILITY_REQUIRED`. | Required |

Value arguments must be constant expressions, such as literals or scalar functions of constant arguments. References to input or outer relation fields are not allowed.

Table functions always follow the `DECLARED_OUTPUT` [nullability handling](scalar_functions.md#nullability-handling) rules, with the additional requirement that the output type is a non-nullable struct.

## Schema Determination

Every call must declare a concrete `output_type`, regardless of whether its YAML implementation declares a `schema` expression. The output type is fixed for the call, and every generated row must conform to it.

### Schemas Derivable from YAML

If an implementation declares a `schema` expression, the call's `output_type` must match that expression evaluated with the bound argument types.

### Schemas Not Derivable from YAML

An implementation may omit `schema` when the output schema cannot be derived from argument types using a YAML type expression. No additional marker is required.

When `schema` is omitted, the producer must still determine a concrete `output_type` using argument values, external information, or other function-specific knowledge. The consumer must agree with the declared `output_type`; it must not substitute a different output schema.

For example, this parsing function uses a schema-defining string argument to determine its output columns. Its YAML definition intentionally omits `schema`.

```yaml
--8<-- "examples/extensions/producer_resolved_table_function.yaml"
```

The producer resolves the schema argument's value and declares the resulting struct in the function call. Function anchor 1 refers to `parse_records:str_str` from the example extension.

```protobuf
--8<-- "examples/proto-textformat/table_function/producer_resolved_schema.textproto"
```

## Standalone Invocation Examples

These examples are `TableFunctionRel` messages. Each contains the function call in its `function` field. Function anchor 1 refers to the indicated function in the `functions_table` extension.

### Generating a Sequence

Generate integers from 1 to 100 using `generate_series:i64_i64_i64` with an explicit step of 1.

```protobuf
--8<-- "examples/proto-textformat/table_function_rel/generate_series.textproto"
```

The SQL equivalent is `SELECT * FROM generate_series(1, 100, 1)`.

=== "TableFunction Message"

    ```proto
%%% proto.algebra.TableFunction %%%
    ```
