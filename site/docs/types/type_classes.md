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
| fp32            | A 4-byte single-precision floating point number with the same range and precision as defined for the [IEEE 754 32-bit floating-point format](https://standards.ieee.org/ieee/754/6210/). | `float`
| fp64            | An 8-byte double-precision floating point number with the same range and precision as defined for the [IEEE 754 64-bit floating-point format](https://standards.ieee.org/ieee/754/6210/). | `double`
| string          | A unicode string of text, [0..2,147,483,647] UTF-8 bytes in length. | `string`
| binary          | A binary value, [0..2,147,483,647] bytes in length.          | `binary`
| timestamp       | A naive timestamp with microsecond precision. Does not include timezone information and can thus not be unambiguously mapped to a moment on the timeline without context. Similar to naive datetime in Python. | `int64` microseconds since 1970-01-01 00:00:00.000000 (in an unspecified timezone)
| timestamp_tz    | A timezone-aware timestamp with microsecond precision. Similar to aware datetime in Python. | `int64` microseconds since 1970-01-01 00:00:00.000000 UTC
| date            | A date within [1000-01-01..9999-12-31].                      | `int32` days since `1970-01-01`
| time            | A time since the beginning of any day. Range of [0..86,399,999,999] microseconds; leap seconds need not be supported. | `int64` microseconds past midnight
| interval_year   | Interval year to month. Supports a range of [-10,000..10,000] years with month precision (= [-120,000..120,000] months). Usually stored as separate integers for years and months, but only the total number of months is significant, i.e. `1y 0m` is considered equal to `0y 12m` or `1001y -12000m`. | `int32` years and `int32` months, with the added constraint that each component can never independently specify more than 10,000 years, even if the components have opposite signs (e.g. `-10000y 200000m` is **not** allowed)
| uuid            | A universally-unique identifier composed of 128 bits. Typically presented to users in the following hexadecimal format: `c48ffa9e-64f4-44cb-ae47-152b4e60e77b`. Any 128-bit value is allowed, without specific adherence to RFC4122. | 16-byte `binary`

## Compound Types

Compound type classes are type classes that need to be configured by means of a parameter pack.

| Type Name                     | Description                                                                                                                                                                                                                                                                                                                                                           | Protobuf representation for literals
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| ------------------------------------------------
| FIXEDCHAR&lt;L&gt;            | A fixed-length unicode string of L characters. L must be within [1..2,147,483,647].                                                                                                                                                                                                                                                                                   | L-character `string`
| VARCHAR&lt;L&gt;              | A unicode string of at most L characters.L must be within [1..2,147,483,647].                                                                                                                                                                                                                                                                                         | `string` with at most L characters
| FIXEDBINARY&lt;L&gt;          | A binary string of L bytes. When casting, values shorter than L are padded with zeros, and values longer than L are right-trimmed.                                                                                                                                                                                                                                    | L-byte `bytes`
| DECIMAL&lt;P, S&gt;           | A fixed-precision decimal value having precision (P, number of digits) <= 38 and scale (S, number of fractional digits) 0 <= S <= P.                                                                                                                                                                                                                                  | 16-byte `bytes` representing a little-endian 128-bit integer, to be divided by 10^S to get the decimal value
| STRUCT&lt;T1,...,Tn&gt;       | A list of types in a defined order.                                                                                                                                                                                                                                                                                                                                   | `repeated Literal`, types matching T1..Tn
| NSTRUCT&lt;N:T1,...,N:Tn&gt;  | **Pseudo-type**: A struct that maps unique names to value types. Each name is a UTF-8-encoded string. Each value can have a distinct type. Note that NSTRUCT is actually a pseudo-type, because Substrait's core type system is based entirely on ordinal positions, not named fields. Nonetheless, when working with systems outside Substrait, names are important. | n/a
| LIST&lt;T&gt;                 | A list of values of type T. The list can be between [0..2,147,483,647] values in length.                                                                                                                                                                                                                                                                              | `repeated Literal`, all types matching T
| MAP&lt;K, V&gt;               | An unordered list of type K keys with type V values. Keys may be repeated. While the key type could be nullable, keys may not be null.                                                                                                                                                                                                                                | `repeated KeyValue` (in turn two `Literal`s), all key types matching K and all value types matching V
| PRECISIONTIMESTAMP&lt;P&gt;   | A timestamp with fractional second precision (P, number of digits) 0 <= P <= 12. Does not include timezone information and can thus not be unambiguously mapped to a moment on the timeline without context. Similar to naive datetime in Python.                                                                                                                     | `int64` seconds, milliseconds, microseconds, nanoseconds or picoseconds since 1970-01-01 00:00:00.000000000000 (in an unspecified timezone)
| PRECISIONTIMESTAMPTZ&lt;P&gt; | A timezone-aware timestamp, with fractional second precision (P, number of digits) 0 <= P <= 12. Similar to aware datetime in Python.                                                                                                                                                                                                                                 | `int64` seconds, milliseconds, microseconds, nanoseconds or picoseconds since 1970-01-01 00:00:00.000000000000 UTC
| INTERVAL_DAY&lt;P&gt;         | Interval day to second. Supports a range of [-3,650,000..3,650,000] days with fractional second precision (P, number of digits) 0 <= P <= 9. Usually stored as separate integers for various components, but only the total number of fractional seconds is significant, i.e. `1d 0s` is considered equal to `0d 86400s`. | `int32` days, `int32` seconds, and `int64` fractional seconds, with the added constraint that each component can never independently specify more than 10,000 years, even if the components have opposite signs (e.g. `3650001d -86400s 0us` is **not** allowed)
| INTERVAL_COMPOUND&lt;P&gt;    | A compound interval type that is composed of elements of the underlying elements and rules of both interval_month and interval_day to express arbitrary durations across multiple grains. Substrait gives no definition for the conversion of values between independent grains (e.g. months to days).

## User-Defined Types

User-defined type classes are defined as part of [simple extensions](../extensions/index.md#simple-extensions). An extension can declare an arbitrary number of user-defined extension types. Once a type has been declared, it can be used in function declarations.

For example, the following declares a type named `point` (namespaced to the associated YAML file) and two scalar functions that operate on it.

```yaml
types:
  - name: "point"

scalar_functions:
  - name: "lat"
    impls:
      - args:
        - name: p
        - value: u!point
    return: fp64
  - name: "lon"
    impls:
      - args:
        - name: p
        - value: u!point
    return: fp64
```

### Handling User-Defined Types

Systems without support for a specific user-defined type:
* Cannot generate values of the type.
* Cannot implement functions operating on the type.
* _May_ support consuming and emitting values of the type _without_ modifying them.

### Communicating User-Defined Types

Specifiers of user-defined types may provide additional structure information for the type to assist in communicating values of the type to and from systems without built-in support.

For example, the following declares a `point` type with two `i32` values named longitude and latitude:

```yaml
types:
  - name: point
    structure:
      longitude: i32
      latitude: i32
```

The name-type object notation used above is syntactic sugar for `NSTRUCT<longitude: i32, latitude: i32>`. The following means the same thing:

```yaml
name: point
structure: "NSTRUCT<longitude: i32, latitude: i32>"
```

The structure field of a type is only intended to inform systems that don't have built-in support for the type about how they can create and transfer values of that type to systems that do support the type.

The structure field _does not_ restrict or bind the internal representation of the type in any system.

As such, it's currently not possible to "unpack" a user-defined type into its structure type or components thereof using `FieldReference`s or any other specialized record expression; if support for this is desired for a particular type, this can be accomplished with an extension function.

### Literals

Literals for user-defined types can be represented in one of two ways:
* Using protobuf [Any](https://developers.google.com/protocol-buffers/docs/proto3#any) messages.
* Using the structure representation of the type.

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
