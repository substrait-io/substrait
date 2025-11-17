# Grammar
This file defines the grammars for:
1. The Substrait Type language used in the YAML extensions.
2. The test grammar language used to unit tests functions.

## Lambda Types and Expressions

### Proto Definition

The Substrait protobuf defines lambda types as follows (from `substrait/type.proto`):

```protobuf
message Lambda {
  // Types of parameters this lambda accepts.
  repeated Type parameter_types = 1;
  // Return type of the lambda.
  Type return_type = 2;
  uint32 type_variation_reference = 3;
  Nullability nullability = 4;
}
```

Lambda expressions are defined in `substrait/algebra.proto`:

```protobuf
message Lambda {
  // Types of parameters this lambda accepts.
  repeated Type parameter_types = 1;
  // Return type of the lambda.
  Type return_type = 2;
  // The lambda body expression.
  Expression body = 3;
}
```

### Text Format Syntax

To represent the above proto definitions in human-readable text format (used in YAML definitions and test cases), we use the following syntax:

#### Function Types (`func`)
Function types represent **generic** lambda types in YAML extension definitions. They use the `func` keyword:

**Single parameter (representing a lambda with 1 entry in `parameter_types`):**
- `func<T -> U>` - Single parameter without parentheses
- `func<(T) -> U>` - Single parameter with parentheses (**equivalent**)

**Multiple parameters (representing a lambda with 2+ entries in `parameter_types`):**
- `func<(T, U) -> V>` - Multiple parameters (parentheses **required**)

Example from `extensions/functions_list.yaml`:
```yaml
# This represents: repeated Type parameter_types = [T, T], Type return_type = i32
value: func<(T, T) -> i32>
```

#### Lambda Types (`lambda`)
Lambda types represent **concrete** lambda types in test cases. They use the `lambda` keyword:

**Single parameter (representing a lambda with 1 entry in `parameter_types`):**
- `lambda<i32 -> boolean>` - Single parameter without parentheses
- `lambda<(i32) -> boolean>` - Single parameter with parentheses (**equivalent**)

**Multiple parameters (representing a lambda with 2+ entries in `parameter_types`):**
- `lambda<(i32, i32) -> i32>` - Multiple parameters (parentheses **required**)

Example from test files:
```
# This represents: repeated Type parameter_types = [i32, i32], Type return_type = i32
((a, b) -> add(a, b))::lambda<(i32, i32) -> i32>
```

**Note:** `func` and `lambda` are compatible types. In YAML definitions, use `func` with generic type parameters (T, U, V). In test cases, use `lambda` with concrete types (i32, boolean, etc.).

## Regenerating
To regenerate all of the parsers use the following command
```sh
make all
```

### Requirements
You will need [ANTLR](https://www.antlr.org/index.html) available on your machine to regenerate the parser.

#### MacOS
```
brew install antlr
```

#### Ubuntu
```
sudo apt-get install antlr4
```