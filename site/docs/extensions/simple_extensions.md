# Simple Extensions

Some kinds of primitives are so frequently extended that Substrait defines a standard YAML format that describes how the extended functionality can be interpreted. This allows different projects/systems to use the YAML definition as a specification so that interoperability isn't constrained to the base Substrait specification. The main types of extensions that are defined in this manner include the following:

* Data types
* Type variations
* Scalar Functions
* Aggregate Functions
* Window Functions

To extend these items, developers can create one or more YAML files at a defined URI that describes the properties of each of these extensions. The YAML file is constructed according to the [YAML Schema](https://github.com/substrait-io/substrait/blob/main/text/simple_extensions_schema.yaml). Each definition in the file corresponds to the YAML-based serialization of the relevant data structure. If a user only wants to extend one of these types of objects (e.g. types), a developer does not have to provide definitions for the other extension points.

A Substrait plan can reference one or more YAML files via URI for extension. In the places where these entities are referenced, they will be referenced using a URI + name reference. The name scheme per type works as follows:

| Category           | Naming scheme                                                |
| ------------------ | ------------------------------------------------------------ |
| Type               | The name as defined on the type object.                      |
| Type Variation     | The name as defined on the type variation object.            |
| Function Signature | In a specific YAML, if there is only one function implementation with a specific name, a extension type declaration can reference the function using either simple or compound references. Simple references are simply the name of the function (e.g. `add`). Compound references (e.g. `add:i8_i8`)are described below. |

## Function Signature Compound Names

A YAML file may contain one or more functions by the same name. When only a single function is declared within the file, it can be referenced using the name of that function or a compound name. When more than one function of the same name is declared within a YAML file, the key used in the function extension declaration is a combination of the name of the function along with a list of input argument types. The format is as follows:

```
<function name>:<short_arg_type0>_<short_arg_type1>_..._<short_arg_typeN>
```

Rather than using a full data type representation, the input argument types (`short_arg_type`) are mapped to single-level short name. The mappings are listed in the table below.

!!! note

It is required that two function implementation with the same simple name must resolve to different compound names using types. If two function implementations in a YAML file resolve to the same compound name, the YAML file is invalid and behavior is undefined. The intention is that consumers only need to generate a name to implementation map that uses these compound names as keys; they should never have to parse a compound name back to a function signature.

| Argument Type              | Signature Name |
| -------------------------- | -------------- |
| Optional Enumeration       | opt            |
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
| user defined type          | u!name         |
| anything else              | any            |

### Examples

| Function Signature                                | Function Name    |
| ------------------------------------------------- | ---------------- |
| `add(optional enumeration, i8, i8) => i8`         | `add:opt_i8_i8`  |
| `avg(fp32) => fp32`                               | `avg:fp32`       |
| `extract(required enumeration, timestamp) => i64` | `extract:req_ts` |
| `sum(any1) => any1`                               | `sum:any`        |
