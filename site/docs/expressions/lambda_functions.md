# Lambda Expressions

Lambda expressions represent inline, anonymous functions within query plans, enabling operations that require nested computation over data. They are primarily used to express array operations (such as `transform` or `filter`) and support higher-order functions that apply computations element-wise over collections. A lambda consists of explicit parameter types and a body expression that references those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions or [invoked directly](#lambda-invocation).

!!! note "Documentation Syntax"
    This documentation uses the syntax `(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. There is no formal syntax specified in the substrait spec for compactly representing lambdas.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameters         | Struct type defining the lambda's parameters. Each field in the struct represents a parameter that can be accessed via FieldReference. The struct's nullability must be `NULLABILITY_REQUIRED`. | `parameters`     | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaParameterReference). The type of this expression is the return type of the lambda. | `body`                | Yes      |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
    ```

### Type Derivation

The return type of a lambda is derived from its body expression. Since all expressions in Substrait have deterministic types, the lambda's return type can be computed by determining the type of the body expression.

!!! note "Future Work"
    A detailed type derivation algorithm should be specified.

## Parameter References

Lambda parameters are referenced within the lambda body using [`FieldReference`](field_references.md) with `LambdaParameterReference` as the root type. Lambda parameters are conceptually treated as a struct, where each parameter occupies a position that can be accessed via `StructField` references.

### LambdaParameterReference Fields

`LambdaParameterReference` is a nested message within `FieldReference` that identifies which lambda scope to reference:

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `lambda_depth`    | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |

=== "LambdaParameterReference Message"
    ```proto
%%% proto.message.Expression.FieldReference.LambdaParameterReference %%%
    ```

To access a specific parameter, wrap `LambdaParameterReference` in a [`FieldReference`](field_references.md) and use `direct_reference` with `StructField` to specify which parameter (field 0 = first parameter, field 1 = second parameter, etc.).

### Simple Example

For a lambda `(x: i32) -> x * 2`:

```protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
```

### Accessing Fields within Parameters

Because lambda parameters are accessed using [`FieldReference`](field_references.md), all field navigation mechanisms are available for drilling into complex objects. For example, when a lambda parameter is a struct, you can access deeply nested fields like `person.address.city`:

```protobuf
--8<-- "examples/proto-textformat/field_references/lambda_param_nested_struct.textproto"
```

## Function Type Syntax

In YAML extension definitions, function types are specified using the `func` keyword with generic type parameters:

This notation applies to extension YAML signatures; in plans, lambdas are always represented as `Expression.Lambda` with `parameters` (a struct type) and `body`.

**Single parameter** (represents a lambda with 1 field in the `parameters` struct):
```yaml
func<any1 -> any2>           # Single parameter without parentheses
func<(any1) -> any2>         # Single parameter with parentheses (equivalent)
```

**Multiple parameters** (represents a lambda with 2+ fields in the `parameters` struct):
```yaml
func<(any1, any2) -> any3>      # Multiple parameters (parentheses required)
func<(any1, any2, any3) -> any4>   # Three parameters
```

### Nullability

The `Func` type has its own nullability field, which applies to the function value itself—not its return type. A nullable function type (`func<i32 -> i32>?`) means the function reference may be null, whereas a function with a nullable return type (`func<i32 -> i32?>`) always exists but may return null.

### Example: The `transform` Function

The `transform` function transforms each element of a list using a lambda. Here's how it's defined in the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml):

```yaml
--8<-- "examples/extensions/lambda_function_example.yaml"
```

The `func<any1 -> any2>` type indicates the lambda accepts one parameter of type `any1` and returns type `any2`. Using numbered [`any` types](../extensions/index.md#any-types) ensures repeated labels within a signature must resolve to the same concrete type.

## Closures

Lambda bodies can reference data from outside their parameter list, enabling closures. This is accomplished through normal expression mechanisms:

### Outer Lambda Parameters

In nested lambdas, use `lambda_depth > 0` in `LambdaParameterReference` to reference an enclosing lambda's parameter struct (1 = immediate parent, 2 = grandparent, etc.). Combine this with `StructField` to access specific parameters from that scope.

```protobuf
--8<-- "examples/proto-textformat/lambdas/nested_lambda_capture.textproto"
```

In this example:
- `lambda_depth: 1` with `struct_field: {field: 0}` accesses the first parameter of the outer lambda (outer_x)
- `lambda_depth: 0` with `struct_field: {field: 0}` accesses the first parameter of the inner lambda (inner_y)

### Input Record References

Use [`FieldReference`](field_references.md) with `RootReference` to capture fields from the input record being processed.

### Outer Query References

Use [`FieldReference`](field_references.md) with `OuterReference` to reference outer query records in correlated subquery contexts.

## Lambda Invocation

Lambda expressions can be invoked directly using the `LambdaInvocation` expression type, allowing a lambda to be defined and called in a single expression.

A lambda invocation consists of:

| Component  | Description                                                                 | Protobuf Field | Required |
|------------|-----------------------------------------------------------------------------|----------------|----------|
| Lambda     | The inline lambda expression to invoke                                      | `lambda`       | Yes      |
| Arguments  | A `Nested.Struct` containing expressions for each lambda parameter. Each field corresponds to a lambda parameter and must evaluate to the matching parameter type. | `arguments`    | Yes      |

The `arguments` field must be a `Nested.Struct` with exactly as many fields as the lambda has parameters. The type of each expression field must match the corresponding parameter type. The return type is derived from the type of the lambda's body expression.

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
