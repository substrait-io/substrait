# Dynamic Parameter Expression

The dynamic parameter expression represents a placeholder within an expression whose value is determined at runtime.
This is particularly useful for parameterized queries where certain values are not known until execution.
Additionally, using dynamic parameters can enable other use cases, such as sharing execution plans without embedding sensitive information.

A dynamic parameter expression includes the following properties:

| Property              | Description                                                                   | Required |
|-----------------------|-------------------------------------------------------------------------------|----------|
| `type`                | Specifies the expected data type of the dynamic parameter.                    | Yes      |
| `parameter_reference` | A surrogate key used within a plan to reference a specific parameter binding. | Yes      |

## Parameter Bindings in Plans

Dynamic parameters are referenced in expressions using a `parameter_reference` anchor. The actual values for these parameters are provided at the plan level using the `parameter_bindings` field in the `Plan` message.

### DynamicParameterBinding

Each binding maps a parameter anchor to a concrete literal value:

```protobuf
Plan {
  parameter_bindings: [
    DynamicParameterBinding {
      parameter_anchor: 1
      value: Literal { i32: 100 }
    },
    DynamicParameterBinding {
      parameter_anchor: 2
      value: Literal { string: "example" }
    }
  ]
  relations: [
    // Relations containing DynamicParameter expressions with parameter_reference 1 and 2
  ]
}
```

### Properties

| Property | Description | Required |
|----------|-------------|----------|
| `parameter_anchor` | The anchor that identifies the dynamic parameter reference (must match a `parameter_reference` in a `DynamicParameter` expression) | Yes |
| `value` | The literal value assigned to the parameter at runtime. The type of the literal must match the type of the corresponding `DynamicParameter` expression. | Yes |

## Use Cases

### Parameterized Queries

Dynamic parameters enable the same plan to be used with different input values:

```sql
-- SQL query
SELECT * FROM users WHERE age > ? AND city = ?

-- Substrait plan uses DynamicParameter expressions with parameter_reference 1 and 2
-- Different executions provide different parameter_bindings
```

**Execution 1:**
```protobuf
parameter_bindings: [
  { parameter_anchor: 1, value: { i32: 25 } },
  { parameter_anchor: 2, value: { string: "New York" } }
]
```

**Execution 2:**
```protobuf
parameter_bindings: [
  { parameter_anchor: 1, value: { i32: 30 } },
  { parameter_anchor: 2, value: { string: "San Francisco" } }
]
```

### Plan Sharing Without Sensitive Data

Plans can be shared without embedding sensitive values:

```sql
-- SQL query with sensitive value
SELECT * FROM accounts WHERE ssn = '123-45-6789'

-- Substrait plan uses DynamicParameter instead of embedding SSN
-- The actual SSN value is provided separately via parameter_bindings
```

This allows the plan to be:
- Cached and reused
- Logged without exposing sensitive data
- Shared across system boundaries without security concerns

## Validation

When using dynamic parameters, consumers must validate:

1. **Type Matching**: The type of the literal in `parameter_bindings` must match the type specified in the `DynamicParameter` expression
2. **Completeness**: All `parameter_reference` values used in expressions must have corresponding bindings in `parameter_bindings`
3. **Uniqueness**: Each `parameter_anchor` should appear at most once in `parameter_bindings`

## Example

Complete example showing dynamic parameters in a filter expression:

```protobuf
Plan {
  parameter_bindings: [
    DynamicParameterBinding {
      parameter_anchor: 100
      value: Literal { i32: 42 }
    }
  ]
  relations: [
    PlanRel {
      root: RelRoot {
        input: Rel {
          filter: FilterRel {
            input: Rel { read: ReadRel { ... } }
            condition: Expression {
              scalar_function: ScalarFunction {
                function_reference: 0  // greater_than function
                arguments: [
                  Expression { selection: FieldReference { ... } },
                  Expression {
                    dynamic_parameter: DynamicParameter {
                      type: { i32: { nullability: REQUIRED } }
                      parameter_reference: 100
                    }
                  }
                ]
              }
            }
          }
        }
      }
    }
  ]
}
```

In this example, the filter condition compares a field to a dynamic parameter with anchor 100, which is bound to the value 42.
