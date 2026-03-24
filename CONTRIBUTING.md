# Contributing to Substrait

Welcome!

## Prerequisites

If you work with this repository you should have the following tools installed:

* [Pixi](https://pixi.prefix.dev)

Pixi will set up the correct development environment for you, including:

* [`buf`](https://docs.buf.build/installation) for easy generation of proto serialization/deserialization code
* [ANTLR](https://www.antlr.org/)
* A Python environment with the dependencies installed

## Common Development Tasks

Pixi provides convenient tasks for common development operations. Here are the most frequently used commands:

### Code Quality and Testing

```bash
# Format code with Black
pixi run black

# Lint Python code with Flake8
pixi run flake8

# Run all tests
pixi run test
```

### Code Generation

```bash
# Generate protobuf Python bindings
pixi run generate-protobuf

# Generate ANTLR parsers
pixi run generate-antlr
```

### Validation and Linting

```bash
# Check protobuf formatting
pixi run check-protobuf

# Lint YAML files
pixi run yamllint

# Validate YAML extensions against schemas
pixi run check-jsonschema --schemafile text/simple_extensions_schema.yaml extensions/*.yaml

# Check editorconfig compliance
pixi run editorconfig-checker
```

### Documentation

```bash
# Build the documentation website
pixi run mkdocs build

# Serve documentation locally with live reload
pixi run mkdocs serve
```

### Release Management

```bash
# Perform a dry-run release (testing only)
pixi run dry-run

# Execute the release process (maintainers only)
pixi run release
```

### Environment Management

Pixi manages two separate environments:

- **dev**: Includes all development dependencies (Black, Flake8, pytest, etc.)
- **docs**: Includes documentation dependencies (mkdocs and plugins)

Most tasks automatically use the appropriate environment. To explicitly work in a specific environment:

```bash
# Activate the dev environment
pixi shell -e dev

# Activate the docs environment
pixi shell -e docs
```


## Documentation Examples

When adding examples to the documentation, please use external example files instead of inline code blocks. This ensures examples are validated against schemas in CI/CD and prevents documentation drift.

See [`site/examples/README.md`](site/examples/README.md) for complete instructions on creating and including validated examples.

Quick example:

```yaml
--8<-- "examples/extensions/my_example.yaml"
```

## Commit Conventions

Substrait follows [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit message structure. You can use [`pre-commit`](https://pre-commit.com/) to check your messages for you, but note that you must install pre-commit using `pre-commit install --hook-type commit-msg` for this to work. CI will also lint your commit messages. Please also ensure that your PR title and initial comment together form a valid commit message; that will save us some work formatting the merge commit message when we merge your PR.

Examples of commit messages can be seen [here](https://www.conventionalcommits.org/en/v1.0.0/#examples).
