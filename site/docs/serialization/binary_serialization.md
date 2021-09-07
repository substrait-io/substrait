# Binary Serialization

The binary format of a Substrait is designed to be easy to work with in many languages. A key requirement is that someone can take the binary format IDL and use standard tools to build a set of primitives that are easy to work with in any of a number of languages. This allows communities to build and use Substrait using only a binary IDL and the specification (and allows the Substrait project to avoid being required to build libraries for each language to work with the specification). 

There are several binary IDLs that exist today. The key requirements for Substrait are the following:

* Strongly typed IDL schema language
* High Quality well supported and idiomatic bindings/compilers for key languages (Python, Javascript, C++, Go, Rust, Java)
* Compact serial representation

The primary formats that exist that roughly qualify under these requirements include: Protobuf, Thrift, Flatbuf, Avro, Cap'N'Proto. The current plan is to use Protobuf, primarily due to its clean typing system and large number of high quality language bindings. Flatbuf is a close second but it's poor support for unions along with the complexity of api use in many languages make it unsuitable to a project that is trying to avoid having to generate per language bindings to grow initial adoption of the core plan specification.

The binary serialization representation is being developed within [GitHub](https://github.com/substrait-io/substrait/tree/main/binary) and is planned to be presented inline in the spec to help clarify the relationship between the specification and the serialized representations.

