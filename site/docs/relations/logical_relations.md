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

| Property           | Description                                                  | Required                             |
| ------------------ | ------------------------------------------------------------ | ------------------------------------ |
| Definition         | The contents of the read property definition.                | Required                             |
| Direct Schema      | Defines the schema of the output of the read (before any projection or emit remapping/hiding). | Required                             |
| Filter             | A boolean Substrait expression that describes a filter that must be applied to the data. The filter should be interpreted against the direct schema. | Optional, defaults to none. |
| Best Effort Filter | A boolean Substrait expression that describes a filter that may be applied to the data.  The filter should be interpreted against the direct schema. | Optional, defaults to none. |
| Projection         | A masked complex expression describing the portions of the content that should be read | Optional, defaults to all of schema  |
| Output Properties  | Declaration of orderedness and/or distribution properties this read produces. | Optional, defaults to no properties. |
| Properties         | A list of name/value pairs associated with the read.         | Optional, defaults to empty          |

### Read Filtering

The read relation has two different filter properties.  A filter, which must be satisfied by the operator and a best effort
filter, which does not have to be satisfied.  This reflects the way that consumers are often implemented.  A consumer is
often only able to fully apply a limited set of operations in the scan.  There can then be an extended set of operations which
a consumer can apply in a best effort fashion.  A producer, when setting these two fields, should take care to only use
expressions that the consumer is capable of handling.

As an example, a consumer may only be able to fully apply (in the read relation) <, =, and > on integral types.  The consumer
may be able to apply <, =, and > in a best effort fashion on decimal and string types.  Consider the filter expression
`my_int < 10 && my_string < "x" && upper(my_string) > "B"`.  In this case the `filter` should be set to
`my_int < 10` and the `best_effort_filter` should be set to `my_string < "x"` and the remaining portion (`upper(my_string) > "B"`) should be put into a filter relation.

A filter expression must be interpreted against the direct schema before the projection expression has been applied. As a result, fields may be referenced by the filter expression which are not included in the relation's output.

### Read Definition Types

???+ info inline end "Adding new Read Definition Types"

    If you have a read definition that's not covered here, see the [process for adding new read definition types](../spec/extending.md).

Read definition types (like the rest of the features in Substrait) are built by the community and added to the specification.

#### Virtual Table

A virtual table is a table whose contents are embedded in the plan itself.  The table data
is encoded as records consisting of literal values or expressions that can be resolved without referencing any input data.
For example, a literal, a function call involving literals, or any other expression that does
not require input.

| Property | Description | Required |
| -------- | ----------- | -------- |
| Data     | Required    | Required |


#### Named Table

A named table is a reference to data defined elsewhere.  For example, there may be a catalog
of tables with unique names that both the producer and consumer agree on.  This catalog would
provide the consumer with more information on how to retrieve the data.

| Property | Description                                                      | Required                |
| -------- | ---------------------------------------------------------------- | ----------------------- |
| Names    | A list of namespaced strings that, together, form the table name | Required (at least one) |

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

#### Iceberg Table Type

A Iceberg Table is a table built on [Apache Iceberg](https://iceberg.apache.org/). Iceberg tables can be read by either directly reading a [metadata file](https://iceberg.apache.org/spec/#table-metadata) or by consulting a [catalog](https://iceberg.apache.org/concepts/catalog/). 

##### Metadata File Reading

Points to an [Iceberg metadata file](https://iceberg.apache.org/spec/#table-metadata) and uses that as a starting point for reading an Iceberg table. This is the simplest form of Iceberg table access but should be limited to use for reads. (Writes often also need to update an external catalog.)

| Property | Description                                                      | Required                |
| -------- | ---------------------------------------------------------------- | ----------------------- |
| metadata_uri    | A URI for an Iceberg metadata file. This current snapshot will be read from this file.  | Required |
| snapshot_id    | The snapshot that should be read using id. If not provided, the current snapshot is read. Only one of snapshot_id or snapshot_timestamp should be set. | Optional |
| snapshot_timestamp    | The snapshot that should be read using timestamp. If not provided, the current snapshot is read. | Optional |


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

The project operation will produce one or more additional expressions based on the inputs of the relation.

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

| Property         | Description                                                  | Required                           |
| ---------------- | ------------------------------------------------------------ | ---------------------------------- |
| Left Input       | A relational input.                                          | Required                           |
| Right Input      | A relational input.                                          | Required                           |
| Join Expression  | A boolean condition that describes whether each record from the left set "match" the record from the right set. Field references correspond to the direct output order of the data. | Required. Can be the literal True. |
| Post-Join Filter | A boolean condition to be applied to each result record after the inputs have been joined, yielding only the records that satisfied the condition. | Optional                           |
| Join Type        | One of the join types defined below.                         | Required                           |

### Join Types

| Type  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ----- |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Inner | Return records from the left side only if they match the right side. Return records from the right side only when they match the left side. For each cross input match, return a record including the data from both sides. Non-matching records are ignored. |
| Outer | Return all records from both the left and right inputs. For each cross input match, return a record including the data from both sides. For any remaining non-match records, return the record from the corresponding input along with nulls for the opposite input. |
| Left  | Return all records from the left input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the left input, return the left record along with nulls for the right input. |
| Right | Return all records from the right input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the right input, return the right record along with nulls for the left input. |
| Left Semi | Returns records from the left input. These are returned only if the records have a join partner on the right side. |
| Right Semi | Returns records from the right input. These are returned only if the records have a join partner on the left side. |
| Left Anti  | Return records from the left input. These are returned only if the records do not have a join partner on the right side. |
| Right Anti  | Return records from the right input. These are returned only if the records do not have a join partner on the left side. |
| Left Single | Return all records from the left input with no join expansion. If at least one record from the right input matches the left, return one arbitrary matching record from the right input. For any left records without matching right records, return the left record along with nulls for the right input. Similar to a left outer join but only returns one right match at most. Useful for nested sub-queries where we need exactly one record in output (or throw exception).  See Section 3.2 of https://15721.courses.cs.cmu.edu/spring2018/papers/16-optimizer2/hyperjoins-btw2017.pdf for more information. |
| Right Single | Same as left single except that the right and left inputs are switched. |
| Left Mark | Returns one record for each record from the left input.  Appends one additional "mark" column to the output of the join. The new column will be listed after all columns from both sides and will be of type nullable boolean.  If there is at least one join partner in the right input where the join condition evaluates to true then the mark column will be set to true.  Otherwise, if there is at least one join partner in the right input where the join condition evaluates to NULL then the mark column will be set to NULL.  Otherwise the mark column will be set to false. |
| Right Mark | Returns records from the right input.  Appends one additional "mark" column to the output of the join. The new column will be listed after all columns from both sides and will be of type nullable boolean.  If there is at least one join partner in the left input where the join condition evaluates to true then the mark column will be set to true.  Otherwise, if there is at least one join partner in the left input where the join condition evaluates to NULL then the mark column will be set to NULL.  Otherwise the mark column will be set to false. |


=== "JoinRel Message"

    ```proto
%%% proto.algebra.JoinRel %%%
    ```


## Set Operation

The set operation encompasses several set-level operations that support combining datasets, possibly excluding records based on various types of record level matching.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 2 or more                                                    |
| Outputs              | 1                                                            |
| Property Maintenance | Maintains distribution if all inputs have the same ordinal distribution. Orderedness is not maintained. |
| Direct Output Order  | The field order of the inputs. All inputs must have identical field *types*, but field nullabilities may vary. |

### Set Properties

| Property           | Description                       | Required              |
| ------------------ | --------------------------------- | --------------------- |
| Primary Input      | The primary input of the relation.| Required              |
| Secondary Inputs   | One or more relational inputs.    | At least one required |
| Set Operation Type | From list below.                  | Required              |

### Set Operation Types

The set operation type determines both the records that are emitted and the type of the output record.

For some set operations, whether a specific record is included in the output and if it appears more than once depends on the number of times it occurs across all inputs. In the following table, treat:
* m: the number of time a records occurs in the primary input (p)
* n1: the number of times a record occurs in the 1st secondary input (s1)
* n2: the number of times a record occurs in the 2nd secondary input (s2)
* ...
* n: the number of times a record occurs in the nth secondary input

| Operation                   | Description                                                                                                                                                                                                                                                                         | Examples                                                                                                                                                        | Output Nullability
|-----------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------| -----------------------------
| Minus (Primary)             | Returns all records from the primary input excluding any matching rows from secondary inputs, removing duplicates.<br/>Each value is treated as a unique member of the set, so duplicates in the first set don’t affect the result.<br/>This operation maps to SQL EXCEPT DISTINCT. | MINUS<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 4}<br/>&nbsp;&nbsp;s1: {1, 2}<br/>&nbsp;&nbsp;s2: {3}<br/>YIELDS<br/>{4}                                            | The same as the primary input.                                                                                                                                                                     
| Minus (Primary All)         | Returns all records from the primary input excluding any matching records from secondary inputs.<br/>For each specific record returned, the output contains max(0, m - sum(n1, n2, …, n)) copies.<br/>This operation maps to SQL EXCEPT ALL.                                        | MINUS ALL<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 3}<br/>&nbsp;&nbsp;s1: {1, 2, 3, 4}<br/>&nbsp;&nbsp;s2: {3}<br/>YIELDS<br/>{2, 3, 3}                            | The same as the primary input.
| Minus (Multiset)            | Returns all records from the primary input excluding any records that are included in *all* secondary inputs.<br/>This operation does not have a direct SQL mapping.                                                                                                                | MINUS MULTISET<br/>&nbsp;&nbsp;p: {1, 2, 3, 4}<br/>&nbsp;&nbsp;s1: {1, 2}<br/>&nbsp;&nbsp;s2: {1, 2, 3}<br/>YIELDS<br/>{3, 4}                                   | The same as the primary input.
| Intersection (Primary)      | Returns all records from the primary input that are present in any secondary input, removing duplicates.<br/>This operation does not have a direct SQL mapping.                                                                                                                     | INTERSECT<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 4}<br/>&nbsp;&nbsp;s1: {1, 2, 3, 5}<br/>&nbsp;&nbsp;s2: {2, 3, 6}<br/>YIELDS<br/>{1, 2, 3}                         | If a field is nullable in the primary input and in any of the secondary inputs, it is nullable in the output. 
| Intersection (Multiset)     | Returns all records from the primary input that match at least one record from *all* secondary inputs.<br/>This operation maps to SQL INTERSECT DISTINCT                                                                                                                            | INTERSECT MULTISET<br/>&nbsp;&nbsp;p: {1, 2, 3, 4}<br/>&nbsp;&nbsp;s1: {2, 3}<br/>&nbsp;&nbsp;s2: {3, 4}<br/>YIELDS<br/>{3}                                     | If a field is required in any of the inputs, it is required in the output.
| Intersection (Multiset All) | Returns all records from the primary input that are present in every secondary input.<br/>For each specific record returned, the output contains min(m, n1, n2, …, n) copies.<br/>This operation maps to SQL INTERSECT ALL.                                                         | INTERSECT ALL<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 4}<br/>&nbsp;&nbsp;s1: {1, 2, 3, 3, 5}<br/>&nbsp;&nbsp;s2: {2, 3, 3, 6}<br/>YIELDS<br/>{2, 3, 3}            | If a field is required in any of the inputs, it is required in the output.
| Union Distinct              | Returns all records from each set, removing duplicates.<br/>This operation maps to SQL UNION DISTINCT.                                                                                                                                                                              | UNION<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 4}<br/>&nbsp;&nbsp;s1: {2, 3, 5}<br/>&nbsp;&nbsp;s2: {1, 6}<br/>YIELDS<br/>{1, 2, 3, 4, 5, 6}                       | If a field is nullable in any of the inputs, it is nullable in the output.                                                                                                                                                                             
| Union All                   | Returns all records from all inputs.<br/>For each specific record returned, the output contains (m + n1 + n2 + … + n) copies.<br/>This operation maps to SQL UNION ALL.                                                                                                             | UNION ALL<br/>&nbsp;&nbsp;p: {1, 2, 2, 3, 3, 3, 4}<br/>&nbsp;&nbsp;s1: {2, 3, 5}<br/>&nbsp;&nbsp;s2: {1, 6}<br/>YIELDS<br/>{1, 2, 2, 3, 3, 3, 4, 2, 3, 5, 1, 6} | If a field is nullable in any of the inputs, it is nullable in the output.

Note that for set operations, NULL matches NULL. That is
```
{NULL, 1, 3} MINUS          {NULL, 2, 4} === (1), (3)
{NULL, 1, 3} INTERSECTION   {NULL, 2, 3} === (NULL)
{NULL, 1, 3} UNION DISTINCT {NULL, 2, 4} === (NULL), (1), (2), (3), (4)
```

#### Output Type Derivation Examples
Given the following inputs, where R is Required and N is Nullable:
```
Input 1: (R, R, R, R, N, N, N, N)  Primary Input
Input 2: (R, R, N, N, R, R, N, N)  Secondary Input
Input 3: (R, N, R, N, R, N, R, N)  Secondary Input
```

The output type is as follows for the various operations

| Property                    | Output Type
|-----------------------------| -----------------------------------------------------------------------------------------------------
| Minus (Primary)             | (R, R, R, R, N, N, N, N)
| Minus (Primary All)         | (R, R, R, R, N, N, N, N)
| Minus (Multiset)            | (R, R, R, R, N, N, N, N)
| Intersection (Primary)      | (R, R, R, R, R, N, N, N)
| Intersection (Multiset)     | (R, R, R, R, R, R, R, N)
| Intersection (Multiset All) | (R, R, R, R, R, R, R, N)
| Union Distinct              | (R, N, N, N, N, N, N, N)
| Union All                   | (R, N, N, N, N, N, N, N)


=== "SetRel Message"

    ```proto
%%% proto.algebra.SetRel %%%
    ```


## Fetch Operation

The fetch operation eliminates records outside a desired window. Typically corresponds to a fetch/offset SQL clause. Will only returns records between the start offset and the end offset.

| Signature            | Value                                   |
| -------------------- | --------------------------------------- |
| Inputs               | 1                                       |
| Outputs              | 1                                       |
| Property Maintenance | Maintains distribution and orderedness. |
| Direct Output Order  | Unchanged from input.                   |

### Fetch Properties

| Property          | Description                                                                                                                                                                                                                          | Required                   |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------- |
| Input             | A relational input, typically with a desired orderedness property.                                                                                                                                                                   | Required                   |
| Offset Expression | An expression which evaluates to a non-negative integer or null (recommended type is `i64`). Declares the offset for retrieval of records. An expression evaluating to null is treated as 0.                                         | Optional, defaults to a 0 literal.   |
| Count Expression  | An expression which evaluates to a non-negative integer or null (recommended type is `i64`). Declares the number of records that should be returned. An expression evaluating to null indicates that all records should be returned. | Optional, defaults to a null literal. |

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
| Direct Output Order  | The list of grouping expressions in declaration order followed by the list of measures in declaration order, followed by an `i32` describing the associated particular grouping set the value is derived from (if applicable). |

In its simplest form, an aggregation has only measures. In this case, all records are folded into one, and a column is returned for each aggregate expression in the measures list.

Grouping sets can be used for finer-grained control over which records are folded. A grouping set consists of zero or more references to the list of grouping expressions. Within a grouping set, two records will be folded together if and only if they have the same values for each of the expressions in the grouping set. The values returned by the grouping expressions will be returned as columns to the left of the columns for the aggregate expressions. Each of the grouping expressions must occur in at least one of the grouping sets. If a grouping set contains no grouping expressions, all rows will be folded for that grouping set. (Having a single grouping set with no grouping expressions is thus equivalent to not having any grouping sets.)

It is possible to specify multiple grouping sets in a single aggregate operation. The grouping sets behave more or less independently, with each returned record belonging to one of the grouping sets. The values for the grouping expression columns that are not part of the grouping set for a particular record will be set to null. The columns for grouping expressions that do *not* appear in *all* grouping sets will be nullable (regardless of the nullability of the type returned by the grouping expression) to accomodate the null insertion.

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

## Reference Operator

The reference operator is used to construct DAGs of operations. In a `Plan` we can have multiple Rel representing various
computations with potentially multiple outputs. The `ReferenceRel` is used to express the fact that multiple `Rel` might be
sharing subtrees of computation. This can be used to express arbitrary DAGs as well as represent multi-query optimizations.

As a concrete example think about two queries `SELECT * FROM A JOIN B JOIN C` and `SELECT * FROM A JOIN B JOIN D`,
We could use the `ReferenceRel` to highlight the shared `A JOIN B` between the two queries, by creating a plan with 3 `Rel`.
One expressing `A JOIN B` (in position 0 in the plan), one using reference as follows: `ReferenceRel(0) JOIN C` and a third one
doing `ReferenceRel(0) JOIN D`. This allows to avoid the redundancy of `A JOIN B`.

| Signature            | Value                                 |
| -------------------- |---------------------------------------|
| Inputs               | 1                                     |
| Outputs              | 1                                     |
| Property Maintenance | Maintains all properties of the input |
| Direct Output Order  | Maintains order                       |


### Reference Properties

| Property                    | Description                                                                    | Required                    |
|-----------------------------|--------------------------------------------------------------------------------| --------------------------- |
| Referred Rel                | A zero-indexed positional reference to a `Rel` defined within the same `Plan`. | Required                    |

=== "ReferenceRel Message"

    ```proto
%%% proto.algebra.ReferenceRel %%%
    ```

## Write Operator

The write operator is an operator that consumes one input and writes it to storage. This can range from writing to a Parquet file, to INSERT/DELETE/UPDATE in a database.

| Signature            | Value                                                    |
| -------------------- |--------------------------------------------------------- |
| Inputs               | 1                                                        |
| Outputs              | 1                                                        |
| Property Maintenance | Output depends on OutputMode (none, or modified records) |
| Direct Output Order  | Unchanged from input                                     |

### Write Properties


| Property       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Required                                         |
|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| Write Type     | Definition of which object we are operating on (e.g., a fully-qualified table name).                                                                                                                                                                                                                                                                                                                                                                                                 | Required                                         |
| CTAS Schema    | The names of all the columns and their type for a CREATE TABLE AS.                                                                                                                                                                                                                                                                                                                                                                                                                   | Required only for CTAS                           |
| Write Operator | Which type of operation we are performing (INSERT/DELETE/UPDATE/CTAS).                                                                                                                                                                                                                                                                                                                                                                                                               | Required                                         |
| Rel Input      | The Rel representing which records we will be operating on (e.g., VALUES for an INSERT, or which records to DELETE, or records and after-image of their values for UPDATE).                                                                                                                                                                                                                                                                                                          | Required                                         |
| Create Mode    | This determines what should happen if the table already exists (ERROR/REPLACE/IGNORE)                                                                                                                                                                                                                                                                                                                                                                                                | Required only for CTAS                           |
| Output Mode    | For views that modify a DB it is important to control which records to "return". Common default is NO_OUTPUT where we return nothing. Alternatively, we can return MODIFIED_RECORDS, that can be further manipulated by layering more rels ontop of this WriteRel (e.g., to "count how many records were updated"). This also allows to return the after-image of the change. To return before-image (or both) one can use the reference mechanisms and have multiple return values. | Required for VIEW CREATE/CREATE_OR_REPLACE/ALTER |


### Write Definition Types

???+ info inline end "Adding new Write Definition Types"

    If you have a write definition that's not covered here, see the [process for adding new write definition types](../spec/extending.md).

Write definition types are built by the community and added to the specification.


=== "WriteRel Message"

    ```proto
%%% proto.algebra.WriteRel %%%
    ```

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


## Update Operator

The update operator applies a set of column transformations on a named table and writes to a storage. 

| Signature            | Value                                 |
| -------------------- |---------------------------------------|
| Inputs               | 0                                     |
| Outputs              | 1                                     |
| Property Maintenance | Output is number of modified records  |

### Update Properties

| Property               | Description                                                                          | Required                                         |
|------------------------|--------------------------------------------------------------------------------------|--------------------------------------------------|
| Update Type            | Definition of which object we are operating on (e.g., a fully-qualified table name). | Required                                         |
| Table Schema           | The names and types of all the columns of the input table                            | Required                                         |
| Update Condition       | The condition that must be met for a record to be updated.                           | Required                                         |
| Update Transformations | The set of column updates to be applied to the table.                                | Required                                         |

=== "UpdateRel Message"

    ```proto
%%% proto.algebra.UpdateRel %%%
    ```


## DDL (Data Definition Language) Operator

The operator that defines modifications of a database schema (CREATE/DROP/ALTER for TABLE and VIEWS).

| Signature            | Value           |
| -------------------- |-----------------|
| Inputs               | 1               |
| Outputs              | 0               |
| Property Maintenance | N/A (no output) |
| Direct Output Order  | N/A             |


### DDL Properties

| Property        | Description                                                     | Required                                         |
|-----------------|-----------------------------------------------------------------|--------------------------------------------------|
| Write Type      | Definition of which type of object we are operating on.         | Required                                         |
| Table Schema    | The names of all the columns and their type.                    | Required (except for DROP operations)            |
| Table Defaults  | The set of default values for this table.                       | Required (except for DROP operations)            |
| DDL Object      | Which type of object we are operating on (e.g., TABLE or VIEW). | Required                                         |
| DDL Operator    | The operation to be performed (e.g., CREATE/ALTER/DROP).        | Required                                         |
| View Definition | A Rel representing the "body" of a VIEW.                        | Required for VIEW CREATE/CREATE_OR_REPLACE/ALTER |

=== "DdlRel Message"

    ```proto
%%% proto.algebra.DdlRel %%%
    ```

???+ question "Discussion Points"

    * How should correlated operations be handled?
