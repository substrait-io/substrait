# Simple Types

Substrait tries to cover the most common types used in data manipulation. Simple types are those that don't support any form of configuration. For simplicity, any generic type that has only a small number of discrete implementations is declared directly (as opposed to via configuration).

To minimize type explosion, the project currently follows the guideline that a logical type should probably only be included in the specification if it is included in at least two of the following open source Projects: Apache Arrow, Apache Iceberg, Apache Spark and Trino.

| Type Name       | Description                                                  | Arrow Analog           | Iceberg Analog | Spark Analog  | Trino Analog           |
| --------------- | ------------------------------------------------------------ | ---------------------- | -------------- | ------------- | ---------------------- |
| boolean         | A value that is either True or False.                  | Bool                   | boolean        | boolean       | boolean                |
| i8              | A signed 8 byte value in [-128..127]                         | Int&lt;8,true&gt;            | -              | ByteType      | tinyint                |
| i16             | A signed 16 byte value between [-32,768..32,767]             | Int&lt;16,true&gt;           | -              | ShortType     | smallint               |
| i32             | A signed 32 byte value between [-2147483648..2,147,483,647]  | Int&lt;32,true&gt;           | int            | IntegerType   | int                    |
| i64             | A signed 64 byte value between [−9,223,372,036,854,775,808..9,223,372,036,854,775,807] | Int&lt;64,true&gt;           | long           | LongType      | bigint                 |
| fp32            | A 4 byte single precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Single-precision_floating-point_format). | Float&lt;SINGLE&gt;          | float          | FloatType     | real                   |
| fp64            | An 8 byte double precision floating point number with range as defined [here](https://en.wikipedia.org/wiki/Double-precision_floating-point_format). | Float&lt;DOUBLE&gt;          | double         | DecimalType   | double                 |
| string          | A string of text. [0..2,147,483,647] bytes in length. String is encoded using UTF8 encoding. | Utf8                   | string         | StringType    | varchar (no len)       |
| binary          | A binary value. [0..2,147,483,647] bytes in length. | Binary                 | binary         | BinaryType    | Varbinary              |
| timestamp | A naive timestamp with microsecond precision that cannot be mapped to a moment on the timeline. Similar to naive datetime in Python. Supports range of [1000-01-01 00:00:00.000000..9999-12-31 23:59:59.999999] | timestamp&lt;MICROSECOND&gt; | timestamp      | TimestampType | timestamp(6)           |
| timestamp_tz | A timestamp with microseconds precision that is mapped to an instant in time. Similar to aware datetime in Python. Supports a range of [1000-01-01 00:00:00.000000..9999-12-31 23:59:59.999999] UTC. | timestamp&lt;micro;utc&gt; | timestamptz | - | timestamp(6) with time zone |
| date            | A date. Range of [1000-01-01..9999-12-31]. | Date&lt;MILLISECOND&gt;      | date           | DateType      | Date                   |
| time      | A time with microsecond precision since the beginning of any day. Range of [0..86,399,999,999] microseconds. | Time&lt;MICROSECOND;64&gt;   | time           | time(6)       | time(6)                |
| interval_year   | Interval year to month. Supports a range of any combination of years and months that total less than or equal to 10,000 years. Each component can be specified as positive or negative. Examples minimums/maximums include: [10000y, -120000m, 1y119988m, 1000y108000m, etc]. Note that each component can never independently specify more than 10,000 years, (even if the components have opposite signs e.g. -10000y200000m is **not** allowed). | INTERVAL&lt;YEAR_MONTH&gt;   | -              | -             | Interval year to month |
| interval_day    | Interval day to second with microsecond precision. Supports a range of  [-3,650,000..3,650,000] days and [-9,223,372,036,854,775..9,223,372,036,854,775] microseconds in any combination. | INTERVAL&lt;MONTH_DAY_NANO&gt; | -              | -             | Interval day to second |
| uuid | A universally unique identifier composed of 128bits. Typically presented to users in hexadecimal format such as: `c48ffa9e-64f4-44cb-ae47-152b4e60e77b`. Any 128 bit value is allowed without specific adherance to RFC4122. |  | uuid |  | UUID |
