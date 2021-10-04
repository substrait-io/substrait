# Technology Principles

* Provide a good suite of well-specified common functionality in databases and data science applications.
* Make it easy for users to privately or publicly extend the representation to support specialized/custom operations.
* Produce something that is language agnostic and requires minimal work to start developing against in a new language.
* Drive towards a common format that avoids specialization for single favorite producer or consumer.
* Establish clear delineation between specifications that MUST be respected to and those that can be optionally ignored.
* Establish a forgiving compatibility approach and versioning scheme that supports cross-version compatibility in maximum number of cases.
* Minimize the need for consumer intelligence by excluding concepts like overloading, type coercion, implicit casting, field name handling, etc. (Note: this is weak and should be better stated.)
* Decomposability/severability: A particular producer or consumer should be able to produce or consume only a subset of the specification and interact well with any other Substrait system as long the specific operations requested fit within the subset of specification supported by the counter system.


