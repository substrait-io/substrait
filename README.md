# CommonPlan



## Project Vision

Define a well-defined, cross-language specification of data compute operations. This includes a declaration of common operations, custom operations and one or more serialized representations of this specification. The spec focuses on the semantics of each operation and a consistent way to describe.

In many ways, the goal of this project is similar to that of the Apache Arrow project. Arrow is focused on a standardized memory representation of columnar data. CommonPlan is focused on what should be done to data.



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



## Technology Principles

* Provide a good suite of well-specified common functionality in databases and data science applications.
* Make it easy for users to privately or publically extend the representation to support specialized/custom operations.
* Produce something that is language agnostic and requires minimal work to start developing against in a new language.
* Drive towards a common format that avoids specialization for single favorite producer or consumer.
* Establish clear delineation between specifications that MUST be respected to and those that can be optionally ignored.
* Establish a forgiving compatibility approach and versioning scheme that supports cross-version compatibility in maximum number of cases.
* Minimize the need for consumer intelligence by excluding concepts like overloading, type coercion, implicit casting, field name handling, etc. (Note: this is weak and should be better stated.)
* Decomposability/severability: A particular producer or consumer should be able to produce or consume only a subset of the specification and interact well with any other CommonPlan system as long the specific operations requested fit within the subset of specification supported by the counter system.



## Development Process

The goal of this project is initially to establish a well-defined specification. Once established, new versions of the specification will follow a normal development/release process. To provide something to peruse while clarifying an openness to the community during the initial development of the specification, we plan to follow the following steps for development of the specification. We will use github branches to describe each of these steps and patches will be proposed to be moved from one branch to the next to allow review of documents while still having strawmen to start with. The steps are: 

1. **Sketch** - Something has been written but should serve more as a conceptual backing for what should be achieved in this part of the specification. No collaboration or consensus has occurred. This will be discussed and iterated on until an initial WIP version can be patched. The WIP version will be held in a PR to iterate on until it is committed to the WIP branch of the repository.
2. **WIP** - An initial version that multiple contributors have agreed to has been produced for this portion of the specification. Any user is welcome to propose additional changes or discussions with regards to this component but it now represents a community intention.
3. **Commit** - Believed to be a well-formed plan for this portion of the specification. Documents that have had no outstanding reviews for 14 days will be moved from WIP to commit. Changes can still be made but the section should no longer be under constant revision. (This status is more for external observers to understand the progress of the specification than something that influences internal project process.)

Once all portions of the specification have been moved to commit (or eliminated), the specification will move to an initial version number. To try to get a working end-to-end model as quickly as possible, a small number of items have been prioritized. The set of components outlined here are proposed as a mechanism for having bite-size review/discussion chunks to make forward progress.



## Specification Components

| Priority | Status | Section                            | Description                                                  |
| -------- | ------ | ---------------------------------- | ------------------------------------------------------------ |
| 1        | sketch | Basic Types                        | A way to describe the set of basic types that will be operated on within a plan. Only includes simple types such as integers and doubles (nothing configurable or compound). |
|          | sketch | Advanced Types                     | Expression of types that go beyond simple scalar values. Key concepts here include: configurable types such as fixed length and numeric types as well as compound types such as structs, maps, lists, etc. |
| 2        | sketch | Field References                   | Expressions to identify which portions of a record should be |
| 3        | sketch | Function Signatures                | Description of how functions are specified. Concepts include arguments, variadic, output type derivation, etc. |
|          | sketch | Common Functions                   | A list of well-known canonical functions.                    |
|          | sketch | Specialized Record Expressions     | Specialized expression types that are more naturally expressed outside the function paradigm. Examples include items such as if/then/else and switch 'value' statements. |
|          | sketch | Aggregate Functions                | Functions that are expressed in aggregation operations. Examples include things such as SUM, COUNT, etc. Operations take many records and collapse them into a single (possibly compound) value. |
|          | sketch | Window Functions                   | Functions that relate a record to a set of encompassing records. Examples in SQL include RANK, NTILE, etc. |
|          | sketch | Table Functions                    | Functions that convert one or more values from an input record into a 0..N output records. Example include operations such as explode, pos-exlode, etc. |
|          | sketch | User Defined Functions             | Reusable named functions that are built beyond the core specification. Implementations are typically registered thorugh external means (drop a file in a directory, send a special command with implementation, etc). |
|          | sketch | Embedded Functions                 | Functions implementations embedded directly within the plan. Frequently used in data scicence workflows where business logic is interpersed with standard operations. |
| 4        | sketch | Relational Basics                  | Basic concepts around relational algebra and the two basic relational operations: read & filter. |
|          | sketch | Logical Operations                 | Common relational operations used in compute plans including project, join, aggregation, etc. |
|          | sketch | Physical Relational Operations     | Specific execution sub-variations of common relational operations that describe how an operation should be down. Examples include hash join, merge join, nested loop join, etc. |
|          | sketch | Exchange Operations                | Operations associated with distributing work between multiple systems. Example might include hashed send, unordered reception, etc. |
|          | sketch | User Defined Relational Operations | Installed and reusable relational operations customized to a particular platform. |
|          | sketch | Embedded Relational Operations     | Relational operations where plans contain the "machine" code to directly execute. |
| 5        | sketch | Text Serialization                 | A human producable & consumable representation of the plan specification. |
| 6        | sketch | Binary Serialization               | A high performance & compact binary representation of the plan specification. |


