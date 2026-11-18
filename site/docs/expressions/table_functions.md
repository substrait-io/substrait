# Table Functions

Table functions produce a relation containing zero or more rows per invocation.

## Function Definition Properties

Table functions are declared under `table_functions` in extension YAML files.

| Property | Description | Required |
| -------- | ----------- | -------- |
| Name | A case-sensitive UTF-8 name used to reference the function. | Required |
| List of arguments | Value, type, or enumeration arguments, following [standard function argument types](scalar_functions.md#argument-types). | Optional, defaults to niladic. |
| Variadic Behavior | Whether the last argument of the function is variadic or a single argument. If variadic, the argument can optionally have a lower bound (minimum number of instances) and an upper bound (maximum number of instances). |
| Options | Named behavioral options declared per implementation, following the [standard function option mechanism](scalar_functions.md#options). | Optional |
| Description | Additional description of function for implementers or users. Should be written human-readable to allow exposure to end users. Can be specified at the function level and/or on individual implementations to document overload-specific behavior. | Optional |
| Schema | A type expression that resolves to a non-nullable struct describing the output row's columns. May be omitted when the output schema cannot be derived from argument types using a YAML type expression. | Optional, see [Schema Determination](#schema-determination). |

## Table Function Calls

Table functions are invoked using the [Table Function relation](../relations/logical_relations.md#table-function-operator), which returns their generated rows.

| Field | Description | Required |
| ----- | ----------- | -------- |
| `function_reference` | A function anchor in the plan referring to a table function in an extension YAML file. Zero is valid, though non-zero anchors are preferred for ergonomics. | Required |
| `arguments` | Arguments matching the selected implementation's signature, including any variadic arguments. | Optional for niladic functions. |
| `options` | Named behavioral preferences using the implementation's declared options. | Optional |
| `schema` | The concrete `Type.Struct` schema of every generated row, with `nullability` set to `NULLABILITY_REQUIRED`. | Required |

Value arguments are evaluated once per invocation. `TableFunctionRel` has no input of its own, but its arguments may reference outer rows in scope, such as the left row of a [lateral join](../relations/logical_relations.md#lateral-join-operation).

Table functions always follow the `DECLARED_OUTPUT` [nullability handling](scalar_functions.md#nullability-handling) rules, with the additional requirement that the output type is a non-nullable struct.

## Schema Determination

Every call must declare a concrete `schema`, regardless of whether its YAML implementation declares a `schema` expression. The output type is fixed for the call, and every generated row must conform to it.

A plan is invalid if the declared `schema` is incompatible with the table function's specified output schema.

### Schemas Derivable from YAML

If an implementation declares a `schema` expression, the call's `schema` must match that expression evaluated with the bound argument types.

For example, this function produces rows containing a value of whatever type its first argument has.

```yaml
--8<-- "examples/extensions/derived_schema_table_function.yaml"
```

In this call, the `value` argument is a `string` literal, so `any1` binds to `string` and the `schema` expression `struct<any1>` evaluates to `struct<string>`. The call's `schema` must therefore be `struct<string>`. Function anchor 1 refers to `repeat_value:any_i64` from the example extension.

```protobuf
--8<-- "examples/proto-textformat/table_function/derived_schema.textproto"
```

### Schemas Not Derivable from YAML

An implementation may omit `schema` when the output schema cannot be derived from argument types using a YAML type expression.

When the YAML implementation omits `schema`, the producer must still determine a concrete `schema` for the call using argument values, external information, or other function-specific knowledge. The consumer must agree with the declared `schema`; it must not substitute a different output schema.

For example, this parsing function interprets comma-separated records according to the function call's declared `schema`. Its YAML definition intentionally omits `schema` because the output columns cannot be derived from the payload's string type.

```yaml
--8<-- "examples/extensions/producer_resolved_table_function.yaml"
```

The producer declares the desired output struct in the function call's `schema`. Function anchor 1 refers to `parse_records:str` from the example extension.

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

## Lateral Invocation Example

Generate a sequence for each row of `series_bounds`, using the row's `n` as the stop argument.

```sql
SELECT b.n, s.value
FROM series_bounds b
CROSS JOIN LATERAL generate_series(1, b.n, 1) AS s(value)
```

```protobuf
--8<-- "examples/proto-textformat/plan_rel/lateral_generate_series.textproto"
```

=== "TableFunction Message"

    ```proto
%%% proto.algebra.TableFunction %%%
    ```
