# Documentation Examples

This directory contains example files that are included in the Substrait documentation.

## Why External Example Files?

By storing examples as separate files instead of inline in markdown:

1. **Schema Validation**: Examples are automatically validated against JSON schemas in CI/CD
2. **Reusability**: The same example can be included in multiple documentation pages
3. **Testing**: Example files can be used in automated tests
4. **Single Source of Truth**: No risk of documentation drift when examples are validated

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

## Validation

All example files are automatically validated in CI/CD:

- **Extension examples** (`examples/extensions/*.yaml`) - validated against `text/simple_extensions_schema.yaml`
- **Type examples** (`examples/types/*.yaml`) - validated against `text/simple_extensions_schema.yaml`
- Files must pass schema validation for PRs to merge

**Note**: JSON examples in the tutorial (`docs/tutorial/sql_to_substrait.md`) are protobuf JSON representations
of Substrait plans. There is currently no JSON schema available for validating these, so they remain inline.

## Adding a New Example

1. Create your example file in the appropriate subdirectory:
   ```bash
   touch site/examples/extensions/my_example.yaml
   ```

2. Write valid content according to the schema:
   ```yaml
   urn: extension:example:my_extension
   # ... rest of your example
   ```

3. Include it in your markdown documentation:
   ````markdown
   ```yaml
   --8<-- "examples/extensions/my_example.yaml"
   ```
   ````

4. Validate locally before committing:
   ```bash
   check-jsonschema --schemafile text/simple_extensions_schema.yaml \
     site/examples/extensions/my_example.yaml
   ```

## Tips

- **Keep examples minimal**: Focus on demonstrating one concept clearly
- **Add comments**: YAML comments help explain what the example demonstrates
- **Test locally**: Always validate before pushing to catch errors early
- **Descriptive names**: Use clear, descriptive filenames like `function_with_dependencies.yaml`

## Schema References

- Extension schema: `text/simple_extensions_schema.yaml`
- Dialect schema: `text/dialect_schema.yaml`
