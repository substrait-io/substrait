# Table Functions

!!! warning "Partial Implementation"
    **Currently implemented:** 0-input table functions - leaf operators that take constant arguments and produce relations.

    **Not yet implemented:** Transformation table functions that accept input relations.

## Definition

Table functions (0-input, currently supported) are **leaf operators** in the query tree that:

- Take a **fixed number of constant arguments** (literals or expressions that can be evaluated without input data)
- Produce **zero or more records** as output (a relation/table)
- Do **not consume an input relation** - they generate data from constants
- Have either a **derived schema** (determinable from the YAML `return` field and argument types) or an **explicit schema** (determined at runtime when YAML omits the `return` field)

See [Schema Determination](#schema-determination) for details on how schemas are specified.

Future extensions may add support for transformation table functions that consume and transform input relations by adding an optional input field to `TableFunctionRel`.

## Function Signatures

Table functions are defined in YAML extension files, similar to scalar, aggregate, and window functions. A table function signature specifies:

- **Arguments**: The parameters the function accepts (must be constant expressions)
- **Schema**: The output schema of the generated relation, expressed as an `ExpressionNamedStruct` that can be static or type-parameterized (may or may not be specified in the YAML definition)

## Schema Determination

Table function schemas can be specified in two ways, depending on whether the YAML definition includes a `return` field:

### Derived Schemas (`derived: true`)

When a table function's YAML definition **includes a `return` field**, the schema can be deterministically derived from the function signature and the types of the bound arguments.

- In the plan: Set `derived: true` and include the schema from YAML in `table_schema` with any type parameters resolved
- The schema is fully determinable from type information alone

This includes both:
- **Concrete types**: Schema is fixed (e.g., `generate_series` always produces `{value: i64}`)
- **Type-parameterized**: Schema depends on argument types (e.g., `unnest(list<T>)` produces `{element: T}` where `T` is resolved from the argument)

**Example YAML definitions** (from `functions_table.yaml`):

```yaml
# Concrete type example - schema is always {value: i64}
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

# Type-parameterized example - schema is {element: T} where T comes from list<T>
- name: "unnest"
  impls:
    - args:
        - name: input
          value: "list<T>"
      return:
        names:
          - element
        struct:
          types:
            - T
```

### Explicit Schemas (`derived: false`)

When a table function's YAML definition **omits the `return` field**, the schema depends on runtime data content and cannot be determined from type information alone.

- In the plan: Set `derived: false` and provide the schema in `table_schema`
- The plan producer determines the schema (e.g., by inspecting file contents, database metadata, etc.)

**Example scenario**: A function like `read_parquet(path)` where the schema depends on the actual Parquet file's structure.

!!! note "Required Constraint"
    **If a table function's YAML definition includes a `return` field, the `derived` field MUST be set to `true` in the plan, and the `table_schema` field MUST match the YAML definition (with any type parameters resolved based on the bound argument types).**

### Plan Examples

Now let's see how these two cases appear in actual Substrait plans:

#### Derived Schema Examples

For functions where the YAML includes a `return` field, set `derived: true`. The schema is derived from the YAML definition, with any type parameters resolved based on argument types.

**Concrete type example** (`generate_series`):
```
TableFunctionRel {
  function_reference: <generate_series>
  arguments: [
    { value: { literal: { i64: 1 } } },
    { value: { literal: { i64: 100 } } },
    { value: { literal: { i64: 1 } } }
  ]
  derived: true  // Schema from YAML return field
  table_schema: {
    names: ["value"]
    struct: {
      types: [{ i64: {} }]  // Matches YAML definition exactly
    }
  }
}
```

**Type-parameterized example** (`unnest`):
```
TableFunctionRel {
  function_reference: <unnest>
  arguments: [
    { value: { literal: { list: [...] } } }  // list<string>
  ]
  derived: true  // Schema from YAML with T resolved
  table_schema: {
    names: ["element"]
    struct: {
      types: [{ string: {} }]  // T resolved to string from list<string> argument
    }
  }
}
```

#### Explicit Schema Example

For functions where the YAML omits the `return` field, set `derived: false` and provide the schema determined by the plan producer:

```
TableFunctionRel {
  function_reference: <read_parquet>
  arguments: [
    { value: { literal: { string: "data.parquet" } } }
  ]
  derived: false  // No return field in YAML - schema from runtime inspection
  table_schema: {
    names: ["id", "name", "age"]
    struct: {
      types: [
        { i32: {} },
        { string: {} },
        { i32: {} }
      ]
    }
  }
}
```

## Usage in Plans

Table functions are represented as their own relation type, `TableFunctionRel`.

### TableFunctionRel Components

- **function_reference**: Points to a function anchor referencing the table function definition
- **arguments**: Must be constant expressions (currently; literals or expressions evaluable without input data)
- **derived**: Boolean flag indicating schema source:
  - `true` - Schema is determinable from the YAML `return` field and argument types (includes both concrete and type-parameterized schemas)
  - `false` - Schema depends on runtime data content (no `return` field in YAML)
- **table_schema**: The output schema (always present). For `derived: true`, must match the YAML `return` field (with type parameters resolved). For `derived: false`, provided by the plan producer.
- **common**: Standard relation properties (emit, hints, etc.)

**Quick reference for setting `derived`:**
- YAML has `return` field → `derived: true`
- YAML omits `return` field → `derived: false`

Table functions can be used anywhere a relation is expected - as a leaf node, or as input to other relational operators like `FilterRel`, `ProjectRel`, etc.

## Examples

### Example 1: Generating a Sequence (Derived Schema - Concrete Types)

Generate integers from 1 to 100:

```
TableFunctionRel {
  function_reference: <generate_series>
  arguments: [
    { value: { literal: { i64: 1 } } },
    { value: { literal: { i64: 100 } } }
  ]
  derived: true
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

### Example 2: Unnest a Literal Array (Derived Schema - Type-Parameterized)

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
  derived: true  // Schema from YAML with T resolved to string
  table_schema: {
    names: ["element"]
    struct: {
      types: [{ string: {} }]
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
      derived: true
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


=== "TableFunctionRel Message"

    ```proto
%%% proto.algebra.TableFunctionRel %%%
    ```
