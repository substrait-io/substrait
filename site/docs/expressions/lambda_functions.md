# Lambda Expressions

Lambda expressions enable higher-order functions that operate on collections, allowing for flexible and composable data transformations. A lambda is an anonymous function with explicit parameter types and a body expression that can reference those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions. They enable powerful functional programming patterns such as `transform`, `filter`, and `reduce` operations on arrays.

!!! note "Documentation Syntax"
    This documentation uses the syntax `lambda(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. This is not an official Substrait syntax.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameter Types    | List of types for the lambda's parameters                                  | `parameter_types`     | Yes      |
| Return Type        | Type of the value returned by the lambda                                   | `return_type`         | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaReference) | `body`                | Yes      |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
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

## Parameter References

Lambda parameters are referenced within the lambda body using `LambdaParameterReference` in field references. This mechanism is similar to how [`OuterReference`](field_references.md#outerreference) works for subqueries.

### LambdaParameterReference Fields

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `lambda_depth`    | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |
| `parameter_index` | Zero-based index into the lambda's parameter list                   | 0, 1, 2... |

### Simple Example

For a lambda `lambda(x: i32) -> x * 2`:

Example of an Expression.Lambda message:
```protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
```

## Nested Lambdas

Lambdas can be nested, and inner lambdas can reference parameters from outer lambdas using `lambda_depth`.

### Example: Nested Array Transform

Transform a 2D array by incrementing each element:

```
transform(matrix, lambda(row: list<i32>) ->
  transform(row, lambda(cell: i32) ->
    cell + 1
  )
)
```

In this example:
- The outer lambda parameter `row` is referenced with `lambda_depth: 0` (within the outer lambda's body)
- The inner lambda parameter `cell` is referenced with `lambda_depth: 0` (within the inner lambda's body)
- Each lambda uses `lambda_depth: 0` to reference its own parameters

Example of an Expression.Lambda message:
```protobuf
--8<-- "examples/proto-textformat/lambdas/nested_array_transform.textproto"
```

## Closures

Lambda expressions naturally support closures - they can reference variables from the enclosing scope beyond their own parameters. There are three ways lambdas can capture external values:

### Capturing from Outer Lambda Parameters

Inner lambdas can access outer lambda parameters by using `lambda_depth` greater than 0 to traverse outward through lambda boundaries. For example, an inner lambda processing array elements could reference the outer lambda's parameter to access surrounding context.

This is demonstrated when `lambda_depth: 1` is used within an inner lambda to reference the first parameter of the immediately enclosing outer lambda.

### Capturing from Input Record

A lambda can reference fields from the input record using [`RootReference`](field_references.md#rootreference).

Consider an input record with this schema:

|             | id  | name   | numbers      | offset |
|-------------|-----|--------|--------------|--------|
| Field Index | 0   | 1      | 2            | 3      |
| Type        | i32 | string | list<i32\>  | i32    |

The lambda can access the `offset` field (index 3) from the input record:

```
transform(numbers, lambda(x: i32) ->
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

Lambdas are primarily used with higher-order functions that operate on collections. Common functions include:

- `transform` - Transform each element of an array
- `list_filter` - Filter elements based on a predicate
- `reduce` - Reduce an array to a single value
- `sort` - Sort with a custom comparator
- `zip_with` - Combine arrays element-wise

See the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) for the complete list of lambda-accepting functions and their signatures.

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
