# Contributing to Substrait

Welcome!

## Prerequisites

If you work with this repository you should have the following tools installed:

* [Pixi](https://pixi.prefix.dev)

Pixi will set up the correct development environment for you, including:

* [`buf`](https://docs.buf.build/installation) for easy generation of proto serialization/deserialization code
* [ANTLR](https://www.antlr.org)
* [Node.js](https://nodejs.org)
* A Python environment with the PyPI dev dependencies installed

You can also use other Python tooling like `uv` with the PyPI dependencies declared in `pyproject.toml`. In this case you need to set up the right versions of the non-PyPI dependencies yourself.

### Dependencies

Pixi manages two types of dependencies:

- **non-PyPI**: Includes all non-PyPI dependencies (Python itself, buf, ANTLR, Node.js, etc.) as `[tool.pixi.dependencies]` in `pyproject.toml`
- **PyPI**: Includes all PyPI development dependencies (Ruff, pytest, check-jsonschema, yamllint, etc.) and documentation dependencies (mkdocs and plugins) as a regular pyproject.toml `dev` dependency group in `pyproject.toml` which can be used with other Python package managers like `uv`.

## Common Development Tasks

Pixi provides convenient tasks for common development operations. Here are the most frequently used commands:

### Code Quality and Testing

```bash
# Format code
pixi run format

# Lint code
pixi run lint

# Run all tests
pixi run test
```

### Code Generation

```bash
# Generate both protobuf Python bindings and ANTLR parsers
pixi run generate
```

Some generated code is committed and some is not:

- **`gen/`** (protobuf Python bindings) is **gitignored** and regenerated on
  demand (`pixi run test` regenerates it first) — never commit it.
- **ANTLR parsers under `tests/*/antlr_parser/`** **are committed** (the
  generator prepends a license header). If you change a `.g4` grammar, run
  `pixi run generate-antlr` and commit the regenerated parsers.

Never hand-edit generated files.

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

## Breaking Changes

Substrait maintains compatibility through deprecation. Before making a breaking
change to the protobuf definitions, read the
[breaking-change policy](site/docs/spec/breaking_change_policy.md) and the
[versioning policy](site/docs/spec/versioning.md): a breaking change must ship a
migration strategy that is implemented in the active libraries before it lands,
and format/specification deprecations require PMC votes (see
[governance](site/docs/governance.md)).

`buf breaking` runs in CI and can be checked locally with:

```bash
pixi run buf breaking --against 'https://github.com/substrait-io/substrait.git#branch=main'
```

When removing a field, reserve its number and name (e.g.
`reserved 3; reserved "microseconds";`) so old plans carry unknown fields rather
than failing to parse. If `buf breaking` flags a change, mark the commit type
breaking with `!` (e.g. `feat(protos)!: remove deprecated field`) and include a
`BREAKING CHANGE: ` line in the PR body, or the breaking-change check will fail.
