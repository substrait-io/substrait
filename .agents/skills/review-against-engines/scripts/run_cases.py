#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Run translated Substrait test cases against in-process reference engines.

Usage:
    run_cases.py CASES_JSON     run cases, write results.json next to the input,
                                print a markdown report
    run_cases.py --versions     print engine versions
    run_cases.py --probe ENGINE SQL
                                run one query on one engine and print the
                                rows, result type and any error

The cases JSON format is documented in ../SKILL.md.
"""

from __future__ import annotations

import datetime as dt
import json
import logging
import math
import os
import re
import sqlite3
import struct
import sys
import time
from decimal import Decimal, InvalidOperation
from pathlib import Path

ENGINES = ("duckdb", "datafusion", "sqlite", "spark")

PASS = "PASS"
PASS_TYPE_DIFF = "PASS_TYPE_DIFF"
CHECK = "CHECK"  # expected an error and got one, but at plan/parse time
# The engine's behavior differs from the test. SQL dialects legitimately
# deviate from Substrait semantics, so this is an outcome, not a failure.
DEVIATES = "DEVIATES"
ERROR = "ERROR"  # the query failed to parse or plan where a value was expected
NA = "N/A"
STATUSES = (PASS, PASS_TYPE_DIFF, CHECK, DEVIATES, ERROR, NA)


# --------------------------------------------------------------------------
# Engine adapters. Each returns (rows: list[tuple], col0_type: str | None) or
# raises EngineError(kind, message) where kind is "compile" or "runtime".
# --------------------------------------------------------------------------


class EngineError(Exception):
    def __init__(self, kind: str, message: str):
        super().__init__(message)
        self.kind = kind
        self.message = message


DUCKDB_COMPILE_ERRORS = {
    "ParserException",
    "BinderException",
    "CatalogException",
    "NotImplementedException",
    "SyntaxException",
}


def run_duckdb(sql: str):
    import duckdb

    con = duckdb.connect()
    try:
        rel = con.sql(sql)
        if rel is None:
            raise EngineError("compile", "statement returned no result set")
        col_type = str(rel.types[0])
        rows = rel.fetchall()
        return rows, col_type
    except duckdb.Error as e:
        kind = "compile" if type(e).__name__ in DUCKDB_COMPILE_ERRORS else "runtime"
        raise EngineError(kind, f"{type(e).__name__}: {e}") from None
    finally:
        con.close()


DATAFUSION_COMPILE_PREFIXES = (
    "SQL error",
    "Error during planning",
    "Schema error",
    "This feature is not implemented",
    "Optimizer rule",
)


def run_datafusion(sql: str):
    from datafusion import SessionContext

    try:
        df = SessionContext().sql(sql)
        schema = df.schema()
        col_type = str(schema.field(0).type)
        table = df.to_arrow_table()
        rows = [tuple(r.values()) for r in table.to_pylist()]
        return rows, col_type
    except Exception as e:  # datafusion raises plain Exception subclasses
        msg = str(e)
        kind = (
            "compile"
            if any(p in msg for p in DATAFUSION_COMPILE_PREFIXES)
            else "runtime"
        )
        raise EngineError(kind, msg) from None


SQLITE_COMPILE_PATTERNS = (
    "no such function",
    "syntax error",
    "no such column",
    "wrong number of arguments",
)


def run_sqlite(sql: str):
    con = sqlite3.connect(":memory:")
    try:
        rows = con.execute(sql).fetchall()
        # SQLite has no static result types; report the storage class of the
        # first non-null value.
        col_type = None
        for r in rows:
            if r[0] is not None:
                col_type = type(r[0]).__name__
                break
        return rows, col_type
    except sqlite3.Error as e:
        msg = str(e)
        kind = (
            "compile" if any(p in msg for p in SQLITE_COMPILE_PATTERNS) else "runtime"
        )
        raise EngineError(kind, f"{type(e).__name__}: {msg}") from None
    finally:
        con.close()


_SPARK = None


def _spark_session():
    """Start one local Spark session per run; its JVM takes seconds to start.

    Spark runs with its defaults, so ANSI mode is on (overflow and invalid
    casts raise errors). Each case gets a fresh newSession() of it.
    """
    global _SPARK
    if _SPARK is None:
        from pyspark.sql import SparkSession

        # Per-error JSON stack traces come from this Python-side logger.
        logging.getLogger("SQLQueryContextLogger").disabled = True
        # The JVM inherits stderr; send its startup banner and warnings to
        # /dev/null while keeping this process's own stderr.
        saved, null = os.dup(2), os.open(os.devnull, os.O_WRONLY)
        os.dup2(null, 2)
        try:
            _SPARK = (
                SparkSession.builder.master("local[1]")
                .appName("substrait-review-against-engines")
                .config("spark.ui.enabled", "false")
                .config("spark.sql.session.timeZone", "UTC")
                .getOrCreate()
            )
        finally:
            os.dup2(saved, 2)
            os.close(saved)
            os.close(null)
        _SPARK.sparkContext.setLogLevel("OFF")
    return _SPARK


def run_spark(sql: str):
    from pyspark.errors import PySparkException

    session = _spark_session().newSession()
    try:
        df = session.sql(sql)
        col_type = df.schema.fields[0].dataType.simpleString()
        rows = [tuple(r) for r in df.collect()]
    except PySparkException as e:
        state = e.getSqlState() or ""
        # SQLSTATE class 42 is syntax or analysis (plan-time) errors.
        kind = "compile" if state.startswith("42") else "runtime"
        msg = str(e).splitlines()[0]
        raise EngineError(kind, f"{type(e).__name__} [{state}]: {msg}") from None
    if col_type == "timestamp":
        # TIMESTAMP is Spark's zoned type; pyspark returns naive datetimes in
        # the process time zone, which main() pins to UTC.
        rows = [
            (r[0].replace(tzinfo=dt.UTC) if r[0] is not None else None, *r[1:])
            for r in rows
        ]
    return rows, col_type


RUNNERS = {
    "duckdb": run_duckdb,
    "datafusion": run_datafusion,
    "sqlite": run_sqlite,
    "spark": run_spark,
}


def versions() -> dict[str, str]:
    import datafusion
    import duckdb
    import pyspark

    return {
        "duckdb": duckdb.__version__,
        "datafusion": datafusion.__version__,
        "sqlite": sqlite3.sqlite_version,
        "spark": pyspark.__version__,
    }


# --------------------------------------------------------------------------
# Types: map engine type names and test-file type names to one canonical form.
# --------------------------------------------------------------------------

SUBSTRAIT_ALIASES = {
    "f32": "fp32",
    "f64": "fp64",
    "decimal": "dec",
    "string": "str",
    "varchar": "str",
    "vchar": "str",
    "fixedchar": "str",
    "fchar": "str",
    "boolean": "bool",
    "binary": "vbin",
    "fbin": "vbin",
    "fixedbinary": "vbin",
    "precision_timestamp": "pts",
    "precision_timestamp_tz": "ptstz",
    "precision_time": "pt",
    "ts": "pts",
    "tstz": "ptstz",
    "time": "pt",
    "timestamp": "pts",
    "timestamp_tz": "ptstz",
    "interval_day": "iday",
    "interval_year": "iyear",
    "interval_compound": "icompound",
}

# Only variable-length string types keep no parameters; everything else that
# matters (dec, pts, pt, ...) keeps them.
PARAMLESS = {"str", "vbin"}


def split_type(t: str) -> tuple[str, list[str]]:
    t = t.strip().replace(" ", "")
    m = re.fullmatch(r"([A-Za-z_!0-9]+)\??(?:<(.*)>)?\??", t)
    if not m:
        return t.lower(), []
    name, params = m.group(1).lower(), m.group(2)
    args, depth, cur = [], 0, ""
    for ch in params or "":
        if ch == "," and depth == 0:
            args.append(cur)
            cur = ""
            continue
        depth += ch == "<"
        depth -= ch == ">"
        cur += ch
    if cur:
        args.append(cur)
    return name, args


def canon_substrait(t: str | None) -> str | None:
    if not t:
        return None
    name, args = split_type(t)
    name = SUBSTRAIT_ALIASES.get(name, name)
    if name == "list" and args:
        return f"list<{canon_substrait(args[0])}>"
    if name in PARAMLESS or not args:
        return name
    return f"{name}<{','.join(a.strip() for a in args)}>"


DUCKDB_TYPES = {
    "TINYINT": "i8",
    "SMALLINT": "i16",
    "INTEGER": "i32",
    "BIGINT": "i64",
    "FLOAT": "fp32",
    "DOUBLE": "fp64",
    "VARCHAR": "str",
    "BOOLEAN": "bool",
    "DATE": "date",
    "TIME": "pt<6>",
    "TIMESTAMP": "pts<6>",
    "TIMESTAMP_S": "pts<0>",
    "TIMESTAMP_MS": "pts<3>",
    "TIMESTAMP_NS": "pts<9>",
    "TIMESTAMP WITH TIME ZONE": "ptstz<6>",
    "INTERVAL": "icompound",
    "BLOB": "vbin",
    "UUID": "uuid",
}

ARROW_TYPES = {
    "int8": "i8",
    "int16": "i16",
    "int32": "i32",
    "int64": "i64",
    "float": "fp32",
    "halffloat": "fp16",
    "double": "fp64",
    "string": "str",
    "large_string": "str",
    "string_view": "str",
    "bool": "bool",
    "date32[day]": "date",
    "time64[us]": "pt<6>",
    "time64[ns]": "pt<9>",
    "time32[ms]": "pt<3>",
    "time32[s]": "pt<0>",
    "binary": "vbin",
    "large_binary": "vbin",
    "binary_view": "vbin",
    "month_day_nano_interval": "icompound",
}

ARROW_TS_UNITS = {"s": 0, "ms": 3, "us": 6, "ns": 9}

SQLITE_TYPES = {"int": "i64", "float": "fp64", "str": "str", "bytes": "vbin"}

SPARK_TYPES = {
    "tinyint": "i8",
    "smallint": "i16",
    "int": "i32",
    "bigint": "i64",
    "float": "fp32",
    "double": "fp64",
    "string": "str",
    "boolean": "bool",
    "date": "date",
    "timestamp": "ptstz<6>",
    "timestamp_ntz": "pts<6>",
    "binary": "vbin",
}


def canon_engine(engine: str, t: str | None) -> str | None:
    if t is None:
        return None
    if engine == "duckdb":
        if t.endswith("[]"):
            inner = canon_engine(engine, t[:-2])
            return f"list<{inner}>" if inner else None
        m = re.fullmatch(r"DECIMAL\((\d+),\s*(\d+)\)", t)
        if m:
            return f"dec<{m.group(1)},{m.group(2)}>"
        return DUCKDB_TYPES.get(t)
    if engine == "datafusion":
        m = re.fullmatch(r"decimal128\((\d+),\s*(\d+)\)", t)
        if m:
            return f"dec<{m.group(1)},{m.group(2)}>"
        m = re.fullmatch(r"timestamp\[(\w+)(?:,\s*tz=.*)?\]", t)
        if m:
            base = "ptstz" if "tz=" in t else "pts"
            return f"{base}<{ARROW_TS_UNITS[m.group(1)]}>"
        m = re.fullmatch(r"(?:large_)?list(?:_view)?<\w+:\s*(.*)>", t)
        if m:
            inner = canon_engine(engine, m.group(1))
            return f"list<{inner}>" if inner else None
        return ARROW_TYPES.get(t)
    if engine == "sqlite":
        return SQLITE_TYPES.get(t)
    if engine == "spark":
        m = re.fullmatch(r"decimal\((\d+),\s*(\d+)\)", t)
        if m:
            return f"dec<{m.group(1)},{m.group(2)}>"
        m = re.fullmatch(r"array<(.*)>", t)
        if m:
            inner = canon_engine(engine, m.group(1))
            return f"list<{inner}>" if inner else None
        if re.fullmatch(r"(?:var)?char\(\d+\)", t):
            return "str"
        m = re.fullmatch(r"time\((\d+)\)", t)
        if m:
            return f"pt<{m.group(1)}>"
        if t.startswith("interval day"):
            return "iday"
        if t.startswith("interval year"):
            return "iyear"
        return SPARK_TYPES.get(t)
    return None


def types_match(expected: str | None, actual: str | None) -> bool | None:
    """True/False, or None when either side is unknown."""
    if expected is None or actual is None:
        return None
    if expected == actual:
        return True
    # Expected without parameters (e.g. "dec") matches any parameterization.
    e_name, e_args = split_type(expected)
    a_name, _ = split_type(actual)
    if not e_args:
        return e_name == a_name
    return False


# --------------------------------------------------------------------------
# Values
# --------------------------------------------------------------------------

SPECIAL_FLOATS = {
    "inf": math.inf,
    "+inf": math.inf,
    "infinity": math.inf,
    "-inf": -math.inf,
    "-infinity": -math.inf,
    "nan": math.nan,
    "snan": math.nan,
    "+0": 0.0,
    "-0": -0.0,
}


def to_float(v, width: str | None) -> float:
    if isinstance(v, str):
        key = v.strip().lower()
        f = SPECIAL_FLOATS[key] if key in SPECIAL_FLOATS else float(v)
    else:
        f = float(v)
    if width == "fp32" and math.isfinite(f):
        try:
            f = struct.unpack("f", struct.pack("f", f))[0]
        except OverflowError:
            f = math.copysign(math.inf, f)
    return f


def values_equal(expected, actual, etype: str | None) -> bool:
    base = split_type(etype)[0] if etype else None
    base = SUBSTRAIT_ALIASES.get(base, base)

    if expected is None or actual is None:
        return expected is None and actual is None

    if base == "list" and isinstance(expected, list):
        if not isinstance(actual, (list, tuple)) or len(actual) != len(expected):
            return False
        inner = split_type(etype)[1]
        inner_t = inner[0] if inner else None
        return all(
            values_equal(e, a, inner_t) for e, a in zip(expected, actual, strict=True)
        )

    if isinstance(actual, bool) or base == "bool":
        if (
            isinstance(actual, int)
            and not isinstance(actual, bool)
            and actual in (0, 1)
        ):
            actual = bool(actual)  # SQLite booleans
        return actual is expected if isinstance(expected, bool) else False

    if isinstance(actual, (dt.datetime, dt.date, dt.time)):
        cls = type(actual)
        try:
            exp = cls.fromisoformat(
                str(expected).replace("T", " ") if cls is dt.datetime else str(expected)
            )
        except ValueError:
            return False
        if isinstance(actual, dt.datetime) and (actual.tzinfo is None) != (
            exp.tzinfo is None
        ):
            return False
        return exp == actual

    if hasattr(actual, "months") and hasattr(actual, "nanoseconds"):
        # Arrow MonthDayNano. Month components have no fixed length in seconds,
        # so only day/time intervals are compared.
        if actual.months:
            return False
        actual = dt.timedelta(days=actual.days, microseconds=actual.nanoseconds / 1000)

    if isinstance(actual, dt.timedelta):
        # Interval expectations are written as total seconds.
        try:
            return math.isclose(
                float(expected), actual.total_seconds(), rel_tol=0, abs_tol=1e-9
            )
        except (TypeError, ValueError):
            return False

    if isinstance(actual, float):
        try:
            exp = to_float(expected, base)
        except (TypeError, ValueError, KeyError):
            return False
        if math.isnan(exp) or math.isnan(actual):
            return math.isnan(exp) and math.isnan(actual)
        return exp == actual and math.copysign(1, exp) == math.copysign(1, actual)

    if isinstance(actual, (int, Decimal)):
        if isinstance(expected, bool):
            return False
        try:
            return Decimal(str(expected)) == Decimal(actual)
        except (InvalidOperation, ValueError):
            return False

    if isinstance(actual, (bytes, bytearray, memoryview)):
        return bytes(actual).hex() == str(expected).lower().removeprefix("0x")

    return str(expected) == str(actual)


def jsonable(v):
    if isinstance(v, float):
        if math.isnan(v):
            return "nan"
        if math.isinf(v):
            return "inf" if v > 0 else "-inf"
        return v
    if isinstance(v, Decimal):
        return str(v)
    if isinstance(v, (dt.date, dt.time, dt.datetime)):
        return v.isoformat()
    if isinstance(v, dt.timedelta):
        return f"{v.total_seconds()}s"
    if hasattr(v, "months") and hasattr(v, "nanoseconds"):
        return f"{v.months}mon {v.days}d {v.nanoseconds}ns"
    if isinstance(v, (bytes, bytearray, memoryview)):
        return "0x" + bytes(v).hex()
    if isinstance(v, (list, tuple)):
        return [jsonable(x) for x in v]
    if isinstance(v, dict):
        return {str(k): jsonable(x) for k, x in v.items()}
    return v


# --------------------------------------------------------------------------
# Running and judging cases
# --------------------------------------------------------------------------


def judge(expected: dict, outcome: dict) -> tuple[str, str]:
    """Return (status, reason)."""
    if "error" in outcome:
        if expected.get("error"):
            if outcome["error_kind"] == "compile":
                return (
                    CHECK,
                    "errored at plan/parse time; confirm it is the expected error",
                )
            return PASS, "errored as expected"
        if outcome["error_kind"] == "compile":
            return ERROR, "plan/parse-time error; check the SQL"
        return DEVIATES, "runtime error where a value was expected"

    rows = outcome["rows"]
    if expected.get("error"):
        return DEVIATES, "expected an error, got a value"
    if expected.get("undefined"):
        return PASS, "result undefined; any value accepted"

    etype = expected.get("type")
    if "column" in expected:
        actual = [r[0] for r in rows]
        want = expected["column"]
        ok = len(actual) == len(want) and all(
            values_equal(e, a, etype) for e, a in zip(want, actual, strict=True)
        )
    else:
        if len(rows) != 1:
            return DEVIATES, f"expected 1 row, got {len(rows)}"
        actual = rows[0][0]
        ok = values_equal(expected.get("value"), actual, etype)
    if not ok:
        return DEVIATES, "value differs"

    tm = types_match(canon_substrait(etype), outcome["substrait_type"])
    if tm is True:
        return PASS, ""
    if tm is False:
        return (
            PASS_TYPE_DIFF,
            f"expected {canon_substrait(etype)}, got {outcome['substrait_type'] or outcome['engine_type']}",
        )
    return PASS_TYPE_DIFF, f"result type not checkable ({outcome['engine_type']})"


def run_case(engine: str, sql: str) -> dict:
    try:
        rows, etype = RUNNERS[engine](sql)
    except EngineError as e:
        return {"error": e.message, "error_kind": e.kind}
    except Exception as e:  # noqa: BLE001 - report anything the engine throws
        return {"error": f"{type(e).__name__}: {e}", "error_kind": "runtime"}
    return {
        "rows": rows,
        "engine_type": etype,
        "substrait_type": canon_engine(engine, etype),
    }


def run_file(path: Path) -> dict:
    spec = json.loads(path.read_text())
    engines_cfg = spec.get("engines", {})
    results = []
    for case in spec["cases"]:
        per_engine = {}
        for engine in ENGINES:
            cfg = engines_cfg.get(engine) or {}
            sql = (case.get("sql") or {}).get(engine)
            if cfg.get("unsupported"):
                per_engine[engine] = {
                    "status": NA,
                    "reason": "unsupported: " + cfg["unsupported"],
                }
                continue
            if (case.get("skip") or {}).get(engine):
                per_engine[engine] = {
                    "status": NA,
                    "reason": "skipped: " + case["skip"][engine],
                }
                continue
            if not sql:
                per_engine[engine] = {"status": NA, "reason": "no SQL provided"}
                continue
            outcome = run_case(engine, sql)
            status, reason = judge(case["expected"], outcome)
            entry = {"status": status, "reason": reason, "sql": sql}
            if "error" in outcome:
                entry["error"] = outcome["error"]
                entry["error_kind"] = outcome["error_kind"]
            else:
                rows = outcome["rows"]
                entry["actual"] = jsonable(
                    [r[0] for r in rows]
                    if "column" in case["expected"]
                    else (rows[0][0] if len(rows) == 1 else [list(r) for r in rows])
                )
                entry["engine_type"] = outcome["engine_type"]
                entry["substrait_type"] = outcome["substrait_type"]
            per_engine[engine] = entry
        results.append(
            {
                k: case.get(k)
                for k in ("line", "group", "substrait", "options", "expected")
            }
            | {"engines": per_engine}
        )

    return {
        "test_file": spec.get("test_file"),
        "function": spec.get("function"),
        "versions": versions(),
        "engines": engines_cfg,
        "results": results,
    }


# --------------------------------------------------------------------------
# Report
# --------------------------------------------------------------------------


def short(v, n=60) -> str:
    s = json.dumps(v) if not isinstance(v, str) else v
    s = s.replace("|", "\\|").replace("\n", " ")
    return s if len(s) <= n else s[: n - 1] + "…"


def render(report: dict) -> str:
    out = []
    out.append(f"# Engine review: `{report['test_file']}` (`{report['function']}`)\n")
    out.append(
        "Engines: "
        + ", ".join(f"{e} {v}" for e, v in report["versions"].items())
        + "\n"
    )

    for engine in ENGINES:
        cfg = report["engines"].get(engine) or {}
        desc = (
            f"unsupported: {cfg['unsupported']}"
            if cfg.get("unsupported")
            else f"`{cfg.get('sql_function', '?')}`"
        )
        if cfg.get("notes"):
            desc += f" ({cfg['notes']})"
        out.append(f"- **{engine}**: {desc}")
    out.append("")

    out.append("| engine | " + " | ".join(STATUSES) + " | total |")
    out.append("|---" * (len(STATUSES) + 2) + "|")
    for engine in ENGINES:
        counts = dict.fromkeys(STATUSES, 0)
        for r in report["results"]:
            counts[r["engines"][engine]["status"]] += 1
        out.append(
            f"| {engine} | "
            + " | ".join(str(counts[s]) for s in STATUSES)
            + f" | {len(report['results'])} |"
        )
    out.append("")

    out.append("| line | case | " + " | ".join(ENGINES) + " |")
    out.append("|---" * (len(ENGINES) + 2) + "|")
    for r in report["results"]:
        cells = []
        for engine in ENGINES:
            e = r["engines"][engine]
            if e["status"] == NA:
                cells.append(NA)
            elif "error" in e:
                cells.append(f"{e['status']}: error")
            else:
                cells.append(f"{e['status']}: {short(e['actual'], 30)}")
        out.append(
            f"| {r['line']} | `{short(r['substrait'], 70)}` | "
            + " | ".join(cells)
            + " |"
        )
    out.append("")

    out.append("## Details (everything except PASS and N/A)\n")
    any_detail = False
    for r in report["results"]:
        for engine in ENGINES:
            e = r["engines"][engine]
            if e["status"] in (PASS, NA) and "error" not in e:
                continue
            if e["status"] == NA:
                continue
            any_detail = True
            line = f"- L{r['line']} **{engine}** {e['status']}"
            if e.get("reason"):
                line += f" ({e['reason']})"
            line += f": `{short(r['substrait'], 90)}`"
            if "error" in e:
                line += f"\n  - error ({e['error_kind']}): {short(e['error'], 200)}"
            else:
                line += f"\n  - actual: `{short(e['actual'], 80)}` type `{e['engine_type']}`"
            line += f"\n  - sql: `{short(e['sql'], 200)}`"
            out.append(line)
    if not any_detail:
        out.append("None.")
    return "\n".join(out) + "\n"


def main(argv: list[str]) -> int:
    if argv[1:] == ["--versions"]:
        for e, v in versions().items():
            print(f"{e} {v}")
        return 0
    # Pin the process time zone so engines that return naive local datetimes
    # (pyspark) do so in UTC, independent of the machine running this.
    os.environ["TZ"] = "UTC"
    time.tzset()
    if len(argv) == 4 and argv[1] == "--probe" and argv[2] in RUNNERS:
        outcome = run_case(argv[2], argv[3])
        if "rows" in outcome:
            outcome["rows"] = [jsonable(list(r)) for r in outcome["rows"]]
        print(json.dumps(outcome, indent=2))
        return 0
    if len(argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    path = Path(argv[1])
    report = run_file(path)
    (path.parent / "results.json").write_text(json.dumps(report, indent=2, default=str))
    sys.stdout.write(render(report))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
