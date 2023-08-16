# Type Classes

In Substrait, the "class" of a type, not to be confused with the concept from object-oriented programming, defines the set of non-null values that instances of a type may assume.

Implementations of a Substrait type must support *at least* this set of values, but may include more; for example, an `i8` could be represented using the same in-memory format as an `i32`, as long as functions operating on `i8` values within [-128..127] behave as specified (in this case, this means 8-bit overflow must work as expected). Operating on values outside the specified range is unspecified behavior.

## Simple Types

Simple type classes are those that don't support any form of configuration. For simplicity, any generic type that has only a small number of discrete implementations is declared directly, as opposed to via configuration.

| Type Name       | Description                                                  | Protobuf representation for literals
| --------------- | ------------------------------------------------------------ | ------------------------------------------------
| boolean         | A value that is either True or False.                        | `bool`
| i8              | A signed integer within [-128..127], typically represented as an 8-bit two's complement number. | `int32`
| i16             | A signed integer within [-32,768..32,767], typically represented as a 16-bit two's complement number. | `int32`
| i32             | A signed integer within [-2147483648..2,147,483,647], typically represented as a 32-bit two's complement number. | `int32`
| i64             | A signed integer within [−9,223,372,036,854,775,808..9,223,372,036,854,775,807], typically represented as a 64-bit two's complement number. | `int64`
| fp32            | A 4-byte single-precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Single-precision_floating-point_format). | `float`
| fp64            | An 8-byte double-precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Double-precision_floating-point_format). | `double`
| string          | A unicode string of text, [0..2,147,483,647] UTF-8 bytes in length. | `string`
| binary          | A binary value, [0..2,147,483,647] bytes in length.          | `binary`
| timestamp       | A naive timestamp within [1000-01-01 00:00:00.000000..9999-12-31 23:59:59.999999], with microsecond precision. Does not include timezone information and can thus not be unambiguously mapped to a moment on the timeline without context. Similar to naive datetime in Python. | `int64` microseconds since 1970-01-01 00:00:00.000000 (in an unspecified timezone)
| timestamp_tz    | A timezone-aware timestamp within [1000-01-01 00:00:00.000000 UTC..9999-12-31 23:59:59.999999 UTC], with microsecond precision. Similar to aware datetime in Python. | `int64` microseconds since 1970-01-01 00:00:00.000000 UTC
| date            | A date within [1000-01-01..9999-12-31].                      | `int32` days since `1970-01-01`
| time            | A time since the beginning of any day. Range of [0..86,399,999,999] microseconds; leap seconds need not be supported. | `int64` microseconds past midnight
| interval_year   | Interval year to month. Supports a range of [-10,000..10,000] years with month precision (= [-120,000..120,000] months). Usually stored as separate integers for years and months, but only the total number of months is significant, i.e. `1y 0m` is considered equal to `0y 12m` or `1001y -12000m`. | `int32` years and `int32` months, with the added constraint that each component can never independently specify more than 10,000 years, even if the components have opposite signs (e.g. `-10000y 200000m` is **not** allowed)
| interval_day    | Interval day to second. Supports a range of [-3,650,000..3,650,000] days with microsecond precision (= [-315,360,000,000,000,000..315,360,000,000,000,000] microseconds). Usually stored as separate integers for various components, but only the total number of microseconds is significant, i.e. `1d 0s` is considered equal to `0d 86400s`. | `int32` days, `int32` seconds, and `int32` microseconds, with the added constraint that each component can never independently specify more than 10,000 years, even if the components have opposite signs (e.g. `3650001d -86400s 0us` is **not** allowed)
| uuid            | A universally-unique identifier composed of 128 bits. Typically presented to users in the following hexadecimal format: `c48ffa9e-64f4-44cb-ae47-152b4e60e77b`. Any 128-bit value is allowed, without specific adherence to RFC4122. | 16-byte `binary`

## Compound Types

Compound type classes are type classes that need to be configured by means of a parameter pack.

| Type Name                    | Description                                                  | Protobuf representation for literals
| ---------------------------- | ------------------------------------------------------------ | ------------------------------------------------
| FIXEDCHAR&lt;L&gt;           | A fixed-length unicode string of L characters. L must be within [1..2,147,483,647]. | L-character `string`
| VARCHAR&lt;L&gt;             | A unicode string of at most L characters.L must be within [1..2,147,483,647]. | `string` with at most L characters
| FIXEDBINARY&lt;L&gt;         | A binary string of L bytes. When casting, values shorter than L are padded with zeros, and values longer than L are right-trimmed. | L-byte `bytes`
| DECIMAL&lt;P, S&gt;          | A fixed-precision decimal value having precision (P, number of digits) <= 38 and scale (S, number of fractional digits) 0 <= S <= P. | 16-byte `bytes` representing a little-endian 128-bit integer, to be divided by 10^S to get the decimal value
| STRUCT&lt;T1,...,Tn&gt;      | A list of types in a defined order. | `repeated Literal`, types matching T1..Tn
| NSTRUCT&lt;N:T1,...,N:Tn&gt; | **Pseudo-type**: A struct that maps unique names to value types. Each name is a UTF-8-encoded string. Each value can have a distinct type. Note that NSTRUCT is actually a pseudo-type, because Substrait's core type system is based entirely on ordinal positions, not named fields. Nonetheless, when working with systems outside Substrait, names are important. | n/a
| LIST&lt;T&gt;                | A list of values of type T. The list can be between [0..2,147,483,647] values in length. | `repeated Literal`, all types matching T
| MAP&lt;K, V&gt;              | An unordered list of type K keys with type V values.         | `repeated KeyValue` (in turn two `Literal`s), all key types matching K and all value types matching V

## User-Defined Types

User-defined type classes can be created using a combination of pre-defined types. User-defined types are defined as part of [simple extensions](../extensions/index.md#simple-extensions). An extension can declare an arbitrary number of user defined extension types. Once a type has been declared, it can be used in function declarations.

A YAML example of an extension type is below:

```yaml
name: point
structure:
  longitude: i32
  latitude: i32
```

This declares a new type (namespaced to the associated YAML file) called "point". This type is composed of two `i32` values named longitude and latitude.

### Structure and opaque types

The name-type object notation used above is syntactic sugar for `NSTRUCT<longitude: i32, latitude: i32>`. The following means the same thing:

```yaml
name: point
structure: "NSTRUCT<longitude: i32, latitude: i32>"
```

The structure field of a type is only intended to inform systems that don't have built-in support for the type how they can transfer the data type from one point to another without unnecessary serialization/deserialization *and* without loss of type safety. Note that it is currently not possible to "unpack" a user-defined type class into its structure type or components thereof using `FieldReference`s or any other specialized record expression; if support for this is desired for a particular type, this can be accomplished with an extension function.

The structure field is optional. If not specified, the type class is considered to be fully opaque. This implies that a systems without built-in support for the type cannot manipulate values in any way, including moving and cloning. This may be useful for exotic, context-sensitive types, such as raw pointers or identifiers that cannot be cloned.

Note however that the vast majority of types can be trivially moved and copied, even if they cannot be precisely represented using Substrait's built-in types. In this case, it is recommended to use `binary` or `FIXEDBINARY<n>` (where n is the size of the type) as the structure type. For example, an unsigned 32-bit integer type could be defined as follows:

```yaml
name: u32
structure: "FIXEDBINARY<4>"
```

In this case, `i32` might also be used.

### Literals

Literals for user-defined types are represented using protobuf [Any](https://developers.google.com/protocol-buffers/docs/proto3#any) messages.

### Compound User-Defined Types

User-defined types may be turned into compound types by requiring parameters to be passed to them. The supported "meta-types" for parameters are data types (like those used in `LIST`, `MAP`, and `STRUCT`), booleans, integers, enumerations, and strings. Using parameters, we could redefine "point" with different types of coordinates. For example:

```yaml
name: point
parameters:
  - name: T
    description: |
      The type used for the longitude and latitude
      components of the point.
    type: dataType
```

or:

```yaml
name: point
parameters:
  - name: coordinate_type
    type: enumeration
    options:
      - integer
      - double
```

or:

```yaml
name: point
parameters:
  - name: LONG
    type: dataType
  - name: LAT
    type: dataType
```

We can't specify the internal structure in this case, because there is currently no support for derived types in the structure.

The allowed range can be limited for integer parameters. For example:

```yaml
name: vector
parameters:
  - name: T
    type: dataType
  - name: dimensions
    type: integer
    min: 2
    max: 3
```

This specifies a vector that can be either 2- or 3-dimensional. Note however that it's not currently possible to put constraints on data type, string, or (technically) boolean parameters.

Similar to function arguments, the last parameter may be specified to be variadic, allowing it to be specified one or more times instead of only once. For example:

```yaml
name: union
parameters:
  - name: T
    type: dataType
variadic: true
```

This defines a type that can be parameterized with one or more other data types, for example `union<i32, i64>` but also `union<bool>`. Zero or more is also possible, by making the last argument optional:

```yaml
name: tuple
parameters:
  - name: T
    type: dataType
    optional: true
variadic: true
```

This would also allow for `tuple<>`, to define a zero-tuple.
