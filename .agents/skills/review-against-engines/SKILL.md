---
name: review-against-engines
description: Review a Substrait function test file (tests/cases/**/*.test) by translating each case to SQL, running it on reference engines (DuckDB, DataFusion, SQLite, Spark), and reporting per engine which cases agree with the file. Use when asked to review, validate, sanity-check, or cross-check .test cases or expected results against real engines.
---

# Review test cases against reference engines

This skill runs each case of a `.test` file on DuckDB, DataFusion, SQLite and
Spark, and reports, per engine, which expectations those engines agree with.
All four engines run in-process from the Pixi `engines` environment (Spark in
local mode, with its default ANSI mode), so there is nothing to start or stop. The goal is to check whether the **behavior** the test
describes matches what the engines do, and to find expectations that look
wrong. Some differences are expected and are not findings:

- **Result types differ between dialects.** A matching value with a different
  type (`PASS_TYPE_DIFF`) counts as agreement. Mention type differences only
  briefly in the report.
- **An engine implements a different option** from the one a case specifies.
  Record which option it follows.

**Each invocation is a separate investigation.** Work out every fact about the
function and the engines in this run: which SQL function matches, how each
engine behaves, and which option it follows. Get them from the extension YAML,
the engine docs, and probe queries. Don't reuse mappings, cases files, results
or conclusions from earlier runs, other conversations or memory. Don't write
what you learn back into this skill, its reference files, or memory.

The test file format is described in [`tests/README.md`](../../../tests/README.md).
Per-engine types, literals and query shapes are in
[`reference/type-mapping.md`](reference/type-mapping.md). Read that file before
writing any SQL.

## Input

A path to a `.test` file, or a function name. For a function name, find its
file under `tests/cases/`. Process one file at a time. If several are given,
run each one and report on each.

## Steps

### 1. Record engine versions

```sh
pixi run -e engines engines-versions
```

### 2. Understand the function and map it per engine

- Read the test file. Then read the function's definition in the extension YAML
  named by `SUBSTRAIT_INCLUDE` (`extension:io.substrait:functions_arithmetic` is
  `extensions/functions_arithmetic.yaml`). Note its options, return types and
  documented semantics.
- For each engine, find the SQL function or operator that matches most closely.
  It must compute the same thing, not merely share a name: check the engine docs
  and run a probe query if unsure. Probe with the runner, which uses the same
  engine setup as a full run (ENGINE is `duckdb`, `datafusion`, `sqlite` or
  `spark`):

  ```sh
  pixi run -e engines engines-run --probe ENGINE "SELECT ..."
  ```

- **If no matching function exists, stop for that engine.** Record
  `{"unsupported": "<why>"}` for it and write no SQL for it. Do not work around a
  missing function by composing an approximation, because that tests your
  composition instead of the engine.

### 3. Translate each case into SQL

Write a cases JSON file to a fresh temporary directory (`mktemp -d`). It holds
one entry for every case in the file, in file order:

```json
{
  "test_file": "tests/cases/arithmetic/add.test",
  "function": "add",
  "engines": {
    "duckdb":     {"sql_function": "+"},
    "datafusion": {"sql_function": "+"},
    "sqlite":     {"sql_function": "+", "notes": "dynamically typed; INTEGER is 64-bit"},
    "spark":      {"sql_function": "+"}
  },
  "cases": [
    {
      "line": 5,
      "group": "basic",
      "substrait": "add(120::i8, 5::i8) = 125::i8",
      "options": {},
      "expected": {"value": 125, "type": "i8"},
      "sql": {
        "duckdb": "SELECT CAST(120 AS TINYINT) + CAST(5 AS TINYINT)",
        "datafusion": "SELECT CAST(120 AS TINYINT) + CAST(5 AS TINYINT)",
        "sqlite": "SELECT 120 + 5",
        "spark": "SELECT CAST(120 AS TINYINT) + CAST(5 AS TINYINT)"
      },
      "skip": {}
    }
  ]
}
```

Rules:

- **Fields.** `line` is the 1-based line in the test file. `substrait` is the
  case text verbatim. `options` holds the case's `[k:v]` options as an object.
- **Literal casts.** Cast every literal to the engine type for its Substrait
  type (see `reference/type-mapping.md`), so the engine runs the typed overload
  being tested. SQLite is the exception, since it has no meaningful casts.
- **Expected values.** Encode `expected` as shown in the type-mapping
  reference: `{"value", "type"}`, `{"error": true}`, `{"undefined": true}`, or
  `{"column": [...], "type"}` for window cases.
- **Skip only when untestable.** Use `skip: {"<engine>": "<reason>"}` only when
  the case can't be expressed on that engine at all (no NaN in SQLite, no
  decimals in SQLite, no fp32 in SQLite for rounding-sensitive cases, and
  similar). Never skip a case because you expect it to fail.
- **Options.** Don't skip cases that have options. Translate and run them
  normally. Engines have one fixed behavior, and the results show which option
  each engine implements.
- **Composition.** When the result side is a function call, use
  `SELECT (lhs) IS NOT DISTINCT FROM (rhs)` with
  `"expected": {"value": true, "type": "bool"}`.

### 4. Run

```sh
pixi run -e engines engines-run <tmpdir>/cases.json
```

The runner writes `<tmpdir>/results.json` and prints a markdown report. It has
a status matrix, a per-case table, and details for every case that didn't pass.
Statuses:

| Status | Meaning |
|---|---|
| `PASS` | Value and result type match. |
| `PASS_TYPE_DIFF` | Value matches, but the result type differs or can't be checked (always the case on SQLite). |
| `CHECK` | An error was expected and raised, but at parse or plan time. Confirm it is the error the case means, not broken SQL. |
| `FAIL` | Wrong value, or a value where an error was expected. |
| `ERROR` | An error where a value was expected. |
| `N/A` | Unsupported engine, or the case was skipped. |

### 5. Triage, fix translation problems, re-run

Go through every non-`PASS` result, and every `PASS` on an `<!ERROR>` case, and
classify it:

1. **Translation problem.** The SQL doesn't express the case: a missing cast, a
   wrong literal, a SQL syntax error, the wrong function. Fix the cases JSON and
   re-run. Iterate until none remain. Plan-time errors (`CHECK`, `ERROR` with
   `compile`) are usually this.
2. **Engine follows a different option.** The case has options, and the engine
   implements a different one. For example, the case expects an error but the
   engine returns a wrapped or saturated value. This is not a test problem.
   Record which option each engine follows, based on its results in this run.
3. **Engine type gap.** Not a test problem either. The engine has no
   equivalent type, or produces a wider or narrower result type.
4. **Expectation looks wrong.** The engines disagree with the test, and the
   disagreement isn't explained by 2 or 3. This is the actionable finding, and
   it is strongest when several engines agree with each other. Before reporting
   it, check the extension YAML's documented semantics. Also check
   floating-point details (exact binary values, fp32 rounding).

For every `<!ERROR>` case that passed, read the error message and confirm it is
the error the case is about. For example, an overflow case should fail with
"Overflow", not "function does not exist".

### 6. Report

Report in the terminal:

1. A header with the test file, the function, and the engine versions.
2. Per engine, the SQL function it was mapped to (or why it is unsupported) and
   counts of passed / failed / N/A after triage. Give a pass count that treats
   `PASS_TYPE_DIFF` as passed, and state the strict `PASS` count separately.
3. **Suspect expectations** (class 4): each with its line number, the case, what
   each engine returned, and why the expectation looks wrong. Put this first if
   there are any.
4. **Option behavior**: for each option exercised (`overflow`, `rounding`, …),
   which option each engine implements.
5. Other notable differences (types, unsupported cases), kept short.

Don't paste the full runner output. Point to `results.json` for details. Don't
edit the `.test` file unless asked. If the user asks for fixes, propose them
with the evidence above.
