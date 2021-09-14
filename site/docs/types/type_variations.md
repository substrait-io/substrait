# Type Variations

Since Substrait is designed to work in both logical and physical contexts, there is a need to support extended attributes in the physical context. Different consumers may have multiple ways to present the same logical type. For example, an engine might support dictionary encoding a string or using either a row-wise or columnar representation of a struct. As such, there is the facility for specification users to express additional type variations for each logical type. These variations are expected to have the same logical properties as the canonical variation and are defined for each organization. The key properties of these variations are:

| Property          | Description                                                  |
| ----------------- | ------------------------------------------------------------ |
| Base Type       | The base type this variation belongs to. Variations can only be expressed for simple types and wild-carded compound types (e.g. i8 or varchar(*)). |
| Name              | The name used to reference this type. Should be unique within type variations for this parent type within an organization. |
| Description       | A human description of the purpose of this type variation    |
| Function Behavior | **Inherits** or **Independent**: Whether this variation supports functions using the canonical variation or whether functions should be resolved independently. For example if one has the function `add(i8,i8)` defined and then defines an i8 variation, can the i8 variation field be bound to the base `add` operation (inherits) or does a specialized version of `add` need to be defined specifically for this type variation (independent). Defaults to inherits. |

