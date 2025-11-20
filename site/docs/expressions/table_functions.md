# Table Functions

Table functions are functions that produce a relation (zero or more records) as output. Table functions are similar to scalar and aggregate functions but produce relations instead of individual values.

!!! warning "Partial Implementation"
    **Currently implemented:** 0-input table functions - leaf operators that take constant arguments and produce relations.

    **Not yet implemented:** Transformation table functions that accept input relations.

## Function Signature Properties

Table function signatures contain the properties defined below:

| Property               | Description                                                                                                                                                                                                                                                                                | Required                            |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------- |
| Name                   | One or more user-friendly, case-sensitive UTF-8 strings that are used to reference this function.                                                                                                                                                                                         | At least one value is required.     |
| List of arguments      | Argument properties follow the same pattern as [scalar functions](scalar_functions.md#argument-types): value arguments, type arguments, and enumerations. Arguments can be fully defined or calculated with a type expression.                                                             | Optional, defaults to niladic.      |
| Variadic Behavior      | Whether the last argument of the function is variadic or a single argument. If variadic, the argument can optionally have a lower bound (minimum number of instances) and an upper bound (maximum number of instances).                                                                    | Optional, defaults to single value. |
| Description            | Additional description of function for implementers or users. Should be written human-readable to allow exposure to end users. Presented as a map with language => description mappings.                                                                                                  | Optional                            |
| Return Schema          | The output schema of the relation, expressed as an `ExpressionNamedStruct`. Can be concrete types or type-parameterized (derived from argument types). May be omitted if schema depends on runtime data content (not determinable from function signature).                               | Optional (see [Schema Determination](#schema-determination)) |
| Implementation Map     | A map of implementation locations for one or more implementations of the given function. Each key is a function implementation type with properties associated with retrieval of that implementation.                                                                                      | Optional                            |

Table functions use the same argument types as scalar functions (value arguments, type arguments, and enumerations). See [Scalar Functions - Argument Types](scalar_functions.md#argument-types) for details.

## Schema Determination

Table function schemas can be specified in two ways, depending on whether the YAML definition includes a `return` field:

### Schemas Derivable from YAML

When a table function's YAML definition includes a `return` field, the schema can be deterministically derived from the function signature and the types of the bound arguments. The plan must include this derived schema in the `table_schema` field with any type parameters resolved. 

**Example YAML definitions** (from `functions_table.yaml`):

Concrete type example - `generate_series` always produces `{value: i64}`:

```yaml
- name: "generate_series"
  impls:
    - args:
        - name: start
          value: i64
        - name: stop
          value: i64
        - name: step
          value: i64
      return:
        names:
          - value
        struct:
          types:
            - i64
```

Type-parameterized and variadic example - `unnest` can take 1 or more lists:

```yaml
- name: "unnest"
  impls:
    - args:
        - name: input
          value: list<any1>
      variadic:
        min: 1
      return:
        names:
          - element
        struct:
          types:
            - any1
```

### Schemas Determined by Plan Producer

When a table function's YAML definition omits the `return` field, the schema cannot be determined from type information alone. In these cases, the plan producer must provide the schema in `table_schema`.

**Example:** A function like `read_parquet(path)` where the schema depends on the actual Parquet file's structure - the YAML would omit the `return` field, and the plan producer would provide the concrete schema in `table_schema`.

!!! note "Required Constraint"
    **If a table function's YAML definition includes a `return` field, the `table_schema` field in the plan MUST match the YAML definition (with any type parameters resolved based on the bound argument types).**

## Variadic Table Functions

Table functions can be variadic, meaning the last parameter can be repeated one or more times. This is specified using the `variadic` field in the YAML definition with a `min` value indicating the minimum number of times the parameter must appear.

**Example:** The `unnest` function is variadic with `min: 1`, allowing it to accept one or more list arguments:

```yaml
- name: "unnest"
  impls:
    - args:
        - name: input
          value: list<any1>
      variadic:
        min: 1
      return:
        names:
          - element
        struct:
          types:
            - any1
```

When multiple arguments are provided to a variadic table function, the behavior depends on the function's semantics. For example, `unnest` expands multiple lists in parallel (like a zip operation).

## Usage in Plans

Table functions are invoked in plans using the `TableFunctionRel` relation type. Table functions can be used anywhere a relation is expected - as a leaf node, or as input to other relational operators like `FilterRel`, `ProjectRel`, etc.

For details on the `TableFunctionRel` message structure and properties, see the [Table Function](../relations/logical_relations.md#table-function) section in Logical Relations.

## Examples

### Example 1: Generating a Sequence

Generate integers from 1 to 100:

```
TableFunctionRel {
  function_reference: <generate_series>
  arguments: [
    { value: { literal: { i64: 1 } } },
    { value: { literal: { i64: 100 } } }
  ]
  table_schema: {
    names: ["value"]
    struct: {
      types: [{ i64: {} }]
    }
  }
}
```

**SQL equivalent:** `SELECT * FROM generate_series(1, 100)`

**Output:**
```
value
-----
1
2
3
...
100
```

### Example 2: Unnest a Literal Array

Unnest a literal list into rows:

```
TableFunctionRel {
  function_reference: <unnest>
  arguments: [
    { value: { literal: {
      list: {
        values: [
          { string: { value: "apple" } },
          { string: { value: "banana" } },
          { string: { value: "cherry" } }
        ]
      }
    } } }  // Type is list<string>
  ]
  table_schema: {
    names: ["element"]
    struct: {
      types: [{ string: {} }]  // T resolved to string from list<string> argument
    }
  }
}
```

**SQL equivalent:** `SELECT * FROM UNNEST(['apple', 'banana', 'cherry'])`

**Output:**
```
element
--------
apple
banana
cherry
```

### Example 2b: Variadic Unnest (Multiple Lists)

Unnest multiple lists to produce their cross product:

```
TableFunctionRel {
  function_reference: <unnest>
  arguments: [
    { value: { literal: {
      list: {
        values: [
          { i32: { value: 1 } },
          { i32: { value: 2 } }
        ]
      }
    } } },  // First list: [1, 2]
    { value: { literal: {
      list: {
        values: [
          { i32: { value: 3 } },
          { i32: { value: 4 } }
        ]
      }
    } } }  // Second list: [3, 4]
  ]
  table_schema: {
    names: ["element"]  // Note: With multiple lists, schema may have multiple fields
    struct: {
      types: [{ i32: {} }]
    }
  }
}
```

**SQL equivalent:** `SELECT * FROM UNNEST([1, 2], [3, 4])`

**Output (parallel expansion / zip):**
```
element
-------
1, 3
2, 4
```

!!! note "Limitation: Correlated Table Functions"
    **The more sophisticated use case - unnesting a column from an existing table - cannot currently be represented.** For example, the SQL query `SELECT element FROM my_table, UNNEST(my_table.array_column)` would require applying the table function once per row of the input table.

    This requires **lateral joins** (correlated subqueries where a table function references columns from an outer relation), which are not yet specified in Substrait. Since TableFunctionRel is currently a leaf operator with no input relation, you cannot use field references in the function arguments.

    Future extensions will add support for transformation table functions and/or lateral join semantics to handle these cases.


### Example 3: Composing with Other Operators

Table functions can be composed with other relational operators. For example, filtering the generated series to get only even numbers:

```
FilterRel {
  input: {
    TableFunctionRel {
      function_reference: <generate_series>
      arguments: [
        { value: { literal: { i64: 1 } } },
        { value: { literal: { i64: 100 } } }
      ]
      table_schema: {
        names: ["value"]
        struct: {
          types: [{ i64: {} }]
        }
      }
    }
  }
  condition: {
    scalar_function: {
      function_reference: <equals>
      arguments: [
        {
          value: {
            scalar_function: {
              function_reference: <modulo>
              arguments: [
                { value: { selection: { direct_reference: { struct_field: { field: 0 } } } } },
                { value: { literal: { i64: 2 } } }
              ]
            }
          }
        },
        { value: { literal: { i64: 0 } } }
      ]
    }
  }
}
```

## Future Extensions

The current specification focuses on 0-input (generator/leaf) table functions. Future versions may support:

- **Transformation table functions**: Functions that take an input relation and transform it (by adding an optional `input` field to `TableFunctionRel`)
- **Set-returning functions**: Functions that process input records and produce multiple output records per input
- **Lateral joins**: Applying table functions to each row of an input relation
