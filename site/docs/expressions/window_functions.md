# Window Functions

A window function produces one result for each input row, using that row and other related rows. Unlike a grouped aggregate, it does not collapse those rows into a single output row. For example, a windowed `SUM` can attach a running total to every row.

## Partitions, Peer Groups, and Frames

Three concepts determine which rows a window function can use:

* **Partition:** rows with the same values for all partitioning expressions. Each partition is processed independently; a frame never crosses a partition boundary. Null partitioning values are considered equal. Without partitioning expressions, the entire input is one partition.
* **Peer group:** rows within a partition that share the same values for **all** ordering expressions. Two null ordering values are considered equal. Ordering arranges rows within a partition, but does not break ties between peers. Without ordering expressions, all rows in a partition are peers.
* **Frame:** the subset of a partition selected by lower and upper bounds relative to the **current row**, which is the row whose result is being calculated. The frame can change as the current row changes. Whether a bound counts rows or measures a distance in ordering values depends on the [bound mode](#bound-modes).

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
| Ordering | `sorts` | Ordering expressions within each partition, highest priority first. Optional, except as required by the [bound mode rules](#bound-modes). This also supplies the inherited aggregate ordering: rows in the frame are passed to the function in this order. |
| Bound mode | `bounds_type` | Required: `BOUNDS_TYPE_ROWS` or `BOUNDS_TYPE_RANGE`. Consumers must reject `BOUNDS_TYPE_UNSPECIFIED`. |
| Lower bound | `lower_bound` | Inclusive start of the frame; defaults to Unbounded if not set. |
| Upper bound | `upper_bound` | Inclusive end of the frame; defaults to Unbounded if not set. |

In a [consistent partition window operation](../relations/physical_relations.md#consistent-partition-window-operation), `partition_expressions` and `sorts` are shared by all functions. Each function still has its own bound mode and frame bounds. The same rules below apply to both representations.

## Frame Bounds

| Bound | Meaning |
| ----- | ------- |
| `Unbounded` | Start of the partition for a lower bound; end of the partition for an upper bound. |
| `CurrentRow` | The current row in ROWS mode; the start or end of its peer group in RANGE mode. |
| `Preceding` | An offset toward rows or values **earlier** in the declared ordering. |
| `Following` | An offset toward rows or values **later** in the declared ordering. |

Both bounds are inclusive, and only rows in the current partition can belong to the frame.

A frame need not contain the current row. If the lower bound is after the upper bound, the frame is empty. The function's empty-frame semantics apply, such as `SUM` yielding null and `COUNT(*)` yielding zero.

## Offset Expressions

The following rules apply to `offset_expr` in both `Preceding` and `Following`, in either bound mode:

* Field references resolve against the input relation schema. The expression must behave as if evaluated once per input row, and must not contain window or aggregate functions.
* A null or negative result is invalid. Use the opposite bound direction, not a negative offset, to move the other way.
* A statically-known zero offset **must** be represented as `CurrentRow`. If a non-literal expression evaluates to zero for a row, the bound is equivalent to `CurrentRow` for that row.

The strictly positive integer `offset` field is deprecated in favor of `offset_expr` and will be removed in a future release. Producers must set at least one. Because `offset` has no field presence, an `offset` of 0 counts as unset, and consumers must reject a `Preceding` or `Following` bound with no `offset_expr` and an `offset` of 0.

When `offset_expr` is present, consumers must use it and ignore `offset`. Producers must write `offset_expr` and must also write `offset` whenever an exact legacy representation exists; otherwise, they must write only `offset_expr`. An exact legacy representation requires one fixed, strictly positive `int64` offset that is semantically equivalent for every input row; literal syntax is not required.

## Bound Modes

### ROWS

`BOUNDS_TYPE_ROWS` counts **rows** in the declared ordering:

* `CurrentRow` means only the current row, even when it has peers.
* `Preceding` and `Following` offset expressions must have type `int64` and evaluate to a non-negative value `n`. This specifies an inclusive frame boundary `n` rows before the current row for `Preceding`, or after it for `Following`.
* Without ordering, no row order is guaranteed. With ordering, the relative order of peers is still not guaranteed.

ROWS boundaries are mathematical row positions, independent of implementation integer width. Include only existing partition rows between them; out-of-partition boundaries neither wrap nor cause arithmetic-overflow errors. Errors evaluating `offset_expr`, including overflow, retain that expression's semantics.

In the example above, one preceding row means B and one following row means D when C is current and the displayed order is used. These offsets count individual rows, not groups of rows sharing a score.

At the last row of a partition, a frame from `Following(1)` through `Following(2)` is empty; clamping both boundaries to the last row would incorrectly include it.

### RANGE

`BOUNDS_TYPE_RANGE` measures **distances in ordering values**, rather than counting rows:

* `CurrentRow` means the first row of the current peer group for a lower bound and the last row of that group for an upper bound.
* If either bound is `Preceding` or `Following`, there must be exactly one ordering expression, and it must not use `SORT_DIRECTION_CLUSTERED` or a custom comparison function.
* An offset must be a non-negative distance with a [compatible type](#offset-type-compatibility).
* When the current row's ordering value is null, an offset bound is equivalent to `CurrentRow`.

The single-ordering-expression restriction does not apply when both bounds are `CurrentRow` or `Unbounded` (or omitted). With multiple ordering expressions, peer equality uses all of them.

A boundary value need not occur in the input. For example, with ascending scores, current score 20, and bounds `Preceding(7)` through `CurrentRow`, the frame includes scores from 13 through 20, including every peer at score 20.

Compatibility and frame membership are defined directly, without resolving scalar arithmetic or comparison functions.

#### Offset Type Compatibility

The valid core ordering/offset type pairs are listed below. Nullability does not affect compatibility; null values follow the [RANGE](#range) and [offset-expression](#offset-expressions) rules.

| Ordering type `T` | Offset type `D` | Interpretation |
| ----------------- | --------------- | -------------- |
| `i8`, `i16`, `i32`, or `i64` | The same integer type as `T` | Numeric distance |
| `fp32` or `fp64` | The same floating-point type as `T` | Numeric distance |
| `decimal<P1,S1>` | `decimal<P2,S2>` | Numeric distance; precision and scale need not match |
| `date` | `interval_day<P>` | Elapsed duration from midnight on the date |
| `precision_time<P>` | `interval_day<Q>` | Elapsed duration from the time, without wrapping at midnight |
| `precision_timestamp<P>` | `interval_day<Q>` | Elapsed duration on the timezone-free timeline |
| `precision_timestamp_tz<P>` | `interval_day<Q>` | Elapsed duration on the UTC timeline |
| `date` or `precision_timestamp<P>` | `interval_year` | Calendar-month distance |

Decimal precisions and scales, and temporal precisions, need not match between ordering and offset types. As defined in [Type Classes](../types/type_classes.md), `interval_day` measures elapsed duration (one day is 86,400 seconds, not a timezone-dependent calendar day), and `interval_year` measures months. Non-negativity depends on the total, not individual component signs.

Other core type pairs are invalid. Integer offsets do not implicitly mean days for `date`; mixed integer widths or integer/floating-point pairs require explicit producer conversion. `precision_timestamp_tz`/`interval_year` is unsupported because the frame cannot supply the required timezone. Consumers must not infer it from session state.

#### Boundary Values and Membership

For a `Preceding` or `Following` bound, the boundary calculation uses two operands: the current row's ordering value `v` and the evaluated offset `d`.

| Sort direction | `Preceding(d)` | `Following(d)` |
| -------------- | -------------- | -------------- |
| Ascending | `v - d` (lower values) | `v + d` (higher values) |
| Descending | `v + d` (higher values) | `v - d` (lower values) |

An operand is finite if it is non-null and, if floating-point, neither NaN nor positive or negative infinity. Nulls follow the [RANGE](#range) and [offset-expression](#offset-expressions) rules.

Numeric boundaries use mathematical operand values; duration boundaries use the elapsed durations above. Calendar-month offsets adjust the year and month, preserving the day and time of day, except that a nonexistent day becomes the month's last day. No session timezone participates.

Membership is inclusive: `lower <= value <= upper` for ascending ordering, or `upper <= value <= lower` for descending ordering. `CurrentRow`, `Unbounded`, peer groups, and null placement retain their meanings above; these inequalities are not nullable SQL predicates.

A boundary is not an output expression and need not be representable as `T`. Consumers may align type parameters, widen intermediates, or avoid materializing boundaries, provided membership is preserved, including under rounding or truncation to fit `T`. Decimal scalar return-type formulas do not restrict valid frames.

##### Floating-Point Boundaries

For finite floating-point operands, membership uses their exact mathematical values, even without overflow. Rounded or infinite intermediates are acceptable only if they preserve the required outcome; an exact boundary need not be materialized. This is a membership contract, not an alternative IEEE-754 scalar operation or rounding mode.

For example, with ascending `fp64` ordering at current value `1`, take a lower bound `2^-54` following and an unbounded upper bound. The exact lower boundary exceeds `1`, excluding the current peer group. Round-to-nearest addition produces `1`, incorrectly including those peers. This is a rounding difference, not overflow.

NaN or infinite ordering values and offsets remain implementation-defined; neither overflow behavior defines their arithmetic or ordering. Infinity produced from finite operands remains subject to the finite-operand rules.

#### Boundary Overflow Behavior

For finite operands, overflow means the mathematical boundary lies outside the declared ordering type `T`'s value range. Its minimum and maximum are Substrait's limits, including decimal precision/scale, temporal precision/ranges, and floating-point finite ranges, not native type, intermediate, or calendar-library limits.

Wider intermediates do not prevent overflow of `T`, nor do intermediate failures establish it. An in-range boundary is not an overflow merely because its precision/scale differs from `T` or it falls between representable values.

The optional `range_boundary_overflow` option controls both RANGE boundaries through either window invocation's existing `options` field. Its name is reserved for window invocations, independently of the referenced aggregate or window function's declared options.

| Value | Behavior |
| ----- | -------- |
| `ERROR` | Report an execution error if either applicable boundary overflows `T`, even if a wider intermediate can represent it. |
| `EXTEND` | Compare against the mathematical boundary outside `T`'s range without an overflow error, wrapping, or clamping. |

Names and values are case-insensitive. The option may appear at most once, with a non-empty preference list. Following [function option preferences](scalar_functions.md#option-preference), consumers must select the first behavior supported for the invocation's ordering/offset types, or reject the plan if the option is unknown or none is supported. Native fallbacks qualify only if they satisfy the requested guarantees.

Omission makes boundary-overflow behavior implementation-defined, without relaxing type compatibility, non-overflow membership, or invalid-offset rules. Producers requiring a particular behavior must specify it.

`EXTEND` neither makes a bound `Unbounded` nor overrides null placement. Depending on the bound and ordering direction, it can include all remaining values on one side or yield an empty frame.

The [floating-point boundary rules](#floating-point-boundaries) apply to both behaviors: producing infinity alone does not implement `EXTEND`.

The option is valid only for RANGE frames and has no effect on `CurrentRow` or `Unbounded`. It does not change `offset_expr` evaluation errors (including overflow), null/negative-offset validation, or the window function's own overflow semantics and options.

`ERROR` must report boundary overflow even if the other bound makes the frame empty. Bound evaluation order and the choice among multiple errors are unspecified.

##### Example: Overflowing Integer Boundaries

For one partition containing `i8` values 125, 126, and 127, compute `COUNT(*)` in ascending order with RANGE bounds from one to two units following. Both offsets are valid `i8` values, but some boundaries exceed 127:

| Current value | Mathematical lower boundary | Mathematical upper boundary | Frame members with `EXTEND` | Count with `EXTEND` | Outcome with `ERROR` |
| ------------- | --------------------------- | --------------------------- | --------------------------- | ------------------- | -------------------- |
| 125 | 126 | 127 | 126, 127 | 2 | 2 |
| 126 | 127 | 128 | 127 | 1 | Execution error |
| 127 | 128 | 129 | None | 0 | Execution error |

At current value 127, clamping both boundaries to 127 would incorrectly return 1 instead of 0.

Computing 128 or 129 in a wider integer type does not avoid `ERROR`, but can implement `EXTEND` comparisons.

This `PlanRel` selects `EXTEND`. The enclosing plan must bind function anchor 1 to the zero-argument `count:` in `extension:io.substrait:functions_aggregate_generic`.

```protobuf
--8<-- "examples/proto-textformat/plan_rel/range_boundary_overflow.textproto"
```

Changing the sole preference to `ERROR` makes execution fail on boundary calculation, not offset evaluation or count overflow. The table shows per-record checks, not guaranteed partial results.

## Function Signatures

Window function signatures inherit all properties of [aggregate functions](aggregate_functions.md) and add:

| Property | Description | Default |
| -------- | ----------- | ------- |
| Window Type | `STREAMING` or `PARTITION`: whether the function can produce results without seeing the entire partition. `SUM` can operate in a streaming manner; `NTILE` needs the entire partition. | `PARTITION` |

Window Type describes the function's data needs, not its bound mode. Aggregate functions such as `AVG`, `COUNT`, `MAX`, `MIN`, and `SUM` can be used as window functions with Window Type `STREAMING`.
