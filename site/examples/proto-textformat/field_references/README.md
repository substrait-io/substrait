# Field Reference Protobuf Examples

This directory contains validated examples of `Expression.FieldReference` messages in protobuf text format (textproto).

## Files

- `root_reference.textproto` - FieldReference using RootReference to access input record fields
- `lambda_param_struct_field.textproto` - FieldReference using Expression root_type to access a field within a struct lambda parameter

## Validation

To validate these examples:

```bash
# Validate a single example
buf convert proto/substrait/algebra.proto \
  --type substrait.Expression.FieldReference \
  --from site/examples/proto-textformat/field_references/root_reference.textproto#format=txtpb \
  --to /dev/null

# Validate all examples
for file in site/examples/proto-textformat/field_references/*.textproto; do
  buf convert proto/substrait/algebra.proto \
    --type substrait.Expression.FieldReference \
    --from "$file#format=txtpb" \
    --to /dev/null
done
```

## Usage in Documentation

These examples are referenced in documentation files such as:
- `site/docs/expressions/lambda_functions.md`
- `site/docs/expressions/field_references.md`

The textproto files can be directly embedded in documentation using:

```markdown
\`\`\`protobuf
--8<-- "examples/proto-textformat/field_references/root_reference.textproto"
\`\`\`
```

## Format Notes

Protobuf text format uses `#` for comments, which are natively supported and don't require stripping during validation. This makes textproto ideal for documented examples.

## CI/CD Integration

These examples are automatically validated in CI (`.github/workflows/pr.yml`) to ensure they remain valid as the proto schema evolves.
