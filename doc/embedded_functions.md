# Embedded Functions

Embedded functions are a special kind of function where the implementation is embedded within the actual plan. They are commonly used in tools where a user intersperses business logic within a data pipeline. This is more common in data science workflows than traditional SQL workflows.

Embedded functions are not pre-registered. Embedded functions require that data be consumed and produced with a standard API, may require memory allocation and have determinate error reporting behavior. They may also have specific runtime dependencies. For example, a python pickle function may depend on pyarrow 5.0 and . 

Discussion Points:

* What are the common embedded function formats?
* How do we expose the data for a function?
* How do we express batching capabilities?
* How do we ensure/declare containerization?







