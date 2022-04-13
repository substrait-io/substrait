# Scalar Functions

A function is a scalar function if that function takes in values from a single record and produces an output value. To clearly specify the definition of functions, Substrait declares a extensible specification plus binding approach to function resolution. A scalar function signature includes the following properties:

| Property               | Description                                                  | Required                            |
| ---------------------- | ------------------------------------------------------------ | ----------------------------------- |
| Name                   | One or more user friendly utf8 strings that are used to reference this function in languages | At least one value is required.     |
| List of arguments      | Argument properties are defined below. Arguments can be fully defined or calculated with a type expression. See further details below. | Optional, defaults to niladic.      |
| Deterministic          | Whether this function is expected to reproduce the same output when it is invoked multiple times with the same input. This informs a plan consumer on whether it can constant reduce the defined function. An example would be a random() function, which is typically expected to be evaluated repeatedly despite having the same set of inputs. | Optional, defaults to true.         |
| Session Dependent      | Whether this function is influenced by the session context it is invoked within. For example, a function may be influenced by a user who is invoking the function, the time zone of a session, or some other non-obvious parameter. This can inform caching systems on whether a particular function is cacheable. | Optional, defaults to false.        |
| Variadic Behavior      | Whether the last argument of the function is variadic or a single argument.  If variadic the argument can optionally have a lower bound (minimum number of instances) and upper bound (maximum number of instances) | Optional, defaults to single value. |
| Nullability Handling | Describes how nullability of input arguments maps to nullability of output arguments. Three options are: `MIRROR`, `DECLARED_OUTPUT` and `DISCRETE`. More details about nullability handling are listed below. | Optional, defaults to `MIRROR` |
| Description            | Additional description of function for implementers or users. Should be written human readable to allow exposure to end users. Presented as a map with language => description mappings. E.g. `{ "en": "This adds two numbers together.", "fr": "cela ajoute deux nombres"}`. | Optional                            |
| Return Value | The output type of the expression.  Return types can be expressed as a fully-defined type or a type expression. See below for more on type expressions. | Required                            |
| Implementation Map     | A map of implementation locations for one or more implementations of the given function. Each key is a function implementation type. Implementation types include examples such as: AthenaArrowLambda, TrinoV361Jar, ArrowCppKernelEnum, GandivaEnum, LinkedIn Transport Jar, etc. [Definition TBD]. Implementation type has one or more properties associated with retrieval of that implementation. | Optional                            |



## Argument Types

There are two main types of arguments: Value Arguments and Type Arguments. 

* Value Arguments: An argument that refers to a data value.  This could be a constant (a literal expression defined in the plan) or variable (A reference expression that references data being processed by the plan). This is the most common type of argument. Value arguments are not available in output derivation. Value arguments can be declared in one of two ways: concrete or parameterized. Concrete Types are either simple types or compound types with all parameters fully defined (without referencing any type arguments). Examples include i32, fp32, VARCHAR(20), List&lt;fp32&gt;, etc. Parameterized types are discussed further below.
* Type Arguments: Type arguments are used to inform the evaluation and/or type derivation of the function. For example, you might have a function which is `truncate(<type> DECIMAL(P0,S0), <value> DECIMAL(P1, S1), <value> i32)`. This function declares two value arguments and a type argument. The type argument's value can be used to determine the output type (The types of the values arguments can also be used for output type derivation.)
* Required Enumeration: Required enumeration arguments are arguments that support a fixed set of declared values as constant arguments. These arguments must be specified as part of an expression. While these could be implemented as constant string value arguments, they are formally included to improve validation/contextual help/etc for frontend processors and IDEs. An example might use might be `extract([DAY|YEAR|MONTH], <date value>)`. In this example, a producer must specify a type of date part to extract. Note, the value of a required enumeration cannot be used in type derivation.
* Optional Enumeration: Similar to required enumeration but more focused on supporting alternative behaviors. An optional enumeration always includes an "unspecified" default option that can be bound based on the capabilities of the plan consumer. When a plan does not specify a behavior, the consumer is expected to resolve the option based on the first option the system can match. An example usecase might be `OVERFLOW_BEHAVIOR:[OVERFLOW, SATURATE, ERROR]` If unspecified, an engine would use the first of these that it implements. If specified, the engine would be expected to behave as specified or fail. Note, the value of an optional enumeration cannot be used in type derivation.

#### Value Argument Properties

| Property | Description                                                  | Required                                                   |
| -------- | ------------------------------------------------------------ | ---------------------------------------------------------- |
| Name     | A human readable name for this argument to help clarify use.          | Optional, defaults to a name based on position (e.g. arg0) |
| Type     | A fully defined type or a type expression.            | Required                                                   |
| Constant | Whether this argument is required to be a constant for invocation. For example, in some system a regular expression pattern would only be accepted as a literal and not a column value reference. | Optional, defaults to false                                |

#### Type Argument Properties

| Property | Description                                                  | Required                                                   |
| -------- | ------------------------------------------------------------ | ---------------------------------------------------------- |
| Type     | A partially or completely parameterized type. E.g. List&lt;K&gt; or K | Required                                                   |
| Name     | A human readable name for this argument to help clarify use.          | Optional, defaults to a name based on position (e.g. arg0) |

#### Required Enumeration Properties

| Property | Description                                                  | Required                                                   |
| -------- | ------------------------------------------------------------ | ---------------------------------------------------------- |
| Options  | List of valid string options for this argument               | Required                                                   |
| Name     | A human readable name for this argument to help clarify use. | Optional, defaults to a name based on position (e.g. arg0) |

#### Optional Enumeration Properties

| Property | Description                                                  | Required                                                   |
| -------- | ------------------------------------------------------------ | ---------------------------------------------------------- |
| Options  | Priority-ordered list of valid string options for this argument. The pseudo-option will be the default "value" for the enumeration unless a binding specifies a specific value. | Required                                                   |
| Name     | A human readable name for this argument to help clarify use. | Optional, defaults to a name based on position (e.g. arg0) |



### Nullability Handling

| Mode            | Description                                                  |
| --------------- | ------------------------------------------------------------ |
| MIRROR          | This means that the function has the behavior that if at least one of the input arguments are nullable, the return type is also nullable. If all arguments are non-nullable, the return type will be non-nullable.  An example might be the `+` function. |
| DECLARED_OUTPUT | Input arguments are accepted of any mix of nullability. The nullability of the output function is whatever the return type expression states. Example use might be the function `is_null()` where the output is always `boolean` independent of the nullability of the input. |
| DISCRETE        | The input and arguments all define concrete nullability and can only be bound to the types that have those nullability. For example, if a type input is declare `i64?` and one has a `i64` literal, the `i64` literal must be specifically cast to `i64?` to allow the operation to bind. |



### Parameterized Types

Types are parameterized by two types of values: by inner types (e.g. `List<K>`) and numeric values (e.g. `DECIMAL(P,S)`). Parameter names are simple strings (frequently a single character). There are two types of parameters: integer parameters and type parameters. 

When the same parameter name is used multiple times in a function definition, the function can only bind if the exact same value is used for all parameters of that name. For example, if one had a function with a signature of `fn(VARCHAR(N), VARCHAR(N)), the function would be only be usable if both VARCHAR types had the same length value N. This necessitates that all instances of the same parameter name must be of the same parameter type (all instances are a type parameter or all instances are an integer parameter).

#### Type Parameter Resolution in variadic functions

When the last argument of a function is variadic and declares a type parameter e.g. `fn(A, B, C...)`, the C parameter can be marked as either consistent or inconsistent. If marked as consistent, the function can only be bound to arguments where all of the C types are the same concrete type. If marked as inconsistent, each unique C can be bound to a different type within the constraints of what T allows.

 

## Output Type Derivation

### Concrete Return Types

A concrete return type is one that is fully known at function definition time. Example simple concrete return types would be things such as i32, fp32. For compound types, a concrete return type must be fully declared. Example of fully defined compound types: VARCHAR(20), DECIMAL(25,5)

### Return Type Expressions

Any function can declare a return type expression. A return type expression uses a simplified set of expressions to describe how the return type should be returned. For example, a return expression could be as simple as the return of parameter declared in the arguments. For example `f(List<K>) =>K ` or can be a simple mathematical or conditional expression such as `add(decimal(a,b), decimal(c,d)) => decimal(a+c, b+d)`. For the simple expression language, there is a very narrow set of types:

* Integer: 64 bit signed integer (can be a literal or a parameter value)
* Boolean: True and False
* Type: A Substrait type (with possibly additional embedded expressions)

These types are evaluated using a small set of operations to support common scenarios. List of valid operations:

```
Math: +, -, *, /, min, max
Boolean: &&, ||, !, <, >, ==
Parameters: type, integer
Literals: type, integer
```

Fully defined with argument types:

* `type_parameter(string name) => type`
* `integer_parameter(string name) => integer`
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
* `covers(Type a, Type b) => boolean`Covers means that type b matches type A for as much as type B is defined. For example, if type A is `VARCHAR(20)` and type B is `VARCHAR(N)`, type B would be considered covering. Similarlily if type A was `List<Struct<a:f32, b:f32>>`and type B was `List<Struct<>>`, it would be considered covering. Note that this is directional "as in B covers A" or "B can be further enhanced to match the definition A". 
* `if(boolean a) then (integer) else (integer)`
* `if(boolean a) then (type) else (type)`

#### Example Type Expressions

For reference, here are are some common output type derivations and how they can be expressed with a return type expression:

| Operation                                                    | Definition                                                   |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| Add item to list                                             | `add(<List<T>, T>) => List<T>`                               |
| Decimal Division                                             | `divide(Decimal(P1,S1), Decimal(P2,S2)) => Decimal(P1 -S1 + S2 + MAX(6, S1 + P2 + 1), MAX(6, S1 + P2 + 1))` |
| Concatenate two fixed sized character strings                | `concat(FIXEDCHAR(A), FIXEDCHAR(B)) => FIXEDCHAR(A+B)`       |
| Make a struct of a set of fields and a struct definition.    | `make_struct(<type> T, K...) => T`                           |



