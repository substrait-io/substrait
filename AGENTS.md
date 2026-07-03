# AGENTS.md

Guidance for AI agents making changes to the Substrait specification repository.

## What this repository is

Substrait is a **cross-language specification** for relational algebra (query
plans). This repo holds the specification itself, not an implementation. The
canonical artifacts are:

- **Protobuf definitions** — [`proto/substrait/*.proto`](proto/substrait) (the
  serialized plan format; the source of truth).
- **Text/grammar** — [`grammar/*.g4`](grammar) (ANTLR grammar for the type and
  test-case text format) and [`text/`](text) JSON schemas for extensions and
  dialects.
- **Extensions** — [`extensions/*.yaml`](extensions) (standard function
  definitions).
- **Docs site** — [`site/`](site) (MkDocs; published spec documentation).

Because this is a spec, changes here ripple into the downstream SDKs. Treat a
change as an API change to an ecosystem, not a local edit.

## The downstream SDKs (critical context)

Any change to `proto/` affects four independent implementations:

- `substrait-io/substrait-java`
- `substrait-io/substrait-go`
- `substrait-io/substrait-python`
- `substrait-io/substrait-rs`

**Before removing or changing a deprecated proto field, analyze whether each SDK
still uses it.** This is the single most common task in this repo's history. The
expected workflow:

1. Determine whether each SDK still *produces* the old field, and whether it has
   migrated to *consuming* the new field. Search the repos with whatever GitHub
   access is available — the `gh` CLI or a GitHub MCP server — or clone them
   locally if the user has them checked out (they often do, as sibling dirs like
   `../substrait-go`).
2. Summarize impact per SDK (e.g. "read-path fallback only", "N/A — regenerates
   from proto") and classify the change: *wire-compatible*, *source-breaking*,
   and/or *semantically breaking*.
3. Only then propose the change, often as a **draft PR** pending community
   discussion, with companion PRs to the SDKs where needed.

Wire-compatibility details matter and are expected in PR descriptions: reserve
removed field numbers and names (`reserved 3; reserved "microseconds";`), note
that a dissolved single-member `oneof` is wire-identical to a plain field, and
note that old plans become *unknown fields* rather than parse failures.

## Environment & tooling

Everything runs through **[Pixi](https://pixi.prefix.dev)**. Do not invoke
`buf`/`antlr`/`ruff`/`pytest` from a global install — use the pixi tasks so
versions match CI.

```bash
pixi run format     # ruff format + buf format -w
pixi run lint       # buf lint, buf format check, ruff, editorconfig, yamllint, jsonschema checks
pixi run test       # pytest (regenerates protobuf bindings first)
pixi run generate   # regenerate ANTLR parsers + protobuf python bindings
```

Individual tasks worth knowing (see `[tool.pixi.tasks]` in
[`pyproject.toml`](pyproject.toml)): `pixi run buf <args>`,
`pixi run generate-antlr`, `pixi run generate-protobuf`, `pixi run mkdocs serve`.

For GitHub work (reading issues/PRs, creating PRs, searching the SDK repos), use
whatever access the user has configured — the `gh` CLI or a GitHub MCP server.
Commands below are shown with `gh` for concreteness; the MCP equivalents work
just as well.

## Generated code

- **`gen/`** (protobuf Python bindings) is **gitignored** — never commit it.
  `pixi run test` regenerates it via `buf generate`.
- **ANTLR parsers under `tests/*/antlr_parser/`** **are committed** and carry a
  license header. If you change a `.g4` grammar, run `pixi run generate-antlr`
  and commit the regenerated parsers (the `make` target prepends the license).
- Never hand-edit generated files.

## Verification before proposing a change

- Proto edits: `pixi run buf` for `lint`, `format --diff --exit-code`, and
  `breaking --against 'https://github.com/substrait-io/substrait.git#branch=main'`.
- Grammar edits: regenerate parsers, then `pixi run test` (e.g. the type-grammar
  and coverage tests under [`tests/`](tests)).
- YAML (extensions/examples/dialects): validated against schemas via
  `check-jsonschema` (part of `pixi run lint`).

## Commit & PR conventions (read carefully — these repeatedly trip up agents)

The repo uses **[Conventional Commits](https://www.conventionalcommits.org/)**
and semantic-release. Two CI checks gate every PR:

1. **PR title check** (`pr_title.yml`): the **PR title + body together must be a
   valid conventional commit message**, because they become the squash-merge
   commit message. commitlint validates it. Denote breaking changes with `!`
   after the type/scope, e.g. `feat(protos)!: remove deprecated field`.
2. **Breaking-change check** (`pr_breaking.yml`): if `buf breaking` detects a
   breaking proto change, the **PR body must contain a line starting with
   `BREAKING CHANGE: `**, or the check fails.

### PR description style (based on maintainer feedback)

Keep descriptions high-signal. The maintainer has repeatedly asked to remove:

- **Lists of files touched** — obvious from the diff.
- **Claims that CI-verified things pass** — e.g. "buf lint passes", "tests
  pass". These run as PR checks; if they failed the checks would be red.
- **Obvious process notes** — e.g. "draft pending review", "coordinating
  companion PRs" once that's implicit in the PR being a draft.

Do include: the rationale, the SDK impact analysis, and wire/source/semantic
compatibility analysis for proto changes.

### Commit hygiene

- Keep commit bodies **clean of git trailers**. The changelog pipeline was
  specifically configured to strip trailers (`Signed-off-by`, etc.), and recent
  history has no `Co-authored-by`/tool-attribution trailers. Match that.
- Don't reference issue numbers inside test-case *descriptions* (use `Closes #N`
  in the PR body instead).
- Commit-message linting can be checked locally with `npx commitlint` or the
  `commitlint` pre-commit hook (install with
  `pre-commit install --hook-type commit-msg`).

## Docs go with the change

Proto/grammar changes that alter semantics usually need matching updates under
[`site/docs/`](site/docs) (e.g. `types/type_classes.md`) and sometimes the
dialect/extension schemas in [`text/`](text). When adding doc examples, use
**external validated example files** (`--8<-- "examples/..."`), not inline code
blocks — see [`site/examples/README.md`](site/examples). Check the docs when the
user proposes a behavioral/semantic change even if they don't explicitly ask.

## When in doubt

- Spec changes are decided by maintainer consensus (community sync). For
  anything beyond a trivial fix, prefer opening a **draft PR or an issue** to
  surface the discussion rather than assuming the design.
- See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the human-facing version of the
  build/test/release setup.
