# SPDX-License-Identifier: Apache-2.0
"""Parameterised type unifier and return-formula evaluator for the coverage
checker.  ``FunctionRegistry.is_same_type`` only matches base names; this
module checks the full decimal/varchar/list/map/any1 parameters against the
YAML ``return`` formula."""
import re


# Short-form aliases used by test files. Long forms come from the YAML.
_SHORT_TO_LONG = {
    "dec": "decimal",
    "vchar": "varchar",
    "fchar": "fixedchar",
    "fbin": "fixedbinary",
    "vbin": "binary",
    "str": "string",
    "bool": "boolean",
    "ts": "timestamp",
    "tstz": "timestamp_tz",
    "pts": "precision_timestamp",
    "ptstz": "precision_timestamp_tz",
    "pt": "precision_time",
    "iyear": "interval_year",
    "iday": "interval_day",
    "icompound": "interval_compound",
}

# Known concrete base names.  Any simple identifier not in this set (and not
# a short alias) is treated as a variable such as ``P1`` or ``prec``.
_KNOWN_BASES = frozenset(
    "i8 i16 i32 i64 fp32 fp64 boolean string binary date time timestamp "
    "timestamp_tz interval_year interval_day interval_compound uuid "
    "decimal varchar fixedchar fixedbinary precision_time precision_timestamp "
    "precision_timestamp_tz list map struct any any1 any2 enum func".split()
)

# Types whose parameters are element/field types (recursed into).
_TYPE_PARAM_BASES = frozenset(["list", "map", "struct"])

# Types whose parameters are integer literals or variables (precision,
# scale, length).
_INT_PARAM_BASES = frozenset(
    [
        "decimal",
        "varchar",
        "fixedchar",
        "fixedbinary",
        "precision_time",
        "precision_timestamp",
        "precision_timestamp_tz",
    ]
)


def canon_base(base):
    base = base.lower()
    return _SHORT_TO_LONG.get(base, base)


def parse_type(s):
    """Parse a type string into ``(base, nullable, params)``.

    ``params`` items are ints, strings (variable or unparsed literal), or
    nested ``parse_type`` tuples.  ``func<... -> ...>`` is parsed into
    ``(arg_types_list, return_type_tuple)``.  Returns ``None`` on empty or
    malformed input.
    """
    if s is None:
        return None
    s = s.strip()
    if not s:
        return None
    lt = s.find("<")
    if lt == -1:
        nullable = s.endswith("?")
        return (canon_base(s.rstrip("?")), nullable, [])

    head = s[:lt]
    close = _find_matching_angle(s, lt)
    if close == -1:
        return None
    inner = s[lt + 1 : close]
    tail = s[close + 1 :].strip()
    nullable = head.endswith("?") or tail == "?"
    base = canon_base(head.rstrip("?"))

    if base == "func":
        parsed_func = _parse_func_inner(inner)
        if parsed_func is None:
            return (base, nullable, [inner.strip()])
        return (base, nullable, list(parsed_func))

    raw_params = _split_top_level_commas(inner)
    params = []
    if base in _TYPE_PARAM_BASES:
        for p in raw_params:
            nested = parse_type(p)
            if nested is None:
                return None
            params.append(nested)
    elif base in _INT_PARAM_BASES:
        for p in raw_params:
            try:
                params.append(int(p))
            except ValueError:
                params.append(p)
    else:
        for p in raw_params:
            if "<" in p:
                nested = parse_type(p)
                if nested is None:
                    return None
                params.append(nested)
            else:
                try:
                    params.append(int(p))
                except ValueError:
                    params.append(p)
    return (base, nullable, params)


def _find_matching_angle(s, open_idx):
    """Return the index of the ``>`` that closes ``s[open_idx]`` (a ``<``).
    ``>`` that follows ``-`` is part of a lambda ``->`` arrow, not a bracket.
    """
    depth = 0
    for i in range(open_idx, len(s)):
        c = s[i]
        if c == "<":
            depth += 1
        elif c == ">":
            if i > 0 and s[i - 1] == "-":
                continue
            depth -= 1
            if depth == 0:
                return i
    return -1


def _split_top_level_commas(inner):
    out = []
    depth = 0
    cur = ""
    for i, c in enumerate(inner):
        if c == "<":
            depth += 1
        elif c == ">" and not (i > 0 and inner[i - 1] == "-"):
            depth -= 1
        if c == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += c
    if cur.strip():
        out.append(cur.strip())
    return out


def _parse_func_inner(inner):
    """Parse ``any1 -> boolean?`` or ``i32, i32 -> i32`` into
    ``(arg_types, return_type)``.  Returns ``None`` on failure."""
    depth = 0
    arrow = -1
    for i in range(len(inner) - 1):
        c = inner[i]
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
        elif c == "-" and depth == 0 and inner[i + 1] == ">":
            arrow = i
            break
    if arrow == -1:
        return None
    arg_types = []
    for piece in _split_top_level_commas(inner[:arrow]):
        parsed = parse_type(piece)
        if parsed is None:
            return None
        arg_types.append(parsed)
    ret_type = parse_type(inner[arrow + 2 :].strip())
    if ret_type is None:
        return None
    return (arg_types, ret_type)


def _is_variable_name(s):
    return (
        isinstance(s, str)
        and re.fullmatch(r"[A-Za-z_][A-Za-z_0-9]*", s) is not None
        and s.lower() not in _KNOWN_BASES
        and s.lower() not in _SHORT_TO_LONG
    )


def unify(impl_t, test_t, bindings):
    """Unify an impl type (possibly containing variables) against a concrete
    test type.  Updates ``bindings`` and returns True on success."""
    if impl_t is None or test_t is None:
        return False
    ib, _, iparams = impl_t
    tb, _, tparams = test_t

    if ib in ("any", "any1", "any2"):
        stripped = (tb, False, _strip_nullable(tparams))
        existing = bindings.get(ib)
        if existing is None:
            bindings[ib] = stripped
            return True
        return structural_equal(existing, stripped)

    if ib != tb:
        return False

    if ib == "func":
        if (
            len(iparams) != 2
            or len(tparams) != 2
            or not isinstance(iparams[0], list)
            or not isinstance(tparams[0], list)
        ):
            return iparams == tparams
        i_args, i_ret = iparams
        t_args, t_ret = tparams
        if len(i_args) != len(t_args):
            return False
        for ia, ta in zip(i_args, t_args):
            if not unify(ia, ta, bindings):
                return False
        return unify(i_ret, t_ret, bindings)

    if len(iparams) != len(tparams):
        # Test may omit numeric parameters (``iday`` for ``interval_day<P>``).
        # Accept without binding; if the return formula depends on the
        # missing variables, ``check_signature`` falls back to the loose
        # check.
        if (
            not tparams
            and iparams
            and all(isinstance(p, str) and _is_variable_name(p) for p in iparams)
        ):
            return True
        return False

    for ip, tp in zip(iparams, tparams):
        if isinstance(ip, tuple):
            if not isinstance(tp, tuple) or not unify(ip, tp, bindings):
                return False
        elif isinstance(ip, int):
            if ip != tp:
                return False
        elif isinstance(ip, str):
            if _is_variable_name(ip):
                existing = bindings.get(ip)
                if existing is None:
                    bindings[ip] = tp
                elif existing != tp:
                    return False
            elif str(ip).lower() != str(tp).lower():
                return False
    return True


def _strip_nullable(params):
    """Return ``params`` with nested types normalised to ``nullable=False``."""
    out = []
    for p in params:
        if isinstance(p, tuple):
            pb, _, pp = p
            out.append((pb, False, _strip_nullable(pp)))
        else:
            out.append(p)
    return out


def structural_equal(a, b):
    """Structural equality ignoring outer and inner nullable flags."""
    if a is None or b is None:
        return a is b
    ab, _, ap = a
    bb, _, bp = b
    if ab != bb or len(ap) != len(bp):
        return False
    for x, y in zip(ap, bp):
        if isinstance(x, tuple) or isinstance(y, tuple):
            if not structural_equal(x, y):
                return False
        elif x != y:
            return False
    return True


def _ternary_to_py(expr):
    """Rewrite ``cond ? a : b`` (possibly nested) as ``(a) if (cond) else (b)``."""
    q_pos = _find_at_depth_zero(expr, "?")
    if q_pos == -1:
        return expr
    colon_pos = _find_at_depth_zero(expr, ":", start=q_pos + 1)
    if colon_pos == -1:
        return expr
    cond = expr[:q_pos].strip()
    a = expr[q_pos + 1 : colon_pos].strip()
    b = expr[colon_pos + 1 :].strip()
    return f"(({_ternary_to_py(a)}) if ({cond}) else ({_ternary_to_py(b)}))"


def _find_at_depth_zero(expr, target, start=0):
    depth = 0
    for i in range(start, len(expr)):
        c = expr[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        elif c == target and depth == 0:
            return i
    return -1


def _evaluate_param_expr(expr, env):
    try:
        return eval(
            _ternary_to_py(expr.strip()),
            {"__builtins__": {}, "min": min, "max": max},
            env,
        )
    except Exception:
        return None


def evaluate_return_formula(formula, bindings):
    """Evaluate a YAML return formula with variable ``bindings`` and return
    a concrete parsed-type tuple, or ``None`` if evaluation fails."""
    if not formula:
        return None
    formula = str(formula).strip()
    env = {k: v for k, v in bindings.items() if isinstance(v, int)}
    lines = [ln.strip() for ln in formula.split("\n") if ln.strip()]
    if not lines:
        return None
    for line in lines[:-1]:
        if "=" not in line:
            return None
        name, expr = line.split("=", 1)
        value = _evaluate_param_expr(expr, env)
        if value is None:
            return None
        env[name.strip()] = value
    return _resolve_type(lines[-1], env, bindings)


def _resolve_type(value, env, any_bindings):
    """Evaluate variable params (and ``any1``/``any2`` references) inside a
    type expression.  ``value`` is either a string to parse or a parsed
    tuple.  Returns a tuple with fully evaluated params, or ``None``."""
    parsed = parse_type(value) if isinstance(value, str) else value
    if parsed is None:
        return None
    base, nullable, params = parsed
    if base in ("any", "any1", "any2"):
        repl = any_bindings.get(base)
        if repl is None:
            return None
        r_base, _, r_params = repl
        return (r_base, nullable, r_params)
    out = []
    for p in params:
        if isinstance(p, int):
            out.append(p)
        elif isinstance(p, tuple):
            sub = _resolve_type(p, env, any_bindings)
            if sub is None:
                return None
            out.append(sub)
        elif isinstance(p, str):
            v = _evaluate_param_expr(p, env)
            if v is None:
                return None
            out.append(v)
    return (base, nullable, out)


def check_signature(impl_args, impl_return, test_args, test_return):
    """Strict signature check.  Returns ``(ok, reason)``.  When the impl's
    return formula can't be evaluated (e.g. test omits a numeric parameter)
    the check accepts the signature — the loose match still applies."""
    if len(impl_args) != len(test_args):
        return (False, f"arg count {len(impl_args)} vs {len(test_args)}")

    bindings = {}
    for i, (ia, ta) in enumerate(zip(impl_args, test_args)):
        parsed_impl = parse_type(ia)
        if parsed_impl is None:
            # impl arg is not a value type (e.g. enum option) — skip.
            continue
        parsed_test = parse_type(ta)
        if parsed_test is None:
            return (False, f"arg {i}: failed to parse test type {ta!r}")
        if not unify(parsed_impl, parsed_test, bindings):
            return (False, f"arg {i}: cannot unify {ia} with {ta}")

    expected = evaluate_return_formula(impl_return, bindings)
    if expected is None:
        return (True, "")

    parsed_test_ret = parse_type(test_return)
    if parsed_test_ret is None:
        return (False, f"failed to parse test return type {test_return!r}")

    if not structural_equal(expected, parsed_test_ret):
        return (
            False,
            f"return: expected {_format_type(expected)} "
            f"but test declares {_format_type(parsed_test_ret)}",
        )
    return (True, "")


def _format_type(t):
    if t is None:
        return "<unknown>"
    base, _, params = t
    if not params:
        return base
    inner = [_format_type(p) if isinstance(p, tuple) else str(p) for p in params]
    return f"{base}<{', '.join(inner)}>"
