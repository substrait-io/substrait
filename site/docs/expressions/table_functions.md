# Table Functions

!!! warning "Partial Implementation"
    **Currently implemented:** 0-input table functions - leaf operators that take constant arguments and produce relations.

    **Not yet implemented:** Transformation table functions that accept input relations.

## Definition

Table functions (0-input, currently supported) are **leaf operators** in the query tree that:

- Take a **fixed number of constant arguments** (literals or expressions that can be evaluated without input data)
- Produce **zero or more records** as output (a relation/table)
- Do **not consume an input relation** - they generate data from constants
- Have either a **derived schema** (determinable from function signature) or an **explicit schema** (depends on runtime data)

Future extensions may add support for transformation table functions that consume and transform input relations by adding an optional input field to `TableFunctionRel`.

## Function Signatures

Table functions are defined in YAML extension files, similar to scalar, aggregate, and window functions. A table function signature specifies:

- **Arguments**: The parameters the function accepts (must be constant expressions)
- **Schema**: The output schema of the generated relation
- **Determinism**: Whether the function produces the same output for the same inputs
- **Session Dependency**: Whether the function depends on session state

## Schema Determination

Like scalar functions' return types, table function schemas follow a clear pattern:

!!! note "Required Constraint"
    **If a table function's YAML definition includes an output schema, the `derived` field MUST be set to `true` in the plan, and the `table_schema` field MUST match the YAML definition (with any type parameters resolved based on the bound argument types).**

**Derived schemas (`derived: true`)** - The schema can be **deterministically derived from the function signature**, including:
- **Static schemas**: Fixed output regardless of argument values (e.g., `generate_series` always produces `{value: i64}`)
- **Type-parameterized schemas**: Schema depends on argument types (e.g., `unnest(list<T>)` produces `{element: T}`)

Both cases use `derived: true` because the schema is fully determinable from the function signature and bound argument types.

**Explicit schemas (`derived: false`)** - The schema **depends on runtime data content** and cannot be determined from the function signature alone.

### Derived Schema Examples

For functions where the schema is determinable from the function signature (either concrete or type-parameterized), set `derived: true`. The `table_schema` field contains the schema derived from the YAML definition:

**Static schema example:**
```
TableFunctionRel {
  function_reference: <generate_series>
  arguments: [
    { value: { literal: { i64: 1 } } },
    { value: { literal: { i64: 100 } } }
  ]
  derived: true  // Schema came from YAML definition
  table_schema: {
    names: ["value"]
    struct: {
      types: [{ i64: {} }]
    }
  }
}
```

**Type-parameterized schema example:**
```
TableFunctionRel {
  function_reference: <unnest>
  arguments: [
    { value: { literal: { list: [...] } } }  // list<string>
  ]
  derived: true  // Schema from YAML with T resolved to string
  table_schema: {
    names: ["element"]
    struct: {
      types: [{ string: {} }]  // T resolved to string from list<string>
    }
  }
}
```

### Explicit Schema Examples

For functions where the schema depends on runtime data content, set `derived: false` and provide the schema in `table_schema`:

```
TableFunctionRel {
  common: { ... }
  function_reference: <some_function>
  arguments: [
    // Function arguments
  ]
  derived: false  // Schema was determined by the plan producer
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
  - `true` - Schema determinable from function signature (concrete types or type parameters). **Required when the YAML definition includes a schema.**
  - `false` - Schema depends on runtime data content. **Only allowed when the YAML definition omits the schema field.**
- **table_schema**: The output schema (always present). Must match the YAML definition if derived is true (with type parameters resolved). Contains the actual schema whether derived from YAML or provided by the producer.
- **common**: Standard relation properties (emit, hints, etc.)

**The key distinction:** Set `derived: true` if the schema can be determined by looking at the function signature and argument types in the YAML definition. Set `derived: false` only if the YAML definition omits the schema field because it requires inspecting runtime data content.

Table functions can be used anywhere a relation is expected - as a leaf node, or as input to other relational operators like `FilterRel`, `ProjectRel`, etc.

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
