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

* `BOUNDS_TYPE_ROWS` defines physical row offsets in the declared ordering. `offset_expr` must evaluate to a strictly positive number of rows and must have type `int64`. If no ordering is declared, the row order is implementation-defined.
* `BOUNDS_TYPE_RANGE` defines value offsets from the current row's ordering value and requires exactly one ordering expression. `offset_expr` must evaluate to a strictly positive distance whose type `D` is compatible with the ordering expression's type `T` — that is, `add(T, D) -> T` and `subtract(T, D) -> T` must be defined (for example `timestamp`/`interval_day`, `decimal`/`decimal`, `i64`/`i64`). For a `Preceding` bound the boundary value is `subtract(current, offset_expr)` under an ascending ordering and `add(current, offset_expr)` under a descending ordering; `Following` is the mirror. A range boundary is inclusive.

`offset_expr` is the recommended way to specify an offset. A literal integer `offset` is still accepted for compatibility but is deprecated and will be removed in a future release. At least one of the two must be set; because `offset` has no explicit presence it is considered set when it is non-zero, and zero was never a valid offset. Consumers must use `offset_expr` when it is set and ignore `offset`. During migration a producer may set both, with `offset` carrying the int64-literal equivalent — possible only when the distance is a positive int64 literal — so that consumers which do not yet read `offset_expr` can still execute the plan.

`offset_expr` is evaluated once per input record and may reference the input's fields, but must not contain window or aggregate functions. Evaluating it to null or a negative value should result in an error; evaluating it to zero is equivalent to `CurrentRow`.

## Aggregate Functions as Window Functions

Aggregate functions can be treated as a window functions with Window Type set to STREAMING.

AVG, COUNT, MAX, MIN and SUM are examples of aggregate functions that are commonly allowed in window contexts.
