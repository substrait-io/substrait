# Physical Types

Since Substrait is designed to work in both logical and physical contexts, there is need to support extended attributes in the physical context.

For each logical type, we declare one or more physical representations of that logical type as approrpriate to the system specializations. Additionally, we describe whether a particular type is dictionary encoded. Each of these representation details is also used when specifiying a function signature to determine which of the specific physical representations of data are supported by a paticular function signature.

In many cases, a system will only have a single physical representation of each type. In those cases, it is expected that the binding of an operation is associated with the system default representation of the data. While a physical types are defined as discrete from logical types within the specification, the serialization formats will typically collapse these into a singular concept.

| Logical Type  | Physical Representations                                    | Support Dictionary Encoding |
| ------------- | ----------------------------------------------------------- | --------------------------- |
| boolean       | 0=System default                                            | no                          |
| i8            | 0=System default                                            | no                          |
| i16           | 0=System default                                            | no                          |
| i32           | 0=System default                                            | no                          |
| i64           | 0=System default                                            | no                          |
| fp32          | 0=System default                                            | no                          |
| fp64          | 0=System default                                            | no                          |
| string        | 0=System default, 1=Arrow Large String                      | yes                         |
| binary        | 0=System default, Arrow Large Binary                        | yes                         |
| timestamp     | 0=System default                                            | no                          |
| date          | 0=System default                                            | no                          |
| time          | 0=System default                                            | no                          |
| interval_year | 0=System default                                            | no                          |
| interval_day  | 0=System default, 1=Arrow MONTH_DAY_NANO                    | no                          |
| fixedchar     | 0=System default                                            | yes                         |
| varchar       | 0=System default                                            | yes                         |
| fixedbinary   | 0=System default                                            | yes                         |
| decimal       | 0=System default, 1=Arrow 128 Bit Width                     | no                          |
| struct        | 0=System default                                            | yes                         |
| list          | 0=System default, 1=Arrow Large List                        | yes                         |
| map           | 0=System default, 1=Map where keys are utf8 ordered strings | yes                         |
| timestamp_tz  | 0=System default                                            | no                          |





