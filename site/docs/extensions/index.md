# Extensions

In many cases, the existing objects in Substrait will be sufficient to accomplish a particular use case. However, it is sometimes helpful to create a new data type, scalar function signature or some other custom representation within a system. For that, Substrait provides a number of extension points. 

## Simple Extensions

Some kinds of primitives are so frequently extended that Substrait defines a standard YAML format that describes how the extended functionality can be interpreted. This allows different projects/systems to use the YAML definition as a specification to interoperate around the extended items so that interoperability isn't constrained solely the base Substrait specification. The main types of extensions that are defined in this manner include the following:

* Data types
* Type variations
* Scalar Functions
* Aggregate Functions
* Window Functions
* Table Functions

To extend these items, developers can create one or more YAML files at a defined URI that describes the properties of each of these extensions. The YAML file is constructed according to the [YAML Schema](https://github.com/substrait-io/substrait/blob/main/text/simple_extensions_schema.yaml). Each definition in the file corresponds to the yaml based serialization of the relevant data structure. If a user only wants to extend one of these types of objects (e.g. types), a developer does not have to provide definitions for the other extension points.

A Substrait plan can reference one or more YAML files via URI for extension. In the places where these entities are referenced, they will be referenced using a URI + name reference. The name scheme per type works as follows:

| Category            | Naming scheme                                                |
| ------------------- | ------------------------------------------------------------ |
| Types               | The name as defined on the type object.                      |
| Type Variations     | The name as defined on the type object.                      |
| Function Signatures | If there is only one impl, the name is defined at the top-level definition of the object. If there are multiple impls, the name is a composite string that includes the top-level name (e.g. `add`) followed by a compound string that describes the types of the arguments (e.g. `add:i8_i8`). How arguments are mapped to a compound name are described below. |

## Advanced Extensions

Less common extensions can be extended using customization support at the serialization level. This includes the following kinds of extensions:

| Extension Type                           | Description                                                  |
| ---------------------------------------- | ------------------------------------------------------------ |
| Semantic Changing Relation Extensions    | Extensions to an existing relation that will alter the semantics of that relation. These kinds of extensions require that any plan consumer understand the extension to be able to manipulate or execute that operator. Ignoring these extensions will result in an incorrect interpretation of the plan. An example extension might be creating a customized version of Aggregate that can optionally apply a filter before aggregating the data. <br /><br />Note: Semantic-changing extensions shouldn't change the core characteristics of the underlying relation. For example, they should *not* change the default direct output field ordering, change the number of fields output or change the behavior of physical property characteristics. If one needs to change one of these behaviors, one should define a new relation as described below. |
| Optimization-Related Relation Extensions | Extensions to an existing relation that can improve the efficiency of a plan consumer but don't fundamentally change the behavior of the operation. An example might be an estimated amount of memory the relation is expected to use or a particular algorithmic pattern that is perceived to be optimal. |
| New Relations                            | This is a developing an entirely new kind of relation. It is the most flexible way to extend Substrait but also make the Substrait plan the least interoperable. In most cases it is better to use a semantic changing relation as oppposed to a new relation as it means existing code patterns can easily be extended to work with the additional properties. |
| New Read Types                           | Defines a new subcategory of read that can be used in a ReadRel. One of Substrait is to provide a fairly extensive set of read patterns within the project as opposed to requiring people to define new types externally. As such, we suggest that you first talk with the Substrait community to determine whether you read type can be incorporated directly in the core specification. |
| New Write Types                          | Similar to a read type but for writes. As with reads, the community recommends that interested extenders first discuss with the community about developing new write types in the community before using the extension mechanisms. |

Because extension mechanisms are different for each serialization format, please refer to the corresponding serialization sections to understand how these extensions are defined in more detail.