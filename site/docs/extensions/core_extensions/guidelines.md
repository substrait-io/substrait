# Guidelines

The process for deciding whether something should or should not be a core extension, and if so, how it should be specified, is based on the following guidelines.

 - Avoid adding a new function if is is merely syntactic sugar and if producers could express its behavior easily enough by composing other existing functions. However, it might be acceptable to add such a function if adding it can enable consumers to more efficiently execute plans, or if adding it can avoid producers deconstructing a syntactic construct that consumers then need to reconstruct.
    - Example: [#287 (comment)](https://github.com/substrait-io/substrait/pull/287#discussion_r942705485)

 - Avoid adding functions that express behaviors that are already expressible with [specialized record expressions](../../expressions/specialized_record_expressions/).
    - Example: the `if_else` function originally proposed in [#291](https://github.com/substrait-io/substrait/issues/291)

 - Prefer adding new options to existing functions instead of adding new functions.
    - Example: [#289](https://github.com/substrait-io/substrait/issues/289)
    - Example: [#295](https://github.com/substrait-io/substrait/issues/295)

 - Aim for syntactic and semantic consistency with widely used SQL dialects, especially PostgreSQL.
    - Example: [#285 (comment)](https://github.com/substrait-io/substrait/pull/285#discussion_r944542030)

 - Generalize the function as much as possible, to reduce the odds that we'll need to update it later.
    - Example: for a function like `add`, consider making it variadic rather than only accepting two arguments.

 - Be consistent when it comes to argument types. It is preferable to define a function that accepts and returns one type class over a function that promotes from one type class or another or accepts a mixture of type classes. This aims to prevent an explosion of function implementations.
    - More information and examples: [#251](https://github.com/substrait-io/substrait/issues/251)

 - Be pedantic when describing functionality. The corner cases that rarely come up in practice are exactly the places where different implementations are likely to differ, so for a plan to be implementation-agnostic, these are exactly the things that need to be specified exhaustively. For especially pedantic things, an optional enumeration argument may be suitable; this allows a producer to explicitly indicate that the consumer can pick the behavior.
    - Example: the verbosity of the description of [regex_match_substring](https://github.com/substrait-io/substrait/blob/fbe5e0949b863334d02b5ad9ecac55ec8fc4debb/extensions/functions_string.yaml#L79-L139).
    - Example: the floating point rounding option defined [here](common_options.md).

 - The core extensions should generally not be defining type classes. If you believe a type class that isn't currently in the specification is important enough to include, it probably makes more sense to simply add it to the built-in types, or otherwise should be a third-party extension.
