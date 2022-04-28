# Embedded Functions

Embedded functions are a special kind of function where the implementation is embedded within the actual plan. They are commonly used in tools where a user intersperses business logic within a data pipeline. This is more common in data science workflows than traditional SQL workflows.

Embedded functions are not pre-registered. Embedded functions require that data be consumed and produced with a standard API, may require memory allocation and have determinate error reporting behavior. They may also have specific runtime dependencies. For example, a Python pickle function may depend on pyarrow 5.0 and pynessie 1.0.

Properties for an embedded function include:

| Property            | Description                                                | Required |
| ------------------- | ---------------------------------------------------------- | -------- |
| Function Type       | The type of embedded function presented.                   | Required |
| Function Properties | Function properties, one of those items defined below.     | Required |
| Output Type         | The fully resolved output type for this embedded function. | Required |

The binary representation of an embedded function is:


=== "Binary Representation"
    ```proto
%%% proto.message.Expression.EmbeddedFunction %%%
    ```
=== "Human Readable Representation"
    n/a
=== "Example"
    n/a


## Function Details

There are many types of possible stored functions. For each, Substrait works to expose the function in as descriptive a way as possible to support the largest number of consumers.



## Python Pickle Function Type

| Property    | Description                                                  | Required                   |
| ----------- | ------------------------------------------------------------ | -------------------------- |
| Pickle Body | binary pickle encoded function using [TBD] API representation to access arguments. | True                       |
| Prereqs     | A list of specific Python conda packages that are prerequisites for access (a structured version of a requirements.txt file). | Optional, defaults to none |



## WebAssembly Function Type

| Property | Description                                                  | Required                   |
| -------- | ------------------------------------------------------------ | -------------------------- |
| Script   | WebAssembly function                                         | True                       |
| Prereqs  | A list of AssemblyScript prerequisites required to compile the assemblyscript function using NPM coordinates. | Optional, defaults to none |



## Discussion Points

* What are the common embedded function formats?
* How do we expose the data for a function?
* How do we express batching capabilities?
* How do we ensure/declare containerization?







