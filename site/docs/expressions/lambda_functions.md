# Lambda Expressions

Lambda expressions enable higher-order functions that operate on collections, allowing for flexible and composable data transformations. A lambda is an anonymous function with explicit parameter types and a body expression that can reference those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions. They enable powerful functional programming patterns such as `map`, `filter`, and `reduce` operations on arrays.

!!! note "Documentation Syntax"
    This documentation uses the syntax `lambda(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. This is not an official Substrait syntax. Actual lambda expressions in Substrait are represented using the protobuf `Expression.Lambda` message format, as shown in the examples throughout this document.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Required |
|--------------------|-----------------------------------------------------------------------------|----------|
| Parameter Types    | List of types for the lambda's parameters                                  | Yes      |
| Return Type        | Type of the value returned by the lambda                                   | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaReference) | Yes      |

## Lambda Type Syntax

In YAML function signatures (such as in `functions_list.yaml`), lambda types are expressed using the syntax:

```
lambda<paramType -> returnType>
```

For single-parameter lambdas:
```yaml
lambda<i32 -> i32>           # Takes i32, returns i32
lambda<string -> boolean>    # Takes string, returns boolean
lambda<list<T> -> T>         # Takes list of T, returns T
```

For multi-parameter lambdas, use a struct to represent the parameter tuple:
```yaml
lambda<struct<T, T> -> i32>        # Takes two T values, returns i32
lambda<struct<U, T> -> U>          # Takes U and T, returns U
lambda<struct<i32, string> -> T>   # Takes i32 and string, returns T
```

See [Type Syntax Parsing](../types/type_parsing.md) for more details on type syntax.

## Parameter References

Lambda parameters are referenced within the lambda body using `LambdaReference` in field references. This mechanism is similar to how [`OuterReference`](field_references.md#outerreference) works for subqueries.

### LambdaReference Fields

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `lambda_depth`    | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |
| `parameter_index` | Zero-based index into the lambda's parameter list                   | 0, 1, 2... |

### Simple Example

For a lambda `lambda(x: i32) -> x * 2`:

```protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
```

## Nested Lambdas

Lambdas can be nested, and inner lambdas can reference parameters from outer lambdas using `lambda_depth`.

### Example: Nested Array Transform

Transform a 2D array by incrementing each element:

```
array_transform(matrix, lambda(row: list<i32>) ->
  array_transform(row, lambda(cell: i32) ->
    cell + 1
  )
)
```

In this example:
- The outer lambda parameter `row` is referenced with `lambda_depth: 0` (within the outer lambda's body)
- The inner lambda parameter `cell` is referenced with `lambda_depth: 0` (within the inner lambda's body)
- Each lambda uses `lambda_depth: 0` to reference its own parameters

In protobuf representation:
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
array_transform(numbers, lambda(x: i32) ->
  x + offset  // 'offset' is field 3 from the input record
)
```

In protobuf representation:
```protobuf
--8<-- "examples/proto-textformat/lambdas/closure_root_reference.textproto"
```

### Capturing from Outer Queries

In correlated subquery contexts, lambdas can also reference outer query records using [`OuterReference`](field_references.md#outerreference), similar to how regular expressions in subqueries can reference outer query fields.

## Higher-Order Functions

Lambdas are primarily used with higher-order functions that operate on collections. Common functions include:

- `array_transform` - Transform each element of an array
- `list_filter` - Filter elements based on a predicate
- `reduce` - Reduce an array to a single value
- `array_sort` - Sort with a custom comparator
- `zip_with` - Combine arrays element-wise

See the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) for the complete list of lambda-accepting functions and their signatures.

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
