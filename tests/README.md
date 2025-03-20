# Substrait Test Format

This document describes the format for Substrait scalar test files.
A test file consists of the following elements:

1. Version declaration
2. Optional include statements
3. One or more test groups, each containing one or more test cases

## Syntax

### Version Declaration
The version declaration must appear as the first line of the file. It defines the type of tests in the file and test file format version.
Each test file must exclusively contain either scalar tests or aggregate tests. Mixing test types in the same file is not allowed.
The version declaration should follow this format:
```code
### SUBSTRAIT_SCALAR_TEST: V1
```
or

```code
### SUBSTRAIT_AGGREGATE_TEST: V1
```

### Include Statements
Include statements should have at least one include statement. The include statement specifies the path to substrait extension functions. The include statement must be in the following format:
```code
### SUBSTRAIT_INCLUDE: /extensions/functions_aggregate_approx.yaml
```

### Test Groups
A test group is a collection of test cases that are logically related. Test groups are purely for categorization purposes and do not affect the execution or meaning of tests.
- **description**: A string describing the test group or case. The description must start with a `#` character.
    ```code
    # Common Maths
    ```
### Scalar Test Cases
A test case consists of the following elements:

- **function**: The name of the function being tested. The function name must be an identifier alphanumeric string.
- **arguments**: Comma-separated list of arguments to the function. The arguments must be literals.
- **options**: Optional comma-separated list of options in `key:value` format. The options describe the behavior of the function. The test should be run only on dialects that support the options. If options are not specified, the test should be run for all permutations of the options.
- **result**: The expected result of the function. Either `SUBSTRAIT_ERROR` or a literal value.
- **literal**: In the format `<literal_value>::<datatype>`
- **description**: A string describing the test case

    ```code
    add(126::i8, 1::i8) = 127::i8  # addition of two numbers
    ```

### Aggregate Test Cases
A test case consists of the following elements:

- **table definition**:
- **function**: The name of the function being tested. The function name must be an identifier alphanumeric string.
- **arguments**: Comma-separated list of arguments to the function. The arguments can be literals or column references.
- **options**: Optional comma-separated list of options in `key:value` format. The options describe the behavior of the function. The test should be run only on dialects that support the options. If options are not specified, the test should be run for all permutations of the options.
- **result**: The expected result of the function. Either `SUBSTRAIT_ERROR` or a literal value.

Aggregate test cases support 3 formats:
1. **Single Argument**: The test case for an aggregate function with single argument as a column in a table. The table is defined in the test case.
    ```code
    sum((1, 2, 3, 4, 5)::i8) = 15::i8  # addition of two numbers
    ```
2. **Multiple Columns Compact**: The test case for an aggregate function with on one or more columns of a table as argument. The table is defined before the function name, in the same line as the testcase.
    ```code
    ((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(col0::fp32, col1::fp32) = 1::fp64
    ```
3. **Multiple Columns Verbose**: The test case for an aggregate function with one or more columns of a table as arguments. The table is defined before the function name. The function arguments refer to the columns in a table.
    ```code
    DEFINE t1(fp32, fp32) = ((20, 20), (-3, -3), (1, 1), (10,10), (5,5))
    corr(t1.col0, t1.col1) = 1::fp64
    ```

#### Example:
A testcase with mixed arguments
```code
    ((20), (-3), (1), (10)) LIST_AGG(col0::fp32, ','::string) = 1::fp64
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
literal     := <literal_value>::<datatype>
result      := <substrait_error> | <literal>
options     := <option>, <option>, ... <option>
option      := <option_name>:<option_value>
literal_value := string | integer | decimal | float | boolean | date | time | timestamp | timestamp_tz | interval year | interval days | null
datatype    := <basic_type> | <parametrized_type> | <complex_type>
basic_type := bool | i8 | i16 | i32 | i64 | f32 | f64 | str | date | time | ts | tstz | iyear | iday | <parametrized_type>
parametrized_type := dec<int,int> | fchar<int> | vchar<int> | vbin<int>
complex_type := <struct> | <list> | <map>
substrait_error := <!ERROR> | <!UNDEFINED>
```

Actual antlr grammar can be found in `grammar/FuncTestCaseParser.g4`
### Literals

`<literal_value>` described in this section.

#### String
- **string**, **fixedchar**, **varchar**: A sequence of characters enclosed in single quotes. To include a single quote or backslash within the sequence, escape them with a backslash (e.g., `\'` for a single quote and `\\` for a backslash). Example: 'Hello, world!'

#### Integer
Integers are represented as sequences of digits. Negative numbers are preceded by a minus sign.
- **i8**: 8-bit integer, range: -128 to 127
- **i16**: 16-bit integer, range: -32768 to 32767
- **i32**: 32-bit integer, range: -2147483648 to 2147483647
- **i64**: 64-bit integer, range: -9223372036854775808 to 9223372036854775807

#### Fixed Point decimals
- **decimal**: Fixed-point decimal number. Maximum 38 digits total, with up to 37 digits after the decimal point.
  Example: 123.456

#### Floating Point numbers
- **float**: General floating-point number, can be represented as:
  * Standard decimal notation: 123.456
  * Scientific notation: 1.23e4
  * Special values: `+inf` (Positive Infinity), `-inf` (Negative Infinity), `+0` (Positive Zero), `-0` (Negative Zero), `nan` (Not a Number), `snan` (Signaling NaN)
- **float32**: Single-precision float, approximately 6 significant digits, range: ~1.2e-38 to ~3.4e38
- **float64**: Double-precision float, approximately 15 significant digits, range: ~2.3e-308 to ~1.7e308

#### Boolean
- Valid values: true, false

#### Date and Time
All date and time literals use ISO 8601 format:

- **date**: `YYYY-MM-DD`, example: `2021-01-01`
- **time**: `HH:MM:SS[.fraction]`, example: `12:00:00.000`
- **timestamp**: `YYYY-MM-DD HH:MM:SS[.fraction]`, example: `2021-01-01 12:00:00`
- **timestamp_tz**: `YYYY-MM-DD HH:MM:SS[.fraction]±HH:MM`, example: `2021-01-01 12:00:00+05:30`
- **interval year**: `'P[n]Y[n]M'`, example: `'P2Y3M'` (2 years, 3 months)
- **interval days**: `'P[n]DT[n]H[n]M[n]S'`, example: `'P2DT3H2M9S'` (2 days, 3 hours, 2 minutes, 9 seconds)
  ex2: 'P1DT2H3M4.45::iday<3>' (1 day, 2 hours, 3 minutes, 4 seconds, 450 milliseconds)`

#### Other complex types

- **list**: A list of values enclosed in brackets and separated by commas. Example: `[1, 2, 3]::list<i8>`, `['a', 'b', 'c']::list<str>`

**TODO** Add support for complex types like structs, maps etc.

### Data Types

Use short names listed in https://substrait.io/extensions/#function-signature-compound-names

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
- **pt**: Precision Time
- **pts**: Precision Timestamp
- **ptstz**: Precision Timestamp with timezone


### Options

Option names and values can be found in extension files. Example: `[overflow:ERROR]`

### Errors

If return type is `<!ERROR>` then the test case should fail with an error. If the return type is `<!UNDEFINED>` then the test case should not fail result could be anything.
### Example of a test file for scalar functions

```code
### SUBSTRAIT_SCALAR_TEST:V1
### SUBSTRAIT_INCLUDE: /extensions/functions_arithmetic.yaml

# Common Maths
add(126::i8, 1::i8) = 127::i8

# Arithmetic Overflow Tests
add(127::i8, 1::i8) [overflow:ERROR] = ERROR  #check overflow
```
The above test file has two test groups "Common Maths" and "Arithmetic Overflow Tests". Each has one test case. The test case in the second group has a name whereas case in the first one does not.


### Example of a test file for aggregate functions

```code
### SUBSTRAIT_AGGREGATE_TEST:V1
### SUBSTRAIT_INCLUDE: /extensions/functions_arithmetic.yaml

# Correlation Tests
((20, 20), (-3, -3), (1, 1), (10,10), (5,5)) corr(col0::fp32, col1::fp32) = 1::fp64
DEFINE t1(fp32, fp32) = ((20, -20), (-3, 3), (1, -1), (10, -10), (5, -5))
corr(t1.col0, t1.col1) = -11::fp64
```
