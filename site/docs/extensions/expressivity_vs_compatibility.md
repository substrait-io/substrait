# Expressivity vs. Compatibility

The Substrait specification aims to both support the needs of a wide variety of database management systems, query engines, and so on, and to provide a compatibility layer between them. These goals fundamentally conflict: in order to support all systems Substrait would need to support a *superset* of their functionality, while a compatibility layer should only include a common *subset* of implementation-agnostic functionality.

If this common subset would not exist, every system would necessarily have to *at least* understand everyone else's functionality well enough to throw a suitable error message, but more commonly they would need to do all kinds of conversions to be compatible in practice. For example, something as simple as an integer addition expression may be represented and executed differently between systems; maybe one system calls the operation `add` while the other calls it `plus` or `sum`, maybe one requires a tree of 2-input additions while another supports variadic additions, and maybe one detects integer overflow and throws an error, another might return NULL, another might saturate silently, and yet another might just throw an error and fail the whole query.

If, on the other hand, Substrait would only define a common subset, we have a different problem. Imagine for instance that Substrait would only define a two-input addition operator called `add` that silently overflows per two's-complement rules. Does that then mean that a consumer desiring to be compatible with Substrait must implement addition that way? The consumer probably existed long before Substrait did, so it would probably not want to change something as fundamental as the addition operator just because a third-party specification says it should.

Substrait reconciles this conflict by means of extensions.

Without (third-party) extensions, Substrait aims to support a fairly primitive, minimal subset of features that are likely to be supported in one way or another across many consumers. This addresses the compatibility layer goal. A consumer may not be able to execute the plan exactly as specified -- some conversion may be required, or some efficiency may need to be sacrificed -- but it generally should be able to emulate the plan. Likewise, a producer may need to do some conversion to go from its internal representation to Substrait, but for common operations, there should be some matching Substrait representation.

At the same time, consumers and producers can define their own extensions. Ideally, Substrait's extensibility should be sufficient to allow consumers and producers to round-trip their internal representations through Substrait. This is useful not only for testing, but also allows power users to use implementation-specific constructs to tweak performance in a way that may not be possible without extensions. Nevertheless, by wrapping these constructs inside Substrait plans, it may still be possible to leverage parts of the Substrait ecosystem, such as a generic transformation library or optimizer. You can think of this like inline assembly or user-defined functions; an optimizer will not be able to do anything with these constructs, but it can still optimize everything around them.

Extensions are subdivided into three different classes:

 - *simple extensions*: used for constructs that are so frequently extended that it makes sense to define a language by which third parties can specify them with. We currently use YAML files for these specifications. You can think of these files as schema definitions for the extensions.
 - *advanced optimization extensions*: used for constructs that do not affect the behavior of the plan, but can be used to convey optimization hints. Consumers may silently ignore these extensions if they come across them. These are represented using protobuf `Any` messages, which means they can only be deserialized using custom code.
 - *advanced extensions that change semantics*: as above, but consumers must reject plans using such extensions if they don't recognize them. These are used for everything that does not fall into the above categories.

Some parts of the core Substrait specification are defined using simple extensions. This is mostly just a uniformity thing: for example, rather than representing all core functions in the protobuf representation, thus forcing consumers to special-case all these functions on top of third-party extension functions, we simply define all of them as extensions. We refer to these extensions as *core* extensions. Their specification can be found [here](https://github.com/substrait-io/substrait/blob/main/extensions).

In summary:

| Class                                           | Expressivity                             | Compatibility                                                                                                                       |
|-------------------------------------------------|------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Substrait (protobuf only)                       | Minimal                                  | Implementation-agnostic                                                                                                             |
| + core simple extensions                        | Sufficient for most queries (e.g. TPC-H) | Implementation-agnostic                                                                                                             |
| + third-party advanced optimization extensions  | + custom performance tweaks              | Implementation-agnostic, with implementation-specific hints for optimization                                                        |
| + third-party simple extensions                 | + custom data types and functions        | Implementation-specific; generally requires custom code to execute, but can still be manipulated using implementation-agnostic code |
| + generalized third-party advanced enhancements | + custom relations and more              | Implementation-specific; generally requires custom code to execute or manipulate                                                    |

Consumers and producers are encouraged to support and publish their own extensions, in addition to supporting as much as possible from Substrait core. Producers should generally allow users to trade implementation-agnosticity for conversion accuracy and vice versa.
