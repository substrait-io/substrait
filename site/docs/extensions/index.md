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

A YAML file can also reference types and type variations defined in another YAML file. To do this, it must declare the YAML file it depends on using a key-value pair in the `dependencies` key, where the value is the URI to the YAML file, and the key is a valid identifier that can then be used as an identifier-safe alias for the URI. This alias can then be used as a `.`-separated namespace prefix wherever a type class or type variation name is expected.

For example, if the YAML file at `file:///extension_types.yaml` defines a type called `point`, a different YAML file can use the type in a function declaration as follows:

```yaml
dependencies:
  ext: file:///extension_types.yaml
scalar_functions:
- name: distance
  description: The distance between two points.
  impls:
  - args:
    - name: a
      value: ext.point
    - name: b
      value: ext.point
    return: f64
```

Here, the choice for the name `ext` is arbitrary, as long as it does not conflict with anything else in the YAML file.

### Function Signature Compound Names

A YAML file may contain one or more functions by the same name. The key used in the function extension declaration to reference a function is a combination of the name of the function along with a list of the required input argument types. The format is as follows:

```
<function name>:<short_arg_type0>_<short_arg_type1>_..._<short_arg_typeN>
```

Rather than using a full data type representation, the input argument types (`short_arg_type`) are mapped to single-level short name. The mappings are listed in the table below.

!!! note

    Every compound function signature must be unique.  If two function implementations in a YAML file would generate the same compound function signature, then the YAML file is invalid and behavior is undefined.

| Argument Type                   | Signature Name |
|---------------------------------|----------------|
| Required Enumeration            | req            |
| i8                              | i8             |
| i16                             | i16            |
| i32                             | i32            |
| i64                             | i64            |
| fp32                            | fp32           |
| fp64                            | fp64           |
| string                          | str            |
| binary                          | vbin           |
| boolean                         | bool           |
| timestamp                       | ts             |
| timestamp_tz                    | tstz           |
| date                            | date           |
| time                            | time           |
| interval_year                   | iyear          |
| interval_day                    | iday           |
| interval_compound               | icompound      |
| uuid                            | uuid           |
| fixedchar&lt;N&gt;              | fchar          |
| varchar&lt;N&gt;                | vchar          |
| fixedbinary&lt;N&gt;            | fbin           |
| decimal&lt;P,S&gt;              | dec            |
| precision_time&lt;P&gt;         | pt             |
| precision_timestamp&lt;P&gt;    | pts            |
| precision_timestamp_tz&lt;P&gt; | ptstz          |
| struct&lt;T1,T2,...,TN&gt;      | struct         |
| list&lt;T&gt;                   | list           |
| map&lt;K,V&gt;                  | map            |
| any[\d]?                        | any            |
| user defined type               | u!name         |

#### Examples

| Function Signature                                | Function Name    |
| ------------------------------------------------- | ---------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:i8_i8`  |
| `avg(fp32) => fp32`                               | `avg:fp32`       |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts` |
| `sum(any1) => any1`                               | `sum:any`        |

### Any Types

```yaml
scalar_functions:
- name: foo
  impls:
  - args:
    - name: a
      value: any
    - name: b
      value: any
    return: int64
```

The `any` type indicates that the argument can take any possible type. In the `foo` function above, arguments `a` and `b` can be of any type, even different ones in the same function invocation.

```yaml
scalar_functions:
- name: bar
  impls:
  - args:
    - name: a
      value: any1
    - name: b
      value: any1
    return: int64
```
The `any[\d]` types (i.e. `any1`, `any2`, ..., `any9`) impose an additional restriction. Within a single function invocation, all any types with same numeric suffix _must_ be of the same type. In the `bar` function above, arguments `a` and `b` can have any type as long as both types are the same.


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
