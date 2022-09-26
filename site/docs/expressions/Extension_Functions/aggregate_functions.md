# Aggregate Functions

Aggregate functions are functions that define an operation which consumes values from multiple records to a produce a single output. Aggregate functions in SQL are typically used in GROUP BY functions. Aggregate functions are similar to scalar functions and function signatures with a small set of different properties.

Aggregate function signatures contain all the properties defined for [scalar functions](scalar_functions.md). Additionally, they contain the properties below:

| Property                 | Description                                                     | Required                        |
| ------------------------ | --------------------------------------------------------------- | ------------------------------- |
| Inherits                 | All properties defined for scalar function.                     | N/A                             |
| Ordered                  | Whether the result of this function is sensitive to sort order. | Optional, defaults to false     |
| Maximum set size         | Maximum allowed set size as an unsigned integer.                | Optional, defaults to unlimited |
| Decomposable             | Whether the function can be executed in one or more intermediate steps. Valid options are: `NONE`, `ONE`, `MANY`, describing how intermediate steps can be taken. | Optional, defaults to `NONE`     |
| Intermediate Output Type | If the function is decomposable, represents the intermediate output type that is used, if the function is defined as either `ONE` or `MANY` decomposable. Will be a struct in many cases. | Required for `ONE` and `MANY`.      |
| Invocation               | Whether the function uses all or only distinct values in the aggregation calculation. Valid options are: `ALL`, `DISTINCT`. | Optional, defaults to `ALL`     |

## Aggregate Binding

When binding an aggregate function, the binding must include the following additional properties beyond the standard scalar binding properties:

| Property | Description                                                  |
| -------- | ------------------------------------------------------------ |
| Phase    | Describes the input type of the data: [INITIAL_TO_INTERMEDIATE, INTERMEDIATE_TO_INTERMEDIATE, INITIAL_TO_RESULT, INTERMEDIATE_TO_RESULT] describing what portion of the operation is required. For functions that are NOT decomposable, the only valid option will be INITIAL_TO_RESULT. |
| Ordering | Zero or more ordering keys along with key order (ASC\|DESC\|NULL FIRST, etc.), declared similar to the sort keys in an `ORDER BY` relational operation. If no sorts are specified, the records are not sorted prior to being passed to the aggregate function. |

When the phase is `*_TO_INTERMEDIATE`, the return type of the aggregate function is overridden to the intermediate type. When the phase is `INTERMEDIATE_TO_*`, non-constant value argument slots are overridden to behave like type argument slots instead, and an extra value argument is expected at the end of the actual argument list, of which the type matches the derived intermediate type exactly. Using the following function as an example:

```
min_max_difference(T??) -> STRUCT<T?,T?> ->
    assert T == i8 || T == i16 || T == i32 || T == i64
    T?
```

 - `INITIAL_TO_RESULT` would bind to `min_max_difference(i32)` and yield `i32?`;
 - `INITIAL_TO_INTERMEDIATE` would bind to `min_max_difference(i32)` and yield `STRUCT<i32?, i32?>`;
 - `INTERMEDIATE_TO_INTERMEDIATE` would bind to `min_max_difference(type i32, STRUCT<i32?, i32?>)` and yield `STRUCT<i32?, i32?>`;
 - `INTERMEDIATE_TO_RESULT` would bind to `min_max_difference(type i32, STRUCT<i32?, i32?>)` and yield `i32?`;

!!! note

    The value to type argument replacement is necessary, because the intermediate and return type derivations may depend on it in a nontrivial and (in general) non-reversable way.

## Pattern Matching and Evaluation Order

The patterns used to define the argument types and return type are processed in the following order.

 - Match the actual argument types against the argument slot patterns from left to right. The pattern from the last argument slot may be matched any number of times if the function is variadic. For `INTERMEDIATE_TO_*` bindings, the intermediate data input is not matched yet.
 - Evaluate any statements in the return type specification from top to bottom/left to right.
 - Evaluate the return type pattern (even for `*_TO_INTERMEDIATE` bindings where the result is not used; evaluation may still affect whether the function exists or not).
 - Evaluate the intermediate type pattern, if one is specified (even for `INITIAL_TO_RESULT` bindings; note also that it is evaluated and *then* matched even for `INTERMEDIATE_TO_RESULT`).
 - For `INTERMEDIATE_TO_*` bindings, match the above evaluation result against the data type passed to the last argument; they must be exactly equal.

If any pattern fails to match or evaluate, the function is said to not match the given argument pack.
