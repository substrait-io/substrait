# Grammar
This file defines the grammars for:
1. The Substrait Type language used in the YAML extensions.
2. The test grammar language used to unit tests functions.

## Requirements
You will need [Pixi](https://pixi.prefix.dev) available on your machine to regenerate the parser.

## Regenerating
To regenerate all of the parsers run the following command from the repository root:

```sh
pixi run generate-antlr
```
