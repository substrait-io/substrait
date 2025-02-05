# Dynamic Parameter Expression

The dynamic parameter expression represents a placeholder within an expression whose value is determined at runtime.
This is particularly useful for parameterized queries where certain values are not known until execution.
Additionally, using dynamic parameters can enable other use cases, such as sharing execution plans without embedding sensitive information.

A dynamic parameter expression includes the following properties:

| Property              | Description                                                                   | Required |
|-----------------------|-------------------------------------------------------------------------------|----------|
| `type`                | Specifies the expected data type of the dynamic parameter.                    | Yes      |
| `parameter_reference` | A surrogate key used within a plan to reference a specific parameter binding. | Yes      |
