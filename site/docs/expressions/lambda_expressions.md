# Lambda Expressions

Lambda expressions represent inline, anonymous functions within query plans, enabling operations that require nested computation over data. They are primarily used to express array operations (such as `transform` or `filter`) and support higher-order functions that apply computations element-wise over collections. A lambda consists of explicit parameter types and a body expression that references those parameters.

## Overview

Lambda expressions are a type of expression in Substrait (like `IfThen`, `Subquery`, or `Nested` expressions) that can be passed as arguments to higher-order functions or [invoked directly](#lambda-invocation). A [named lambda reference](#named-lambda-references) can also be passed as a function-valued argument without repeating its definition.

!!! note "Documentation Syntax"
    This documentation uses the syntax `(param: type, ...) -> expression` as an illustrative notation to explain lambda concepts in a readable form. There is no formal syntax specified in the Substrait spec for compactly representing lambdas.

## Lambda Expression Structure

A lambda expression consists of:

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameters         | Struct type defining the lambda's parameters. Each field in the struct represents a parameter that can be accessed via LambdaParameterReference (a type of FieldReference). The struct's nullability must be `NULLABILITY_REQUIRED`. | `parameters`     | Yes      |
| Body Expression    | The expression to evaluate (can reference parameters via LambdaParameterReference). The type of this expression is the return type of the lambda. | `body`                | Yes      |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
    ```

### Type Derivation

The type of a lambda expression is a `func` type. The parameters of the `func` are the parameters of the lambda. The return type of the `func` is determined by the type of the expression comprising the body of the lambda. 

## Parameter References

Lambda parameters are referenced within the lambda body using [`FieldReference`](field_references.md)s with `LambdaParameterReference` as the root type. Lambda parameters are conceptually treated as a struct, where each parameter occupies a position that can be accessed via `StructField` references.

### LambdaParameterReference Fields

`LambdaParameterReference` is a nested message within `FieldReference` that identifies which lambda scope to reference:

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `steps_out`       | Number of lambda boundaries to traverse (0 = current lambda)        | 0, 1, 2... |

=== "LambdaParameterReference Message"
    ```proto
%%% proto.message.Expression.FieldReference.LambdaParameterReference %%%
    ```

To access a specific parameter, wrap `LambdaParameterReference` in a [`FieldReference`](field_references.md) and use `direct_reference` with `StructField` to specify which parameter (field 0 = first parameter, field 1 = second parameter, etc.).

### Simple Example

```protobuf
--8<-- "examples/proto-textformat/lambda/simple_multiply.textproto"
```

### Accessing Fields within Parameters

Because lambda parameters are accessed using [`FieldReference`](field_references.md), all field navigation mechanisms are available for drilling into complex objects. For example, when a lambda parameter is a struct, you can access deeply nested fields like `person.address.city`:

```protobuf
--8<-- "examples/proto-textformat/field_reference/lambda_param_nested_struct.textproto"
```

## Function Type Syntax

In YAML extension definitions, function types are specified using the `func` keyword with generic type parameters:

This notation applies to extension YAML signatures. In plans, inline lambdas are represented as `Expression.Lambda` with `parameters` (a struct type) and `body`. A function value can also be represented by an `Expression.NamedLambdaReference`, whose type is derived from the referenced definition.

**Single parameter** (represents a lambda with 1 field in the `parameters` struct):
```yaml
func<any1 -> any2>                 # Single parameter without parentheses
func<(any1) -> any2>               # Single parameter with parentheses (equivalent)
```

**Multiple parameters** (represents a lambda with 2+ fields in the `parameters` struct):
```yaml
func<(any1, any2) -> any3>         # Multiple parameters (parentheses required)
func<(any1, any2, any3) -> any4>   # Three parameters
```

### Nullability

The `Func` type has its own nullability field, which applies to the function value itself — not its return type. A nullable function type (`func?<i32 -> i32>`) means the function reference may be null, whereas a non-nullable function with a nullable return type (`func<i32 -> i32?>`) always exists but may return null.

### Example: The `transform` Function

The `transform` function transforms each element of a list using a lambda. It is declared in the [functions_list extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml); the declaration looks like this:

```yaml
--8<-- "examples/extensions/lambda_function_example.yaml"
```

The `func<any1 -> any2>` type indicates the lambda accepts one parameter of type `any1` and returns type `any2`. Using numbered [`any` types](../extensions/index.md#any-types) ensures repeated labels within a signature must resolve to the same concrete type.

## Closures

Lambda bodies can reference data from outside their parameter list through [`FieldReference`](field_references.md)s. References to input records (via `RootReference`) and outer query records (via `OuterReference`) work as they do elsewhere in Substrait. There is also a way to capture lambda parameters from outer lambdas.

### Outer Lambda Parameters

`steps_out = 0` refers to the current lambda's parameters. To reference an enclosing lambda (i.e., a lambda further out in scope), use `steps_out > 0` (1 = immediate parent, 2 = grandparent, etc.). Combine this with `StructField` to access specific parameters from that scope.

```protobuf
--8<-- "examples/proto-textformat/lambda/nested_lambda_capture.textproto"
```

## Lambda Invocation

Lambda expressions can be invoked using the `LambdaInvocation` expression type. An invocation can call either an inline lambda or a named lambda defined at the plan level.

A lambda invocation consists of:

| Component    | Description                                                                 | Protobuf Field            | Required |
|--------------|-----------------------------------------------------------------------------|---------------------------|----------|
| Lambda       | The inline lambda expression to invoke                                      | `lambda`       | One of `lambda` or `named_lambda` |
| Named lambda | A reference to a lambda in `Plan.named_lambdas`                             | `named_lambda` | One of `lambda` or `named_lambda` |
| Arguments    | A `Nested.Struct` containing expressions for each lambda parameter. Each field corresponds to a lambda parameter and must evaluate to the matching parameter type. | `arguments` | Yes |

The `arguments` field must be a `Nested.Struct` with exactly as many fields as the lambda has parameters. The type of each expression field must match the corresponding parameter type. The return type is derived from the type of the lambda's body expression.

=== "LambdaInvocation Message"
    ```proto
%%% proto.message.Expression.LambdaInvocation %%%
    ```

### Example

Invoking `((x: i32) -> x * 2)(5)` to compute 10:

```protobuf
--8<-- "examples/proto-textformat/lambda_invocation/inline_invocation.textproto"
```

## Named Lambdas

A named lambda is defined once in `Plan.named_lambdas`, where each entry is represented by a `NamedLambda`. Once defined, it can be invoked from multiple `LambdaInvocation` expressions and passed as a function value using `NamedLambdaReference`. Named lambdas use their own anchor namespace, separate from extension function anchors.

Each definition consists of a `lambda_anchor`, an `Expression.Lambda`, and an optional human-readable `name`. The name is only for diagnostics and plan inspection: it has no semantic meaning and cannot be used as a reference. An invocation supplies a `named_lambda` reference and a `Nested.Struct` of arguments. The argument count and types must exactly match the lambda parameters, and the invocation's return type is the type of the lambda body.

Named lambdas are closed over their parameters. Their bodies cannot reference relational input, outer records, or parameters from a caller's enclosing lambda. They may use dynamic parameters, execution context variables, and other named lambdas.

Named-lambda references may reference definitions that appear later in `Plan.named_lambdas`, but the definition dependency graph must be acyclic. A definition depends on another whenever its body invokes that named lambda or reference it as a function value, including inside nested expressions and arguments to higher-order functions. Self-references and indirect cycles are invalid, even if the reference is only passed as an argument or occurs in a branch that is not evaluated.

Within a plan, every named-lambda anchor must be unique and every named-lambda invocation or reference must target an existing definition. A named lambda may be referenced or invoked any number of times.

```protobuf
--8<-- "examples/proto-textformat/named_lambda/reused_identity.textproto"
```

### Named Lambda References

`Expression.NamedLambdaReference` denotes a function value without invoking the lambda. Its `lambda_reference` identifies a `lambda_anchor` in `Plan.named_lambdas`, using the same namespace as `LambdaInvocation.named_lambda`. Anchor 0 is valid.

A named lambda reference has a non-nullable function type, with parameter and return types derived from the referenced lambda as described in [Lambda Type Derivation](#type-derivation).

Unlike a `LambdaInvocation`, which produces the result of a call, a `NamedLambdaReference` supplies the function itself. It can be placed in `FunctionArgument.value` wherever a compatible function type is expected, such as the transformer argument of `transform` or the predicate argument of `filter`. The higher-order function supplies the arguments when it invokes the referenced lambda.

=== "NamedLambdaReference Message"
    ```proto
%%% proto.message.Expression.NamedLambdaReference %%%
    ```

The following plan passes a named identity lambda to `transform`. For each input row, the projected expression produces `[1, 2, 3]`. Both the extension function and the named lambda use anchor 1, demonstrating their separate namespaces.

```protobuf
--8<-- "examples/proto-textformat/named_lambda/transform_identity.textproto"
```

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
