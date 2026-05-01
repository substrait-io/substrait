# Subqueries

Subqueries are scalar expressions comprised of another query.

## Forms

### Scalar

Scalar subqueries are subqueries that return one row and one column.

| Property | Description    | Required |
| -------- | -------------- | -------- |
| Input    | Input relation | Yes      |

### `IN` predicate

An `IN` subquery predicate checks that the left expression is contained in the
right subquery.

#### Examples

```sql
SELECT *
FROM t1
WHERE x IN (SELECT * FROM t2)
```

```sql
SELECT *
FROM t1
WHERE (x, y) IN (SELECT a, b FROM t2)
```

| Property | Description                                 | Required |
| -------- | ------------------------------------------- | -------- |
| Needles  | Expressions whose existence will be checked | Yes      |
| Haystack | Subquery to check                           | Yes      |

### Set predicates

A set predicate is a predicate over a set of rows in the form of a subquery.

`EXISTS` and `UNIQUE` are common SQL spellings of these kinds of predicates.

| Property  | Description                                | Required |
| --------- | ------------------------------------------ | -------- |
| Operation | The operation to perform over the set      | Yes      |
| Tuples    | Set of tuples to check using the operation | Yes      |

### Set comparisons

A set comparison subquery is a subquery comparison using `ANY` or `ALL` operations.

#### Examples

```sql
SELECT *
FROM t1
WHERE x < ANY(SELECT y from t2)
```

| Property              | Description                                    | Required |
| --------------------- | ---------------------------------------------- | -------- |
| Reduction operation   | The kind of reduction to use over the subquery | Yes      |
| Comparison operation  | The kind of comparison operation to use        | Yes      |
| Expression            | Left-hand side expression to check             | Yes      |
| Subquery              | Subquery to check                              | Yes      |



## Outer References in Subqueries

Subqueries may contain *outer references*, which are field references that reach
outside the subquery boundary to access records from an enclosing relation.
The `OuterReference` root type provides two resolution fields:

* `steps_out`: Resolves the reference by counting subquery boundaries
  upward. This works correctly when the plan is a tree (each relation has a
  single parent).

* `rel_reference`: Resolves the reference by naming the binding relation
  via its plan-wide unique `RelCommon.rel_anchor`. Must be used instead of
  `steps_out` when an outer reference appears inside a relation shared via

  `ReferenceRel` and that shared relation can be reached through multiple

  paths with different subquery depths, making `steps_out` ambiguous.


Exactly one of these fields must be set. See
[Field References — Outer References](field_references.md#outer-references)
for details.

=== "Protobuf Representation"

    ```proto
%%% proto.message.Expression.Subquery %%%
    ```
