# Window Functions

Window functions are functions which consume values from multiple records to produce a single output. They are similar to aggregate functions, but also have a focused window of analysis to compare to their partition window. Window functions are similar to scalar values to an end user, producing a single value for each input record. However, the consumption visibility for the production of each single record can be many records.



Window function signatures contain all the properties defined for [aggregate functions](aggregate_functions.md). Additionally, they contain the properties below

| Property    | Description                                                  | Required                        |
| ----------- | ------------------------------------------------------------ | ------------------------------- |
| Inherits    | All properties defined for aggregate functions.              | N/A                             |
| Window Type | STREAMING or PARTITION. Describes whether the function needs to see all data for the specific partition operation simultaneously. Operations like SUM can produce values in a streaming manner with no complete visibility of the partition. NTILE requires visibility of the entire partition before it can start producing values. | Optional, defaults to PARTITION |



When binding an aggregate function, the binding must include the following additional properties beyond the standard scalar binding properties:

| Property    | Description                                                  | Required                                                     |
| ----------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| Partition   | A list of partitioning expressions.                          | False, defaults to a single partition for the entire dataset |
| Lower Bound | Bound Following(int64), Bound Trailing(int64) or CurrentRow. | False, defaults to start of partition                        |
| Upper Bound | Bound Following(int64), Bound Trailing(int64) or CurrentRow. | False, defaults to end of partition                          |



