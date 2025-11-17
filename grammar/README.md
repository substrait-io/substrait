# Grammar
This file defines the grammars for:
1. The Substrait Type language used in the YAML extensions.
2. The test grammar language used to unit tests functions.

## Function Types and Lambdas

### Function Types (`func`)
Function types are used in YAML extension definitions to specify generic function parameters. They use the `func` keyword:

**Single parameter:**
- `func<T -> U>` - Single parameter without parentheses
- `func<(T) -> U>` - Single parameter with parentheses (equivalent to above)

**Multiple parameters:**
- `func<(T, U) -> V>` - Multiple parameters (parentheses required)

### Lambda Types (`lambda`)
Lambda types are used in test cases to specify concrete lambda expressions. They use the `lambda` keyword:

**Single parameter:**
- `lambda<i32 -> boolean>` - Single parameter without parentheses
- `lambda<(i32) -> boolean>` - Single parameter with parentheses (equivalent to above)

**Multiple parameters:**
- `lambda<(i32, i32) -> i32>` - Multiple parameters (parentheses required)

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