# Documentation Examples

This directory contains example files that are included in the Substrait documentation.

By storing examples as separate files instead of inline in markdown, we can easily validate against schemas via CI/CD.

## Directory Structure

```
examples/
├── extensions/        # Example extensions (functions, types, type variations)
├── proto-textformat/  # Plan and expression examples in protobuf text format
├── types/             # User-defined type examples
└── README.md          # This file
```

The YAML files under `extensions/` and `types/` are validated against
`text/simple_extensions_schema.yaml` in CI/CD.

## Extension URNs in examples

Every example extension uses the `extension:org.example:…` URN owner. `example.org`
is reserved for documentation by
[RFC 2606](https://datatracker.ietf.org/doc/html/rfc2606) and follows the
reverse-domain convention that the [extension docs](../docs/extensions/index.md)
prescribe for the owner segment.

The `extension:io.substrait:…` owner is reserved for the official extensions in
[`extensions/`](../../extensions). Examples are not catalog entries: do not
reference their URNs from real plans, and expect their contents and URNs to change
without a deprecation cycle. An example must never reuse the URN of an official
extension, since downstream artifacts bundle the two together.

## Including Examples in Markdown

Use the pymdownx.snippets syntax to include example files:

````markdown
```yaml
--8<-- "examples/extensions/distance_functions.yaml"
```
````

The snippet will be rendered with syntax highlighting and the actual file content.
