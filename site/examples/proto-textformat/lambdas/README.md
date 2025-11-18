# Lambda Expression Protobuf Examples

This directory contains validated examples of `Expression.Lambda` messages in protobuf text format (textproto).

## Files

- `simple_multiply.textproto` - Basic lambda that multiplies parameter by 2
- `nested_transform.textproto` - Nested lambdas transforming a 2D array (each lambda references its own parameters)
- `closure_outer_lambda_simple.textproto` - Nested lambdas where inner lambda references outer lambda parameter using `lambda_depth: 1`
- `closure_root_reference.textproto` - Lambda with closure over input record field using RootReference
- `inline_invocation.textproto` - Direct lambda invocation using LambdaInvocation expression

## Validation

To validate these examples:

```bash
# Validate a single example
buf convert proto/substrait/algebra.proto \
  --type substrait.Expression.Lambda \
  --from site/examples/proto-textformat/lambdas/simple_multiply.textproto#format=txtpb \
  --to /dev/null

# Validate all examples
for file in site/examples/proto-textformat/lambdas/*.textproto; do
  buf convert proto/substrait/algebra.proto \
    --type substrait.Expression.Lambda \
    --from "$file#format=txtpb" \
    --to /dev/null
done
```

## Usage in Documentation

These examples are referenced in the lambda functions documentation:
- `site/docs/expressions/lambda_functions.md`

The textproto files can be directly embedded in documentation using:

```markdown
\`\`\`protobuf
--8<-- "examples/proto-textformat/lambdas/simple_multiply.textproto"
\`\`\`
```

## Format Notes

Protobuf text format uses `#` for comments, which are natively supported and don't require stripping during validation. This makes textproto ideal for documented examples.

## CI/CD Integration

These examples are automatically validated in CI (`.github/workflows/pr.yml`) to ensure they remain valid as the proto schema evolves.
