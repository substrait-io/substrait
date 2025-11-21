# Type Aliases

In a Substrait plan, types are spelled out whenever and wherever they are needed. For parameterized types, all type parameters are spelled out per type reference. For a parameterized type with a large number of parameters, complex nested type parameters, or string parameters, this can significantly bloat the size of the plan proportional to the number of such type references in the plan as such type parameters are repeatedly spelled out.

To alleviate the problem, Substrait offers a type alias mechanism.

Type aliases allow a plan to declare a type once and reference it multiple times within a plan. A type alias can be used wherever a type is expected. Type aliases are scoped to a single plan and are defined in the `Plan.type_aliases` field.

## Type Alias

A type alias is a mapping from an anchor to a concrete Substrait type. A valid type alias is described below.

* All type parameters must be specified.
* Cannot directly be another alias.
* Type parameters can reference other aliased types as long as no circular dependencies are introduced.
* Nullability of aliased type is **ignored**. Nullability must be specified when the aliased type is referenced.
* Type variation may be specified in the aliased type.

## Type Alias Reference

A type alias reference is a Substrait type and can appear wherever a Substrait type is expected. The reference must specify the nullability of the aliased type.

## Examples

```
type alias 1 --> VARCHAR<100> // OK to alias Substrait VARCHAR.
type alias 2 --> UserDefined<i8, type alias ref (1, NULLABLE)> // OK to reference other type alias 1. UserDefined<i8, VARCHAR<100>?>.
type alias 3 --> UserDefined<i8, type alias ref (2, REQUIRED)> // OK to reference other type alias 1. UserDefined<i8, VARCHAR<100>>.

type alias 4 --> type alias ref (1, NULLABLE)  // NOT OK to alias another alias directly.
type alias 5 --> STRUCT<i8, type alias ref (0, NULLABLE)> // NOT OK to reference an undefined type alias 0.
type alias 6 --> STRUCT<type alias ref (6, REQUIRED) // NOT OK to reference itself.
type alias 7 --> STRUCT<i8, type alias ref (8, REQUIRED)>
type alias 8 --> STRUCT<type alias ref (7, REQUIRED)> // NOT OK because type alias 7 and 8 have a circular dependency.
```

## Using Type Aliases in Plans

Type aliases are defined at the plan level using the `type_aliases` field in the `Plan` message. Each type alias has:

* **Anchor**: A surrogate key (uint32) used to reference the alias within the plan
* **Type**: The concrete Substrait type being aliased

### Example Plan with Type Aliases

```protobuf
Plan {
  type_aliases: [
    TypeAlias {
      type_alias_anchor: 1
      type: VarChar { length: 100, nullability: REQUIRED }
    },
    TypeAlias {
      type_alias_anchor: 2
      type: Struct {
        types: [
          Type { i32: { nullability: REQUIRED } },
          Type { alias: { type_alias_reference: 1, nullability: NULLABLE } }
        ]
        nullability: REQUIRED
      }
    }
  ]
  relations: [
    // Relations can now reference type aliases using TypeAliasReference
  ]
}
```

In this example:
- Type alias 1 defines `VARCHAR<100>` that can be reused throughout the plan
- Type alias 2 defines a struct that itself references type alias 1
- Both aliases can be referenced using `TypeAliasReference` wherever a type is expected

### Referencing Type Aliases

To reference a type alias, use a `TypeAliasReference` type:

```protobuf
Type {
  alias: TypeAliasReference {
    type_alias_reference: 1  // References the alias with anchor 1
    nullability: NULLABLE     // Nullability must be specified
  }
}
```

The nullability in the `TypeAliasReference` determines the nullability of the referenced type at the point of use, overriding any nullability specified in the type alias definition itself.

## Benefits

Type aliases provide several benefits:

1. **Reduced Plan Size**: Complex types are defined once and referenced multiple times
2. **Improved Readability**: Named types make plans easier to understand
3. **Consistency**: Ensures the same type definition is used throughout the plan
4. **Maintainability**: Changes to a type only need to be made in one place

Type aliases are particularly useful for:
- User-defined types with many parameters
- Deeply nested struct types
- Types with string parameters (e.g., long VARCHAR lengths)
- Frequently reused complex types