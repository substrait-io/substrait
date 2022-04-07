# Text Serialization

To maximize the new user experience, it is important for Substrait to have a text representation of plans. This allows people to experiment with basic tooling. Building simple CLI tools that do things like SQL > Plan and Plan > SQL or REPL plan construction can all be done relatively straightforwardly with a text representation.

The recommended text serialization format is JSON. Since the text format is not designed for performance, the format can be produced to maximize readability. This also allows nice symmetry between the construction of plans and the configuration of various extensions such as function signatures and user defined types.

To ensure the JSON is valid, the object will be defined using the [OpenApi 3.1 specification](https://spec.openapis.org/oas/latest.html). This not only allows strong validation, the OpenApi specification enables [code generators](https://github.com/OpenAPITools/openapi-generator) to be easily used to produce plans in many languages.

While JSON will be used for much of the plan serialization, Substrait uses a custom simplistic grammar for record level expressions. While one can construct an equation such as `(10 + 5)/2` using a tree of function and literal objects, it is much more human-readable to consume a plan when the information is written similarly to the way one typically consumes scalar expressions. This grammar will be maintained in an ANTLR grammar (targetable to multiple programming languages) and is also planned to be supported via JSON schema definition format tag so that the grammar can be validated as part of the schema validation.

