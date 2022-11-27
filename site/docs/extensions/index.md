# Extensions

In many cases, the existing objects in Substrait will be sufficient to accomplish a particular use case. However, it is sometimes helpful to create a new data type, scalar function signature or some other custom representation within a system. For that, Substrait provides a number of extension points. 

## Simple Extensions

Some kinds of primitives are so frequently extended that Substrait defines a standard YAML format that describes how the extended functionality can be interpreted. This allows different projects/systems to use the YAML definition as a specification so that interoperability isn't constrained to the base Substrait specification. The main types of extensions that are defined in this manner include the following:

* Data types
* Type variations
* Scalar Functions
* Aggregate Functions
* Window Functions
* Table Functions

To extend these items, developers can create one or more YAML files at a defined URI that describes the properties of each of these extensions. The YAML file is constructed according to the [YAML Schema](https://github.com/substrait-io/substrait/blob/main/text/simple_extensions_schema.yaml). Each definition in the file corresponds to the YAML-based serialization of the relevant data structure. If a user only wants to extend one of these types of objects (e.g. types), a developer does not have to provide definitions for the other extension points.

A Substrait plan can reference one or more YAML files via URI for extension. In the places where these entities are referenced, they will be referenced using a URI + name reference. The name scheme per type works as follows:

| Category           | Naming scheme                                                |
| ------------------ | ------------------------------------------------------------ |
| Type               | The name as defined on the type object.                      |
| Type Variation     | The name as defined on the type variation object.            |
| Function Signature | In a specific YAML, if there is only one function implementation with a specific name, a extension type declaration can reference the function using either simple or compound references. Simple references are simply the name of the function (e.g. `add`). Compound references (e.g. `add:i8_i8`)are described below. |

### Function Signature Compound Names

A YAML file may contain one or more functions by the same name. When only a single function is declared within the file, it can be referenced using the name of that function or a compound name. When more than one function of the same name is declared within a YAML file, the key used in the function extension declaration is a combination of the name of the function along with a list of the required input argument types. Optional arguments are not included in the signature.  The format is as follows:

```
<function name>:<short_arg_type0>_<short_arg_type1>_..._<short_arg_typeN>
```

Rather than using a full data type representation, the input argument types (`short_arg_type`) are mapped to single-level short name. The mappings are listed in the table below. 

!!! note

It is required that two function implementations with the same simple name must resolve to different compound names using types. If two function implementations in a YAML file resolve to the same compound name, the YAML file is invalid and behavior is undefined.

| Argument Type              | Signature Name |
| -------------------------- | -------------- |
| Required Enumeration       | req            |
| i8                         | i8             |
| i16                        | i16            |
| i32                        | i32            |
| i64                        | i64            |
| fp32                       | fp32           |
| fp64                       | fp64           |
| string                     | str            |
| binary                     | vbin           |
| timestamp                  | ts             |
| timestamp_tz               | tstz           |
| date                       | date           |
| time                       | time           |
| interval_year              | iyear          |
| interval_day               | iday           |
| uuid                       | uuid           |
| fixedchar&lt;N&gt;         | fchar          |
| varchar&lt;N&gt;           | vchar          |
| fixedbinary&lt;N&gt;       | fbin           |
| decimal&lt;P,S&gt;         | dec            |
| struct&lt;T1,T2,...,TN&gt; | struct         |
| list&lt;T&gt;              | list           |
| map&lt;K,V&gt;             | map            |
| any[\d]?                   | any            |
| user defined type          | u!name         |

#### Examples

| Function Signature                                | Function Name    |
| ------------------------------------------------- | ---------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:i8_i8`  |
| `avg(fp32) => fp32`                               | `avg:fp32`       |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts` |
| `sum(any1) => any1`                               | `sum:any`        |



## Advanced Extensions

Less common extensions can be extended using customization support at the serialization level. This includes the following kinds of extensions:

| Extension Type                       | Description                                                  |
| ------------------------------------ | ------------------------------------------------------------ |
| Relation Modification (semantic)     | Extensions to an existing relation that will alter the semantics of that relation. These kinds of extensions require that any plan consumer understand the extension to be able to manipulate or execute that operator. Ignoring these extensions will result in an incorrect interpretation of the plan. An example extension might be creating a customized version of Aggregate that can optionally apply a filter before aggregating the data. <br /><br />Note: Semantic-changing extensions shouldn't change the core characteristics of the underlying relation. For example, they should *not* change the default direct output field ordering, change the number of fields output or change the behavior of physical property characteristics. If one needs to change one of these behaviors, one should define a new relation as described below. |
| Relation Modification (optimization) | Extensions to an existing relation that can improve the efficiency of a plan consumer but don't fundamentally change the behavior of the operation. An example might be an estimated amount of memory the relation is expected to use or a particular algorithmic pattern that is perceived to be optimal. |
| New Relations                        | Creates an entirely new kind of relation. It is the most flexible way to extend Substrait but also make the Substrait plan the least interoperable. In most cases it is better to use a semantic changing relation as oppposed to a new relation as it means existing code patterns can easily be extended to work with the additional properties. |
| New Read Types                       | Defines a new subcategory of read that can be used in a ReadRel. One of Substrait is to provide a fairly extensive set of read patterns within the project as opposed to requiring people to define new types externally. As such, we suggest that you first talk with the Substrait community to determine whether you read type can be incorporated directly in the core specification. |
| New Write Types                      | Similar to a read type but for writes. As with reads, the community recommends that interested extenders first discuss with the community about developing new write types in the community before using the extension mechanisms. |
| Plan Extensions                      | Semantic and/or optimization based additions at the plan level. |

Because extension mechanisms are different for each serialization format, please refer to the corresponding serialization sections to understand how these extensions are defined in more detail.
