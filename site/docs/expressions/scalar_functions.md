# Scalar Functions

Scalar functions are a function that takes in values from a single record and produces an output value. To clearly specify the definition of functions, Substrait declares a extensible specification plus binding approach to function resolution. This ensures a clear definition of function behavior separate from a particular use of each function while avoiding operations 

Substrait supports a number of functions. A scalar function signature includes the following properties:

| Property           | Description                                                  | Required                            |
| ------------------ | ------------------------------------------------------------ | ----------------------------------- |
| Name               | One or more user friendly utf8 strings that are used to reference this function in languages | At least one value is required.     |
| List of arguments  | Argument properties are defined below                        | Optional, defaults to niladic.      |
| Deterministic      | Whether this function is expected to reproduce the same output when it is invoked multiple times with the same input. This informs a plan consumer on whether it can constant reduce the defined function. An example would be a random() function, which is typically expected to be evaluated repeatedly despite having the same set of inputs. | Optional, defaults to true.         |
| Session Dependent  | Whether this function is influenced by the session context it is invoked within. For example, a function may be influenced by a user who is invoking the function, the time zone of a session, or some other non-obvious parameter. This can inform caching systems on whether a particular function is cacheable. | Optional, defaults to false.        |
| Variadic Behavior  | Whether the last argument of the function is variadic. Options include: [Single argument, M..N]. M must be less than or equal to N. N can be defined or left as "unlimited". | Optional, defaults to single value. |
| Description        | Additional description of function for implementers or users. Should be written human readable to allow exposure to end users. Presented as a map with language => description mappings. E.g. `{ "en": "This adds two numbers together.", "fr": "cela ajoute deux nombres"}`. | Optional                            |
| Output Type        | The output type is expected to be a physical type of the expression. There are two categories of output type:<br />Direct: Output type is declared<br />Complex: Behavior definitions are either declared within the specification or embedded in the signature using [TBD]. | Required                            |
| Implementation Map | A map of implementation locations for one or more implementations of the given function. Each key is a function implementation type. Implementation types include examples such as: AthenaArrowLambda, TrinoV361Jar, ArrowCppKernelEnum, GandivaEnum, LinkedIn Transport Jar, etc. [Definition TBD]. Implementation type has one or more properties associated with retrieval of that implementation. | Optional                            |



## Argument Properties

| Property      | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| Physical Type | The physical types this argument requires. Physical types are used here so that different binds are used when working in a physical plan depending on the execution engines capabilities. In common cases, the system default representation is used. |
| Constant      | Whether this argument is required to be a constant for invocation. For example, in some system a regular expression pattern would only be accepted as a literal and not a column value reference. |



## Function Ids

Each function signature is categorized based on a function signature id. The identifier includes two components:

| Property        | Description                                                  |
| --------------- | ------------------------------------------------------------ |
| Organization Id | An unsigned 32 bit integer mapped to a list of known organizations listed in the [Substrait repository](https://github.com/substrait-io/substrait/blob/main/extensions/organizations.yaml). |
| Function Id     | An unsigned 32 bit integer mapped to a list of known functions for the specific organization. For the Substrait organization, the function is listed in [here](https://github.com/substrait-io/substrait/blob/main/extensions/scalar_functions.yaml). |

An organization is responsible for managing their own lists of functions. A function id should be used forever. If a function is deleted for any reason, the id should not be reused. Ideally each function signature will have one or more 

## Scalar Function Bindings

For scalar functions (one value produced for each record), a function binding declares the function identifier to be used and the input arguments.





## Key Discussion Points

* Currently avoiding binary operator concepts (e.g. +)
* How to define complex behavior definitions? Start with only allowing definition in Substrait? Come up with some kind of way to express output type derivation using something language agnostic (e.g. a WebAssembly scripting language)?
* Should we have a function version for each function signature? What happens if someone redefines or changes the semantics of a function?