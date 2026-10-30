# Lambda Expressions

Lambdas define functions using typed parameters and an expression body. They can appear inline or be defined once at plan scope.

Lambdas can be [invoked directly](#lambda-invocation) or passed to higher-order functions such as `transform` and `filter`. A [named lambda reference](#named-lambda-references) supplies a function value without repeating its definition.

!!! note "Documentation Syntax"
    The notation `(param: type, ...) -> expression` is illustrative, not a formal Substrait syntax.

## Lambda Expression Structure

| Component          | Description                                                                 | Protobuf Field        | Required |
|--------------------|-----------------------------------------------------------------------------|-----------------------|----------|
| Parameters         | Struct with one field per parameter. Its nullability must be `NULLABILITY_REQUIRED`. | `parameters` | Yes |
| Body Expression    | Expression evaluated when the lambda is invoked. | `body` | Yes |
| Declared Return Type | Explicit return type; see [Type Derivation](#type-derivation). | `output_type` | For named lambdas in a [dependency cycle](#recursive-named-lambdas) |

=== "Lambda Message"
    ```proto
%%% proto.message.Expression.Lambda %%%
    ```

### Type Derivation

A lambda has a non-nullable `func` type with the lambda's parameter types. Its return type is:

- The declared `output_type`, when present. The body must match it exactly, including nullability and type variations. This is a type assertion, not an implicit cast.
- The body expression's type, otherwise.

`output_type` is optional for inline lambdas and acyclic named lambdas. Every named lambda in a [definition dependency cycle](#recursive-named-lambdas) must declare it. Missing required declarations and mismatched body types are invalid.

## Parameter References

Lambda parameters are accessed using a [`FieldReference`](field_references.md) with `LambdaParameterReference` as its root type and a `StructField` selecting the parameter's zero-based position.

### LambdaParameterReference Fields

`LambdaParameterReference` is a nested message within `FieldReference` that identifies which lambda scope to reference:

| Field             | Description                                                          | Values   |
|-------------------|----------------------------------------------------------------------|----------|
| `steps_out`       | Number of lexical lambda boundaries to traverse (0 = current lambda) | 0, 1, 2... |

=== "LambdaParameterReference Message"
    ```proto
%%% proto.message.Expression.FieldReference.LambdaParameterReference %%%
    ```

Use `direct_reference` with `StructField` to select a parameter. `steps_out` follows lexical nesting, not the recursive call stack. Subqueries and detachment do not add lambda boundaries.

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

Extension YAML signatures use `func` with generic type parameters. This notation is distinct from plan expressions, which represent function values using `Expression.Lambda` or `Expression.NamedLambdaReference`.

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

Inline lambdas can capture input records through `RootReference`, outer query records through `OuterReference`, and parameters of enclosing lambdas. [Named lambdas](#scope) cannot capture the caller's scope.

### Outer Lambda Parameters

`steps_out = 0` refers to the current lambda's parameters. To reference an enclosing lambda (i.e., a lambda further out in scope), use `steps_out > 0` (1 = immediate parent, 2 = grandparent, etc.). Combine this with `StructField` to access specific parameters from that scope.

```protobuf
--8<-- "examples/proto-textformat/lambda/nested_lambda_capture.textproto"
```

## Lambda Invocation

`LambdaInvocation` calls an inline or named lambda. Each invocation binds its own arguments.

| Component    | Description                                                                 | Protobuf Field            | Required |
|--------------|-----------------------------------------------------------------------------|---------------------------|----------|
| Lambda       | The inline lambda expression to invoke                                      | `lambda`       | One of `lambda` or `named_lambda` |
| Named lambda | A reference to a lambda in `Plan.named_lambdas`                             | `named_lambda` | One of `lambda` or `named_lambda` |
| Arguments    | A `Nested.Struct` of argument expressions, in parameter order | `arguments` | Yes |

The argument count and types must exactly match the lambda's parameters. The invocation returns the lambda's result, with the type specified by [Type Derivation](#type-derivation).

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

A named lambda is defined once in `Plan.named_lambdas` and can be invoked or passed as a function value any number of times.

Each `NamedLambda` contains a `lambda_anchor`, an `Expression.Lambda`, an optional diagnostic `name`, and optional [detachment tables](#detached-named-lambda-bodies). The name has no semantic meaning and cannot be used as a reference.

Named-lambda anchors form a separate namespace from extension function anchors. Each anchor must be unique within the plan, and every reference must resolve to an existing definition. Anchor 0 is valid. Definitions may reference later entries.

### Scope

A named lambda cannot capture the caller's relational input, outer records, or enclosing lambda parameters.

- Its body may use its parameters, dynamic parameters, execution context variables, and other named lambdas.
- [Subqueries](subqueries.md) may access their own relational input and enclosing relational scopes established within the same body. Neither form of `OuterReference` may reach into the caller's scope.
- Inline lambdas nested within the body may capture that invocation's parameters, following the [parameter-reference rules](#parameter-references).

A lambda returns an expression value, including a subquery result, not a relation directly.

### Named Lambda References

`Expression.NamedLambdaReference` supplies a function value, not the result of a call. Its `lambda_reference` identifies a `lambda_anchor` in `Plan.named_lambdas`, using the same namespace as `LambdaInvocation.named_lambda`.

Its function type follows [Type Derivation](#type-derivation) for the referenced lambda. It can be passed in `FunctionArgument.value` wherever a compatible function type is expected. The higher-order function supplies arguments when it invokes the lambda.

Dialects can enable named lambda references by including `NAMED_LAMBDA_REFERENCE` in `supported_expressions`. Direct calls to named lambdas require both `NAMED_LAMBDA_REFERENCE` and `LAMBDA_INVOCATION`; `LAMBDA_INVOCATION` alone supports only inline calls.

=== "NamedLambdaReference Message"
    ```proto
%%% proto.message.Expression.NamedLambdaReference %%%
    ```

The following examples contrast invocation with passing a function value:

=== "Direct invocation"

    Defines an identity lambda once and invokes it twice.

    ```protobuf
    --8<-- "examples/proto-textformat/named_lambda/reused_identity.textproto"
    ```

=== "Function-valued argument"

    Passes the identity lambda to `transform`, producing `[1, 2, 3]` for each input row. The extension function and named lambda both use anchor 1 in their separate namespaces.

    ```protobuf
    --8<-- "examples/proto-textformat/named_lambda/transform_identity.textproto"
    ```

### Recursive Named Lambdas

Named lambdas support self-recursion and mutual recursion. A definition depends on every named lambda its body invokes or references as a function value, including itself. This includes references in nested lambdas, subquery relation trees, higher-order function arguments, and unevaluated branches.

Determine dependencies after resolving [detached relations and expressions](#detached-named-lambda-bodies); unreachable detached entries do not contribute dependencies. Consumers apply the [type-derivation rules](#type-derivation) by establishing declared signatures before checking recursive bodies, without recursively expanding definitions. Recursion does not introduce recursive types.

Consumers need not prove termination, and the specification imposes no recursion-depth limit. Consumers must report an error if their resource limits are exceeded, rather than truncate the computation or substitute a result. Consumers that do not support recursive named lambdas must reject plans that require them.

This plan defines mutually recursive `is_even` and `is_odd` functions for non-negative `i32` arguments. Both declare non-nullable boolean results; `is_even(4)` returns true.

```protobuf
--8<-- "examples/proto-textformat/named_lambda/mutual_recursion.textproto"
```

### Detached Named Lambda Bodies

Each `NamedLambda` owns two optional detachment tables:

| Table | Subtrees | Reference |
|-------|----------|-----------|
| `detached_expressions` | Body expressions | `Expression.detached_expression_ordinal` |
| `detached_rels` | Relations within subqueries | `Rel.detached_rel_ordinal` |

Entries may reference further subtrees in either table. Apply the shared [encoding and validation rules](../serialization/binary_serialization.md#detached-relations-and-expressions), starting from `lambda.body`:

- Ordinals resolve within the definition, never against a caller's or another definition's tables.
- The single-reference rule applies per definition, not per invocation.
- Detachment changes storage, not lexical scope, field-reference binding, or computation sharing.

**Named-lambda recursion is not a detached-reference cycle.** Calls may be recursive, but detached ordinals must reconstruct finite relation and expression trees. Resolving a detached reference does not expand the bodies of named lambdas it invokes or references.

=== "Detached factorial"

    Detaches the body and recursive call of factorial. For non-negative `i32` arguments whose factorial fits in `i32`, `factorial(5)` produces 120.

    ```protobuf
    --8<-- "examples/proto-textformat/named_lambda/detached_factorial.textproto"
    ```

=== "Detached subquery"

    `lookup_exists(key)` checks a local table containing 42. Its detached filter condition compares local input with the lambda parameter, without capturing caller input. Calls with 42 and 7 return true and false. Caller and definition each use ordinal 0 in their own expression and relation tables.

    ```protobuf
    --8<-- "examples/proto-textformat/named_lambda/detached_rels.textproto"
    ```

## See Also

- [Field References](field_references.md) - How to reference data in expressions
- [Scalar Functions](scalar_functions.md) - General scalar function documentation
- [functions_list Extension](https://github.com/substrait-io/substrait/blob/main/extensions/functions_list.yaml) - Complete list of higher-order functions
