# Lambda Expressions

Lambda expressions represent inline, anonymous functions within query plans, enabling operations that require nested computation over data. They are primarily used to express array operations (such as `map` or `filter`) and support higher-order functions that apply computations element-wise over collections. A lambda consists of explicit parameter types and a body expression that references those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions or [invoked directly](#lambda-invocation).

!!! note "Documentation Syntax"
    This documentation uses the syntax `(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. There is no formal syntax specified in the substrait spec for compactly representing lambdas.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameter Types    | List of types for the lambda's parameters. The length of this list defines the lambda's arity (number of parameters). | `parameter_types`     | Yes      |
| Return Type        | Type of the value returned by the lambda                                   | `return_type`         | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaParameterReference) | `body`                | Yes      |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
    ```

## Parameter References

Lambda parameters are referenced within the lambda body using `LambdaParameterReference` expressions.

### LambdaParameterReference Fields

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `lambda_depth`    | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |
| `reference`       | Zero-based index into the lambda's parameter list                   | 0, 1, 2... |

=== "LambdaParameterReference Message"
    ```proto
%%% proto.message.Expression.LambdaParameterReference %%%
    ```

### Simple Example

For a lambda `(x: i32) -> x * 2`:

```protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
```

### Accessing Fields within Parameters

For scalar parameters, use `LambdaParameterReference` directly (as shown above). For struct or complex type parameters, wrap the `LambdaParameterReference` in a [`FieldReference`](field_references.md) using the `expression` root_type to drill into specific fields:

```protobuf
--8<-- "examples/proto-textformat/field_references/lambda_param_struct_field.textproto"
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

### Example: The `map` Function

The `map` function transforms each element of a list using a lambda. Here's how it's defined in the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml):

```yaml
--8<-- "examples/extensions/lambda_function_example.yaml"
```

The `func<T -> U>` type indicates the lambda accepts one parameter of type `T` and returns type `U`, allowing the list element type to be transformed.

## Closures

Lambda bodies can reference data from outside their parameter list, enabling closures. This is accomplished through normal expression mechanisms:

**Outer Lambda Parameters**: In nested lambdas, use `lambda_depth > 0` in `LambdaParameterReference` to reference an enclosing lambda's parameters (1 = immediate parent, 2 = grandparent, etc.).

**Input Record**: Use [`FieldReference`](field_references.md) with `RootReference` to capture fields from the input record being processed:

**Outer Queries**: Use [`FieldReference`](field_references.md) with `OuterReference` to reference outer query records in correlated subquery contexts.

## Higher-Order Functions

Lambdas are primarily used with higher-order functions that operate on collections. Current functions include:

| Function | Description |
|----------|-------------|
| `map` | Apply an expression to each element of an array |
| `filter` | Select elements matching a predicate |

See the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) for the complete list of lambda-accepting functions and their signatures.

## Lambda Invocation

Lambda expressions can be invoked directly using the `LambdaInvocation` expression type, allowing a lambda to be defined and called in a single expression.

A lambda invocation consists of:

| Component  | Description                                                                 | Protobuf Field | Required |
|------------|-----------------------------------------------------------------------------|----------------|----------|
| Lambda     | The inline lambda expression to invoke                                      | `lambda`       | Yes      |
| Arguments  | The values to pass as parameters to the lambda                              | `arguments`    | Yes      |

The number of arguments must match the lambda's `parameter_types` count exactly, and each argument's type must match the corresponding parameter type. The return type is derived from the lambda's `return_type` field - no separate output type declaration is needed.

=== "LambdaInvocation Message"
    ```proto
%%% proto.message.Expression.LambdaInvocation %%%
    ```

### Example

Invoking `((x: i32) -> x * 2)(5)` to compute 10:

```protobuf
--8<-- "examples/proto-textformat/lambda_invocations/inline_invocation.textproto"
```

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
