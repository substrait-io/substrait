# Specification



## Status

The specification has passed the initial design phase and is now in the final stages of being fleshed out.  The community is encouraged to identify (and address) any perceived gaps in functionality using GitHub issues and PRs.  Once all of the planned implementations have been completed all deprecated fields will be eliminated and version 1.0 will be released.


## Components (Complete)

| Section                                                      | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [Simple Types](../types/type_classes.md#simple-types)             | A way to describe the set of basic types that will be operated on within a plan. Only includes simple types such as integers and doubles (nothing configurable or compound). |
| [Compound Types](../types/type_classes.md#compound-types)         | Expression of types that go beyond simple scalar values. Key concepts here include: configurable types such as fixed length and numeric types as well as compound types such as structs, maps, lists, etc. |
| [Type Variations](../types/type_variations.md)                    | Physical variations to base types.                           |
| [User Defined Types](../types/type_classes.md#user-defined-types) | Extensions that can be defined for specific IR producers/consumers. |
| [Field References](../expressions/field_references.md)            | Expressions to identify which portions of a record should be operated on.     |
| [Scalar Functions](../expressions/scalar_functions.md)            | Description of how functions are specified. Concepts include arguments, variadic functions, output type derivation, etc. |
| [Scalar Function List](https://github.com/substrait-io/substrait/blob/main/extensions/scalar_functions.yaml) | A list of well-known canonical functions in YAML format.     |
| [Specialized Record Expressions](../expressions/specialized_record_expressions.md) | Specialized expression types that are more naturally expressed outside the function paradigm. Examples include items such as if/then/else and switch statements. |
| [Aggregate Functions](../expressions/aggregate_functions.md)      | Functions that are expressed in aggregation operations. Examples include things such as SUM, COUNT, etc. Operations take many records and collapse them into a single (possibly compound) value. |
| [Window Functions](../expressions/window_functions.md)            | Functions that relate a record to a set of encompassing records. Examples in SQL include RANK, NTILE, etc. |
| [User Defined Functions](../expressions/user_defined_functions.md) | Reusable named functions that are built beyond the core specification. Implementations are typically registered thorough external means (drop a file in a directory, send a special command with implementation, etc.) |
| [Embedded Functions](../expressions/embedded_functions.md)        | Functions implementations embedded directly within the plan. Frequently used in data science workflows where business logic is interspersed with standard operations. |
| [Relation Basics](../relations/basics.md)                         | Basic concepts around relational algebra, record emit and properties. |
| [Logical Relations](../relations/logical_relations.md)            | Common relational operations used in compute plans including project, join, aggregation, etc. |
| [Text Serialization](../serialization/text_serialization.md)      | A human producible & consumable representation of the plan specification. |
| [Binary Serialization](../serialization/binary_serialization.md)  | A high performance & compact binary representation of the plan specification. |


## Components (Designed but not Implemented)

| Section                                                      | Description                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [Table Functions](../expressions/table_functions.md)              | Functions that convert one or more values from an input record into 0..N output records. Example include operations such as explode, pos-explode, etc. |
| [User Defined Relations](../relations/user_defined_relations.md)  | Installed and reusable relational operations customized to a particular platform. |
| [Embedded Relations](../relations/embedded_relations.md)          | Relational operations where plans contain the "machine code" to directly execute the necessary operations. |
| [Physical Relations](../relations/physical_relations.md)          | Specific execution sub-variations of common relational operations that describe have multiple unique physical variants associated with a single logical operation. Examples include hash join, merge join, nested loop join, etc. |
