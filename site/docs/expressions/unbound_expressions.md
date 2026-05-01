# Unbound Expressions

Substrait normally represents bound relational expressions: field references are positional, value types are known, and function invocations identify implementations whose argument and return types have been derived. Some producers need to serialize expressions earlier in planning, before names and types have been resolved.

An expression tree is partially bound when it contains either an [`unknown`](../types/type_classes.md#unknown-type) type or a `NamedExpression`. Consumers may validate and transform partially bound expressions, but must resolve them before execution unless they define their own semantics for unresolved names or unknown-typed values.

## Detecting Partial Binding

There is no separate "unbound expression" message. Instead, partially bound state is detected structurally:

- If any expression, function argument, or schema field type is `unknown`, the expression is partially bound.
- If any expression contains `NamedExpression`, the expression is partially bound.

This is the canonical way to distinguish fully bound expressions from partially bound expressions in Substrait.

## Unknown Type

The `unknown` type marks an expression whose concrete type is not known yet. It may be used anywhere a concrete type would normally be expected in a partially bound function call. If only the nullability is known, set the nullability field; otherwise leave it unspecified.

## Named Expression

`NamedExpression` represents a reference by name instead of ordinal position. The `names` field stores one or more namespace components, such as `["foo"]` for an unqualified name or `["orders", "amount"]` for a qualified name. Until resolved, a named expression's type is `unknown`. Resolution of these components is intentionally external to Substrait and must be understood by both producer and consumer.

```protobuf
--8<-- "examples/proto-textformat/unbound_expression/named_expression.textproto"
```

## Function Example

Partially bound function calls can use named expressions as value arguments and `unknown` as the output type when the return type cannot be derived yet.

```protobuf
--8<-- "examples/proto-textformat/unbound_expression/scalar_function_unknown.textproto"
```

## Extended Expression Protocols

Expression-level APIs, such as filters and projections exchanged outside a full `Plan`, should use `ExtendedExpression`. This lets the producer include output names, any known input names, and extension declarations next to the expression tree.

If the input names are known but their types are not, `base_schema` can contain fields with `unknown` types. If the function overload is also unresolved, the function can refer to the `extension:io.substrait:unknown` extension until a downstream binder replaces it with a concrete function reference and concrete output type. In that case, the referenced function name should use the normal Substrait function-signature form with `unknown` short names, such as `add:unknown_unknown`.

```protobuf
--8<-- "examples/proto-textformat/extended_expression/unbound_named_projection.textproto"
```

Consumers that execute expressions must reject or bind away all `NamedExpression` and `unknown` types before execution unless they explicitly support unresolved semantics. A typical binder resolves `NamedExpression` values to `FieldReference`, replaces `unknown` input and output types with concrete types, and updates unresolved function references to concrete overloads.
