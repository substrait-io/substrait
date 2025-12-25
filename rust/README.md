# Rust Artifact Generation
The `rust` directory handles the generation of the following artifacts as part of a new Substrait release:
* `substrait-extensions`
* `substrait-protobufs`
* `substrait-version`

## Artifacts

### substrait-extensions

Packages code for parsing Simple Extensions generated from the `text/simple_extensions_schema.yaml`.

Additionally, packages all core extensions under `extensions/` for downstream consumers.

### substrait-protobuf

Packages Prost generated code for working with protobufs under `proto/substrait`.

### substrait-version

Packages version constants for easy consumption by downstreams.