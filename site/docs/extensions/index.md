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

Use **optimizations** for performance hints that don't change semantics and can be safely ignored. Use **enhancements** for semantic changes that must be understood by consumers or the plan cannot be executed correctly.

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

#### Where AdvancedExtension Messages Can Be Used

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

| Extension Type                 | Description                                   | Examples                     |
| ------------------------------ | --------------------------------------------- | ---------------------------- |
| **`ReadRel.ExtensionTable`**   | Define entirely new table source types        | APIs, specialized formats    |
| **`WriteRel.ExtensionObject`** | Define entirely new write destination types   | APIs, specialized formats    |
| **`DdlRel.ExtensionObject`**   | Define entirely new DDL destination types     | Catalogs, schema registries  |

