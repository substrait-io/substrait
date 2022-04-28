# Specification



## Process

The goal of this project is initially to establish a well-defined specification. Once established, new versions of the specification will follow a normal development/release process. To provide something to peruse while clarifying an openness to the community during the initial development of the specification, we plan to follow the following steps for development of the specification. We will use GitHub branches to describe each of these steps and patches will be proposed to be moved from one branch to the next to allow review of documents while still having strawmen to start with. The steps are:

1. **Empty** - No outline has been produced. A sketch needs to be produced for people to react and iterate on.
2. **Sketch** - Something has been written but should serve more as a conceptual backing for what should be achieved in this part of the specification. No collaboration or consensus has occurred. This will be discussed and iterated on until an initial WIP version can be patched. The WIP version will be held in a PR to iterate on until it is committed to the WIP branch of the repository.
3. **WIP** - An initial version that multiple contributors have agreed to has been produced for this portion of the specification. Any user is welcome to propose additional changes or discussions regarding this component, but it now represents a community intention.
4. **Commit** - Believed to be a well-formed plan for this portion of the specification. Documents that have had no outstanding reviews for 14 days will be moved from WIP to commit. Changes can still be made, but the section should no longer be under constant revision. (This status is more for external observers to understand the progress of the specification than something that influences internal project process.)

Once all portions of the specification have been moved to commit (or eliminated), the specification will move to an initial version number. To try to get a working end-to-end model as quickly as possible, a small number of items have been prioritized. The set of components outlined here are proposed as a mechanism for having bite-size review/discussion chunks to make forward progress.



## Components

| Priority | Status | Section                                                      | Description                                                  |
| -------- | ------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| 1        | wip    | [Simple Types](/types/simple_logical_types)                  | A way to describe the set of basic types that will be operated on within a plan. Only includes simple types such as integers and doubles (nothing configurable or compound). |
|          | wip    | [Compound Types](/types/compound_logical_types)              | Expression of types that go beyond simple scalar values. Key concepts here include: configurable types such as fixed length and numeric types as well as compound types such as structs, maps, lists, etc. |
|          | wip    | [Type Variations](/types/type_variations)                    | Physical variations to base types.                           |
|          | sketch | [User Defined Types](/types/user_defined_types)              | Extensions that can be defined for specific IR producers/consumers. |
| 2        | sketch | [Field References](/expressions/field_references)            | Expressions to identify which portions of a record should be operated on.     |
| 3        | sketch | [Scalar Functions](/expressions/scalar_functions)            | Description of how functions are specified. Concepts include arguments, variadic functions, output type derivation, etc. |
|          | sketch | [Scalar Function List](https://github.com/substrait-io/substrait/blob/main/extensions/scalar_functions.yaml) | A list of well-known canonical functions in YAML format.     |
|          | sketch | [Specialized Record Expressions](/expressions/specialized_record_expressions) | Specialized expression types that are more naturally expressed outside the function paradigm. Examples include items such as if/then/else and switch statements. |
|          | sketch | [Aggregate Functions](/expressions/aggregate_functions)      | Functions that are expressed in aggregation operations. Examples include things such as SUM, COUNT, etc. Operations take many records and collapse them into a single (possibly compound) value. |
|          | sketch | [Window Functions](/expressions/window_functions)            | Functions that relate a record to a set of encompassing records. Examples in SQL include RANK, NTILE, etc. |
|          | empty  | [Table Functions](/expressions/table_functions)              | Functions that convert one or more values from an input record into 0..N output records. Example include operations such as explode, pos-explode, etc. |
|          | sketch | [User Defined Functions](/expressions/user_defined_functions) | Reusable named functions that are built beyond the core specification. Implementations are typically registered thorough external means (drop a file in a directory, send a special command with implementation, etc.) |
|          | sketch | [Embedded Functions](/expressions/embedded_functions)        | Functions implementations embedded directly within the plan. Frequently used in data science workflows where business logic is interspersed with standard operations. |
| 4        | sketch | [Relation Basics](/relations/basics)                         | Basic concepts around relational algebra, record emit and properties. |
|          | sketch | [Logical Relations](/relations/logical_relations)            | Common relational operations used in compute plans including project, join, aggregation, etc. |
|          | sketch | [Physical Relations](/relations/physical_relations)          | Specific execution sub-variations of common relational operations that describe have multiple unique physical variants associated with a single logical operation. Examples include hash join, merge join, nested loop join, etc. |
|          | empty  | [User Defined Relations](/relations/user_defined_relations)  | Installed and reusable relational operations customized to a particular platform. |
|          | empty  | [Embedded Relations](/relations/embedded_relations)          | Relational operations where plans contain the "machine code" to directly execute the necessary operations. |
| 5        | sketch | [Text Serialization](/serialization/text_serialization)      | A human producible & consumable representation of the plan specification. |
| 6        | sketch | [Binary Serialization](/serialization/binary_serialization)  | A high performance & compact binary representation of the plan specification. |

