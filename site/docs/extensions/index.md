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

To extend these items, developers can create one or more YAML files that describe the properties of each of these extensions. Each YAML file must include a required `urn` field that uniquely identifies the extension. While these identifiers are URN-like but not technically URNs (they lack the `urn:` prefix), they will be referred to as `extension URNs` for clarity.

This extension URN uses the format `extension:<OWNER>:<ID>`, where:

- `OWNER` represents the organization or entity providing the extension and should follow [reverse domain name convention](https://en.wikipedia.org/wiki/Reverse_domain_name_notation) (e.g., `io.substrait`, `com.example`, `org.apache.arrow`) to prevent name collisions
- `ID` is the specific identifier for the extension (e.g., `functions_arithmetic`, `custom_types`)

The YAML file is constructed according to the [YAML Schema](https://github.com/substrait-io/substrait/blob/main/text/simple_extensions_schema.yaml). Each definition in the file corresponds to the YAML-based serialization of the relevant data structure. If a user only wants to extend one of these types of objects (e.g. types), a developer does not have to provide definitions for the other extension points.

A Substrait plan can reference one or more YAML files via their extension URN. In the places where these entities are referenced, they will be referenced using an extension URN + name reference. The name scheme per type works as follows:

| Category           | Naming scheme                                                |
| ------------------ | ------------------------------------------------------------ |
| Type               | The name as defined on the type object.                      |
| Type Variation     | The name as defined on the type variation object.            |
| Function Signature | A function signature as described below.       |

A YAML file can also reference types and type variations defined in another YAML file. To do this, it must declare the extension it depends on using a key-value pair in the `dependencies` key, where the value is the extension URN, and the key is a valid identifier that can then be used as an identifier-safe alias for the extension URN. This alias can then be used as a `.`-separated namespace prefix wherever a type class or type variation name is expected.

For example, if the extension with extension URN `extension:io.substrait:extension_types` defines a type called `point`, a different YAML file can use the type in a function declaration as follows:

```yaml
--8<-- "examples/extensions/distance_functions.yaml"
```

Here, the choice for the name `ext` is arbitrary, as long as it does not conflict with anything else in the YAML file.

### Function Signature

A YAML file may contain one or more functions with the same name, each with one or more implementations (impls). A specific function implementation within a YAML file can be identified using a Function Signature which consists of two components:

* Function Name: the name of the function
* Argument Signature: a signature based on the defined arguments of the function

These components are defined as follows:
```
<function_signature> ::= <function_name>:<argument_signature>
<argument_signature> ::= <short_arg_type> { _ <short_arg_type> }*
```

and the resulting function signatures look like:
`<function name>:<short_arg_type0>_<short_arg_type1>_..._<short_arg_typeN>`

Argument types (`short_arg_type`) are encoded using the Type Short Names given below.

#### Variadic Functions

For variadic functions, the variadic argument is included *once* in the argument signature.

#### Uniqueness Constraint

A function signature uniquely identifies a function implementation within a single YAML file. As such, every function implementation within a YAML **must** have a distinct function signature in order for references to the implementation to remain unambiguous. A YAML file in which this is not the case is invalid.

#### Type Short Names

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
| func&lt;T-&gt;R&gt;, func&lt;(T1,...,TN)-&gt;R&gt; | func |
| any[\d]?                        | any            |
| user defined type               | u!name         |

#### Examples

| Function Signature                                | Function Name       |
| ------------------------------------------------- | ------------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:i8_i8`         |
| `avg(fp32) => fp32`                               | `avg:fp32`          |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts`    |
| `sum(any1) => any1`                               | `sum:any`           |
| `concat(str...) => str`                           | `concat:str`        |
| `transform(list<any1>, func<any1 -> any2>) => list<any2>` | `transform:list_func` |

### Any Types

```yaml
--8<-- "examples/extensions/any_type_function.yaml"
```

The `any` type indicates that the argument can take any possible type. In the `foo` function above, arguments `a` and `b` can be of any type, even different ones in the same function invocation.

```yaml
--8<-- "examples/extensions/any1_type_function.yaml"
```
The `any[\d]` types (i.e. `any1`, `any2`, ..., `any9`) impose an additional restriction. Within a single function invocation, all any types with same numeric suffix _must_ be of the same type. In the `bar` function above, arguments `a` and `b` can have any type as long as both types are the same.

### Extension Metadata

Extensibility is a core principle of Substrait. To ensure that the extension mechanism itself remains extensible, extension files support an optional `metadata` field that can contain arbitrary data created by the extension author. If you find that the standard YAML schema lacks a field you need, the metadata field provides a forward-compatible way to add it without waiting for schema changes.

This field is available at multiple levels to provide flexibility:

- **Top-level**: Metadata about the extension file itself
- **Type definitions**: Metadata about custom types
- **Functions**: Metadata about functions (scalar, aggregate, and window functions)

Example:
```yaml
--8<-- "examples/extensions/metadata_example.yaml"
```

Consumers of extension files are not required to understand or validate metadata fields.

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
