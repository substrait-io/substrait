# Logical Relations



## Read Operator

The read operator is an operator that produces one output. A simple example would be the reading of a Parquet file. It is expected that many types of reads will be added over time.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 0                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | N/A (no inputs)                                              |
| Direct Output Order  | Defaults to the schema of the data read after the optional projection (masked complex expression) is applied. |

### Read Properties

| Property          | Description                                                  | Required                             |
| ----------------- | ------------------------------------------------------------ | ------------------------------------ |
| Definition        | The contents of the read property definition.                | Required                             |
| Direct Schema     | Defines the schema of the output of the read (before any projection or emit remapping/hiding). | Required                             |
| Filter            | A boolean Substrait expression that describes the filter of an iceberg dataset. TBD: define how field referencing works. | Optional, defaults to none.          |
| Projection        | A masked complex expression describing the portions of the content that should be read | Optional, defaults to all of schema  |
| Output properties | Declaration of orderedness and/or distribution properties this read produces. | Optional, defaults to no properties. |
| Properties        | A list of name/value pairs associated with the read.         | Optional, defaults to empty          |

### Read Definition Types

Read definition types are built by the community and added to the specification. This is a portion of specification that is expected to grow rapidly.

#### Virtual Table

| Property | Description | Required |
| -------- | ----------- | -------- |
| Data     | Required    | Required |



#### Files Type

| Property                    | Description                                                       | Required |
| --------------------------- | ----------------------------------------------------------------- | -------- |
| Items                       | An array of Items (path or path glob) associated with the read.   | Required |
| Format per item             | Enumeration of available formats. Only current option is PARQUET. | Required |
| Slicing parameters per item | Information to use when reading a slice of a file.                | Optional |

##### Slicing Files

A read operation is allowed to only read part of a file. This is convenient, for example, when distributing
a read operation across several nodes. The slicing parameters are specified as byte offsets
into the file.

Many file formats consist of indivisible "chunks" of data (e.g. Parquet row groups). If this
happens the consumer can determine which slice a particular chunk belongs to. For example, one
possible approach is that a chunk should only be read if the midpoint of the chunk (dividing by
2 and rounding down) is contained within the asked-for byte range.

=== "ReadRel Message"

    ```proto
%%% proto.algebra.ReadRel %%%
    ```


## Filter Operation

The filter operator eliminates one or more records from the input data based on a boolean filter expression.

| Signature            | Value                                       |
| -------------------- | ------------------------------------------- |
| Inputs               | 1                                           |
| Outputs              | 1                                           |
| Property Maintenance | Orderedness, Distribution, remapped by emit |
| Direct Output Order  | The field order as the input.               |



### Filter Properties

| Property   | Description                                                  | Required |
| ---------- | ------------------------------------------------------------ | -------- |
| Input      | The relational input.                                        | Required |
| Expression | A boolean expression which describes which records are included/excluded. | Required |


=== "FilterRel Message"

    ```proto
%%% proto.algebra.FilterRel %%%
    ```


## Sort Operation

The sort operator reorders a dataset based on one or more identified sort fields and a sorting function for each.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Will update orderedness property to the output of the sort operation. Distribution property only remapped based on emit. |
| Direct Output Order  | The field order of the input.                                |



### Sort Properties

| Property    | Description                                                  | Required                |
| ----------- | ------------------------------------------------------------ | ----------------------- |
| Input       | The relational input.                                        | Required                |
| Sort Fields | List of one or more fields to sort by. Uses the same properties as the [orderedness](basics.md#orderedness) property. | One sort field required |

=== "SortRel Message"

    ```proto
%%% proto.algebra.SortRel %%%
    ```


## Project Operation

The project operation will produce one or more additional expressions based on the inputs of the dataset.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution maintained, mapped by emit. Orderedness: Maintained if no window operations. Extended to include projection fields if fields are direct references. If window operations are present, no orderedness is maintained. |
| Direct Output Order  | The field order of the input + the list of new expressions in the order they are declared in the expressions list. |

### Project Properties

| Property    | Description                                          | Required                         |
| ----------- | ---------------------------------------------------- | -------------------------------- |
| Input       | The relational input.                                | Required                         |
| Expressions | List of one or more expressions to add to the input. | At least one expression required |

=== "ProjectRel Message"

    ```proto
%%% proto.algebra.ProjectRel %%%
    ```


## Cross Product Operation

The cross product operation will combine two separate inputs into a single output. It pairs every record from the left input with every record of the right input.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution is maintained. Orderedness is empty post operation. |
| Direct Output Order  | The emit order of the left input followed by the emit order of the right input. |

### Cross Product Properties

| Property        | Description                                                  | Required                           |
| --------------- | ------------------------------------------------------------ | ---------------------------------- |
| Left Input      | A relational input.                                          | Required                           |
| Right Input     | A relational input.                                          | Required                           |


=== "CrossRel Message"

    ```proto
%%% proto.algebra.CrossRel %%%
    ```


## Join Operation

The join operation will combine two separate inputs into a single output, based on a join expression. A common subtype of joins is an equality join where the join expression is constrained to a list of equality (or equality + null equality) conditions between the two inputs of the join.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution is maintained. Orderedness is empty post operation. Physical relations may provide better property maintenance. |
| Direct Output Order  | The emit order of the left input followed by the emit order of the right input. |

### Join Properties

| Property        | Description                                                  | Required                           |
| --------------- | ------------------------------------------------------------ | ---------------------------------- |
| Left Input      | A relational input.                                          | Required                           |
| Right Input     | A relational input.                                          | Required                           |
| Join Expression | A boolean condition that describes whether each record from the left set "match" the record from the right set. Field references correspond to the direct output order of the data. | Required. Can be the literal True. |
| Join Type       | One of the join types defined below.                         | Required                           |

### Join Types

| Type  | Description                                                  |
| ----- | ------------------------------------------------------------ |
| Inner | Return records from the left side only if they match the right side. Return records from the right side only when they match the left side. For each cross input match, return a record including the data from both sides. Non-matching records are ignored. |
| Outer | Return all records from both the left and right inputs. For each cross input match, return a record including the data from both sides. For any remaining non-match records, return the record from the corresponding input along with nulls for the opposite input. |
| Left  | Return all records from the left input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the left input, return the left record along with nulls for the right input. |
| Right | Return all records from the right input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the right input, return the left record along with nulls for the right input. |
| Semi | Returns records from the left input. These are returned only if the records have a join partner on the right side. |
| Anti  | Return records from the left input. These are returned only if the records do not have a join partner on the right side. |
| Single | Returns one join partner per entry on the left input. If more than one join partner exists, there are two valid semantics. 1) Only the first match is returned. 2) The system throws an error. If there is no match between the left and right inputs, NULL is returned. |


=== "JoinRel Message"

    ```proto
%%% proto.algebra.JoinRel %%%
    ```


## Set Operation

The set operation encompasses several set-level operations that support combining datasets based, possibly excluding records based on various types of record level matching.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2 or more                                                    |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution if all inputs have the same ordinal distribution. Orderedness is not maintained. |
| Direct Output Order  | All inputs are ordinally matched and returned together. All inputs must have matching record types. |

### Set Properties

| Property           | Description                       | Required              |
| ------------------ | --------------------------------- | --------------------- |
| Primary Input      | The primary input of the dataset. | Required              |
| Secondary Inputs   | One or more relational inputs.    | At least one required |
| Set Operation Type | From list below.                  | Required              |

### Set Operation Types

| Property                | Description                                                  |
| ----------------------- | ------------------------------------------------------------ |
| Minus (Primary)         | Returns the primary input excluding any matching records from secondary inputs. |
| Minus (Multiset)        | Returns the primary input minus any records that are included in all sets. |
| Intersection (Primary)  | Returns all rows primary rows that intersect at least one secondary input. |
| Intersection (Multiset) | Returns all rows that intersect at least one record from each secondary inputs. |
| Union Distinct          | Returns all the records from each set, removing any rows that are duplicated (within or across sets). |
| Union All               | Returns all records from each set, allowing duplicates.      |


=== "SetRel Message"

    ```proto
%%% proto.algebra.SetRel %%%
    ```


## Fetch Operation

The fetch operation eliminates records outside a desired window. Typically corresponds to a fetch/offset SQL clause. Will only returns records between the start offset and the end offset.

| Signature            | Value                                   |
| -------------------- | --------------------------------------- |
| Inputs               | 2 or more                               |
| Outputs              | 1                                       |
| Property Maintenance | Maintains distribution and orderedness. |
| Direct Output Order  | Unchanged from input.                   |



### Fetch Properties

| Property | Description                                                  | Required                 |
| -------- | ------------------------------------------------------------ | ------------------------ |
| Input    | A relational input, typically with a desired orderedness property. | Required                 |
| Offset   | A positive integer. Declares the offset for retrieval of records. | Optional, defaults to 0. |
| Count    | A positive integer. Declares the number of records that should be returned. | Required                 |

=== "FetchRel Message"

    ```proto
%%% proto.algebra.FetchRel %%%
    ```

## Aggregate Operation

The aggregate operation groups input data on one or more sets of grouping keys, calculating each measure for each combination of grouping key.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution if all distribution fields are contained in every grouping set. No orderedness guaranteed. |
| Direct Output Order  | The list of distinct columns from each grouping set (ordered by their first appearance) followed by the list of measures in declaration order, followed by an `i32` describing the associated particular grouping set the value is derived from (if applicable). |

In its simplest form, an aggregation has only measures. In this case, all records are folded into one, and a column is returned for each aggregate expression in the measures list.

Grouping sets can be used for finer-grained control over which records are folded. Within a grouping set, two records will be folded together if and only if each expressions in the grouping set yields the same value for each. The values returned by the grouping sets will be returned as columns to the left of the columns for the aggregate expressions. If a grouping set contains no grouping expressions, all rows will be folded for that grouping set.

It's possible to specify multiple grouping sets in a single aggregate operation. The grouping sets behave more or less independently, with each returned record belonging to one of the grouping sets. The values for the grouping expression columns that are not part of the grouping set for a particular record will be set to null. Two grouping expressions will be returned using the same column if they represent the protobuf messages describing the expressions are equal. The columns for grouping expressions that do *not* appear in *all* grouping sets will be nullable (regardless of the nullability of the type returned by the grouping expression) to accomodate the null insertion.

To further disambiguate which record belongs to which grouping set, an aggregate relation with more than one grouping set receives an extra `i32` column on the right-hand side. The value of this field will be the zero-based index of the grouping set that yielded the record.

If at least one grouping expression is present, the aggregation is allowed to not have any aggregate expressions. An aggregate relation is invalid if it would yield zero columns.

### Aggregate Properties

| Property         | Description                                                  | Required                                |
| ---------------- | ------------------------------------------------------------ | --------------------------------------- |
| Input            | The relational input.                                        | Required                                |
| Grouping Sets    | One or more grouping sets.                                   | Optional, required if no measures.      |
| Per Grouping Set | A list of expression grouping that the aggregation measured should be calculated for. | Optional.      |
| Measures         | A list of one or more aggregate expressions along with an optional filter. | Optional, required if no grouping sets. |


=== "AggregateRel Message"

    ```proto
%%% proto.algebra.AggregateRel %%%
    ```


## Write Operator

The write operator is an operator that consumes one output and writes it to storage. A simple example would be writing Parquet files. It is expected that many types of writes will be added over time.

| Signature            | Value           |
| -------------------- | --------------- |
| Inputs               | 1               |
| Outputs              | 0               |
| Property Maintenance | N/A (no output) |
| Direct Output Order  | N/A (no output) |

### Write Properties

| Property                    | Description                                                  | Required                    |
| --------------------------- | ------------------------------------------------------------ | --------------------------- |
| Definition                  | The contents of the write property definition.               | Required                    |
| Field names                 | The names of all struct fields in breadth-first order.       | Required                    |
| Masked Complex Expression   | The masking expression applied to the input record prior to write. | Optional, defaults to all   |
| Rotation description fields | A list of fields that can be used for stream description whenever a stream is reset. | Optional, defaults to none. |
| Rotation indicator          | An input field ID that describes when the current stream should be "rotated". Individual write definition types may support the ability to rotate the output into one or more streams. This could mean closing and opening a new file, finishing and restarting a TCP connection, etc. If a rotation indicator is available, it will be 0 except when a rotation should occur. Rotation indication are frequently defined by things like discrete partition values but could be done based on number of records or other arbitrary criteria. | Optional, defaults to none. |

### Write Definition Types

Write definition types are built by the community and added to the specification. This is a portion of specification that is expected to grow rapidly.

#### Virtual Table

| Property | Description                                                  | Required                     |
| -------- | ------------------------------------------------------------ | ---------------------------- |
| Name     | The in-memory name to give the dataset.                      | Required                     |
| Pin      | Whether it is okay to remove this dataset from memory or it should be kept in memory. | Optional, defaults to false. |



#### Files Type

| Property | Description                                                  | Required |
| -------- | ------------------------------------------------------------ | -------- |
| Path     | A URI to write the data to. Supports the inclusion of field references that are listed as available in properties as a "rotation description field". | Required |
| Format   | Enumeration of available formats. Only current option is PARQUET. | Required |



## Discussion Points

* How to handle correlated operations?
