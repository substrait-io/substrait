# Substrait Dialects

## Overview

Substrait dialects provide a standardized way to describe what subset of the Substrait specification a particular system supports. A dialect file codifies system-specific behaviors, supported types, functions, and options, enabling better interoperability and validation between systems.

Dialects serve multiple purposes:

* **Capability Declaration**: Systems can declare which Substrait features they support
* **Compatibility Validation**: Producers can validate plans against a consumer's capabilities before sending
* **Documentation**: Dialects serve as machine-readable documentation of system capabilities
* **Testing**: Enable automated testing of system conformance to declared capabilities

## Dialect File Format

Dialect files are YAML documents that describe a system's Substrait support. The basic structure includes:

```yaml
name: system_name
type: sql  # or other system type
dependencies:
  extension_name: extension_uri_or_path
supported_types:
  - type definitions
scalar_functions:
  - function definitions
aggregate_functions:
  - function definitions
```

### Core Fields

| Field | Description | Required |
|-------|-------------|----------|
| `name` | The name of the dialect (e.g., `duckdb`, `postgres`) | Yes |
| `type` | The type of system (e.g., `sql`) | No |
| `dependencies` | Extension URIs that this dialect depends on | No |
| `supported_types` | List of Substrait types supported by the system | No |
| `scalar_functions` | Scalar functions supported | No |
| `aggregate_functions` | Aggregate functions supported | No |

## Supported Types

The `supported_types` section declares which Substrait types are supported by the system. Types can be specified in simple or detailed format.

### Simple Format

```yaml
supported_types:
  - I8
  - I16
  - I32
  - I64
  - FP32
  - FP64
  - STRING
```

### Detailed Format

```yaml
supported_types:
  i8:
    sql_type_name: tinyint
  i32:
    sql_type_name: integer
    supported_as_column: true
  user_defined:
    source: geo  # reference to dependency
    name: geometry
```

The detailed format allows specifying:

* `sql_type_name`: The SQL type name used by the system (for SQL dialects)
* `supported_as_column`: Whether the type can be used as a column type
* `source`: For user-defined types, references a dependency extension
* `name`: The specific type name from the extension

## Function Support

Functions are specified with their Substrait name and can include system-specific customization.

### Basic Function Declaration

```yaml
scalar_functions:
  - name: arithmetic.add
    local_name: +
    infix: true
    supported_kernels:
      - i8_i8
      - i16_i16
      - i32_i32
      - i64_i64
      - fp32_fp32
      - fp64_fp64
```

### Function Fields

| Field | Description | Required |
|-------|-------------|----------|
| `name` | The Substrait function name (e.g., `arithmetic.add`) | Yes |
| `local_name` | The system's native name for the function | No |
| `infix` | Whether the function uses infix notation (e.g., `+`) | No |
| `required_options` | Options that must be set to specific values | No |
| `supported_kernels` | List of type combinations (kernels) supported | No |
| `unsupported_kernels` | List of kernels explicitly not supported | No |

### Required Options

Some systems may require specific option values for functions:

```yaml
scalar_functions:
  - name: arithmetic.add
    required_options:
      overflow: ERROR
      rounding: TIE_TO_EVEN
```

This indicates that for `arithmetic.add`, the system only supports the `ERROR` overflow behavior and `TIE_TO_EVEN` rounding mode.

### Supported Kernels

Kernels specify which type combinations are supported for a function:

```yaml
scalar_functions:
  - name: arithmetic.multiply
    supported_kernels:
      - i32_i32      # multiply(i32, i32)
      - i64_i64      # multiply(i64, i64)
      - fp32_fp32    # multiply(fp32, fp32)
      - fp64_fp64    # multiply(fp64, fp64)
      - dec_dec      # multiply(decimal, decimal)
```

## Dependencies

Dependencies declare which extension URIs the dialect relies on:

```yaml
dependencies:
  aggregate_generic: 
    https://github.com/substrait-io/substrait/blob/main/extensions/substrait/extensions/functions_aggregate_generic.yaml
  arithmetic: 
    https://github.com/substrait-io/substrait/blob/main/extensions/substrait/extensions/functions_arithmetic.yaml
  string: 
    https://github.com/substrait-io/substrait/blob/main/extensions/substrait/extensions/functions_string.yaml
```

Dependencies can be referenced using short names (like `arithmetic`) throughout the dialect file.

## Complete Example

Here's a simplified dialect file for a hypothetical system:

```yaml
name: example_system
type: sql

dependencies:
  arithmetic: 
    https://github.com/substrait-io/substrait/blob/main/extensions/functions_arithmetic.yaml
  comparison:
    https://github.com/substrait-io/substrait/blob/main/extensions/functions_comparison.yaml

supported_types:
  i32:
    sql_type_name: integer
  i64:
    sql_type_name: bigint
  fp64:
    sql_type_name: double
  str:
    sql_type_name: varchar

scalar_functions:
  - name: arithmetic.add
    local_name: +
    infix: true
    required_options:
      overflow: ERROR
    supported_kernels:
      - i32_i32
      - i64_i64
      - fp64_fp64
      
  - name: arithmetic.subtract
    local_name: '-'
    infix: true
    required_options:
      overflow: ERROR
    supported_kernels:
      - i32_i32
      - i64_i64
      - fp64_fp64

  - name: comparison.equal
    local_name: =
    infix: true
    supported_kernels:
      - any_any

aggregate_functions:
  - name: sum
    supported_kernels:
      - i32
      - i64
      - fp64
```

## Use Cases

### Plan Validation

Before sending a plan to a consumer, a producer can validate it against the consumer's dialect:

```python
# Pseudocode
plan = create_substrait_plan()
consumer_dialect = load_dialect("duckdb.yaml")

if not validate_plan_against_dialect(plan, consumer_dialect):
    print("Plan uses features not supported by DuckDB")
```

### Feature Discovery

A system can query a dialect to determine capabilities:

```python
# Pseudocode
dialect = load_dialect("postgres.yaml")

if dialect.supports_function("arithmetic.add", ["i32", "i32"]):
    # Use add function
else:
    # Use alternative approach
```

### Testing

Dialects enable automated testing:

```python
# Pseudocode
for dialect in discover_dialects():
    for function in dialect.scalar_functions:
        test_function_conformance(dialect, function)
```

## Available Dialects

Several system dialects are maintained as part of the Substrait ecosystem:

* **DuckDB** - An in-process SQL OLAP database
* **DataFusion** - Apache Arrow-native query engine
* **PostgreSQL** - Popular relational database
* **SQLite** - Lightweight embedded database
* **Snowflake** - Cloud data warehouse
* **Velox/Presto** - High-performance vectorized execution engine
* **cuDF** - GPU-accelerated DataFrame library

Dialect files can be found in the [BFT (Basic Function Tests) dialect directory](https://github.com/substrait-io/substrait/tree/main/bft/dialects).

## Creating a Custom Dialect

To create a dialect for your system:

1. **Start with a template**: Copy an existing dialect file that's similar to your system
2. **Update basic information**: Change the `name` field to your system name
3. **Declare dependencies**: List all extension URIs your system uses
4. **Specify supported types**: List all Substrait types you support
5. **Document functions**: For each function, specify:
   - The Substrait function name
   - Your system's local name (if different)
   - Required option values
   - Supported type kernels
6. **Test the dialect**: Use the dialect testing framework to validate
7. **Contribute back**: Consider contributing your dialect to the Substrait repository

## Dialect Testing

Substrait provides testing infrastructure for dialects to ensure:

* Syntax validity of the dialect YAML
* Consistency between declared capabilities and actual support
* Proper reference to extension URIs
* Valid type and function declarations

Test files can be found in the `substrait/dialects/tests/` directory.

## Best Practices

1. **Be specific about kernels**: Rather than declaring all possible kernels, list only those you actually support
2. **Document required options**: If your system only supports specific option values, declare them
3. **Keep dialects updated**: Update your dialect file when you add or remove support for features
4. **Version your dialects**: Consider versioning dialect files alongside system versions
5. **Test thoroughly**: Use the dialect testing framework to validate your declarations

## Related Concepts

* [Extensions](../extensions/index.md) - Dialect files reference extension URIs
* [Simple Extensions](../extensions/index.md#simple-extensions) - Function and type extensions
* [Type System](../types/type_system.md) - Understanding Substrait types
* [Scalar Functions](../expressions/scalar_functions.md) - Function semantics


