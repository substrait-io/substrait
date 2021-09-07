# Logical Relations



## Read Operator

The read operator is an operator that produces one output. A simple example would be the reading of a Parquet file. It is expected that many types of reads will be added over time

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 0                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | N/A (no inputs)                                              |
| Direct Output Order  | Defaults to the schema of the data read after the optional projection (masked complex expression) is applied. |

### Read Properties

| Property          | Description                                                  | Required                             |
| ----------------- | ------------------------------------------------------------ | ------------------------------------ |
| Read Type         | The type of read to complete.                                | Required                             |
| Definition        | The contents of the read property definition, validated to the read type signature | Required                             |
| Direct Schema     | Defines the schema of the output of the read (before any emit remapping/hiding). | Required                             |
| Filter            | A boolean Substrait expression that describes the filter of a iceberg dataset. TBD: define how field referencing works. | Optional, defaults to none.          |
| Projection        | A masked complex expression describing the portions of the content that should be read | Optional, defaults to all of schema  |
| Output properties | Declaration of orderedness and/or distribution properties this read produces | Optional, defaults to no properties. |
| Properties        | A list of name/value pairs associated with the read          | Optional, defaults to empty          |

### Read Definition Types

Read definition types are built by the community and added to the specification. This is a portion of specification that is expected to grow rapidly.



#### Virtual Table

| Property | Description | Required |
| -------- | ----------- | -------- |
| Data     | Required    | Required |



#### Files Type

| Property        | Description                                                  | Required |
| --------------- | ------------------------------------------------------------ | -------- |
| Items           | An array Items (path or path glob) associated with the read  | Required |
| Format per item | Enumeration of available formats. Only current option is PARQUET. | Required |



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
| Expression | A boolean expression which describes which records are included/excluded. | Required |


## Sort Operation

The sort operator reorders a dataset based on one or more identified sort fields as well as a sorting function. 

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Will update orderedness property to the output of the sort operation. Distribution property only remapped based on emit. |
| Direct Output Order  | The field order of the input.                                |



### Sort Properties

| Property    | Description                                                  | Required                |
| ----------- | ------------------------------------------------------------ | ----------------------- |
| Sort Fields | List of one or more fields to sort by. Uses the same properties as the [orderedness](basics.md#orderedness) property. | One sort field required |



## Project Operation

The project operation will produce one or more additional expressions based on the inputs of the dataset.

| Signature            | Value                                                        |
| -------------------- | ------------------------------------------------------------ |
| Inputs               | 1                                                            |
| Outputs              | 1                                                            |
| Property Maintenance | Distribution maintained, mapped by emit. Orderedness: Maintained if no window operations. Extended to include projection fields if fields are direct references. If window operations are present, no orderedness is maintained. |
| Direct Output Order  | The field order of the input + the list of new expressions in the order they are declared in the expressions list. |



### Project Properties

| Property    | Description                                         | Required                         |
| ----------- | --------------------------------------------------- | -------------------------------- |
| Expressions | List of one or more expression to add to the input. | At least one expression required |



## Join Operation

The join operation will combine two separate inputs into a single output, based on a join expression.

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
| Join Expression | A boolean condition that describes whether each record from the left set "match" the record from the right set. | Required. Can be the literal True. |
| Join Type       | One of the join types defined below.                         | Required                           |

### Join Types

| Type  | Description                                                  |
| ----- | ------------------------------------------------------------ |
| Inner | Return records from the left side only if they match the right side. Return records from the right side only when they match the left side. For each cross input match, return a record including the data from both sides. Non-matching records are ignored. |
| Outer | Return all records from both the left and right inputs. For each cross input match, return a record including the data from both sides. For any remaining non-match records, return the record from the corresponding input along with nulls for the opposite input. |
| Left  | Return all records from the left input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the left input, return the left record along with nulls for the right input. |
| Right | Return all records from the right input. For each cross input match, return a record including the data from both sides. For any remaining non-matching records from the right input, return the left record along with nulls for the right input. |

