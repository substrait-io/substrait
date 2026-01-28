# Type System

Substrait tries to cover the most common types used in data manipulation. Types beyond this common core may be represented using [simple extensions](../extensions/index.md#simple-extensions).

Substrait types fundamentally consist of four components:

| Component                       | Condition           | Examples                                                          | Description
| ------------------------------- | ------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------
| [Class](type_classes.md)        | Always              | `i8`, `string`, `STRUCT`, extensions                              | Together with the parameter pack, describes the set of non-null values supported by the type. Subdivided into simple and compound type classes.
| Nullability                     | Always              | Either `NULLABLE` (`?` suffix) or `REQUIRED` (no suffix)          | Describes whether values of this type can be null. Note that null is considered to be a special value of a nullable type, rather than the only value of a special null type.
| [Variation](type_variations.md) | Always              | No suffix or explicitly `[0]` (system-preferred), or an extension | Allows different variations of the same type class to exist in a system at a time, usually distinguished by in-memory format.
| Parameters                      | Compound types only | `<10, 2>` (for `DECIMAL`), `<i32, string>` (for `STRUCT`)         | Some combination of zero or more data types or integers. The expected set of parameters and the significance of each parameter depends on the type class.

Refer to [Type Parsing](type_parsing.md) for a description of the syntax used to describe types.

!!! note "Note"
    Substrait employs a strict type system without any coercion rules. All changes in types must be made explicit via [cast expressions](../expressions/specialized_record_expressions.md).

## Type Equality

Two types are considered equal if and only if all four components match: type class, nullability, variation, and parameters (compared recursively for compound types).

For example, `i32` and `i32?` are different types because they differ in nullability. Similarly, `list<i32>` and `list<i32?>` are different types because their parameters differ.

!!! note
    Some contexts explicitly ignore nullability when comparing types:

    - Functions with [MIRROR nullability](../expressions/scalar_functions.md#mirror-default) ignore outermost nullability when binding `any[\d]` type parameters
    - Functions with [DECLARED_OUTPUT nullability](../expressions/scalar_functions.md#declared_output) accept any mix of nullability in inputs; output nullability is determined solely by return type
    - [Variadic type parameters](../expressions/scalar_functions.md#type-parameter-resolution-in-variadic-functions) can be marked as consistent or inconsistent for type binding across variadic arguments
