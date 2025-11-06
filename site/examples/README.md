# Documentation Examples

This directory contains example files that are included in the Substrait documentation.

By storing examples as separate files instead of inline in markdown, we can easily validate against schemas via CI/CD.

## Directory Structure

```
examples/
├── extensions/     # Extension function examples (e.g., any types)
├── types/          # User-defined type examples
└── README.md       # This file
```

All examples are validated against `text/simple_extensions_schema.yaml` in CI/CD.

## Including Examples in Markdown

Use the pymdownx.snippets syntax to include example files:

````markdown
```yaml
--8<-- "examples/extensions/distance_functions.yaml"
```
````

The snippet will be rendered with syntax highlighting and the actual file content.
