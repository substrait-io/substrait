# Complex Types

CommonPlan tries to cover the most common types used in data manipulation. Since CommonPlan is focused on both logical and physical manipulation, the type system needs to support all the common logical types. Simple types are those that don't support any form of configuration. For simplicity, any generic type that has only a small number of discrete implementations is declared directly (as opposed to via configuration).



OPEN QUESTION: how do we map differing logical types to physical types, minimizing producer complexity without losing power. For example, there are two different physical representations of decimal in arrow, two different list length types in arrow, etc.

| Type Name                 | Description                                                  | Arrow Analog       | Iceberg Analog | Spark Analog   | Trino Analog |
| ------------------------- | ------------------------------------------------------------ | ------------------ | -------------- | -------------- | ------------ |
| FIXEDCHAR(L)              | A fixed length field of length L. L can be between [1..2,147,483,647]. Values less that are less in length than the length of the field are padded with spaces. | None               | None           | CharType(L)    | CHAR(L)      |
| VARCHAR(L)                | A field that can holds UTF8 encoded strings between 0 and L length. The length of each value can be between [0..2,147,483,647]. The value of L can be between [1..2,147,483,647]. Values shorter than L are not padded. | None               | None           | VarcharType(L) | VARCHAR(L)   |
| FIXEDBINARY(L)            | A binary field that is fixed in width to L. Values that are shorter than L are 0-byte padded. | FixedSizeBinary<L> | FIXED<L>       | -              | -            |
| DECIMAL(P,S)              | Fixed point decimal with precision P and scale S. Precision must be 38 or less. | DECIMAL(P,S,256)   | DECIMAL(P,S)   | DECIMAL(P,S)   | DECIMAL(P,S) |
| STRUCT&lt;N:T,...,N:T&gt; |                                                              | struct_<*>         | struct<*>      | struct<*>      | row          |
| LIST<T>                   | A list of values of type T. The list can be between [0..2,147,483,647] values in length. Maps to the | list               | list           | list           | array        |
| MAP<K, V>                 | An unordered list of type K keys with type V values.         | map<k,v>           | map<k,v>       |                | map          |
| ~~UNION<T, T, T>~~        | A union of types. Each type is dereferenced based on type position. Each value can | union              | -              | -              | -            |
| TIMESTAMP_MICRO_TZ(TZ)    | A timestamp in microseconds with a timezone TZ.              |                    |                |                |              |
|                           |                                                              |                    |                |                |              |
|                           |                                                              |                    |                |                |              |
|                           |                                                              |                    |                |                |              |
|                           |                                                              |                    |                |                |              |
|                           |                                                              |                    |                |                |              |

