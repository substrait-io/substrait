---
title: FAQ
---

# Frequently Asked Questions

## What is the purpose of the post-join filter field on Join relations?

The post-join filter on the various Join relations is not always equivalent to an explicit Filter relation AFTER the Join.

See the example [here](https://facebookincubator.github.io/velox/develop/joins.html#hash-join-implementation) that highlights how the post-join filter behaves differently than a Filter relation in the case of a left join.

## Why does the project relation keep existing columns?

In several relational algebra systems ([DuckDB](https://duckdb.org/), [Velox](https://velox-lib.io/), [Apache Spark](https://spark.apache.org/), [Apache DataFusion](https://datafusion.apache.org/), etc.) the project relation is used both
to add new columns and remove existing columns. It is defined by a list of expressions and there is one output
column for each expression.

In Substrait, the project relation is only used to add new columns. Any relation can remove columns by using the
`emit` property in `RelCommon`. This is because it is very common for optimized plans to discard columns once they
are no longer needed and this can happen anywhere in a plan. If this discard required a project relation then
optimized plans would be cluttered with project relations that only remove columns.

As a result, Substrait's project relation is a little different. It is also defined by a list of expressions.
However, the output columns are a combination of the input columns and one column for each of the expressions.

## Where are field names represented?

Some relational algebra systems, such as Spark, give names to the output fields of a relation. For example, in
PySpark I might run `df.withColumn("num_chars", length("text")).filter("num_chars > 10")`. This creates a
project relation, which calculates a new field named `num_chars`. This field is then referenced in the filter
relation. Spark's logical plan maps closely to this and includes both the expression (`length("text")`) and the
name of the output field (`num_chars`) in its project relation.

Substrait does not name intermediate fields in a plan. This is because these field names have no effect on
the computation that must be performed. In addition, it opens the door to name-based references, which Substrait
also does not support, because these can be a source of errors and confusion. One of the goals of Substrait is
to make it very easy for consumers to understand plans. All references in Substrait are done with ordinals.

In order to allow plans that do use named fields to round-trip through Substrait there is a hint that can be
used to add field names to a plan. This hint is called `output_names` and is located in `RelCommon`. Consumers
should not rely on this hint being present in a plan but, if present, it can be used to provide field names to
intermediate relations in a plan for round-trip or debugging purposes.

There are a few places where Substrait DOES define field names:

- Read relations have field names in the base schema. This is because it is quite common for reads to do a
  name-based lookup to determine the columns that need to be read from source files.
- The root relation has field names. This is because the root relation is the final output of the plan and
  it is useful to have names for the fields in the final output.
