# Compound Types

Compound types include any type that is configurable including complex types as well as configurable scalar types.

| Type Name                   | Description                                                  | Arrow Analog        | Iceberg Analog | Spark Analog   | Trino Analog                |
| --------------------------- | ------------------------------------------------------------ | ------------------- | -------------- | -------------- | --------------------------- |
| FIXEDCHAR&lt;L&gt;    | A fixed-length field of length L. L can be between [1..2,147,483,647]. Values shorter than the length of the field are padded with spaces. | -                   | -              | CharType(L)    | CHAR(L)                     |
| VARCHAR&lt;L&gt;         | A field that can holds UTF-8-encoded strings between 0 and L in character length. The value of L can be between [1..2,147,483,647]. Values shorter than L are not padded. | -                   | -              | VarcharType(L) | VARCHAR(L)                  |
| FIXEDBINARY&lt;L&gt;     | A binary field that is fixed in width to L bytes. Values that are shorter than L are 0-padded. | FixedSizeBinary&lt;L&gt;  | FIXED&lt;L&gt;       | -              | -                           |
| DECIMAL&lt;P, S&gt;   | A fixed-precision decimal value having precision (P, number of digits) <= 38 and scale (S, number of fractional digits) 0 <= S <= P. | Decimal&lt;P, S, bitwidth=128&gt; | DECIMAL(P,S)   | DECIMAL(P,S)   | DECIMAL(P,S)                |
| STRUCT&lt;T1,...,T2&gt; | A list of types in a defined order. | struct_&lt;*&gt;                  | struct&lt;*&gt; | struct&lt;*&gt; | row&lt;*&gt;   |
| NSTRUCT&lt;N:T1,...,N:T2&gt; | **Pseudo-type**: A struct that maps unique names to value types. Each name is a UTF-8-encoded string. Each value can have a distinct type. Note that NSTRUCT is actually a pseudo-type. This is because Substrait's core type system is based entirely on ordinal positions, not named fields. Nonetheless, when working with systems outside Substrait, names are important. | struct_&lt;*&gt; | struct&lt;*&gt; | struct&lt;*&gt; | row&lt;*&gt; |
| LIST&lt;T&gt;               | A list of values of type T. The list can be between [0..2,147,483,647] values in length. | list                | list           | list           | array                       |
| MAP&lt;K, V&gt;                   | An unordered list of type K keys with type V values.         | map&lt;k,v&gt;            | map&lt;k,v&gt;       | -              | map&lt;k,v&gt;                    |

