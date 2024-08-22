# Named Structs

A Named Struct is a special type construct that combines:
* A Struct type
* A list of names for the fields in the Struct, in depth-first search order

The depth-first search order for names arises from the the ability to nest Structs within other types. All struct fields must be named, even nested fields.

Named Structs are most commonly used to model the schema of Read relations.

## Determining Names
When producing/consuming names for a NamedStruct, some types requires special handling:

### Struct
A struct has names for each of its inner fields.

For example, the following Struct
```
        a     b
struct<i64, fp64>

```
has 2 names, one for each of its inner fields.

### Structs within Compound Types
Struct types nested in compound types must also be be named.

#### Structs within Maps
If a Map contains Structs, either as keys or values or both, the Struct fields must be named. Keys are named before values. For example the following Map
```
             a     b            c    d    e
map<struct<i64, fp64>, struct<i64, i64, i64>>
```
has 5 named fields
* 2 names [a, b] for the struct fields used as a key
* 3 names [c, d, e] for the struct fields used as a value

#### Structs within List
If a List contains Structs, the Struct fields must be named. For example the following List
```
              a     b
list<struct<i64, fp64>>
```
has 2 named fields [a, b] for the struct fields.

#### Structs within Struct
Structs can also be embedded within Structs.

A Struct like
```
            a   b     c        d   e    f    g
struct<struct<i64, fp64>, struct<i64, i64, i64>>
```
has 7 names
* 1 name [a] for the 1st nested struct field
* 2 names [b, c] for the fields within the 1st nested struct
* 1 name [d] the for the 2nd nested struct field
* 3 names [e, f, g] for the fields within the 2nd nested struct

### Putting It All Together

#### Simple Named Struct
```
NamedStruct {
    names: [a, b, c, d]
    //               a     b         c                d
    struct: struct<i64, list<i64>, map<i64, fp64>, fp64>
}
```

#### Structs in Compound Types
```
NamedStruct {
    names: [a, b, c, d, e, f, g, h]
    //               a     b          c    d      e                f     g       h
    struct: struct<i64, list<struct<i64, i64>>, map<i64, struct<fp64, fp64>>, fp64>
}
```

#### Structs in Structs
```
NamedStruct {
    names: [a, b, c, d, e, f, g, h, i]
    //               a       b   c       d    e     f     g       h   i    j
    struct: struct<i64, struct<i64, struct<fp64, fp64>, i64, struct<i64, i64>>>>
}
```

