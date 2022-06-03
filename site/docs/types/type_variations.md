# Type Variations

Type variations may be used to represent differences in representation between different consumers. For example, an engine might support dictionary encoding for a string, or could be using either a row-wise or columnar representation of a struct. All variations of a type are expected to have the same semantics when operated on by functions or other expressions.

All variations except the "system-preferred" variation (a.k.a. `[0]`, see [Type Parsing](type_parsing.md)) must be defined using [simple extensions](../extensions/index.md#simple-extensions). The key properties of these variations are:

| Property          | Description                                                  |
| ----------------- | ------------------------------------------------------------ |
| Base Type Class   | The type class that this variation belongs to.               |
| Name              | The name used to reference this type. Should be unique within type variations for this parent type within a simple extension. |
| Description       | A human description of the purpose of this type variation.   |
| Function Behavior | **INHERITS** or **SEPARATE**: whether functions that support the system-preferred variation implicitly also support this variation, or whether functions should be resolved independently. For example, if one has the function `add(i8,i8)` defined and then defines an `i8` variation, this determines whether the `i8` variation can be bound to the base `add` operation (inherits) or whether a specialized version of `add` needs to be defined specifically for this variation (separate). Defaults to inherits. |
