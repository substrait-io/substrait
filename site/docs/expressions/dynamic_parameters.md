# Dynamic Parameter Expression

The dynamic parameter expression represents a placeholder within an expression whose value is determined at runtime.
This is particularly useful for parameterized queries where certain values are not known until execution.
Additionally, using dynamic parameters can enable other use cases, such as sharing execution plans without embedding sensitive information.

A dynamic parameter expression includes the following properties:

| Property | Description                                                                 | Required |
|----------|-----------------------------------------------------------------------------|----------|
| `type`   | Specifies the expected data type of the dynamic parameter.                  | Yes      |
| `index`  | Indicates the 0-based position of the parameter within the plan.            | Yes      |
