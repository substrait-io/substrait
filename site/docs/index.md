# Substrait

## Project Vision

Create a well-defined, cross-language [specification](spec/specification) for data compute operations. This includes a declaration of common operations, custom operations and one or more serialized representations of this specification. The spec focuses on the semantics of each operation and a consistent way to describe.

In many ways, the goal of this project is similar to that of the Apache Arrow project. Arrow is focused on a standardized memory representation of columnar data. Substrait is focused on what should be done to data.



## Example Use Cases

* Communicate a compute plan between a SQL parser and an execution engine (e.g. Calcite SQL parsing to Arrow C++ compute kernel)
* Serialize a plan that represents a view for consumption in multiple systems (e.g. Iceberg views in Spark and Trino)
* Create an alternative plan generation implementation that can connect an existing end-user compute expression system to an existing end-user processing engine (e.g. Pandas operations executed inside SingleStore)
* Build a pluggable plan visualization tool (e.g. D3 based plan visualizer)



## Community Principles

* Be inclusive and open to all. If you want to join the project, open a PR or issue or join the Slack channel (TBD)
* Ensure a diverse set of contributors that come from multiple data backgrounds to maximize general utility.
* Build a specification based on open consensus.
* Make the specification and all tools freely available on a permissive license (ApacheV2)
