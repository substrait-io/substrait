# Physical Relations

There is no true distinction between logical and physical operations in Substrait. By convention, certain operations are classified as physical but all operations can be potentially used in any kind of plan. A particular set of transformations or target operators may (by convention) be considered the "physical plan" but this is a characteristic of the system consuming substrait as opposed to a definition within Substrait.



## Hash Equijoin Operator

The hash equijoin join operator will build a hash table out of the right input based on a set of join keys. It will then probe that hash table for incoming inputs, finding matches. 

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution is maintained. Orderedness of the left set is maintained in INNER join cases, otherwise it is eliminated. |
| Direct Output Order  | Same as the [Join](logical_relations.md#join-operator) operator. |

### Hash Equijoin Properties

| Property            | Description                                                  | Required                 |
| ------------------- | ------------------------------------------------------------ | ------------------------ |
| Left Input          | A relational input.                                          | Required                 |
| Right Input         | A relational input.                                          | Required                 |
| Join Expression     | A boolean condition that describes whether each record from the left set "match" the record from the right set. The condition must only include the following operations: AND, ==, field references, is not distinct from. Field references correspond to the direct output order of the data. | Required.                |
| Post Join Predicate | An additional expression that can be used to reduce the output of the join operation post the equality condition. Minimizes the overhead of secondary join conditions that cannot be evaluated using the equijoin keys. | Optional, defaults true. |
| Join Type           | One of the join types defined in the Join operator.          | Required                 |



## NLJ Operator

The nested loop join operator does a join by holding the entire right input and then iterating over it using the left input, evaluating the join expression on the cartesian product of all rows, only outputting rows where the expression is true. Will also include non-matching rows in the OUTER, LEFT and RIGHT operations per the join type requirements.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution is maintained. Orderedness is eliminated.       |
| Direct Output Order  | Same as the [Join](logical_relations.md#join-operator) operator. |

### NLJ Properties

| Property        | Description                                                  | Required                                       |
| --------------- | ------------------------------------------------------------ | ---------------------------------------------- |
| Left Input      | A relational input.                                          | Required                                       |
| Right Input     | A relational input.                                          | Required                                       |
| Join Expression | A boolean condition that describes whether each record from the left set "match" the record from the right set. | Optional. Defaults to true (a cartesian join). |
| Join Type       | One of the join types defined in the Join operator.          | Required                                       |



## Merge Equijoin Operator

The merge equijoin does a join by taking advantage of two sets that are sorted on the join keys. This allows the join operation to be done in a streaming fashion.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution is maintained. Orderedness is eliminated.       |
| Direct Output Order  | Same as the [Join](logical_relations.md#join-operator) operator. |

### Merge Join Properties

| Property            | Description                                                  | Required                                      |
| ------------------- | ------------------------------------------------------------ | --------------------------------------------- |
| Left Input          | A relational input.                                          | Required                                      |
| Right Input         | A relational input.                                          | Required                                      |
| Join Expression     | A boolean condition that describes whether each record from the left set "match" the record from the right set. The condition must only include the following operations: AND, ==, field references, is not distinct from. Field references correspond to the direct output order of the data. | Optional. Defaults to tue (a cartesian join). |
| Post Join Predicate | An additional expression that can be used to reduce the output of the join operation post the equality condition. Minimizes the overhead of secondary join conditions that cannot be evaluated using the equijoin keys. | Optional, defaults true.                      |
| Join Type           | One of the join types defined in the Join operator.          | Required                                      |

## Distribute Operator

The distribute operator will redistribute data based on zero or more distribution expressions. Applying this operation will lead to an output that presents the desired distribution.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Orderedness is maintained. Distribution is overwritten based on configuration. |
| Direct Output Order  | Order of the input.                                          |

### Distribute Properties

| Property           | Description                                                  | Required                                                     |
| ------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| Input              | The relational input                                         | Required.                                                    |
| Expressions        | A list of expressions that describe how the data should be distributed. | Optional. If undefined, data is expected to be distributed fairly evenly amongst destinations. |
| Partition Count    | The number of partitions targeted for output                 | Optional, defaults to the number of discrete values produced by the expressions. |
| Expression Mapping | A set of distribution expression tuples that are mapped to particular destinations. | Optional, expressions of the same type are expected to be mapped to the destination given a consistent number of target partitions. |



## Merging Capture

A receiving operation that will merge multiple ordered streams to maintain orderedness.

| Signature            | Value                                        |
| -------------------- | -------------------------------------------- |
| Inputs               | 1                                            |
| Outputs              | 1                                            |
| Property Maintenance | Orderedness and distribution are maintained. |
| Direct Output Order  | Order of the input.                          |

### Merging Capture Properties

| Property | Description                                                  | Required                    |
| -------- | ------------------------------------------------------------ | --------------------------- |
| Blocking | Whether the merging should block incoming data. Blocking should be used carefully, based on whether a deadlock can be produced. | Optional, defaults to false |



## Simple Capture

A receiving operation that will merge multiple streams in an arbitrary order.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Orderness is empty after this operation. Distribution are maintained. |
| Direct Output Order  | Order of the input.                                          |

### Naive Capture Properties

| Property | Description          | Required |
| -------- | -------------------- | -------- |
| Input    | The relational input | Required |



## TopN Operation

The topn operator reorders a dataset based on one or more identified sort fields as well as a sorting function. Rather than sort the entire dataset, the top-n will only maintain the total number of records required to ensure a limited output. A top-n is a combination of a logical sort and logical fetch operations.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Will update orderedness property to the output of the sort operation. Distribution property only remapped based on emit. |
| Direct Output Order  | The field order of the input.                                |

### TopN Properties

| Property    | Description                                                  | Required                 |
| ----------- | ------------------------------------------------------------ | ------------------------ |
| Input       | The relational input                                         | Required                 |
| Sort Fields | List of one or more fields to sort by. Uses the same properties as the [orderedness](basics.md#orderedness) property. | One sort field required  |
| Offset      | A positive integer. Declares the offset for retrieval of records. | Optional, defaults to 0. |
| Count       | A positive integer. Declares the number of records that should be returned. | Required                 |



## Hash Aggregate Operation

The hash aggregate operation maintains a hash table for each grouping set to coalesce equivalent tuples.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution if all distribution fields are contained in every grouping set. No orderness guaranteed. |
| Direct Output Order  | Same as defined by [Aggregate](logical_relations.md#aggregate-operation) operation. |

### Hash Aggregate Properties

| Property         | Description                                                  | Required                                |
| ---------------- | ------------------------------------------------------------ | --------------------------------------- |
| Input            | The relational input                                         | Required                                |
| Grouping Sets    | One or more grouping sets                                    | Optional, required if no measures.      |
| Per Grouping Set | A list of expression grouping that the aggregation measured should be calculated for. | Optional, defaults to 0.                |
| Measures         | A list of one or more aggregate expressions. Implementations may or may not support aggregate ordering expressions. | Optional, required if no grouping sets. |



## Streaming Aggregate Operation

The streaming aggregate operation leverages data ordered by the grouping expressions to calculate data each grouping set tuple-by-tuple in streaming fashion. All grouping sets and orderings requested on each aggregate must be compatible to allow multiple grouping sets or aggregate orderings.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution if all distribution fields are contained in every grouping set. Maintains input ordering. |
| Direct Output Order  | Same as defined by [Aggregate](logical_relations.md#aggregate-operation) operation. |

### Streaming Aggregate Properties

| Property         | Description                                                  | Required                                |
| ---------------- | ------------------------------------------------------------ | --------------------------------------- |
| Input            | The relational input                                         | Required                                |
| Grouping Sets    | One or more grouping sets. If multiple grouping sets are declared, sets must all be compatible with the the input sortedness. | Optional, required if no measures.      |
| Per Grouping Set | A list of expression grouping that the aggregation measured should be calculated for. | Optional, defaults to 0.                |
| Measures         | A list of one or more aggregate expressions. Aggregate expressions ordering requirements must be compatible with expected ordering. | Optional, required if no grouping sets. |



## Hashing Window Operation

A window aggregate operation that will build hash tables for each distinct partition expression.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution. Eliminates ordering.                 |
| Direct Output Order  | Same as Project operator (input followed by each window expression). |

### Hashing Window Properties

| Property           | Description                    | Required               |
| ------------------ | ------------------------------ | ---------------------- |
| Input              | The relational input           | Required               |
| Window Expressions | One or more window expressions | At least one required. |



## Streaming Window Operation

A window aggregate operation that relies on a partition/ordering sorted input.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution. Eliminates ordering.                 |
| Direct Output Order  | Same as Project operator (input followed by each window expression). |

### Streaming Window Properties

| Property           | Description                                                  | Required               |
| ------------------ | ------------------------------------------------------------ | ---------------------- |
| Input              | The relational input                                         | Required               |
| Window Expressions | One or more window expressions. Must be supported by the sorteness of the input. | At least one required. |

