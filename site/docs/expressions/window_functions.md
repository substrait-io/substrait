# Window Functions

Window functions are functions which consume values from multiple records to produce a single output. They are similar to aggregate functions, but also have a focused window of analysis to compare to their partition window. Window functions are similar to scalar values to an end user, producing a single value for each input record. However, the consumption visibility for the production of each single record can be many records.



Window function signatures contain all the properties defined for [aggregate functions](aggregate_functions.md). Additionally, they contain the properties below

| Property    | Description                                                  | Required                        |
| ----------- | ------------------------------------------------------------ | ------------------------------- |
| Inherits    | All properties defined for aggregate functions.              | N/A                             |
| Window Type | STREAMING or PARTITION. Describes whether the function needs to see all data for the specific partition operation simultaneously. Operations like SUM can produce values in a streaming manner with no complete visibility of the partition. NTILE requires visibility of the entire partition before it can start producing values. | Optional, defaults to PARTITION |



When binding a window function, the binding must include the following additional properties beyond the standard aggregate binding properties:

| Property    | Description                                                  | Required |
| ----------- | ------------------------------------------------------------ | -------- |
| Partition   | A list of partitioning expressions. Empty list means a single partition for the entire dataset. | True     |
| Order By    | A list of ordering expressions with sort directions. Empty list means unordered. | True     |
| Bounds Type | ROWS or RANGE. ROWS bounds count physical rows. RANGE bounds consider value equivalence based on ordering columns. | True     |
| Lower Bound | Preceding(int64), Following(int64), CurrentRow, or Unbounded. | True     |
| Upper Bound | Preceding(int64), Following(int64), CurrentRow, or Unbounded. | True     |

### RANGE Bounds with Multiple Ordering Columns

When using RANGE bounds with numeric offsets (Preceding or Following with offset > 0), only a single ordering column is allowed. This is because numeric offsets require arithmetic on the ordering column values (e.g., current_value - offset), which is ambiguous with multiple columns.

RANGE bounds with UNBOUNDED or CURRENT ROW work with any number of ordering columns. CURRENT ROW includes all rows with matching values across all ordering columns (peer rows).

## Aggregate Functions as Window Functions

Aggregate functions can be treated as a window functions with Window Type set to STREAMING.

AVG, COUNT, MAX, MIN and SUM are examples of aggregate functions that are commonly allowed in window contexts.
