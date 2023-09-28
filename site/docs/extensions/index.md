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
| Function Signature | A function signature compound name as described below.       |

### Function Signature Compound Names

A YAML file may contain one or more functions by the same name. The key used in the function extension declaration to reference a function is a combination of the name of the function along with a list of the required input argument types. The format is as follows:

```
<function name>:<short_arg_type0>_<short_arg_type1>_..._<short_arg_typeN>
```

Rather than using a full data type representation, the input argument types (`short_arg_type`) are mapped to single-level short name. The mappings are listed in the table below.

!!! note 

    Every compound function signature must be unique.  If two function implementations in a YAML file would generate the same compound function signature, then the YAML file is invalid and behavior is undefined.

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
| boolean                    | bool           |
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

#### Special Cases for Nullability

In some functions the nullability of the return value depends on the nullability of all the input
arguments.  In these cases there is a special syntax to express this:

| Return Value      | Meaning                                                           |
| ----------------- | ----------------------------------------------------------------- |
| &lt;type&gt;&?    | The return type is only nullable if all the inputs are nullable   |
| &lt;type&gt;|?    | The return type is nullable if any of the inputs is nullable      |

In addition, if the input type is `any?` and the return type is `any?` it is not clear if the return
type is always nullable or only nullable if the input type is nullable.

| Return Value | Meaning                                                             |
| ------------ | ------------------------------------------------------------------- |
| any!         | The return type is non-nullable, even if the input type is nullable |
| any          | The return type is nullable only if the input type is nullable      |
| any?         | The return type is nullable even if the input type is non-nullable  |

#### Examples

| Function Signature                                | Function Name    |
| ------------------------------------------------- | ---------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:i8_i8`      |
| `avg(fp32) => fp32`                               | `avg:fp32`       |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts` |
| `sum(any1) => any1`                               | `sum:any`        |
| `coalesce(any) => any&?`                          | `coalesce:any`   |
| `least(any, any) => any|?`                        | `least:any`      |
| `nullif(any) => any!`                             | `nullif:any`     |

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
