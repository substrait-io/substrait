# Basics

Substrait is designed to allow a user to construct an arbitrarily complex data transformation plan. The plan is composed of one or more relational operations. Relational operations are well-defined transformation operations that work by taking zero or more input datasets and transforming them into zero or more output transformations. Substrait defines a core set of transformations and users are also able to extend the operations with their own specialized operations. 

Each relational operation is composed of several different properties. Common properties for relational operations include the following:

| Property   | Description                                                  | Type               |
| ---------- | ------------------------------------------------------------ | ------------------ |
| Emit       | The set of columns output from this operation and the order of those columns. | Logical & Physical |
| Hints      | A set of optionally provided, optionally consumed information about an operation that better inform execution. These might include estimated number of input and output records, estimated record size, likely filter reduction, estimated dictionary size, etc. These can also include implementation specific pieces of execution information. | Physical           |
| Constraint | A set of runtime constraints around the operation, limiting its consumption based on irl resources (CPU, memory) as well as virtual resources like number of records produced, largest record size, etc. | Physical           |



## Relational Signatures

In functions, function signatures are declared externally to the use of those signatures (function bindings). In the case of relational operations, signatures are declared directly in the specification. This is due to the speed of change and number of total operations. Relational operations in the specification are expected to be &lt;100 for several years with additions being infrequent. On the other hand, there is an expectation of both a much larger number of functions (1,000s) and a much higher velocity of additions.

Each Relational Operation must declare the following:

* Transformation logic around properties of the data. For example, does a relational operation maintain sortedness of a field? Does an operation change the distribution of data? 
* How many input sets does an operation produce? 
* Does the operator produce an output (by specification, we limit relational operations to a single output at this time)
* What is the schema and field ordering of an output (see emit below)?

### Emit: Output Ordering

A relational operation uses field references to access specific fields of the input stream. Field references are always ordinal based on the order of the incoming streams. Each relational operation must declare the order of its output data. To simplify things, each relational operation can be in one of two modes: 

1. **Direct output**: The order of outputs is based on the definition declared by the relational operation. 
2. **Remap**: A listed ordering of the direct outputs. This remapping can be also used to drop columns no longer used (such as a filter field or join keys after a join). Note that remapping/exclusion can only be done at the outputs root struct. Filtering of compound values or extracting subsets must be done through other operation types (e.g. projection).

## Relation Properties

There are number of predefined properties that exist in Substrait relations. These include:

### Distribution

When data is partitioned across multiple sibling sets, distribution describes that set of properties that apply to any one partition. This is based on a set of distribution expression properties. A distribution is declared as a set of one or more fields and a distribution type across all fields.

| Property            | Description                                                  | Required                                                     |
| ------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| Distribution Fields | List of fields references that describe distribution (e.g. [0,2:4,5:0:0]). The order of these references do not impact results. | Required for Partitioned distribution type. Disallowed for Singleton distribution type. |
| Distribution Type   | PARTITIONED: For a discrete tuple of values for the declared distribution fields, all records with that tuple are located in the same partition. SINGLETON: there will only be a single partition for this operation. | Required                                                     |



### Orderedness

A guarantee that data output from this operation is provided with a sort order. The sort order will be declared based on a set of sort field definitions based on the emitted output of this operation.

| Property              | Description                                                  | Required               |
| --------------------- | ------------------------------------------------------------ | ---------------------- |
| Sort Fields           | A list of fields that the data are ordered by. The list is in order of the sort. If sort by [0,1] this means we only consider the data for field 1 is only ordered within each discrete value of field 0. | At least one required. |
| Per - Sort Field      | A field reference that the data is sorted by.                | Required               |
| Per - Sort Direction  | The direction of the data. See direction options below.      | Required               |

#### Ordering Directions

| Direction                  | Descriptions                                                 | Nulls Position                                  |
| -------------------------- | ------------------------------------------------------------ | ----------------------------------------------- |
| Ascending                  | Returns data in a ascending order based on the quality function associated with the type. Nulls are included before any values. | First                                           |
| Descending                 | Returns data in a descending order based on the quality function associated with the type. Nulls are included before any values. | First                                           |
| Ascending                  | Returns data in a ascending order based on the quality function associated with the type. Nulls are included after any values. | Last                                            |
| Descending                 | Returns data in a descending order based on the quality function associated with the type. Nulls are included after any values. | Last                                            |
| Custom function identifier | Returns data using a custom function that returns -1, 0, or 1 depending on the order of the data. | Per Function                                    |
| Clustered                  | Ensures that all equal values are coalesced (but no ordering betwixt values is defined). E.g, for values 1,2,3,1,2,3, output could be any of the following 1,1,2,2,3,3 or 2,2,3,3,1,1 or 3,3,2,2,1,1 or 3,3,1,1,2,2. | N/A, may appear anywhere but will be coalesced. |





## Discussion Points

* Do we try to make read definition types more extensible ala function signatures? Is that necessary if we have a custom relational operator?
* How do we express decomposed types. For example, the Iceberg type above is for early logical planning. Once we do some operations, it may produce a list of Iceberg file reads. This is likely a secondary type of object.
* We currently include a generic properties property on read type. Do we want this dumping ground? 

