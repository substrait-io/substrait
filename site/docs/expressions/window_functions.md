# Window Functions

A window function produces one result for each input row, using that row and other related rows. Unlike a grouped aggregate, it does not collapse those rows into a single output row. For example, a windowed `SUM` can attach a running total to every row.

## Partitions, Peer Groups, and Frames

Three concepts determine which rows a window function can use:

* **Partition:** the rows with the same values for all partitioning expressions. Each partition is processed independently; a frame never crosses a partition boundary. Null partitioning values are considered equal. Without partitioning expressions, the entire input is one partition.
* **Peer group (row group):** rows within a partition that share the same values for **all** ordering expressions. Two null ordering values are considered equal. Ordering arranges the rows within a partition, but does not break ties between peers. Without ordering expressions, all rows in a partition are peers.
* **Window frame:** the subset of a partition selected by lower and upper bounds for the **current row**, the row whose result is being calculated. The frame can change as the current row changes. Whether a bound counts rows or measures a distance in ordering values depends on the [bound mode](#bound-modes).

For example, partition by `team` and order by `score` ascending:

| Row | team | score | amount | Peer group within the partition |
| --- | ---- | ----- | ------ | ------------------------------- |
| A | North | 10 | 2 | 1 |
| B | North | 20 | 3 | 2 |
| **C (current)** | **North** | **20** | **5** | **2** |
| D | North | 30 | 7 | 3 |
| E | North | 50 | 11 | 4 |
| F | South | 20 | 100 | 1 |

Rows B and C are peers. Row F has the same score, but belongs to a different partition. For current row C, these frames give different results for `SUM(amount)`:

| Mode | Lower bound | Upper bound | Rows in frame | Result |
| ---- | ----------- | ----------- | ------------- | ------ |
| ROWS | `CurrentRow` | `CurrentRow` | C | 5 |
| RANGE | `CurrentRow` | `CurrentRow` | B, C | 8 |
| ROWS | `Preceding(1)` | `CurrentRow` | B, C | 8 |
| RANGE | `Preceding(10)` | `CurrentRow` | A, B, C (scores 10 through 20) | 10 |
| ROWS | `CurrentRow` | `Following(1)` | C, D | 12 |
| RANGE | `CurrentRow` | `Following(10)` | B, C, D (scores 20 through 30) | 15 |

`Preceding(n)` and `Following(n)` are shorthand here for bounds with an `offset_expr` evaluating to `n`. The ROWS examples assume B appears before C; ordering only by `score` does not guarantee that order among peers. Adding a tie-breaking ordering expression makes the row order deterministic, but also changes which rows are peers.

The RANGE `Following(10)` example includes B even though it is displayed before C: the `CurrentRow` lower bound starts at the beginning of their shared peer group.

RANGE frames use peer groups, but peer groups are not a bound mode of their own: Substrait currently defines ROWS and RANGE, not a mode that counts peer groups.

## Window Bindings

A window binding inherits the [aggregate binding properties](aggregate_functions.md#aggregate-binding) and adds the following:

| Property | Protobuf field | Meaning and default |
| -------- | -------------- | ------------------- |
| Partitioning | `partitions` | Partitioning expressions; none means one partition for the entire input. |
| Ordering | `sorts` | Ordering expressions, highest priority first. Optional; only allowed when the function supports sorting. |
| Bound mode | `bounds_type` | Required: `BOUNDS_TYPE_ROWS` or `BOUNDS_TYPE_RANGE`. Consumers must reject `BOUNDS_TYPE_UNSPECIFIED`. |
| Lower bound | `lower_bound` | Inclusive start of the frame; defaults to the start of the partition. |
| Upper bound | `upper_bound` | Inclusive end of the frame; defaults to the end of the partition. |

In a [consistent partition window operation](../relations/physical_relations.md#consistent-partition-window-operation), `partition_expressions` and `sorts` are shared by all functions. Each function still has its own bound mode and frame bounds. The same rules below apply to both representations.

## Frame Bounds

| Bound | Meaning |
| ----- | ------- |
| `Unbounded` | Start of the partition for a lower bound; end of the partition for an upper bound. |
| `CurrentRow` | The current row in ROWS mode; the start or end of its peer group in RANGE mode. |
| `Preceding` | An offset toward rows or values **earlier** in the declared ordering. |
| `Following` | An offset toward rows or values **later** in the declared ordering. |

Both bounds are inclusive, and only rows in the current partition can belong to the frame. If neither bound is specified, the frame is the whole partition, even when ordering is specified.

A frame need not contain the current row. If the lower bound is after the upper bound, the frame is empty. The function's empty-frame semantics apply, such as `SUM` yielding null and `COUNT(*)` yielding zero.

## Bound Modes

### ROWS

`BOUNDS_TYPE_ROWS` counts **physical rows** in the declared ordering:

* `CurrentRow` means only the current row, even when it has peers.
* `Preceding` and `Following` offsets must have type `int64` and evaluate to a non-negative number of rows.
* Without ordering, no row order is guaranteed. With ordering, the relative order of peers is still not guaranteed.

In the example above, one preceding row means B and one following row means D when C is current and the displayed order is used. These offsets count individual rows, not groups of rows sharing a score.

### RANGE

`BOUNDS_TYPE_RANGE` measures **distances in ordering values**, rather than counting rows:

* `CurrentRow` means the first row of the current peer group for a lower bound and the last row of that group for an upper bound.
* If either bound is `Preceding` or `Following`, there must be exactly one ordering expression, and it must not use `SORT_DIRECTION_CLUSTERED` or a custom comparison function.
* An offset must be a non-negative distance with a [compatible type](#offset-type-compatibility).
* When the current row's ordering value is null, an offset bound is equivalent to `CurrentRow`.

The single-ordering-expression restriction does not apply when both bounds are `CurrentRow` or `Unbounded` (or omitted). With multiple ordering expressions, peer equality uses all of them.

For an offset `d` and current ordering value `v`, compute the boundary in the direction of the declared ordering:

| Sort direction | `Preceding(d)` | `Following(d)` |
| -------------- | -------------- | -------------- |
| Ascending | `subtract(v, d)` (lower values) | `add(v, d)` (higher values) |
| Descending | `add(v, d)` (higher values) | `subtract(v, d)` (lower values) |

A boundary value need not occur in the input. For example, with ascending scores, current score 20, and bounds `Preceding(7)` through `CurrentRow`, the frame includes scores from 13 through 20, including every peer at score 20.

#### Offset Type Compatibility

Let `T` be the ordering expression's type and `D` the offset expression's type. `D` must be compatible with `T`: `add(T, D) -> T` and `subtract(T, D) -> T` must be defined.

For example, the standard extensions allow:

| Ordering type `T` | Offset type `D` | Boundary type |
| ----------------- | --------------- | ------------- |
| `i64` | `i64` | `i64` |
| `precision_timestamp<P>` | `interval_day<P>` | `precision_timestamp<P>` |

!!! note "Open compatibility questions"
    This restates the existing compatibility rule. Widened boundary types, arithmetic requiring additional arguments, and selection among extension declarations are under discussion in [#1227](https://github.com/substrait-io/substrait/issues/1227); changes to that rule are deferred to that issue.

## Offset Expressions

The following rules apply to `offset_expr` in both `Preceding` and `Following`, in either bound mode:

* Field references resolve against the containing relation's input schema. The expression must behave as if evaluated once per input row, and must not contain window or aggregate functions.
* A null or negative result is invalid. Use the opposite bound direction, not a negative offset, to move the other way.
* A statically-known zero offset **must** be represented as `CurrentRow`. If a non-literal expression evaluates to zero for a row, the bound is equivalent to `CurrentRow` for that row.

The strictly positive integer `offset` field is deprecated in favor of `offset_expr`. Producers must set at least one, and consumers must reject a `Preceding` or `Following` bound with neither set.

Following the [field replacement migration policy](../spec/breaking_change_policy.md#replacing-a-protobuf-field), consumers use `offset_expr` when present and ignore `offset`. Producers also write an equivalent `offset` when `offset_expr` has an exact legacy representation as one fixed, strictly positive `int64` offset, semantically equivalent for every input row; literal syntax is not required. Otherwise, producers write only `offset_expr`.

## Function Signatures

Window function signatures inherit all properties of [aggregate functions](aggregate_functions.md) and add:

| Property | Description | Default |
| -------- | ----------- | ------- |
| Window Type | `STREAMING` or `PARTITION`: whether the function can produce results without seeing the entire partition. `SUM` can operate in a streaming manner; `NTILE` needs the entire partition. | `PARTITION` |

Window Type describes the function's data needs, not its bound mode. Aggregate functions such as `AVG`, `COUNT`, `MAX`, `MIN`, and `SUM` can be used as window functions with Window Type `STREAMING`.
