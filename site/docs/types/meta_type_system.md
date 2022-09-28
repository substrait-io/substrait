# Meta Type System

In addition to the data type system, Substrait defines a second, much more restricted type system that is internally used for type parameters, constraint patterns for function argument types, return type derivations, and similar static constructs. In addition to basic scalar primitive types, data types (from the normal type system) are *themselves* values in this system, allowing them to be manipulated.

The meta type system also includes a syntax definition. Combined with the pattern matching and evaluation semantics, it effectively forms a tiny domain-specific language. The syntax for data types especially is also used throughout the documentation.

The grammar for the language can be found in ANTLR form [here](https://github.com/substrait-io/substrait/blob/main/text/SubstraitType.g4). Note that this grammar includes extensive comments about how to interpret the various syntax rules. These comments are leading in case they conflict with the definitions on this page.

## Metatypes and Values

Unlike the data type system, the meta type system is dynamically typed. This means that, ultimately, only one type needs to exist. Nevertheless, it makes sense to subdivide the values that this type can assume into groups, which we call metatypes.

| Name       | Description |
|------------|-------------|
| `metabool` | Contains the values `true` and `false`. |
| `metaint`  | Contains all integer values from `-2^63` to `2^63-1`, i.e. the 64-bit two's-complement integers. |
| `metaenum` | Contains the set of all case-insensitive strings that form valid identifiers (matching `[a-zA-Z_$][a-zA-Z0-9_$]*`). These are normally constrained to a particular subset in order to behave like a proper enumeration. |
| `metastr`  | Contains the set of all Unicode strings. |
| `typename` | Contains the set of all valid Substrait data type quadruplets (class, nullability, variation, and parameter pack). Parameters may include names in the binding for documentation purposes and/or to represent `NSTRUCT`. |

Note that metatypes cannot be extended, only include scalar types, and don't include a `null` type or value.

### Syntax Parsing

While the syntax for representing concrete metavalues is fully covered by the pattern syntax defined below, let us first introduce it outside of pattern context to ease the learning curve.

| Metatype     | Syntax for values |
|--------------|-------------------|
| Booleans     | `true` or `false`. Case insensitive, customarily written in lowercase. |
| Integers     | Represented using the usual syntax for decimal integers. In regex form: `[+-](0|[1-9][0-9]*)`. Integers in other bases (hexadecimal, binary, or octal) are not supported. |
| Enumerations | Represented using the enumeration variant identifier on its own. Case insensitive, customarily written in UPPERCASE. |
| Strings      | Represented using double quotes (`"`) as delimiters. Escape sequences are not currently supported, so `"` itself cannot be represented. Support for this may be added later by using two successive double quotes (`""`) as the escape sequence for a single `"`. |
| Data types   | See below. |

Data types are a bit more complicated. The basic structure is:

```
name?[variation]<param0,...,paramN>
```

The components of this expression are:

| Component             | Description | Required |
|-----------------------|-------------|----------|
| Name                  | Each type has a name, also referred to as the type class. The name is matched case-insensitively (e.g. `varchar` and `vArChAr` are equivalent), but is customarily written in lowercase for simple types and uppercase for compound types. | Yes |
| Nullability indicator | A type is either non-nullable or nullable. To express nullable types, a type name is appended with a question mark. The lack of a question mark suffix or (in rare occasions) an exclamation mark suffix expresses non-nullable. To explicitly refer to a type with either nullability, one may append two question marks. | Optional, defaults to non-nullable |
| Variation             | When expressing a type, a user can define the type based on a type variation. Some systems use type variations to describe different underlying representations of the same data type. Such custom variations are usually specified by their name between square brackets. Within the context of a plan, they may also be referred to by their anchor index. To refer to the implicit system-preferred variation, use `[0]` or leave the variation unspecified. | Optional, defaults to [0] |
| Parameters            | Compound types may have one or more configurable properties, specified via parameters. The expected number of parameters and their types is defined along with the type class. User-defined compound types may mark parameter slots as optional, in which case the parameter slot may be skipped using the `null` keyword. Non-null parameters may be named using `name: value` or `"name": value` syntax, where the latter must be used if `name` does not match `[a-zA-Z_$][a-zA-Z0-9_$]*`. | Required for (concrete) compound types, illegal for simple types |

Parameter names are currently only used for the `NSTRUCT` pseudotype. Here, each parameter corresponds with a field data type, and the parameter name is used to represent the field name. Note however that in core Substrait algebra fields are unnamed and references are always based on zero-based ordinal positions; the named structs are only intended to be used to annotate types used at the inputs and outputs of a plan to ease understanding for humans, and to support consumers and producers that rely on field names at the peripheries of the plan.

## Metapatterns

All operations on metavalues are done using metapatterns. Most metapattern can be conceptualized as a procedurally-generated *set* of metavalues. We then define two operations on these patterns:

| Name     | Approximate prototype        | Description |
|----------|------------------------------|-------------|
| Match    | `match(pattern, value)`      | Asserts that the given value is contained in the set represented by the given pattern. Throws an error if this is not the case. |
| Evaluate | `evaluate(pattern) -> value` | Asserts that the given pattern contains only one value, and then returns that value. Throws an error if this is not the case. |

Note that for some pattern types the set analogy does not apply exactly or is heavily influenced by context.

The following metapatterns are defined.

| Syntax                              | Name                  | Metavalues contained in set                                            |
|-------------------------------------|-----------------------|------------------------------------------------------------------------|
| `?`                                 | Any                   | All metavalues.                                                        |
| `metabool`                          | Boolean any           | `true` and `false`.                                                    |
| `true`                              | True                  | Only `true`.                                                           |
| `false`                             | False                 | Only `false`.                                                          |
| `metaint`                           | Integer any           | All integer metavalues.                                                |
| `<int>..`                           | Integer at least      | All integers greater than or equal to the given literal.               |
| `..<int>`                           | Integer at most       | All integers less than or equal to the given literal.                  |
| `<int>..<int>`                      | Integer within        | All integers between and including the given literals.                 |
| `<int>`                             | Integer exactly       | Only the given integer literal.                                        |
| `metaenum`                          | Enum variant any      | All enumeration variant metavalues.                                    |
| `{<ident>, ...}`                    | Enum variant set      | Exactly those enumeration variants specified between the curly braces. |
| `<ident>`*                          | Enum variant exactly  | Only the given enumeration variant.                                    |
| `metastr`                           | String any            | All string metavalues.                                                 |
| `<str>`                             | String exactly        | Exactly the given string, delimited by double quotes. Escape sequences are currently not defined, so strings containing `"` literally cannot be parsed. |
| `typename<null?>`                   | Type with nullability | All data type metavalues matching the given nullability pattern.       |
| `<ident><null?><var?><params?>`*    | Type with class       | All data type metavalues belonging to the given type class that match the given nullability, variation, and parameter pack suffixes. |
| `<ident><null?>`*                   | Consistent binding    | Context-sensitive. See section on bindings.                            |
| `?<ident><null?>`*                  | Inconsistent binding  | Context-sensitive. See section on bindings.                            |
| `<ident>(<pattern*>)` + expressions | Function call         | Only the metavalue returned by the function. See section on functions. |

Note some of these patterns are syntactically ambiguous. The parser must resolve the ambiguity based on what the identifier resolves to, being either a type class, an enum variant, or a binding. If a name does not resolve to anything, a binding is implicitly declared.

### Nullability Suffixes

Nullability suffixes are used to match or evaluate the nullability of a data type. The following nullability suffixes are defined.

| Syntax              | Name                    | Metavalues contained in set                                                                                           |
|---------------------|-------------------------|-----------------------------------------------------------------------------------------------------------------------|
| No suffix           | Non-nullable            | Non-nullable typenames and all non-typename metavalues.                                                               |
| `!` suffix          | Explicitly non-nullable | Non-nullable typenames only.                                                                                          |
| `?` suffix          | Nullable                | Nullable typenames only.                                                                                              |
| `?<pattern>` suffix | Pattern nullability     | Only typenames, with nullability as specified in the pattern, using `true` for nullable and `false` for non-nullable. |

For "type with class" patterns, not specifying a nullability suffix means the same thing as the `!` suffix. It is customarily omitted in this case. Because this is by far the most common pattern, you may not encounter it very often. For the other pattern types, however, not specifying a nullability suffix syntactically results in a different pattern type.

Some additional syntactic sugar exists for function definitions, to make common nullability behavior a bit more readable. This is specified in the YAML files by means of the `nullability` field. The effect of this field on the patterns is as follows:

| Syntax             | Effect                                                                                                                         |
|--------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `MIRROR` (default) | Replace all optional nullability patterns of toplevel argument, return type, and intermediate type patterns with `??nullable`. |
| `DECLARED_OUTPUT`  | Replace all optional nullability patterns of toplevel arguments with `??nullable`.                                             |
| `DISCRETE`         | No effect.                                                                                                                     |

In the above, "toplevel pattern" means that a pattern like `i8` on its own, but not like the `i8` in `STRUCT<i8>`. Combined with the semantics of inconsistent bindings, this replacement models the specified behavior of the `nullability` field exactly, so no further special cases are required to match functions against their prototypes.

For some exotic functions, `MIRROR` and `DECLARED_OUTPUT` may not capture the intended behavior exactly. For example, a function may have one argument that doesn't participate in the `MIRROR` or `DECLARED_OUTPUT` behavior, or the nullability of nested types may need to participate. In these cases, `DISCRETE` nullability must be used, and the nullability patterns must be specified manually.

### Type Variation Suffixes

Type variation suffixes are used to match or evaluate the type variation of a data type. In the syntax column of the pattern table, they are represented using `<var?>`. The following type variation suffixes are defined.

| Syntax             | Name                       | Type variations contained in set                  |
|--------------------|----------------------------|---------------------------------------------------|
| No suffix          | Compatible variations      | When matching, the system-preferred variation and any user-defined variations with `INHERIT` function behavior defined for the related type class. When evaluating, always returns the system-preferred variation. |
| `[?]` suffix       | Any variation              | Any variation defined for the related type class. |
| `[0]` suffix       | System-preferred variation | Only the system-preferred variation.              |
| `[<ident>]` suffix | User-defined variation     | Only the specified user-defined variation.        |

### Parameter Pack Suffixes

Parameter pack suffixes are used to match or evaluate the parameter packs of compound data types. In the syntax column of the pattern table, they are represented using `<params?>`. The following parameter pack suffixes are defined.

| Syntax                  | Name                    | Parameter packs contained in set                                                                                            |
|-------------------------|-------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| No suffix               | Any parameter pack      | All parameter packs that the related type class supports. When evaluating, always returns an empty parameter pack.          |
| `<>` suffix             | Empty parameter pack    | Only the empty parameter pack.                                                                                              |
| `<<param>, ...>` suffix | Matching parameter pack | Only parameter packs where each parameter matches the given parameter pattern. The number of parameters must match exactly. |

The parameter binding patterns themselves are defined as follows.

| Syntax      | Name                     | Parameter bindings contained in set                                 |
|-------------|--------------------------|---------------------------------------------------------------------|
| `null`      | Skipped parameter        | Matches only explicitly-skipped optional parameters.                |
| `?`         | Skipped or any metavalue | Matches any parameter, including skipped parameters.                |
| `<pattern>` | Pattern parameter        | Match the metavalue bound to the parameter using the given pattern. |

Parameter binding patterns can also be given an optional prefix to associated a name with the parameter. Note that names are only used when evaluating the pattern, and that they are currently only applicable to the `NSTRUCT` pseudotype.

| Prefix      | Name                     | Parameter bindings contained in set                        |
|-------------|--------------------------|------------------------------------------------------------|
| No prefix   | Unnamed parameter        | The parameter binding is unnamed.                          |
| `<ident>:`  | Named parameter          | The parameter binding is named using the given identifier. |
| `<str>:`    | String-named parameter   | The parameter binding is named using the given string.     |

### Bindings

"Binding" is the name we use for identifiers that get bound to a value. For example, if we define a function with prototype `func(T) -> T`, the identifier `T` is a binding.

The basic logic is quite simple: when a name is first used in a binding pattern, that name is bound to a value, and when the same name is used again later, the binding pattern will make use of the previously bound value. Another way to put it is that bindings behave more or less like single-assignment variables. The behavior is, however, complicated by nullability. In order to match the behavior of a pattern like `i32`, which only matches non-nullable `i32`s, `T` must also only match non-nullable patterns. Furthermore, if `T` were bound to a nullable type via `T?`, using `T` later would still refer to the non-nullable variant. This is captured by the following rules:

 - non-typename metavalues can only be bound and used by binding patterns with no nullability suffix;
 - if a typename is bound to a name, the bound typename is always non-nullable;
 - when a binding evaluates to a typename, the nullability suffix overrides the nullability.

We further distinguish between two separate sets of semantics for binding patterns: consistent and inconsistent bindings. Consistent bindings behave the most like single-assignment variables and are what you would normally use, whereas inconsistent bindings have special-cased semantics that are useful for representing `MIRROR` nullability and inconsistently-typed variadic argument/parameter slots. Note that inconsistent bindings only very rarely need to be specified manually. Refer to the subsections below for more information.

The exact semantics of the various binding patterns are as follows.

| Binding pattern type                  | Match, not yet bound | Match, previously bound | Evaluate, not yet bound | Evaluate, previously bound |
|---------------------------------------|----------------------|-------------------------|-------------------------|----------------------------|
| Consistent binding w/o nullability    | Match any metavalue, except nullable typenames. Bind name to matched value. | Match only the previously bound metavalue. | Always fails. | Evaluate to the previously bound metavalue. |
| Consistent binding with nullability   | Match any typename metavalue for which the nullability suffix matches. Bind name to matched value, but with nullability overridden to non-nullable. | Match only typename metavalues, and fail if the previously bound metavalue is not a typename. Match the class, variation, and parameter pack against the previously bound value. Match the nullability against the nullability suffix. Bound value is not modified. | Always fails. | Fail if the previously bound value is not a typename. Otherwise, evaluate to the previously bound metavalue, with nullability overridden by the evaluation result of the nullability pattern. |
| Inconsistent binding w/o nullability  | Match any metavalue, except nullable typenames. Bind name to matched value. | Match any metavalue, except nullable typenames. If the matched value is `true` and the previously bound value is `false`, rebind the name to `true`; otherwise, bound value is not modified. | Returns `false`. | Evaluate to the previously bound metavalue. |
| Inconsistent binding with nullability | Match any typename metavalue for which the nullability suffix matches. Bind name to matched value, but with nullability overridden to non-nullable. | Match only typename metavalues, and fail if the previously bound metavalue is not a typename. Only match the nullability against the nullability suffix. Bound value is not modified. | Always fails. | Fail if the previously bound value is not a typename. Otherwise, evaluate to the previously bound metavalue, with nullability overridden by the evaluation result of the nullability pattern. |

#### Inconsistent Variadic Argument/Parameter Slots

Variadic functions with `INCONSISTENT` argument behavior can be represented by using inconsistent bindings instead of consistent bindings. For example, a function defined like `func(?T...) -> T` with will return the type of the first argument passed to it but will allow any type to be specified for any additional arguments passed to it, whereas if it were defined like `func(T...) -> T`, all arguments must be of type `T`. As a more complex example, `func(STRUCT<?T, S>...) -> STRUCT<T, S>` will take two-tuples where the first field is inconsistent and the second is consistent.

The exact behavior of specifying that a function has `INCONSISTENT` argument behavior is for all bindings in the last argument slot to be implicitly turned inconsistent. `CONSISTENT` argument behavior has no effect. Therefore, specifications of exotic behavior like the `STRUCT` example above must use `CONSISTENT` argument behavior.

#### Mirror and Declared-Output Nullability

When acting on the nullability booleans (as used in the `??nullable` nullability suffixes for `MIRROR` and `DECLARED_OUTPUT`), inconsistent bindings will bind and yield true to `nullable` if and only if any nullable arguments were matched. The special case of returning `false` when the name is not yet bound handles niladic functions, or variadic functions with no bound arguments.

The binding pattern types with nullability override are automatically used in place of the regular versions when `MIRROR` or `DECLARED_OUTPUT` is specified.

### Functions

Function patterns allow a metavalue to be derived based on a number of other metavalues. All functions can be written using the usual `<ident>(<pattern*>)` syntax (for example `add(1, 3)`), but many functions can also be specified implicitly with infix operators (for example `1 + 3`).

Many of the functions have metatype requirements on their inputs. If a value is passed that does not match this metatype, evaluation fails.

The following functions are defined. Note that this set of functions cannot be user-extended.

| Prototype                                     | Description                                                                                          |
|-----------------------------------------------|------------------------------------------------------------------------------------------------------|
| `not(metabool) -> metabool`                   | Boolean NOT.                                                                                         |
| `and(metabool*) -> metabool`                  | Boolean AND. Evaluated lazily from left to right.                                                    |
| `or(metabool*) -> metabool`                   | boolean OR. Evaluated lazily from left to right.                                                     |
| `negate(metaint) -> metaint`                  | Integer negation. 64-bit two's complement overflow must be detected and cause evaluation to fail.    |
| `add(metaint*) -> metaint`                    | Integer sum. 64-bit two's complement overflow must be detected and cause evaluation to fail.         |
| `subtract(metaint, metaint) -> metaint`       | Integer subtraction. 64-bit two's complement overflow must be detected and cause evaluation to fail. |
| `multiply(metaint*) -> metaint`               | Integer product. 64-bit two's complement overflow must be detected and cause evaluation to fail.     |
| `divide(metaint, metaint) -> metaint`         | Integer division. Divisions by zero and 64-bit two's complement overflow (-2^63 / -1) must be detected and cause evaluation to fail. Divisions round toward zero. |
| `min(metaint+) -> metaint`                    | Returns the minimum integer value.                                                                   |
| `max(metaint+) -> metaint`                    | Returns the maximum integer value.                                                                   |
| `equal(T, T) -> metabool`                     | Returns whether the two metavalues are equal. Data type parameter names should be ignored.           |
| `not_equal(T, T) -> metabool`                 | Returns whether the two metavalues are not equal. Data type parameter names should be ignored.       |
| `greater_than(metaint, metaint) -> metabool`  | Returns whether the left integer is greater than the right.                                          |
| `less_than(metaint, metaint) -> metabool`     | Returns whether the left integer is less than the right.                                             |
| `greater_equal(metaint, metaint) -> metabool` | Returns whether the left integer is greater than or equal to the right.                              |
| `less_equal(metaint, metaint) -> metabool`    | Returns whether the left integer is less than or equal to the right.                                 |
| `covers(value, pattern) -> metabool`          | Returns whether the left value matches the right pattern. Side effects of the match operation (i.e. changes to bound values) should only be committed when the complete pattern matches. For example, `covers(struct<i8, i32>, struct<T, i16>)` yields false, even though `T` matched `i8` before the match failed. Therefore, `T` should *not* be bound to `i8`. |
| `if_then_else(metabool, T, T) -> T`           | If-then-else expression. Evaluated lazily. That is, the second argument is only evaluated if the first evaluated to true, and the third is only evaluated if the first evaluated to false. |

In addition, the following infix expressions are defined. Parentheses can be used to override precedence order.

| Syntax               | Function        | Precedence | Associativity |
|----------------------|-----------------|------------|---------------|
| `!A`                 | `not`           | 1          | Right to left |
| `-A`                 | `negate`        | 1          | Right to left |
| `A * B`              | `multiply`      | 2          | Left to right |
| `A / B`              | `divide`        | 2          | Left to right |
| `A + B`              | `add`           | 3          | Left to right |
| `A - B`              | `subtract`      | 3          | Left to right |
| `A < B`              | `less_than`     | 4          | Left to right |
| `A <= B`             | `less_equal`    | 4          | Left to right |
| `A > B`              | `greater_than`  | 4          | Left to right |
| `A >= B`             | `greater_equal` | 4          | Left to right |
| `A == B`             | `equal`         | 5          | Left to right |
| `A != B`             | `not_equal`     | 5          | Left to right |
| `A && B`             | `and`           | 6          | Left to right |
| `A || B`             | `or`            | 7          | Left to right |
| `if A then B else C` | `if_then_else`  | N/A        | N/A           |

Note that the C-style `A ? B : C` ternary operator syntax is not supported, as the question mark makes it highly ambiguous with nullability patterns for parsers that do not support arbitrary recursive descent.

#### Matching Behavior

Functions are typically used only in evaluation context, but also work in match context; a function pattern will simply match the incoming value against its evaluation result.

!!! warning

    Constructs like `A + B = C` do *not* work as you may expect them to. Here, `A + B` is used in match context, but as part of the match operation, `A` and `B` will end up being evaluated. The net result is that all three bindings need to have been previously defined for this operation to work, in which case it will fail if the equality is not true. This may still be useful for checking constraints, but rewriting the expression to `A = C - B`, `B = C - A`, or `C = A + B` makes it more powerful, in that now the name on the left-hand side need not be bound yet.

## Metastatements and Derivation Programs

Metastatements allow for the specification of more complex constraints and expressions than can be described using just patterns.

| Syntax               | Name            | Description                                                                                                        |
|----------------------|-----------------|--------------------------------------------------------------------------------------------------------------------|
| `A = B`              | Assignment      | Evaluate `B`, then match the result against `A`. If either the evaluation or the match fails, the statement fails. |
| `assert A`           | Assertion       | Evaluate `A`. If this fails or returns anything other than `true`, the statement fails.                            |
| `assert A matches B` | Match assertion | Evaluate `A`, then match the result against `B`. If either the evaluation or the match fails, the statement fails. |

Note that, ultimately, these are all variations of the same thing; assertions can be regarded as syntactic sugar for assignments. The assertion variants exist because `assert A && B` and `assert T matches struct<A, B>` communicate intent better than `true = A && B` and `struct<A, B> = T`, but are otherwise exactly the same thing.

Synactically, statements usually appear in derivation programs, used to derive the return type or intermediate type of a function, or the structure type of a user-defined type class. Such derivation programs consist of zero or more newline-separated statements followed by a final pattern. The statements are executed in-order before the final pattern is evaluated to compute the desired type. If the final pattern does not yield a typename in these contexts, derivation fails.
