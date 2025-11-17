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

A YAML file may contain one or more functions with the same name, each with one or more implementations (impls). A specific function implementation within a YAML file can be identified using a Function Signature which consists of two components
* Function Name: the name of the function
* Argument Signature: a signature based on the defined arguments of the function

These component are defined as follows
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
| any[\d]?                        | any            |
| user defined type               | u!name         |

#### Examples

| Function Signature                                | Function Name    |
| ------------------------------------------------- | ---------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:i8_i8`      |
| `avg(fp32) => fp32`                               | `avg:fp32`       |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts` |
| `sum(any1) => any1`                               | `sum:any`        |
| `concat(str...) => str`                           | `concat:str`     |

### Any Types

```yaml
--8<-- "examples/extensions/any_type_function.yaml"
```

The `any` type indicates that the argument can take any possible type. In the `foo` function above, arguments `a` and `b` can be of any type, even different ones in the same function invocation.

```yaml
--8<-- "examples/extensions/any1_type_function.yaml"
```
The `any[\d]` types (i.e. `any1`, `any2`, ..., `any9`) impose an additional restriction. Within a single function invocation, all any types with same numeric suffix _must_ be of the same type. In the `bar` function above, arguments `a` and `b` can have any type as long as both types are the same.


## Advanced Extensions

Advanced extensions provide a way to embed custom functionality that goes beyond the standard YAML-based simple extensions. Unlike simple extensions, advanced extensions use Protocol Buffer's `google.protobuf.Any` type to embed arbitrary extension data directly into Substrait messages.

### How Advanced Extensions Work

Advanced extensions come in several main forms, discussed below:

1. Embedded extensions: These use the `AdvancedExtension` message for adding custom data to existing Substrait messages
2. Custom relation types: For defining entirely new relational operations
3. Custom read/write types: for defining new ways to read from or write to data sources

### Embedded Extensions via `AdvancedExtension`

The simplest forms of advanced extensions use the `AdvancedExtension` message, which contains two types of extensions:

```proto
message AdvancedExtension {
  // Optimizations are helpful information that don't influence semantics.
  // May be ignored by a consumer.
  repeated google.protobuf.Any optimization = 1;

  // Enhancements alter semantics. Cannot be ignored by a consumer.
  google.protobuf.Any enhancement = 2;
}
```

!!! note "Enhancements vs Optimizations"

    * Use **optimizations** for performance hints that don't change semantics and can be safely ignored.
    * Use **enhancements** for semantic changes that must be understood by consumers or the plan cannot be executed correctly.

#### Optimizations

- Provide hints to improve performance but don't change the meaning of operations
- Can be safely ignored by consumers that don't understand them
- Multiple optimizations can be attached to a single message
- Examples: memory usage hints, preferred algorithms, caching strategies

#### Enhancements

- Modify the semantic behavior of operations
- Must be understood by consumers or the plan cannot be executed correctly
- Only one enhancement per message
- Examples: specialized join conditions (e.g. fuzzy matching, geospatial) or sorting (e.g. clustering)

!!! note "Enhancement Constraints"

    Semantic-changing extensions shouldn't change the core characteristics of the underlying relation. For example, they should *not* change the default direct output field ordering, change the number of fields output or change the behavior of physical property characteristics. If one needs to change one of these behaviors, one should define a new relation as described below.

#### Where `AdvancedExtension` Messages Can Be Used

The `AdvancedExtension` message can be attached to various parts of a Substrait plan:

| Location                          | Usage                                       |
| --------------------------------- | ------------------------------------------- |
| **`Plan`**                        | Global extensions affecting the entire plan |
| **`RelCommon`**                   | Extensions for any relational operator      |
| **Relations** (e.g. `ProjectRel`) | Extensions for a specific relation type     |
| **Hints**                         | Extensions within optimization hints        |
| **`ReadRel.NamedTable`**          | Custom metadata to named table references   |
| **`ReadRel.LocalFiles`**          | Custom metadata to local file sources       |
| **`WriteRel.NamedObjectWrite`**   | Custom metadata to write targets            |
| **`DdlRel.NamedObjectWrite`**     | Custom metadata to DDL targets              |

### Custom Relations

The second form of advanced extensions provides entirely new relational operations via dedicated extension relation types. These allow you to define custom relations while maintaining proper integration with the type system:

| Relation Type          | Description                                     | Examples |
| ---------------------- | ----------------------------------------------- | -------- |
| **`ExtensionLeafRel`**   | Custom relations with no inputs | Custom table sources   |
| **`ExtensionSingleRel`** | Custom relations with one input | Custom transforms      |
| **`ExtensionMultiRel`**  | Custom relations with multiple inputs | Custom joins     |

These extension relations are first-class relation types in Substrait and can be used anywhere a standard relation would be used.

!!! note "Interoperability Guidance"

    Custom relations are the most flexible but least interoperable option. In most cases it is better to use enhancements to existing relations rather than defining new custom relations, as it means existing code patterns can easily be extended to work with the additional properties.

### Custom Read and Write Types

The third form of advanced extensions allows you to define extension data sources and destinations:

| Extension Type                 | Description                          | Examples                     |
| ------------------------------ | ------------------------------------ | ---------------------------- |
| **`ReadRel.ExtensionTable`**   | Define new table source types        | APIs, specialized formats    |
| **`WriteRel.ExtensionObject`** | Define new write destination types   | APIs, specialized formats    |
| **`DdlRel.ExtensionObject`**   | Define new DDL destination types     | Catalogs, schema registries  |

!!! note "Consider Core Specification First"

    Before implementing custom read/write types as extensions, consider checking with the Substrait community. If your scenario turns out to be common enough, it may be more appropriate to add it directly to the specification rather than as an extension.
