# Simple Logical Types

Substrait tries to cover the most common types used in data manipulation. Simple types are those that don't support any form of configuration. For simplicity, any generic type that has only a small number of discrete implementations is declared directly (as opposed to via configuration).

To minimize type explosion, the project currently follows the guideline that a logical type should probably only be included in the specification if it is included in at least two of the following open source Projects: Apache Arrow, Apache Iceberg, Apache Spark and Trino.

| Type Name       | Description                                                  | Arrow Analog           | Iceberg Analog | Spark Analog  | Trino Analog           |
| --------------- | ------------------------------------------------------------ | ---------------------- | -------------- | ------------- | ---------------------- |
| boolean         | A value that is either true or false.                        | Bool                   | boolean        | boolean       | boolean                |
| i8              | A signed 8 byte value in [-128..127]                         | Int&lt;8,true&gt;            | -              | ByteType      | tinyint                |
| i16             | A signed 16 byte value between [-32,768..32,767]             | Int&lt;16,true&gt;           | -              | ShortType     | smallint               |
| i32             | A signed 32 byte value between [-2147483648..2,147,483,647]  | Int&lt;32,true&gt;           | int            | IntegerType   | int                    |
| i64             | A signed 64 byte value between [−9,223,372,036,854,775,808..9,223,372,036,854,775,807] | Int&lt;64,true&gt;           | long           | LongType      | bigint                 |
| fp32            | A 4 byte single precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Single-precision_floating-point_format). | Float&lt;SINGLE&gt;          | float          | FloatType     | real                   |
| fp64            | An 8 byte double precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Double-precision_floating-point_format). | Float&lt;DOUBLE&gt;          | double         | DecimalType   | double                 |
| string          | A string of text that can be up to 2,147,483,647 bytes in length. String is encoded in UTF8 | Utf8                   | string         | StringType    | varchar (no len)       |
| binary          | A binary value that can be up to 2,147,483,647 bytes in length. | Binary                 | binary         | BinaryType    | Varbinary              |
| timestamp_micro | A timestamp with microsecond precision                       | Timestamp&lt;MICROSECOND&gt; | timestamp      | TimestampType | timestamp(6)           |
| timestamp_milli | A timestamp with millisecond precision                       | Timestamp&lt;MILLISECOND&gt; | -              | -             | timestamp(3)           |
| date            | Date, expressed as number of seconds since epoch             | Date&lt;MILLISECOND&gt;      | date           | DateType      | Date                   |
| time_micro      | A time expressed in microseconds since start of day          | Time&lt;MICROSECOND;64&gt;   | time           | time(6)       | time(6)                |
| time_milli      | A time expressed in milliseconds since start of day          | Time&lt;MILLISECOND;32&gt;   | -              | time(3)       | time(3)                |
| interval_year   | Interval day to month                                        | INTERVAL&lt;YEAR_MONTH&gt;   | -              | -             | Interval year to month |
| interval_day    | Interval day to second                                       | INTERVAL&lt;DAY_TIME&gt;     | -              | -             | Interval day to second |

#### Discussion points

* How do we ensure Substrait is adoptable by different communities while avoiding type explosion? Is it important to avoid type explosion? Current proposal is to avoid types unless they exist in at least two projects.
* Does it make sense to support user defined types?