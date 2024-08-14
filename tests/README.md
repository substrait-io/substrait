# Substrait Test Format

This document describes the format for Substrait scalar test files.
A test file consists of the following elements:

1. Version declaration
2. Optional include statements
3. One or more test groups, each containing one or more test cases

## Syntax

### Version Declaration
The version declaration must be the first line of the file. It specifies the version of the test file format. The version declaration must be in the following format:
```
### SUBSTRAIT_SCALAR_TEST: V1
```

### Include Statements
Include statements should have at least one include statement. The include statement specifies the path to substrait extension functions. The include statement must be in the following format:
```
### SUBSTRAIT_INCLUDE: /extensions/functions_aggregate_approx.yaml
```

### Test Groups
A test group is a collection of test cases that are logically related.
- **description**: A string describing the test group or case. The description must start with a `#` character.
    ```code
    # Common Maths
    ```
### Test Cases
A test case consists of the following elements:

- **function**: The name of the function being tested. The function name must be a string.
- **arguments**: Comma-separated list of arguments to the function. The arguments must be literals.
- **options**: Optional comma-separated list of options in `key:value` format. The options describe the behavior of the function. The test should be run only on dialects that support the options. If options are not specified, the test should be run for all permutations of the options.
- **result**: The expected result of the function. Either `SUBSTRAIT_ERROR` or a literal value.
- **literal**: In the format `<name>::<datatype>`
- **description**: A string describing the test case

    ```code
    add(126::i8, 1::i8) = 127::i8  # addition of two numbers
    ```

### Spec

```
doc         := <version>
               (<include>)+
               ((<test_group>)?(<test_case>)+\n)+
version     := ### SUBSTRAIT_SCALAR_TEST: <test_library_version>
include     := ### SUBSTRAIT_INCLUDE: <uri>
test_group  := # <description>
test_case   := <function>(<arguments>) ([<options>])? = <result> (#<description>)?
description := string
function    := string
arguments   := <argument>, <argument>, ... <argument>
argument    := <literal>
literal     := <name>::<datatype>
result      := SUBSTRAIT_ERROR | <literal>
options     := <optLiteral>, <optLiteral>, ... <optLiteral>
optLiteral  := <option_name>:<option_value>
```

**TODO:** use ANTLR to describe the grammar and generate parser
### Literals

#### String
- **string**, **fixedchar**, **varchar**: A sequence of characters enclosed in single quotes. Example: 'Hello, world!'

#### Integer
Integers are represented as sequences of digits. Negative numbers are preceded by a minus sign.
- **i8**: 8-bit integer, range: -128 to 127
- **i16**: 16-bit integer, range: -32,768 to 32,767
- **i32**: 32-bit integer, range: -2,147,483,648 to 2,147,483,647
- **i64**: 64-bit integer, range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

#### Fixed Point decimals
- **decimal**: Fixed-point decimal number. Maximum 38 digits total, with up to 37 digits after the decimal point.
  Example: 123.456

#### Floating Point numbers
- **float**: General floating-point number, can be represented as:
  * Standard decimal notation: 123.456
  * Scientific notation: 1.23e4
  * Special values: `nan` (Not a Number), `+inf` (Positive Infinity), `-inf` (Negative Infinity)
- **float32**: Single-precision float, approximately 6 significant digits, range: ~1.2e-38 to ~3.4e38
- **float64**: Double-precision float, approximately 15 significant digits, range: ~2.3e-308 to ~1.7e308

#### Boolean
- Valid values: TRUE, FALSE, NULL

#### Date and Time
All date and time literals use ISO 8601 format:

- **date**: `YYYY-MM-DD`, example: `2021-01-01`
- **time**: `HH:MM:SS[.fraction]`, example: `12:00:00.000`
- **timestamp**: `YYYY-MM-DD HH:MM:SS[.fraction]`, example: `2021-01-01 12:00:00`
- **timestamp_tz**: `YYYY-MM-DD HH:MM:SS[.fraction]±HH:MM`, example: `2021-01-01 12:00:00+05:30`
- **interval year**: `INTERVAL 'P[n]Y[n]M'`, example: `INTERVAL 'P2Y3M'` (2 years, 3 months)
- **interval days**: `INTERVAL 'P[n]DT[n]H[n]M[n]S'`, example: `INTERVAL 'P2DT3H2M9S'` (2 days, 3 hours, 2 minutes, 9 seconds)

#### Other complex types
**TODO** Add support for complex types like arrays, structs, maps etc.

### Data Types

- **bool**: Boolean
- **i8**: 8-bit signed integer
- **i16**: 16-bit signed integer
- **i32**: 32-bit signed integer
- **i64**: 64-bit signed integer
- **f32**: 32-bit floating point number
- **f64**: 64-bit floating point number
- **dec**: Fixed-point `decimal<P,S>`
- **str**: Variable-length string
- **fchar**: Fixed-length string `fixedchar<N>`
- **vchar**: Variable-length string `varchar<N>`
- **vbin**: Fixed-length binary `fixedbinary<N>`
- **date**: Date
- **time**: Time
- **ts**: Timestamp
- **tstz**: Timestamp with timezone
- **iyear**: Interval year
- **iday**: Interval days

### Example of a test file

```code
### SUBSTRAIT_SCALAR_TEST:V1
### SUBSTRAIT_INCLUDE: /extensions/functions_arithmetic.yaml

# Common Maths
add(126::i8, 1::i8) = 127::i8

# Arithmetic Overflow Tests
add(127::i8, 1::i8) [overflow:ERROR] = <!ERROR>  #check overflow
```
The above test file has two test groups "Common Maths" and "Arithmetic Overflow Tests". Each has one test case. The test case in the second group has a name whereas case in the first one does not.
