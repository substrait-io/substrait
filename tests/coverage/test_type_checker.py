# SPDX-License-Identifier: Apache-2.0
from tests.coverage.type_checker import (
    parse_type,
    unify,
    evaluate_return_formula,
    check_signature,
    structural_equal,
)


def test_parse_simple_base_type():
    assert parse_type("i32") == ("i32", False, [])


def test_parse_nullable_base_type():
    assert parse_type("i32?") == ("i32", True, [])


def test_parse_decimal_with_literals():
    assert parse_type("dec<38, 2>") == ("decimal", False, [38, 2])


def test_parse_decimal_with_variables():
    assert parse_type("DECIMAL<P1, S1>") == ("decimal", False, ["P1", "S1"])


def test_parse_decimal_nullable_before_angle():
    assert parse_type("dec?<38, 2>") == ("decimal", True, [38, 2])


def test_parse_decimal_nullable_after_angle():
    assert parse_type("dec<38, 2>?") == ("decimal", True, [38, 2])


def test_parse_list_of_i32():
    assert parse_type("list<i32>") == ("list", False, [("i32", False, [])])


def test_parse_nested_list():
    assert parse_type("list<list<i32>>") == (
        "list",
        False,
        [("list", False, [("i32", False, [])])],
    )


def test_parse_map_of_str_to_i32():
    assert parse_type("map<string, i32>") == (
        "map",
        False,
        [("string", False, []), ("i32", False, [])],
    )


def test_parse_struct_with_decimal():
    assert parse_type("STRUCT<DECIMAL<38,S>, i64>") == (
        "struct",
        False,
        [("decimal", False, [38, "S"]), ("i64", False, [])],
    )


def test_parse_func_simple():
    assert parse_type("func<i32 -> bool>") == (
        "func",
        False,
        [[("i32", False, [])], ("boolean", False, [])],
    )


def test_parse_func_polymorphic():
    assert parse_type("func<any1 -> boolean?>") == (
        "func",
        False,
        [[("any1", False, [])], ("boolean", True, [])],
    )


def test_parse_func_multi_arg():
    assert parse_type("func<i32, i32 -> i32>") == (
        "func",
        False,
        [[("i32", False, []), ("i32", False, [])], ("i32", False, [])],
    )


def test_parse_canonicalises_short_to_long():
    assert parse_type("dec<10, 2>")[0] == "decimal"
    assert parse_type("vchar<20>")[0] == "varchar"
    assert parse_type("bool")[0] == "boolean"
    assert parse_type("str")[0] == "string"


def test_unify_binds_variables_to_literals():
    b = {}
    assert unify(parse_type("DECIMAL<P1,S1>"), parse_type("dec<38,2>"), b)
    assert b == {"P1": 38, "S1": 2}


def test_unify_rejects_mismatched_literal():
    assert not unify(parse_type("DECIMAL<38,0>"), parse_type("dec<10,2>"), {})


def test_unify_binds_any1_to_whole_type():
    b = {}
    assert unify(parse_type("any1"), parse_type("dec<10,2>"), b)
    assert b["any1"] == ("decimal", False, [10, 2])


def test_unify_any1_must_be_consistent_across_args():
    b = {}
    assert unify(parse_type("any1"), parse_type("i32"), b)
    assert not unify(parse_type("any1"), parse_type("i64"), b)


def test_unify_list_of_any1_with_list_of_i32():
    b = {}
    assert unify(parse_type("list<any1>"), parse_type("list<i32>"), b)
    assert b["any1"] == ("i32", False, [])


def test_unify_func_polymorphic_with_concrete():
    b = {}
    assert unify(
        parse_type("func<any1 -> boolean?>"), parse_type("func<i32 -> bool?>"), b
    )
    assert b["any1"] == ("i32", False, [])


def test_unify_allows_test_to_omit_variable_only_params():
    # ``iday`` is accepted for ``interval_day<P>`` without binding P.
    assert unify(parse_type("interval_day<P>"), parse_type("iday"), {})


def test_evaluate_single_line_formula():
    assert evaluate_return_formula("DECIMAL<38, 2>", {}) == ("decimal", False, [38, 2])


def test_evaluate_any1_resolution():
    got = evaluate_return_formula("any1?", {"any1": ("decimal", False, [38, 0])})
    assert got == ("decimal", True, [38, 0])


ADD_FORMULA = (
    "init_scale = max(S1,S2)\n"
    "init_prec = init_scale + max(P1 - S1, P2 - S2) + 1\n"
    "min_scale = min(init_scale, 6)\n"
    "delta = init_prec - 38\n"
    "prec = min(init_prec, 38)\n"
    "scale_after_borrow = max(init_scale - delta, min_scale)\n"
    "scale = init_prec > 38 ? scale_after_borrow : init_scale\n"
    "DECIMAL<prec, scale>"
)

DIVIDE_FORMULA = (
    "init_scale = max(6, S1 + P2 + 1)\n"
    "init_prec = P1 - S1 + P2 + init_scale\n"
    "min_scale = min(init_scale, 6)\n"
    "delta = init_prec - 38\n"
    "prec = min(init_prec, 38)\n"
    "scale_after_borrow = max(init_scale - delta, min_scale)\n"
    "scale = init_prec > 38 ? scale_after_borrow : init_scale\n"
    "DECIMAL<prec, scale>"
)


def test_evaluate_add_formula():
    # dec<10,2> + dec<5,1>: init_scale=2, init_prec=11 → dec<11,2>
    got = evaluate_return_formula(ADD_FORMULA, {"P1": 10, "S1": 2, "P2": 5, "S2": 1})
    assert got == ("decimal", False, [11, 2])


def test_evaluate_add_formula_overflow_borrow():
    # dec<38,10> + dec<38,10>: init_prec=39 forces scale_after_borrow=9
    got = evaluate_return_formula(ADD_FORMULA, {"P1": 38, "S1": 10, "P2": 38, "S2": 10})
    assert got == ("decimal", False, [38, 9])


def test_evaluate_divide_formula():
    # dec<10,2> / dec<5,1>: init_scale=8, init_prec=21
    got = evaluate_return_formula(DIVIDE_FORMULA, {"P1": 10, "S1": 2, "P2": 5, "S2": 1})
    assert got == ("decimal", False, [21, 8])


def test_check_signature_ok_for_correct_decimal_add():
    ok, reason = check_signature(
        ["decimal<P1,S1>", "decimal<P2,S2>"],
        ADD_FORMULA,
        ["dec<10,2>", "dec<5,1>"],
        "dec<11,2>",
    )
    assert ok, reason


def test_check_signature_catches_sum_decimal_bug():
    # sum returns DECIMAL?<38,S>; input scale 1 → scale 1, not 2.
    ok, reason = check_signature(
        ["DECIMAL<P, S>"], "DECIMAL?<38,S>", ["dec<2,1>"], "dec<38,2>"
    )
    assert not ok
    assert "decimal<38, 1>" in reason and "decimal<38, 2>" in reason


def test_check_signature_catches_nullif_bug():
    # nullif is any1 -> any1?; with dec<38,0> args the result cannot be bool.
    ok, reason = check_signature(
        ["any1", "any1"], "any1?", ["dec<38,0>", "dec<38,0>"], "bool"
    )
    assert not ok
    assert "decimal" in reason and "boolean" in reason


def test_check_signature_falls_back_when_formula_unevaluable():
    # Unbound variable → accept (test opted out of strict on that dim).
    ok, _ = check_signature(
        ["i64"],
        "fp_precision = UNBOUND + 1\nDECIMAL<fp_precision, 0>",
        ["i64"],
        "dec<5,0>",
    )
    assert ok


def test_check_signature_tolerates_test_omitting_decimal_params():
    # ``power(dec, dec<38,0>) -> fp64`` — first arg drops precision/scale.
    ok, reason = check_signature(
        ["DECIMAL<P1,S1>", "DECIMAL<P2,S2>"],
        "fp64",
        ["dec", "dec<38,0>"],
        "fp64",
    )
    assert ok, reason


def test_structural_equal_ignores_outer_nullable():
    assert structural_equal(parse_type("dec<38,2>"), parse_type("dec?<38,2>"))


def test_structural_equal_rejects_param_differences():
    assert not structural_equal(parse_type("dec<38,2>"), parse_type("dec<38,1>"))
