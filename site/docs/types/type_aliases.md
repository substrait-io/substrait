# Type Aliases

In a Substrait plan, types are spelled out whenever and wherever they are needed. For parameterized types, all type parameters are spelled out per type reference. For a parameterized type with a large number of parameters, complex nested type parameters, or string parameters, this can significantly bloat the size of the plan proportional to the number of such type references in the plan as such type parameters are repeatedly spelled out.

To alleviate the problem, Substrait offers type alias mechanism.

Type alias allows a plan can declare a type once and reference it multiple times within a plan like extensions; you can reference an aliased type where a type is expected.

## Type Alias

A type alias is simply a mapping from anchor to a concrete Substrait type. A valid type alias is described below.

* All type parameters must be spelled out.
* Can not alias another alias.
* Type parameters can reference other aliased types as long as there are no circular dependencies among involved type aliases.
* Nullability of aliased type is **ignored**. Nullability must be specified when the aliased type is referenced.
* Type variation may be specified in the aliased type.

## Type Alias Reference

Type alias reference is a Substrait type thus can appear wherever a Substrait type is expected. The reference must specify nullability of the aliased type.

## Examples

```
type alias 1 --> VARCHAR<100> // OK to alias Substrait VARCHAR.
type alias 2 --> UserDefined<i8, type alias ref (2, NULLABLE)> // OK to reference other type alias 2. UserDefined<i8, VARCHAR<100>?>.
type alias 3 --> UserDefined<i8, type alias ref (2, REQUIRED)> // OK to reference other type alias 2. UserDefined<i8, VARCHAR<100>>.

type alias 4 --> type alias ref (1, NULLABLE)  // NOT OK to alias another alias.
type alias 5 --> STRUCT<i8, type alias ref (0, NULLABLE)> // NOT OK to reference an undefined type alias 0.
type alias 6 --> STRUCT<type alias ref (6, REQUIRED) // NOT OK to reference itself.
type alias 7 --> STRUCT<i8, type alias ref (8, REQUIRED)>
type alias 8 --> STRUCT<type alias ref (7, REQUIRED)> // NOT OK because type alias 7 and 8 have a circular dependency.
```