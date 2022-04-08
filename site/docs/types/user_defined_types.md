# User Defined Types

User Defined Types can be created using a combination of pre-defined simple and compound types. User defined types are defined as part of [simple extensions](../extensions/index.md#simple-extensions). An extension can declare an arbitrary number of user defined extension types. Initially, user defined types must be simple types (although they can be constructed of a number of inner compound and simple types).

A yaml example of an extension type is below:

```yaml
  name: point
  structure:
    longitude: i32
    latitude: i32
```

This declares a new type (namespaced to the associated YAML file) called "point". This type is composed of two i32 values named longitude and latitude. Once a type has been declared, it can be used in function declarations.  [TBD: should field references be allowed to dereference the components of a user defined type?]

