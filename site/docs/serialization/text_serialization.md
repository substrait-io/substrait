# Text Serialization

To maximize the new user experience, it is important for Substrait to have a text representation of plans. This allows people to experiment with basic tooling. Building simple CLI tools that do things like SQL > Plan and Plan > SQL or REPL plan construction can all be done relatively straightforwardly with a text representation.

The recommended text serialization format is YAML. It provides structure similar to JSON but is easier for people to write/read. Since the text format is not designed for performance, the format can be produced to maximize readability. This also allows nice symmetry between the construction of plans and the configuration of various extensions such as function signatures and user defined types.

To ensure the YAML is valid, the YAML will defined using the YAML schema definition language (an extension of the JSON schema definition language).

While YAML will be used for much of the plan serialization, Substrait uses a very basic expression grammar for record level expressions. While one can construct an equation such as `(10 + 5)/2` using a tree of function and literal objects, it is much more human readable to consume a plan when the information is written similarly to the way one typically consumes scalar expressions.

