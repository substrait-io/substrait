# Type System

Substrait tries to cover the most common types used in data manipulation. Types beyond this common core may be represented using [simple extensions](../extensions/index.md#simple-extensions).

Substrait types fundamentally consist of four components:

| Component                       | Condition           | Examples                                                          | Description
| ------------------------------- | ------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------
| [Class](type_classes.md)        | Always              | `i8`, `string`, `STRUCT`, extensions                              | Together with the parameter pack, describes the set of non-null values supported by the type. Subdivided into simple and compound type classes.
| Nullability                     | Always              | Either `NULLABLE` (`?` suffix) or `REQUIRED` (no suffix)          | Describes whether values of this type can be null. Note that a null of a known type is considered to be a special value of that nullable type; the dedicated [`null`](type_classes.md#null-type) type class describes only nulls that carry no type information of their own, and is itself inherently nullable.
| [Variation](type_variations.md) | Always              | No suffix or explicitly `[0]` (system-preferred), or an extension | Allows different variations of the same type class to exist in a system at a time, usually distinguished by in-memory format.
| Parameters                      | Compound types only | `<10, 2>` (for `DECIMAL`), `<i32, string>` (for `STRUCT`)         | Some combination of zero or more data types or integers. The expected set of parameters and the significance of each parameter depends on the type class.

Refer to [Type Parsing](type_parsing.md) for a description of the syntax used to describe types.

!!! note "Note"
    Substrait employs a strict type system without any coercion rules. All changes in types must be made explicit via [cast expressions](../expressions/specialized_record_expressions.md).

    Partially bound plans and expressions may use the [`unbound`](type_classes.md#unbound-type) type as a placeholder until a downstream binder assigns a concrete type.

    A null that carries no type information of its own — for example a bare `NULL` literal, or an empty list or map originating from a system that models these with a null or unknown type — has the [`null`](type_classes.md#null-type) type. The strict type system applies to it unchanged: the `null` type does not coerce, so an explicit cast is required to use such a value where another type is expected.
