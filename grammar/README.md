# Grammar
This directory defines the grammars for:
1. The Substrait Type language used in the YAML extensions.
2. The test grammar language used to unit test functions.

## Requirements
You will need [Pixi](https://pixi.prefix.dev) available on your machine to regenerate the parsers.

## Regenerating
To regenerate all language targets, run the following command from the repository root:

```sh
pixi run generate-antlr
```

Each language also has an independent task:

```sh
pixi run generate-antlr-python
pixi run generate-antlr-go
pixi run generate-antlr-java
```

Python output lives under `tests/*/antlr_parser/`. All other targets live under
`gen/antlr/<language>/`, including Go under `gen/antlr/go/`.
All generated files are gitignored.

All targets use the ANTLR tool supplied by Pixi.

## Checking generated parsers
Run `pixi run check-antlr-<language>` to generate and compile a single target.
The Python check runs the parser and type tests instead. CI runs the non-Python
checks in parallel using a language matrix; Python is covered by the full test
suite. The aggregate generation task runs its dependencies sequentially in Pixi.
For parallel local generation, run
`pixi run make -C grammar -j3 all`.

Java and Python tooling comes from Pixi. The Go check requires Go 1.23 or later
on `PATH`.

`pixi run test` and `pixi run test-extension-coverage` generate only the Python
parsers they need, without generating the other language targets.
