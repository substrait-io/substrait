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

| Property | Description                               | Required |
| -------- | ----------------------------------------- | -------- |
| Needles  | Expressions who existence will be checked | Yes      |
| Haystack | Subquery to check                         | Yes      |

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
| Expression            | Left hand side expression to check             | Yes      |
| Subquery              | Subquery to check                              | Yes      |
