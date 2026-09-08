# AGENTS.md

Entry point for AI agents working on the Substrait specification repository.
Substrait is a **cross-language spec** for relational algebra (query plans) —
this repo holds the specification, not an implementation. Read the shared docs
below first, then keep the agent-specific notes in mind.

## Start here

- **[`README.md`](README.md)** — what Substrait is and how the repo is laid out.
- **[`CONTRIBUTING.md`](CONTRIBUTING.md)** — environment ([Pixi](https://pixi.prefix.dev)),
  the `pixi run` build/test/lint/generate tasks (use these rather than global
  tool installs so versions match CI), committed vs. generated code, doc
  examples, commit conventions, and breaking-change mechanics.
- **Spec policy** — the [breaking-change policy](site/docs/spec/breaking_change_policy.md),
  [versioning policy](site/docs/spec/versioning.md), and
  [governance](site/docs/governance.md) (PMC votes for spec deprecations).

For GitHub work (issues, PRs, searching the SDK repos), use whatever access is
configured — the `gh` CLI or a GitHub MCP server.

## What agents specifically need to get right

These are the things agents tend to miss even after reading the docs above.

### Check migration requirements

Specification changes can require coordinated ecosystem migrations. Read the
[breaking-change policy](site/docs/spec/breaking_change_policy.md) before changing
or removing existing behavior.

### Docs travel with the change

Proto/grammar changes that alter semantics usually need matching updates under
[`site/docs/`](site/docs) (e.g. `types/type_classes.md`) and sometimes the
dialect/extension schemas in [`text/`](text). Check the docs on any
behavioral/semantic change even when not explicitly asked.

### Keep PR descriptions high-signal

The PR title and body together become the squash-merge commit message, so they
must form a valid conventional commit (see `CONTRIBUTING.md`). Beyond that, leave
out the noise agents tend to add:

- **Lists of files touched** — they're in the diff.
- **Claims that CI-verified things pass** — e.g. "buf lint passes", "tests pass".
  If they didn't, the checks would be red.
- **Process notes that are already implicit** — e.g. "draft pending review".

Do include the rationale and any migration strategy required by policy. Keep
commit bodies free of git trailers (`Signed-off-by`,
`Co-authored-by`, tool attribution) — the changelog pipeline strips them and repo
history doesn't use them.

## When in doubt

Spec changes are decided by maintainer consensus (community sync). For anything
beyond a trivial fix, prefer opening a **draft PR or an issue** to surface the
discussion rather than assuming the design. See [`CONTRIBUTING.md`](CONTRIBUTING.md)
for the human-facing build/test/release setup.
