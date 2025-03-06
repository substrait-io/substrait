---
title: Home
---


## What is Substrait?

Substrait is a format for describing compute operations on structured data. It is designed for interoperability across different languages and systems.



## How does it work?

Substrait provides a well-defined, cross-language [specification](spec/specification.md) for data compute operations.  This includes a consistent declaration of common operations, custom operations and one or more serialized representations of this specification.  The spec focuses on the semantics of each operation.  In addition to the specification the Substrait ecosystem also includes a number of libraries and [useful tools](tools/producer_tools.md).

We highly recommend the [tutorial](tutorial/sql_to_substrait.md) to learn how a Substrait plan is constructed.



## Benefits

* Avoids every system needing to create a communication method between every other system -- each system merely supports ingesting and producing Substrait and it instantly becomes a part of the greater ecosystem.
* Makes every part of the system upgradable.  There's a new query engine that's ten times faster?  Just plug it in!
* Enables heterogeneous environments -- run on a cluster of an unknown set of execution engines!
* The text version of the Substrait plan allows you to quickly see how a plan functions without needing a visualizer (although there are Substrait visualizers as well!).



## Example Use Cases

* Communicate a compute plan between a SQL parser and an execution engine (e.g. Calcite SQL parsing to Arrow C++ compute kernel)
* Serialize a plan that represents a SQL view for consistent use in multiple systems (e.g. Iceberg views in Spark and Trino)
* Submit a plan to different execution engines (e.g. Datafusion and Postgres) and get a consistent interpretation of the semantics.
* Create an alternative plan generation implementation that can connect an existing end-user compute expression system to an existing end-user processing engine (e.g. Pandas operations executed inside SingleStore)
* Build a pluggable plan visualization tool (e.g. D3 based plan visualizer)

