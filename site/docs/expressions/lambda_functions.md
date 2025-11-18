# Lambda Expressions

Lambda expressions enable higher-order functions that operate on collections, allowing for flexible and composable data transformations. A lambda is an anonymous function with explicit parameter types and a body expression that can reference those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions. They enable powerful functional programming patterns such as `map` and `filter` operations on arrays.

!!! note "Documentation Syntax"
    This documentation uses the syntax `(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. The type annotations shown here are for clarity only; actual Substrait syntax uses type declarations in the protobuf message or YAML definitions.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameter Types    | List of types for the lambda's parameters                                  | `parameter_types`     | Yes      |
| Return Type        | Type of the value returned by the lambda                                   | `return_type`         | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaParameterReference) | `body`                | Yes      |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
    ```

## Parameter References

Lambda parameters are referenced within the lambda body using `LambdaParameterReference` in field references. This mechanism is similar to how [`OuterReference`](field_references.md#outerreference) works for subqueries.

### LambdaParameterReference Fields

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `lambda_depth`    | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |
| `reference`       | Zero-based index into the lambda's parameter list                   | 0, 1, 2... |

### Simple Example

For a lambda `(x: i32) -> x * 2`:

Example of an Expression.Lambda message:
```protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
```

## Function Type Syntax

In YAML extension definitions, function types are specified using the `func` keyword with generic type parameters:

**Single parameter** (represents a lambda with 1 entry in `parameter_types`):
```yaml
func<T -> U>           # Single parameter without parentheses
func<(T) -> U>         # Single parameter with parentheses (equivalent)
```

**Multiple parameters** (represents a lambda with 2+ entries in `parameter_types`):
```yaml
func<(T, U) -> V>      # Multiple parameters (parentheses required)
func<(T, U, V) -> W>   # Three parameters
```

Example function definition with multi-parameter lambda:
```yaml
--8<-- "examples/extensions/lambda_function_example.yaml"
```

## Closures

Lambda expressions naturally support closures - they can reference variables from the enclosing scope beyond their own parameters. There are three ways lambdas can capture external values:

### Capturing from Outer Lambda Parameters

Lambdas can be nested, and inner lambdas can reference parameters from outer lambdas using `lambda_depth`.

When an inner lambda only references its own parameters (using `lambda_depth: 0`), it's simply nested without capturing from the outer scope:

```
map(matrix, (row: list<i32>) ->
  map(row, (cell: i32) ->
    cell + 1
  )
)
```

Example of an Expression.Lambda message:
```protobuf
--8<-- "examples/proto-textformat/lambdas/nested_map.textproto"
```

To capture variables from the outer lambda, use `lambda_depth` greater than 0. This example adds the first element of each row to all cells in that row:

```
map(matrix, (row: list<i32>) ->
  map(row, (cell: i32) ->
    cell + row[0]  // 'row' is from the outer lambda (lambda_depth: 1)
  )
)
```

Example of an Expression.Lambda message demonstrating `lambda_depth: 1`:
```protobuf
--8<-- "examples/proto-textformat/lambdas/closure_outer_lambda_simple.textproto"
```

### Capturing from Input Record

A lambda can reference fields from the input record using [`RootReference`](field_references.md#rootreference).

Consider an input record with this schema:

|             | id  | name   | numbers      | offset |
|-------------|-----|--------|--------------|--------|
| Field Index | 0   | 1      | 2            | 3      |
| Type        | i32 | string | list<i32\>  | i32    |

The lambda can access the `offset` field (index 3) from the input record:

```
map(numbers, (x: i32) ->
  x + offset  // 'offset' is field 3 from the input record
)
```

Example of an Expression.Lambda message:
```protobuf
--8<-- "examples/proto-textformat/lambdas/closure_root_reference.textproto"
```

### Capturing from Outer Queries

In correlated subquery contexts, lambdas can also reference outer query records using [`OuterReference`](field_references.md#outerreference), similar to how regular expressions in subqueries can reference outer query fields.

## Higher-Order Functions

Lambdas are primarily used with higher-order functions that operate on collections. Current functions include:

- `map` - Map each element of an array to a new value
- `filter` - Filter elements based on a predicate

See the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) for the complete list of lambda-accepting functions and their signatures.

## Lambda Invocation

Lambda expressions can be invoked directly using the `LambdaInvocation` expression type. This enables immediate invocation patterns where a lambda is defined and invoked in a single expression.

A lambda invocation consists of:
- **Lambda**: The inline lambda expression to invoke
- **Arguments**: The values to pass as parameters to the lambda

The return type is derived from the lambda's `return_type` field - no separate output type declaration is needed.

=== "LambdaInvocation Message"
    ```proto
%%% proto.message.Expression.LambdaInvocation %%%
    ```

### Example

Invoking `((x: i32) -> x * 2)(5)` to compute 10:

```protobuf
--8<-- "examples/proto-textformat/lambdas/inline_invocation.textproto"
```

### Validation

When validating a lambda invocation:
1. The number of arguments must match the lambda's `parameter_types` count exactly
2. Each argument's type must match the corresponding parameter type
3. The invocation's effective return type is `lambda.return_type`

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
