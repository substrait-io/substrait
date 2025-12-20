# Protobuf Text Format Examples

Each subdirectory contains examples of different protobuf message types in text format (textproto). These examples are embedded in the documentation and validated in CI/CD to ensure they remain valid as the proto schema evolves.

We use protobuf text format (textproto) rather than JSON for these examples because textproto supports comments. This allows us to annotate examples inline with explanatory notes about what each field does. JSON does not support comments, which would make the examples less instructive.

## Directories

- `lambda/` - Examples of `Expression.Lambda` messages
- `lambda_invocation/` - Examples of `Expression.LambdaInvocation` messages
- `field_reference/` - Examples of `Expression.FieldReference` messages
