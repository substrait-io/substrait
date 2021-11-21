# Binary Serialization

Substrait can be serialized into a [protobuf](https://developers.google.com/protocol-buffers) based binary representation. The proto schema/idl files can be found on [GitHub](https://github.com/substrait-io/substrait/tree/main/binary). Proto files are place in the `io.substrait` namespace for C++/Java and the `Substrait.Protobuf` namespace for C#. 


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

Since references to simple extensions are used extensively in most plans, the serialized representation is normalized within the plan message type minimize space and serialization costs. For each simple extension YAML referenced, the plan will defined an ExtensionURI object and in-plan surrogate key that can be used in each simple extension extension definition. 

=== "Advanced Extension Protobuf"
    ```proto
%%% proto.message.SimpleExtensionURI %%%
    ```

Once the YAML file is defined with a surrogate key, the surrogate key will be referenced by zero or more `SimpleExtensionDefinition`s. For each simple extension definition, a surrogate key is defined for that specific extension entity. This surrogate key is then pointed to within lower-level primitives (functions, etc) to reference that specific extension. Message properties are named `*_surrogate_key` where the surrogate key is defined and `*_pointer` when referencing the surrogate key. For example `function_surrogate_key` and `function_pointer`.

=== "Advanced Extension Protobuf"
    ```proto
%%% proto.message.SimpleExtensionDefinition %%%
    ```

!!! note
  Surrogate keys only have meaning within a single plan for the purposes of plan size reduction. They are not some form of global identifier. Different plans may use different surrogate keys for the same specific functions, types, type variations, etc.

!!! note
  It is valid for a plan to include `SimpleExtensionURI`s and/or `SimpleExtensionDefinition`s that are not referenced directly. It is an optimization to keep this list to the minimum set required.



### Advanced Extensions

Substrait protobuf exposes a special object in multiple places in the representation to expose extension capabilities. Extensions are done via this object. Extensions are separated into main concepts: 

| Advanced Extension Type | Description                                                  |
| ----------------------- | ------------------------------------------------------------ |
| Optimization            | A change to the plan that may help some consumers work more efficiently with the plan. These properties should be propagated through plan pipelines where possible but do not impact the meaning of the plan. A consumer can safely ignore some/all of these properties. |
| Enhancement             | A change to the plan that functionally changes the behavior of the plan. Use these sparingly as they will impact plan interoperability. |

=== "Advanced Extension Protobuf"
    ```proto
%%% proto.message.AdvancedExtension %%%
    ```

## Capabilities

When two systems exchanging substrait plans want to understand each others capabilities, they may exchange a `Capabilities` message. The capabilities message provides information on the set of simple and advanced extensions that the system supports.

=== "Capabilities Message"
    ```proto
%%% proto.message.Capabilities %%%
    ```

## Protobuf Rationale

The binary format of a Substrait is designed to be easy to work with in many languages. A key requirement is that someone can take the binary format IDL and use standard tools to build a set of primitives that are easy to work with in any of a number of languages. This allows communities to build and use Substrait using only a binary IDL and the specification (and allows the Substrait project to avoid being required to build libraries for each language to work with the specification). 

There are several binary IDLs that exist today. The key requirements for Substrait are the following:

* Strongly typed IDL schema language
* High Quality well supported and idiomatic bindings/compilers for key languages (Python, Javascript, C++, Go, Rust, Java)
* Compact serial representation

The primary formats that exist that roughly qualify under these requirements include: Protobuf, Thrift, Flatbuf, Avro, Cap'N'Proto. The choise was made to Protobuf, primarily due to its clean typing system and large number of high quality language bindings. 

The binary serialization IDLs can be found on [GitHub](https://github.com/substrait-io/substrait/tree/main/binary) as well as sampled throughout the doumentation.





