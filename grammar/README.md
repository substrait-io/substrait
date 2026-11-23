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
pixi run generate-antlr-cpp
pixi run generate-antlr-csharp
```

Python output lives under `tests/*/antlr_parser/`. All other targets live under
`gen/antlr/<language>/`, including Go under `gen/antlr/go/`.
All generated files are gitignored.

All targets use the ANTLR tool supplied by Pixi.

## Checking generated parsers
CI checks Go, Java, C++, and C# parsers in separate, parallel jobs.
The compile commands live in [the PR workflow](../.github/workflows/pr.yml).
Go and C# jobs create temporary build projects in their generated output directories.
The C++ job checks generated sources against matching runtime headers using
`-fsyntax-only`; it does not link or execute a parser. Python is covered by the
full test suite.

The aggregate generation task runs its dependencies sequentially in Pixi.
For parallel local generation, run `pixi run make -C grammar -j5 all`.

Java and Python tooling comes from Pixi. The other checks require Go 1.23 or
later, a C++17 compiler, or .NET SDK 10 or later for their respective targets.
The C++ check uses the `antlr-cpp` Pixi environment, which supplies matching
ANTLR tool and runtime versions. This environment supports Linux x86-64 and
Intel macOS; it is separate from the default environment so ARM development
setups are unaffected.

`pixi run test` and `pixi run test-extension-coverage` generate only the Python
parsers they need, without generating the other language targets.
