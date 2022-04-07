# Binary Serialization

Substrait can be serialized into a [protobuf](https://developers.google.com/protocol-buffers)-based binary representation. The proto schema/IDL files can be found on [GitHub](https://github.com/substrait-io/substrait/tree/main/binary). Proto files are place in the `io.substrait` namespace for C++/Java and the `Substrait.Protobuf` namespace for C#.


## Plan

The main top-level object used to communicate a Substrait plan using protobuf is a Plan message. The plan message is composed of a set of data structures that minimize repetition in the serialization along with one (or more) Relation trees. 

=== "Plan Message"

    ```proto
%%% proto.message.Plan %%%
    ```

## Extensions
Protobuf supports both [simple](/extensions/#simple-extensions) and [advanced](/extensions/#advanced-extensions) extensions. Simple extensions are declared at the plan level and advanced extensions are declared at multiple levels of messages within the plan.

### Simple Extensions

For simple extensions, a plan references the URIs associated with the simple extensions to provide additional plan capabilities. These URIs will list additional relevant information for the plan. 

Simple extensions within a plan are split into three components: an extension URI, an extension declaration and a number of references.

* **Extension URI**: A unique identifier for the extension pointing to a YAML document specifying one or more specific extensions. Declares an anchor that can be used in extension declarations.  
* **Extension Declaration**: A specific extension within a single YAML document. The declaration combines a reference to the associated Extension URI along with a unique key identifying the specific item within that YAML document (see [Function Signature Compound Names](/extensions/#function-signature-compound-names)). It also defines a declaration anchor. The anchor is a plan-specific unique value that the producer creates as a key to be referenced elsewhere.
* **Extension Reference**: A specific instance or use of an extension declaration within the plan body.

Extension URIs and declarations are encapsulated in the top level of the plan. Extension declarations are then referenced throughout the body of the plan itself. The exact structure of these references will depend on the extension point being used, but they will always include the extension's anchor (or key). For example, all scalar function expressions contain references to an extension declaration which defines the semantics of the function.

=== "Simple Extension URI"

    ```proto
%%% proto.message.SimpleExtensionURI %%%
    ```

Once the YAML file URI anchor is defined, the anchor will be referenced by zero or more `SimpleExtensionDefinition`s. For each simple extension definition, an anchor is defined for that specific extension entity. This anchor is then referenced to within lower-level primitives (functions, etc.) to reference that specific extension. Message properties are named `*_anchor` where the anchor is defined and `*_reference` when referencing the anchor. For example `function_anchor` and `function_reference`.

=== "Simple Extension Declaration"

    ```proto
%%% proto.message.SimpleExtensionDeclaration %%%
    ```

!!! note
  Anchors only have meaning within a single plan and exist simply to reduce plan size. They are not some form of global identifier. Different plans may use different anchors for the same specific functions, types, type variations, etc.

!!! note
  It is valid for a plan to include `SimpleExtensionURI`s and/or `SimpleExtensionDeclaration`s that are not referenced directly.



### Advanced Extensions

Substrait protobuf exposes a special object in multiple places in the representation to expose extension capabilities. Extensions are done via this object. Extensions are separated into main concepts: 

| Advanced Extension Type | Description                                                  |
| ----------------------- | ------------------------------------------------------------ |
| Optimization            | A change to the plan that may help some consumers work more efficiently with the plan. These properties should be propagated through plan pipelines where possible but do not impact the meaning of the plan. A consumer can safely ignore these properties. |
| Enhancement             | A change to the plan that functionally changes the behavior of the plan. Use these sparingly as they will impact plan interoperability. |

=== "Advanced Extension Protobuf"
    ```proto
%%% proto.message.AdvancedExtension %%%
    ```

## Capabilities

When two systems exchanging Substrait plans want to understand each other's capabilities, they may exchange a `Capabilities` message. The capabilities message provides information on the set of simple and advanced extensions that the system supports.

=== "Capabilities Message"
    ```proto
%%% proto.message.Capabilities %%%
    ```

## Protobuf Rationale

The binary format of Substrait is designed to be easy to work with in many languages. A key requirement is that someone can take the binary format IDL and use standard tools to build a set of primitives that are easy to work with in any of a number of languages. This allows communities to build and use Substrait using only a binary IDL and the specification (and allows the Substrait project to avoid being required to build libraries for each language to work with the specification).

There are several binary IDLs that exist today. The key requirements for Substrait are the following:

* Strongly typed IDL schema language
* High-quality well-supported and idiomatic bindings/compilers for key languages (Python, Javascript, C++, Go, Rust, Java)
* Compact serial representation

The primary formats that exist that roughly qualify under these requirements include: Protobuf, Thrift, Flatbuf, Avro, Cap'N'Proto. Protobuf was chosen due to its clean typing system and large number of high quality language bindings. 

The binary serialization IDLs can be found on [GitHub](https://github.com/substrait-io/substrait/tree/main/binary) and are sampled throughout the documentation.





