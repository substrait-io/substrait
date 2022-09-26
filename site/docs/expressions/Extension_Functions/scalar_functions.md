# Scalar Functions

A function is a scalar function if that function takes in values from a single record and produces an output value. To clearly specify the definition of functions, Substrait declares an extensible specification plus binding approach to function resolution.

A scalar function implementation includes the following properties.

| Property               | Description                                                                                                                    | Required                            |
|------------------------|--------------------------------------------------------------------------------------------------------------------------------|-------------------------------------|
| Name                   | The UTF-8 string that is used to case-insensitively reference this function.                                                   | Required                            |
| Description            | Additional description of function for implementers or users. Should be written human-readable to allow exposure to end users. | Optional                            |
| Arguments              | As defined [here](index.md#arguments).                                                                                         | Optional, defaults to niladic       |
| Return type            | As defined [here](index.md#return-values).                                                                                     | Required                            |
| Deterministic          | Whether this function is expected to reproduce the same output when it is invoked multiple times with the same input. This informs a plan consumer on whether it can constant-reduce the defined function. An example would be a random() function, which is typically expected to be evaluated repeatedly despite having the same set of inputs. | Optional, defaults to true |
| Session-dependent      | Whether this function is influenced by the session context it is invoked within. For example, a function may be influenced by a user who is invoking the function, the time zone of a session, or some other non-obvious parameter. This can inform caching systems on whether a particular function is cacheable. | Optional, defaults to false |
| Implementation Map     | A map of implementation locations for one or more implementations of the given function. Each key is a function implementation type. Implementation types include examples such as: AthenaArrowLambda, TrinoV361Jar, ArrowCppKernelEnum, GandivaEnum, LinkedIn Transport Jar, etc. [Definition TBD]. Implementation type has one or more properties associated with retrieval of that implementation. | Optional |

## Pattern Matching and Evaluation Order

The patterns used to define the argument types and return type are processed in the following order.

 - Match the actual argument types against the argument slot patterns from left to right. The pattern from the last argument slot may be matched any number of times if the function is variadic.
 - Evaluate any statements in the return type specification from top to bottom/left to right.
 - Evaluate the return type pattern.

If any pattern fails to match or evaluate, the function is said to not match the given argument pack.
