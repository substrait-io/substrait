# Scalar Functions

Scalar functions are a function that takes in values from a single record and produces an output value. To clearly specify the definition of functions, Substrait declares a extensible specification plus binding approach to function resolution. This ensures a clear definition of function behavior separate from a particular use of each function while avoiding operations 

Substrait supports a number of functions. A scalar function signature includes the following properties:

| Property           | Description                                                  | Required                            |
| ------------------ | ------------------------------------------------------------ | ----------------------------------- |
| Name               | One or more user friendly utf8 strings that are used to reference this function in languages | At least one value is required.     |
| List of arguments  | Argument properties are defined below. Arguments can be fully defined, parameterized or wildcarded. See further details below. | Optional, defaults to niladic.      |
| Deterministic      | Whether this function is expected to reproduce the same output when it is invoked multiple times with the same input. This informs a plan consumer on whether it can constant reduce the defined function. An example would be a random() function, which is typically expected to be evaluated repeatedly despite having the same set of inputs. | Optional, defaults to true.         |
| Session Dependent  | Whether this function is influenced by the session context it is invoked within. For example, a function may be influenced by a user who is invoking the function, the time zone of a session, or some other non-obvious parameter. This can inform caching systems on whether a particular function is cacheable. | Optional, defaults to false.        |
| Variadic Behavior  | Whether the last argument of the function is variadic. Options include: [Single argument, M..N]. M must be less than or equal to N. N can be defined or left as "unlimited". | Optional, defaults to single value. |
| Description        | Additional description of function for implementers or users. Should be written human readable to allow exposure to end users. Presented as a map with language => description mappings. E.g. `{ "en": "This adds two numbers together.", "fr": "cela ajoute deux nombres"}`. | Optional                            |
| Return Type        | The output type is expected to be a physical type of the expression. Return types can be either or simple, fully-defined compound type or a type expression. See below for more on type expressions. | Required                            |
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



## Argument Types & Output Type Derivation

### Input Argument Types

Input arguments can be declared in one of two ways: materialized or parameterized.

* Materialized Type: Materialized types are either simple types or compound types with all parameters fully defined (without any parameters). Examples include i32, fp32, VARCHAR(20), List&lt;fp32&gt;, etc.
* Parameterized types: Types can be parameterized such that the parameter can be used in a output derivation e.g. `f(K) => K` or to guarantee consistency of type between input arguments `f(K,K) => boolean`. Example parameterized types would be VARCHAR(T), List&lt;E&gt;, MAP&lt;K, fp32&gt;, etc. A parameters is named with a simple UTF8 character or string.

### Direct Return Types

A direct return type is one that is fully known at function definition type. Example simple direct return types would be things such as i32, fp32. For compound types, a direct return type must be fully declared. Example fully defined types: VARCHAR(20), DECIMAL(25,5)

### Return Type Expressions

Any function can declare a return type expression. A return type expression uses a simplified set of expressions to describe how the return type should be returned. For example, a return expression could be as simple as the return of parameter declared in the arguments. For example `f(List<K>) =>K ` or can be a simple mathematical or conditional expression such as `add(decimal(a,b), decimal(c,d)) => decimal(a+c, b+d)`. For the simple expression language, there is a very narrow set of types:

* Integer: 64 bit signed integer (can be a literal or a parameter value)
* Boolean: True and False
* Type: A Substrait type (with possibly additional embedded expressions)

These types are evaluated using a small set of operations to support common scenarios. Since it is possible that output derivation ultimately determines that binding is not allowed, a `not_bindable` operation is provided that states that the particular binding is not allowed. List of valid operations:

```
Math: +, -, *, /, min, max
Boolean: &&, ||, !, <, >, ==
Input Parameters: type parameter, literal parameter
Literals: integer, type
Special: !bindable
```

Fully defined with argument types:

* `not(boolean x) => boolean` 
* `and(boolean a, boolean b) => boolean` 
* `or(boolean a, boolean b) => boolean` 
* `multiply(integer a, integer b) => integer`
* `divide(integer a, integer b) => integer`
* `add(integer a, integer b) => integer`
* `subtract(integer a, integer b) => integer`
* `min(integer a, integer b) => integer`
* `max(integer a, integer b) => integer`
* `equal(integer a, integer b) => boolean`
* `greater_than(integer a, integer b) => boolean`
* `less_than(integer a, integer b) => boolean`
* `equal(Type a, Type b) => boolean`
* `if(boolean a) then (integer) else (integer)`
* `if(boolean a) then (type) else (type)`
* `not_bindable()`

#### Example Type Expressions Uses

For reference, here are are some common output type derivations and how they can be expressed with a return type expression:

| Operation                                     | Definition                                                   |
| --------------------------------------------- | ------------------------------------------------------------ |
| Add item to list                              | `add(<List<T>, T>) => List<T>`                               |
| Decimal Division                              | `divide(Decimal(P1,S1), Decimal(P2,S2)) => Decimal(P1 -S1 + S2 + MAX(6, S1 + P2 + 1), MAX(6, S1 + P2 + 1))` |
| Do regex on only string maps to return values | `extract_values(Map<K,V>) => if(K==STRING OR K==VARCHAR(*)) THEN LIST<V> ELSE !bindable` |





