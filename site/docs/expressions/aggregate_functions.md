# Aggregate Functions

Aggregate functions are functions that define an operation which consumes values from multiple records to a produce a single output. Aggregate functions in SQL are typically used in GROUP BY functions. Aggregate functions are similar to scalar functions and function signatures with a small set of different properties.

Aggregate function signatures contain all of the properties defined for [scalar functions](scalar_functions.md). Additionally, they contain the properties below:

| Property                 | Description                                                  | Required                        |
| ------------------------ | ------------------------------------------------------------ | ------------------------------- |
| Inherits                 | All properties defined for scalar function                   | N/A                             |
| Ordered                  | Whether this aggregation function should allow user ordering | Optional, defaults to false     |
| Maximum set size         | Maximum allowed set size as an unsigned integer              | Optional, defaults to unlimited |
| Decomposable             | Whether the function can be executed in one or more intermediate steps. Valid options are: `NONE`, `ONE`, `MANY`, describing how intermediate steps can be taken. | Optional, defaults to `NONE`     |
| Intermediate Output Type | If the function is decomposable, represents the intermediate output type that is used, if the function is defined as either `ONE` or `MANY` decomposable. Will be a struct in many cases. | Required for `ONE` and `MANY`.      |



## Aggregate Binding

When Binding an Aggregate function, the binding must include the following additional properties beyond the standard scalar binding properties:

| Property | Description                                                  |
| -------- | ------------------------------------------------------------ |
| Phase    | Describes the input type of the data: [INITIAL_TO_INTERMEDIATE, INTERMEDIATE_TO_INTERMEDIATE, INITIAL_TO_RESULT, INTERMEDIATE_TO_RESULT] describing what portion of the operation is required. For functions that are NOT decomposable, the only valid option will be INITIAL_TO_RESULT. |
| Ordering | One or more ordering keys along with key order (ASC\|DESC\|NULL FIRST, etc), declared similar to the sort keys in an `ORDER BY` relational operation. Only allowed in cases where the function signature supports Ordering. |

