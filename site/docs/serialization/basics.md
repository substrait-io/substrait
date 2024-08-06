# Basics

Substrait is designed to be serialized into various different formats. Currently we support a binary serialization for
transmission of plans between programs (e.g. IPC or network communication) and a text serialization for debugging and human readability. Other formats may be added in the future.

These formats serialize a collection of plans. Substrait does not define how a collection of plans is to be interpreted.
For example, the following scenarios are all valid uses of a collection of plans:

- A query engine receives a plan and executes it. It receives a collection of plans with a single root plan. The
  top-level node of the root plan defines the output of the query. Non-root plans may be included as common subplans
  which are referenced from the root plan.
- A transpiler may convert plans from one dialect to another. It could take, as input, a single root plan. Then
  it could output a serialized binary containing multiple root plans. Each root plan is a representation of the
  input plan in a different dialect.
- A distributed scheduler might expect 1+ root plans. Each root plan describes a different stage of computation.

Libraries should make sure to thoroughly describe the way plan collections will be produced or consumed.

## Root plans

We often refer to query plans as a graph of nodes (typically a DAG unless the query is recursive). However, we
encode this graph as a collection of trees with a single root tree that references other trees (which may also
transitively reference other trees). Plan serializations all have some way to indicate which plan(s) are "root"
plans. Any plan that is not a root plan and is not referenced (directly or transitively) by some root plan
can safely be ignored.
