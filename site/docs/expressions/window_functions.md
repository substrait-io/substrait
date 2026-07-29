# Window Functions

Window functions are functions which consume values from multiple records to produce a single output. They are similar to aggregate functions, but also have a focused window of analysis to compare to their partition window. Window functions are similar to scalar values to an end user, producing a single value for each input record. However, the consumption visibility for the production of each single record can be many records.



Window function signatures contain all the properties defined for [aggregate functions](aggregate_functions.md). Additionally, they contain the properties below

| Property    | Description                                                  | Required                        |
| ----------- | ------------------------------------------------------------ | ------------------------------- |
| Inherits    | All properties defined for aggregate functions.              | N/A                             |
| Window Type | STREAMING or PARTITION. Describes whether the function needs to see all data for the specific partition operation simultaneously. Operations like SUM can produce values in a streaming manner with no complete visibility of the partition. NTILE requires visibility of the entire partition before it can start producing values. | Optional, defaults to PARTITION |



When binding a window function, the binding must include the following additional properties beyond the standard aggregate binding properties:

| Property    | Description                                                  | Required                                                     |
| ----------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| Partition   | A list of partitioning expressions.                          | False, defaults to a single partition for the entire dataset |
| Lower Bound | Preceding, Following, CurrentRow, or Unbounded.               | False, defaults to start of partition                        |
| Upper Bound | Preceding, Following, CurrentRow, or Unbounded.               | False, defaults to end of partition                          |

`Preceding` and `Following` define offsets relative to the current record and with respect to the declared ordering. `Preceding` moves toward records or values earlier in that ordering, while `Following` moves toward records or values later in that ordering. They therefore apply in opposite value directions for ascending and descending sorts:

| Sort direction | `Preceding` boundary              | `Following` boundary            |
| -------------- | --------------------------------- | ------------------------------- |
| Ascending      | Toward lower ordering values      | Toward higher ordering values   |
| Descending     | Toward higher ordering values     | Toward lower ordering values    |

The interpretation of an offset depends on `BoundsType`:

* `BOUNDS_TYPE_ROWS` defines physical row offsets in the declared ordering. `offset_expr` must evaluate to a strictly positive integer number of rows; `int64` is the recommended type.
* `BOUNDS_TYPE_RANGE` defines value offsets from the current row's ordering value and requires exactly one ordering expression. `offset_expr` must evaluate to a strictly positive distance compatible with that ordering expression. A range boundary is inclusive.

`offset_expr` is the recommended way to specify an offset. A literal integer `offset` is still accepted for compatibility but is deprecated. Evaluating `offset_expr` to null, zero, or a negative value should result in an error. Use `CurrentRow` for zero and the opposite bound direction for a negative distance.

## Aggregate Functions as Window Functions

Aggregate functions can be treated as a window functions with Window Type set to STREAMING.

AVG, COUNT, MAX, MIN and SUM are examples of aggregate functions that are commonly allowed in window contexts.
