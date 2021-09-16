# Compound Types

Compound types include any type that is configurable including complex types as well as configurable scalar types.

| Type Name                   | Description                                                  | Arrow Analog        | Iceberg Analog | Spark Analog   | Trino Analog                |
| --------------------------- | ------------------------------------------------------------ | ------------------- | -------------- | -------------- | --------------------------- |
| FIXEDCHAR(L)                | A fixed length field of length L. L can be between [1..2,147,483,647]. Values less that are less in length than the length of the field are padded with spaces. | None                | None           | CharType(L)    | CHAR(L)                     |
| VARCHAR(L)                  | A field that can holds UTF8 encoded strings between 0 and L length. The length of each value can be between [0..2,147,483,647]. The value of L can be between [1..2,147,483,647]. Values shorter than L are not padded. | None                | None           | VarcharType(L) | VARCHAR(L)                  |
| FIXEDBINARY(L)              | A binary field that is fixed in width to L. Values that are shorter than L are 0-byte padded. | FixedSizeBinary&lt;L&gt;  | FIXED&lt;L&gt;       | -              | -                           |
| DECIMAL(P,S)                | A fixed precision decimal value having precision (P, number of digits) <= 38 and Scale (S, number of fractional digits) 0 <= S <=  P) | Decimal&lt;P, S, bitwidth=128&gt; | DECIMAL(P,S)   | DECIMAL(P,S)   | DECIMAL(P,S)                |
| STRUCT&lt;N:T1,...,N:T2&gt; | A struct that maps unique names to value types. Each name is a UTF8 string. Each value can have a distinct type. | struct_&lt;*&gt;          | struct&lt;*&gt;      | struct&lt;*&gt;      | row&lt;*&gt;                      |
| LIST&lt;T&gt;               | A list of values of type T. The list can be between [0..2,147,483,647] values in length. | list                | list           | list           | array                       |
| MAP&lt;K, V&gt;                   | An unordered list of type K keys with type V values.         | map&lt;k,v&gt;            | map&lt;k,v&gt;       | -              | map&lt;k,v&gt;                    |

