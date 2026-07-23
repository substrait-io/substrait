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

### A spec change is an ecosystem API change

A change to `proto/` ripples into every SDK that implements the spec. The active
libraries are the release gate — the maintained list is
[`active_libraries.md`](site/docs/community/active_libraries.md); treat it as the
source of truth rather than hardcoding the set. External consumers (e.g. Apache
DataFusion, DuckDB's substrait extension) are useful *usage signal* for gauging
blast radius but are not release-blocking.

Removing or changing a deprecated field is the most common breaking change here.
Beyond the breaking-change policy (migration must land in all active libraries
*before* the breaking change), the agent-specific workflow is:

1. Propose an explicit migration strategy (dual-write → prefer-consume-new →
   remove after a soak — see the URI→URN cookbook in the policy).
2. For each active library, determine whether it still *produces* the old field
   and whether it *consumes* the new one, and classify the change as
   *wire-compatible*, *source-breaking*, and/or *semantically breaking*. Search
   the repos via `gh`/MCP, or clone them (they're often sibling dirs like
   `../substrait-go`).
3. Land the companion migration PRs first, then remove the field here.

Put this per-library compatibility analysis in the PR (or draft PR) body — there
is no need to open a separate issue per PR. Issues are for surfacing design
discussion on larger or contentious changes before the design is settled.

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

Do include the rationale, the migration strategy, and the compatibility analysis
above. Keep commit bodies free of git trailers (`Signed-off-by`,
`Co-authored-by`, tool attribution) — the changelog pipeline strips them and repo
history doesn't use them.

## When in doubt

Spec changes are decided by maintainer consensus (community sync). For anything
beyond a trivial fix, prefer opening a **draft PR or an issue** to surface the
discussion rather than assuming the design. See [`CONTRIBUTING.md`](CONTRIBUTING.md)
for the human-facing build/test/release setup.
