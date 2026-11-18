# Substrait → engine SQL: types and literals

These forms were checked against DuckDB 2.0, DataFusion 55, SQLite 3.54 and
Spark 4.2.
**Always cast every literal**: untyped integer literals are `INTEGER` in DuckDB
but `Int64` in DataFusion, so a bare `120` does not test i8 semantics.

## Types

| Substrait | DuckDB | DataFusion | SQLite | Spark |
|---|---|---|---|---|
| `i8` | `TINYINT` | `TINYINT` | `INTEGER` (always 64-bit) | `TINYINT` |
| `i16` | `SMALLINT` | `SMALLINT` | `INTEGER` | `SMALLINT` |
| `i32` | `INTEGER` | `INT` | `INTEGER` | `INT` |
| `i64` | `BIGINT` | `BIGINT` | `INTEGER` | `BIGINT` |
| `fp32` | `REAL` | `REAL` | `REAL` (always 64-bit; fp32 rounding is not testable) | `REAL` |
| `fp64` | `DOUBLE` | `DOUBLE` | `REAL` | `DOUBLE` |
| `dec<P,S>` | `DECIMAL(P,S)` | `DECIMAL(P,S)` | none (becomes float; skip unless the case is about the value only) | `DECIMAL(P,S)` |
| `bool` | `BOOLEAN` | `BOOLEAN` | `0`/`1` integers | `BOOLEAN` |
| `str`, `vchar<N>`, `fchar<N>` | `VARCHAR` | `VARCHAR` | `TEXT` | `STRING` |
| `vbin` | `X'AA'` | `X'AA'` | `X'AA'` | `X'AA'` |
| `date` | `DATE '2020-01-02'` | `DATE '2020-01-02'` | text via `DATE('2020-01-02')` | `DATE '2020-01-02'` |
| `pt<6>` | `TIME '12:00:00.5'` | `arrow_cast('12:00:00.5', 'Time64(Microsecond)')` | text | `TIME '12:00:00.5'` |
| `pts<6>` | `TIMESTAMP '…'` | `arrow_cast('…', 'Timestamp(Microsecond, None)')` | text | `TIMESTAMP_NTZ '…'` |
| `pts<3>` / `pts<9>` / `pts<0>` | `CAST(… AS TIMESTAMP_MS / TIMESTAMP_NS / TIMESTAMP_S)` | `arrow_cast(…, 'Timestamp(Millisecond/Nanosecond/Second, None)')` | text | none (microseconds only) |
| `ptstz<6>` | `TIMESTAMPTZ '…+05:30'` (normalized to UTC) | `arrow_cast('…+05:30', 'Timestamp(Microsecond, Some("UTC"))')` | text | `TIMESTAMP '…+05:30'` (normalized to UTC) |
| `iday` | `INTERVAL '2 days 3 hours'` | `INTERVAL '2 days 3 hours'` | none | `INTERVAL '2 days 3 hours'` |
| `list<T>` | `[1, 2]::INTEGER[]` | `make_array(CAST(1 AS INT), CAST(2 AS INT))` | none | `array(CAST(1 AS INT), CAST(2 AS INT))` |

Notes:

- **DataFusion's default precision is nanoseconds.** A plain `TIMESTAMP '…'` or
  `TIME '…'` there is `pts<9>` / `pt<9>`. Use `arrow_cast` to get other precisions.
- **SQLite has no static types.** The runner reports the type of the Python
  value it gets back (`int`→i64, `float`→fp64, `str`). A typed case is
  therefore at best `PASS_TYPE_DIFF` on SQLite. That's expected and not a
  finding.
- **Spark runs with its defaults, so ANSI mode is on.** `VARCHAR(N)` and
  `CHAR(N)` are read as `STRING`. Spark can't return year-month intervals to
  Python; wrap them in `CAST(… AS STRING)` and compare the text.
- **Some engine result types have no Substrait equivalent**, for example
  DuckDB `HUGEINT` (128-bit) or DataFusion unsigned integers. They are reported
  as "not checkable".

## Special values

| Value | DuckDB | DataFusion | SQLite | Spark |
|---|---|---|---|---|
| `inf` / `-inf` | `CAST('inf' AS DOUBLE)` | `CAST('inf' AS DOUBLE)` | `1e999` / `-1e999` | `CAST('inf' AS DOUBLE)` |
| `nan` | `CAST('nan' AS DOUBLE)` | `CAST('NaN' AS DOUBLE)` | none (`0.0/0.0` is NULL): skip | `CAST('NaN' AS DOUBLE)` |
| `-0` | `CAST('-0' AS DOUBLE)` | `CAST('-0' AS DOUBLE)` | `-0.0` | `CAST('-0' AS DOUBLE)` |
| `null::T?` | `CAST(NULL AS T)` | `CAST(NULL AS T)` | `NULL` | `CAST(NULL AS T)` |
| string escape `\'` | `''` | `''` | `''` | `''` |

## Query shapes

**Scalar**: `SELECT f(CAST(a AS T), CAST(b AS T))`.

**Aggregate**: write the rows untyped in one CTE, then cast them in a second.
Casting only the first `VALUES` row is not enough, because DataFusion widens
the column to `Int64`. SQLite does not accept `(VALUES …) AS v(c)`, but CTE
column lists work on all four engines:

```sql
WITH v(c0, c1) AS (VALUES (20, 20), (-3, NULL), (1, 1)),
     t(col0, col1) AS (SELECT CAST(c0 AS REAL), CAST(c1 AS REAL) FROM v)
SELECT corr(col0, col1) FROM t
```

Empty table:

```sql
WITH t(col0) AS (SELECT CAST(NULL AS SMALLINT) WHERE 1 = 0) SELECT sum(col0) FROM t
```

**Window**: add an ordering column, and order the outer query by it as well:

```sql
WITH v(ord, c0) AS (VALUES (1, 1998), (2, 1999), (3, 2000)),
     t(ord, col0) AS (SELECT ord, CAST(c0 AS INTEGER) FROM v)
SELECT lag(col0) OVER (ORDER BY ord ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING)
FROM t ORDER BY ord
```

Use `"expected": {"column": [...], "type": ...}` for these.

**Composition** (a function call on the right-hand side of `=`):

```sql
SELECT (lhs) IS NOT DISTINCT FROM (rhs)
```

Use `"expected": {"value": true, "type": "bool"}`. All four engines support
`IS NOT DISTINCT FROM`; on SQLite the result comes back as `1`.

## Expected-value encoding (cases JSON)

| Test file | `expected` |
|---|---|
| `= 125::i8` | `{"value": 125, "type": "i8"}` |
| `= 1.5::fp32` | `{"value": 1.5, "type": "fp32"}` (the runner rounds to fp32) |
| `= inf::fp64` / `nan` | `{"value": "inf", "type": "fp64"}` / `"nan"` |
| `= 12.30::dec<38,2>` | `{"value": "12.30", "type": "dec<38,2>"}` |
| `= 'abc'::str` | `{"value": "abc", "type": "str"}` |
| `= null::i64?` | `{"value": null, "type": "i64?"}` |
| `= '2020-01-02'::date` | `{"value": "2020-01-02", "type": "date"}` (ISO; times and timestamps too) |
| `= 'PT2H'::iday<6>` | `{"value": 7200, "type": "iday<6>"}` (**total seconds**) |
| `= [1, 2]::list<i32>` | `{"value": [1, 2], "type": "list<i32>"}` |
| `= <!ERROR>` | `{"error": true}` |
| `= <!UNDEFINED>` | `{"undefined": true}` |

Year-month intervals (`iyear`) can't be compared automatically. DuckDB turns
months into 30-day spans. Review those cases by hand.
