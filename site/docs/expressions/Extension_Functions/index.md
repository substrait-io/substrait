# Basics

Rather than specifying a wide variety of operators and standard functions to express behavior in expression context, Substrait only specifies a bare minimum of special expression types (field references, literals, casts, a few conditionals, and subqueries), and covers the rest with generalized functions. Function implementations can be embedded in a plan, but more commonly are declared using simple extensions. This allows the implementation of a function's behavior to be decoupled from its behavior specification, such that Substrait plans only have to be concerned with the latter.

The expectation is that each consumer will use its own extensions to describe exactly the set of functions they support internally. This allows producers to generate plans that are tailored specifically to a particular consumer.

!!! note

    Outside of Substrait, the term "user-defined function" (or just UDF) refers to a function provided by the *end* user, i.e. the producer. Within Substrait, however, the term "user-defined function" usually refers to a consumer-specific function. This is the exact opposite in practice, so be careful not to confuse the terms! What's normally called a UDF is called an "[embedded function](../embedded_functions.md)" in Substrait, instead.

Substrait also defines a number of functions of its own. These functions provide primitives like arithmetic operators, inequalities, basic string manipulation, timestamp handling, and so on. While they are *inspired* by the functions provided by existing database management systems, they do not necessarily match any particular implementation exactly; rather, they aim to provide the building blocks needed to represent the behavior of implementations in general. This allows producers to also generate plans that are *not* tailored to any particular consumer yet; rather than defining exactly which physical function implementations they want the consumer to use, they specify the desired behavior, and leave it up to the consumer to choose suitable implementations.

Supporting *both* consumer-specific and consumer-agnostic plans is useful because it allows generic, vendor-agnostic transformations to be written. Examples might include (cross) compilers, optimizers, and planners. The ability to centralize the development of such tools helps avoid duplication of this logic across vendors. Furthermore, by using the *same* format for consumer-specific and consumer-agnostic plans (and indirectly also supporting everything in between), such tools can also be chained together. A complete pipeline might consist of a SQL parser from vendor A, a generic optimizer from vendor B, a more engine-specific optimizer from vendor C, a generic planner and distribution engine from vendor D, finally resulting in a number of distributed plans to be run on an otherwise single-node query engine from vendor C, all managed by a framework from vendor E. The vendors can now focus on what they're good at, rather than having to do a little bit of everything, and the user can swap out bits of the pipeline as their needs change without breaking their workflow.

For all of this to work and actually be generic, though, the generic tools need to be able to manipulate consumer-specific functions, at least to some degree. For example, in order to do common subexpression elimination, a transformation tool needs to know whether the functions used in the common subexpression are deterministic. If consumer-specific functions are basically black boxes, the tool would not be able to make this assumption. Thus, Substrait requires all function extensions to be declared using [simple extensions](../../extensions/#simple-extensions). Even the consumer-agnostic functions defined by Substrait are published as extensions; we call these the "core" extensions. The expectation is that a plan that uses only core extensions is consumer-agnostic, although a consumer may not support everything supported by core Substrait without some compilation happening in between.

While a function declaration *may* include everything down to the implementation of the function (expressed in formats such as WebAssembly or a pickled Python function), this is not required, and as such the provided information is usually not enough for a *machine* to execute the function. However, all behavior that cannot easily be expressed generically in a machine-readable way can instead be covered using descriptions written in natural language. This also makes the extensions useful as a source of documentation. This documentation use case is also the primary reason why simple extension files are normally written in a human-friendly YAML format, rather than in a more machine-friendly format like protobuf.

!!! note

    The first producer and last consumer in the pipeline typically don't need to use the extension files for anything. The information contained in them is redundant, because the consumer already needs an innate understanding of what the functions do in order to implement them, and the producer typically needs to know what the functions do in order to use them. In fact, the extension files need not necessarily even exist for a compatible consumer/producer pair to exchange a Substrait plan successfully. That being said, it is strongly recommended for consumers to publish simple extension files, if only for the purpose of documentation.

Of particular importance for transformation tools is the ability to do type propagation, and to determine whether a candidate function is usable in a certain context. Therefore, extension files must include machine-readable specifications for constraints on the use of a function, and for the data type returned by a function. The majority of this is described using a domain-specific mini-language, operating on a vastly simplified type system that we refer to as the [meta type system](../../types/meta_type_system.md) (named such because data types are a *value* in that system, allowing them to be manipulated). In broad terms, the programs formed using this mini language are used to match the data types of an incoming argument pack against a number of patterns, and ultimately evaluate the data type returned by the function based on how these patterns matched.

!!! note

    While the system is *very* minimalistic compared to a typical programming language, the exact specifications of programming languages tend to be hard to read, and this is no exception. Since the majority of our users need not concern themselves with the implementation, we will describe the commonly-used features using intuitive language and examples in the following subsections. When in doubt, refer to the more in-depth specification in the [Meta Type System](../../types/meta_type_system.md) section. The definitions in that section, and in particular the ANTLR grammar specification, are leading in case of conflict.

## Function Declarations and Identification

As stated, functions are specified using [simple extension files](../../extensions/#simple-extensions). In order for a plan to bind a function, it must thus somehow refer to the extension file and to a particular function declared within it. The former is always done using a URI within Substrait; the latter is done via case-insensitive name matching. A function name can be any non-empty UTF-8 string.

In addition to the name, a function declaration typically includes a behavior description written in a natural language (typically English). The purpose of this description is to capture all behavior of the function that the declaration cannot describe in a machine-readable way. For example, Substrait does not provide a structured way to indicate that a particular function adds two numbers or matches a string against a regular expression. Its purpose is similar to that of a docstring; in conjunction with the name and structured behavior specifications, it should be clear to a person how the function behaves for all supported inputs.

Substrait allows multiple function declarations within a single extension to share the same simple name. In this case, functions must be referred to by their [compound name](../../extensions/#function-signature-compound-names). The compound name is derived from the data type patterns provided by the function declaration. The compound names resulting from this derivation must be unique among all function declarations within the scope of an extension file. In addition to the above, a single function declaration may accept a variety of argument packs. The reason for this complexity is to be able to describe functions and operators that exist within the comparatively weak type systems typically employed by query engines.

For example, a simple function that adds two `i8` numbers in a source language where nullability is not a first-class concept actually requires two implementations, one nullable (`add(i8?, i8?) -> i8?`) and one non-nullable (`add(i8, i8) -> i8`). This captures the behavior that if neither input can be nullable, the output can never be nullable either, but if either input is null, the output will be null. Because of how common this pattern is, Substrait only requires you to specify the `add(i8, i8) -> i8` variant, and by default automatically derives the nullable variant, and in fact also the mixed cases (`add(i8?, i8) -> i8?` and `add(i8, i8?) -> i8?`), so you don't need to promote a non-nullable type to a nullable type using a cast to match the other argument. We call this `MIRROR` nullability.

Variadic functions are another example. We might for example want to define our `i8` addition function such that it can take any number of arguments. This definition technically matches an infinite number of argument packs, especially when we combine it with `MIRROR` nullability: the number of implementations is then exponential with the maximum number of arguments.

Of course, addition is normally defined on all numeric types, not just `i8`, and some of these types have much more complicated rules. Take decimal addition as an example:

```
add(decimal<P1,S1>, decimal<P2,S2>) ->
  init_scale = max(S1,S2)
  init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
  min_scale = min(init_scale, 6)
  delta = init_prec - 38
  prec = min(init_prec, 38)
  scale_after_borrow = max(init_scale - delta, min_scale)
  scale = if init_prec > 38 then scale_after_borrow else init_scale
  DECIMAL<prec, scale>
```

While it's probably possible to construct constraints for a single function declaration that cover a whole variety of numeric types in one go, it often makes more sense to describe the individual types independenty. This makes it easier for a person to understand the extension, and probably more closely matches the implementation anyway: an execution engine is likely to use a completely different implementation for adding integers than it does for adding floating-point numbers or decimals, but will likely have a single implementation that covers all variations of decimals.

!!! note

    The above means that the compound name of a function does *not* in general uniquely identify the argument pack expected by a function; it merely identifies a declaration. The declaration may be as generic as `function(T...) -> T`, which, depending on the nullability and variadic behavior specifications, may support *any* argument pack. Nevertheless, the compound name for this function will be `function:any`, regardless of the actual arguments bound to the function. Phrased differently, it is not possible to determine the compound name of a function using only a simple name and the data types of the bound arguments; you need the actual list of declarations and their argument patterns for that as well.

## Function Types

Substrait functions are distinguished by the vector/scalar nature of their inputs and outputs. This leads to four function types.

| Function type                       | Input  | Output | Contexts                                                                       |
|-------------------------------------|--------|--------|--------------------------------------------------------------------------------|
| [Scalar](scalar_functions.md)       | Scalar | Scalar | Scalar expressions, evaluated piecewise.                                       |
| [Aggregate](aggregate_functions.md) | Vector | Scalar | Measures in aggregation relations.                                             |
| [Window](window_functions.md)       | Vector | Vector | Scalar expressions, including context from some number of neighboring records. |
| [Table](table_functions.md)         | Scalar | Vector | TBD                                                                            |

The properties specific to these function types are described in more detail in the linked sections.

!!! note

    Do not confuse the scalar/vector nature of a function with the nestedness of the argument and return types. A scalar function may operate on or return `LIST` types, for example, and still be called scalar in this context. Scalar vs. vector refers to whether the function implicitly operates on/returns just one instance of the specified data types, or multiple of them.

## Arguments

When a function is used in a plan, it is bound to an argument pack, consisting of zero or more individual arguments. Currently, all arguments are positionally matched and mandatory; that is, the number of arguments bound to a function must match the number of argument slots in the declaration exactly, except when the last argument slot is variadic.

There are four main types of arguments: value arguments, type arguments, required enumerations, and optional enumerations. These are described in detail in the following subsections.

### Value Arguments

Value arguments are arguments that refer to a data value. These could be constants (literal expressions defined in the plan) or variables (a reference expression that references data being processed by the plan). This is the most common type of argument. The data type expected by the argument is defined using a [data type pattern](#data-type-patterns).

| Property | Description                                     | Required |
|----------|-------------------------------------------------|----------|
| Metadata | See [common metadata](#common-metadata).        | Optional |
| Pattern  | A [data type pattern](#data-type-patterns).     | Required |
| Constant | Whether this argument is required to be a constant for invocation. For example, in some system a regular expression pattern would only be accepted as a literal and not a column value reference. | Optional, defaults to false |

!!! note

    The *value* of the argument is not usable for return type derivations and constraints, even for constant value arguments. For example, it is not currently possible to assert that a particular function only accepts `i8` values between 0 and 100, nor is it possible to declare a function like `new_zero_decimal(P: i8, S: i8) -> DECIMAL<P,S>`.

!!! note

    Constant value arguments effectively always act like scalars, even for function types that take vectors as input. For example, an aggregate function like `count_matching(regex: const string, values: string) -> i64` would always only need to compile the passed regular expression once, while `count_matching(regex: string, values: string) -> i64` is expected to work even if the regex varies from record to record.

### Type Arguments

Type arguments are arguments that are used only to inform the evaluation and/or type derivation of the function. For example, you might have a function which is `truncate(type DECIMAL<P0,S0>, DECIMAL<P1, S1>, i32)`. This function declares two value arguments and a type argument. The difference between them is that the type argument has no value at runtime, while the value arguments do.

| Property | Description                                     | Required |
|----------|-------------------------------------------------|----------|
| Metadata | See [common metadata](#common-metadata).        | Optional |
| Pattern  | A [data type pattern](#data-type-patterns).     | Required |

### Required Enumeration Arguments

Required enumerations are arguments that support a fixed set of declared values as constant arguments. These arguments must be specified as part of an expression. While these could also have been implemented as constant string value arguments, they are formally included to improve validation/contextual help/etc. for frontend processors and IDEs. An example might use might be `extract({DAY, YEAR, MONTH}, date) -> i32`. In this example, a producer must specify a type of date part to extract. Note that the value of a required enumeration cannot be used in return type derivation.

| Property | Description                                     | Required |
|----------|-------------------------------------------------|----------|
| Metadata | See [common metadata](#common-metadata).        | Optional |
| Options  | List of valid string options for this argument. | Required |

### Optional Enumeration Arguments

Optional enumeration arguments are similar to required enumeration arguments, but are more focused on supporting alternative behaviors, usually for corner cases. An optional enumeration always includes an "unspecified" default option that can be bound based on the capabilities of the plan consumer. When a plan does not specify a behavior, the consumer is expected to resolve the option based on the first option the system can match. An example use case might be `OVERFLOW_BEHAVIOR: {OVERFLOW, SATURATE, ERROR}`. If unspecified, an engine would use the first of these that it implements. If specified, the engine would be expected to behave as specified or fail. Note that the value of an optional enumeration cannot be used in type derivation.

| Property | Description                                     | Required |
|----------|-------------------------------------------------|----------|
| Metadata | See [common metadata](#common-metadata).        | Optional |
| Options  | Priority-ordered list of valid string options for this argument. The pseudo-option will be the default "value" for the enumeration unless a binding specifies a specific value. | Required |

!!! note

    Despite being classified as "optional," optional enumeration argument slots must still be bound; unlike required enumeration arguments, however, they may be bound to a special "unspecified" value.

### Common Metadata

All arguments share the same metadata properties.

| Property    | Description                                                  | Required |
|-------------|--------------------------------------------------------------|----------|
| Name        | A human-readable name for this argument to help clarify use. | Optional |
| Description | Additional multi-line information about the argument.        | Optional |

### Data Type Patterns

Data type patterns are used to constrain the data types supported by value and type arguments, and to bind (parts of them) to names, to be reused by later patterns or the return type derivation.

For the most part, data type patterns are a generalization of the [syntax](../../types/meta_type_system/#syntax-parsing) Substrait uses to describe concrete types, that also allows types to be specified partially, by replacing a type class name or a parameter with an otherwise unused name. This tries to bind said name to the part of the pattern that was matched, while only allowing the name to be bound to one single partial value or type. For example, a function like `concatenate(FIXEDCHAR<A>, FIXEDCHAR<B>) -> FIXEDCHAR<A + B>` will bind `A` to the length of the first `FIXEDCHAR` and `B` to the length of the second. If instead we'd write something like `is_equal(FIXEDCHAR<A>, FIXEDCHAR<A>) -> boolean` however, `A` is bound to *both* the lengths, and thus the lengths of both arguments must be equal. In extreme cases, we might replace an entire data type with a letter; for example, in `coalesce(T, T) -> T`, both arguments must have the same data type (though there may be exceptions for [nullability](#nullability)), but any data type will match.

The above examples probably cover 99.9% of all practical function declarations. Refer to the section on [metapatterns](../../types/meta_type_system/#metapatterns) for a more precise description or for the remaining 0.1%.

### Nullability

Most SQL-inspired functions, in particular scalar functions, are defined to return null if any only if any argument is null. While this behavior could be implemented in Substrait by accepting only nullable arguments and always returning a nullable data type, we can do better than that: we can define a function such that it accepts any combination of nullabilities for its arguments, and returns nullable if and only if any argument is nullable. This is worth doing because knowing that something can never be nullable may open up possibilities for optimizations. We call this `MIRROR` nullability.

Another common pattern is for a function to accept any combination of nullabilities for its arguments, but always return a nullable or non-nullable result. Most SQL-inspired aggregate functions behave like the former; they ignore nulls at the input, and return null if the input is empty. A function like `is_null(T) -> boolean` is an example of the latter; it will by definition always return true or false. We call this `DECLARED_OUTPUT` nullability.

Because it is rather cumbersome to write these behaviors down using the metapattern system, the YAML format provides syntactic sugar for it by way of the nullability mode parameter.

| Nullability mode   | Desugaring behavior                                                                                                                                        |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `MIRROR` (default) | Modify nullability of argument and return patterns to accept any combination of nullabilities and return nullable if and only if any argument is nullable. |
| `DECLARED_OUTPUT`  | Modify nullability of argument patterns to accept any combination of nullabilities.                                                                        |
| `DISCRETE`         | Do not modify any patterns.                                                                                                                                |

The exact effect of the first two modes is that the nullability suffix of any toplevel data type or binding pattern used in an argument is replaced with `??nullable`. For `MIRROR`, the same is also done for the return type. For example, the function `coalesce(T, T) -> T` with `MIRROR` nullability means the same as `coalesce(T??nullable, T??nullable) -> T??nullable` with `DISCRETE` nullability. In this pattern, the [nullability suffix](../../types/meta_type_system/#nullability-suffixes) in `T?X` means "match data types with nullability `X` (expressed as a boolean)," and the [inconsistent binding pattern](../../types/meta_type_system/#bindings) `?nullable` in [this context](../../types/meta_type_system/#mirror-and-declared-output-nullability) means "match any boolean, or return the boolean OR of any booleans matched previously."

Note that, armed with this knowledge, we can do much better for coalesce and define it as follows: `coalesce(T?, T?nullable) -> T?nullable`. Now the first argument must always be nullable (otherwise the function would be no-op) and the function is specified to only return a nullable type if the inputs can both be null. We have to use `DISCRETE` nullability here though, or the default `MIRROR` behavior will cause our nullability suffixes to be ignored.

### Variadics

Some functions are variadic in such a way that it's either impractical to specify all argument pack variations exhaustively, or downright impossible because there is no well-defined upper limit. An example is a string formatting function that takes a format argument, followed by any number of subsequent arguments (corresponding to the format string). We might want to write something like this: `string_format(const string, T...) -> string`.

To support declaration of such functions, Substrait allows the last argument *slot* of the function to bind any number of *actual* arguments. The slot will match one or more actual arguments unless otherwise specified, but a custom minimum (including 0) and maximum number can be specified as well.

When the last argument slot uses a binding like `T`, we need some extra information: does our `string_format` declaration above require all the actual arguments mapping to `T` to be the *same* `T`? That's probably not what we meant for `string_format`, but it probably *is* what we mean for `coalesce(T...) -> T`. To make it easier to specify the difference, we can mark a variadic function as either `CONSISTENT` (like `coalesce`) or `INCONSISTENT` (like `string_format`).

| Consistency            | Intuitive behavior                                                            | Desugaring behavior                                                                        |
|------------------------|-------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| `CONSISTENT` (default) | Names used in the variadic argument slot refer to the same type/value.        | Do not modify any patterns.                                                                |
| `INCONSISTENT`         | Names used in the variadic argument slot can refer to different types/values. | Replace all consistent bindings used in the last argument slot with inconsistent bindings. |

Just like the nullability mode, we can also just write what we mean directly with patterns, the syntax just gets a little weird. In this case, the `string_format` declaration above could also be written as `string_format(const string, ?T...) -> string` explicitly to avoid needing the desugaring behavior; the `?` in front of the `T` means that the first usage of `T` will bind it, but later matches will ignore the previously bound value or type and match anything that the pattern would have matched if it had not been bound yet. The exact behavior is specified [here](../../types/meta_type_system/#inconsistent-variadic-argumentparameter-slots).

## Return Values

Functions in Substrait return either a scalar or vector (depending on the [function type](#function-types)) of a single data type, and it must be possible to unambiguously derive what this data type is when given only the actual argument types and a particular function declaration. This derivation is accomplished with the same syntax as the argument data type patterns, but rather than matching an incoming data type, we use the pattern to generate a data type.

Note that this imposes some constraints on the allowed patterns. For example, you can't define a function like `cast(T) -> S` because `S` is not yet defined and is therefore ambiguous. Most patterns have well-defined semantics in both match and evaluation context, though.

### Expressions

For some functions, such as those operating on `FIXEDCHAR` or `DECIMAL` types, we may need to do some math to derive the return type. Here's a simple example: `concatenate(FIXEDCHAR<A>, FIXEDCHAR<B>) -> FIXEDCHAR<A + B>`.

To keep things simple, Substrait defines only a basic set of operators, functions, and metatypes for type derivations, and this set of functions is not extensible. The complete list can be found [here](../../types/meta_type_system/#functions).

!!! note

    These functions are not to be confused with extension functions or embedded functions, or anything else that exists when a plan is being executed. They exist on a completely different abstraction layer! In fact, we need these functions in order to be able to even begin thinking about defining the more complex functions that we use in plans, and we can't use functions to define themselves.

## Derivation Programs

For more advanced functions, like the decimal addition we used as an example earlier, a single pattern is just not enough to derive the return type. For this purpose, the return type can include a number of statements before the final pattern, separated from each other and the pattern by newlines or semicolons. These statements more or less look and behave like assignment statements, except that the left-hand side can be any pattern, and the assignment utilizes the same matching process used for arguments.

!!! warning

    The semantics for matching mean that the names used in these assignment statements don't act like variables when they are reused! For example, `bad_concatenate(FIXEDCHAR<A>, FIXEDCHAR<B>) -> A = A + B; FIXEDCHAR<A>` will *not* behave like the `concatenate` example from before, because unless `B` equals 0, the statement would fail to match `A` against its intended new value.

This simple rule not only lets us eliminate common subexpressions to make the decimal `add` somewhat comprehensible, but it also lets us write essentially arbitrary constraints. For example, we could define an integer addition that works with any combination of integers as follows:

```
casting_add(LHS, RHS, type RETURN) ->
  true = LHS == i8 || LHS == i16 || LHS == i32 || LHS == i64
  true = RHS == i8 || RHS == i16 || RHS == i32 || RHS == i64
  true = RETURN == i8 || RETURN == i16 || RETURN == i32 || RETURN == i64
  RETURN
```

!!! note

    We *have* to specify the return type as a type argument here, because otherwise the return type would be ambiguous.

The `true = ...` pattern works because the `true` pattern *matches* `true`, so as long as we assign something that is also true, the match will succeed. However, because this pattern looks rather odd and is relatively common, we also have some syntactic sugar for it: `assert ...`. So, we can rewrite the above declaration a bit nicer as follows:

```
casting_add(LHS, RHS, type RETURN) ->
  assert LHS == i8 || LHS == i16 || LHS == i32 || LHS == i64
  assert RHS == i8 || RHS == i16 || RHS == i32 || RHS == i64
  assert RETURN == i8 || RETURN == i16 || RETURN == i32 || RETURN == i64
  RETURN
```
