// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating function calls.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::relations;
use crate::parse::types;
use std::sync::Arc;

/// Parse a scalar subquery.
fn parse_scalar(
    x: &substrait::expression::subquery::Scalar,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse the relation and get its schema.
    let schema = y.enter_relation_root(|y| {
        proto_boxed_required_field!(x, y, input, relations::parse_rel)
            .0
            .data_type()
    });

    // Scalar subqueries must return one row and one column. We can't check the
    // row count statically, but we can check the schema.
    let return_type = if let Some(return_type) = schema.unwrap_singular_struct() {
        return_type
    } else {
        if !schema.is_unresolved() {
            diagnostic!(
                y,
                Error,
                ExpressionIllegalSubquery,
                "subquery must return a single column"
            );
        }
        Arc::default()
    };

    // FIXME: what is the behavior when the query doesn't yield one row? Should
    // the returned data type be made nullable?

    // Describe node.
    y.set_data_type(return_type);
    summary!(
        y,
        "Executes the contained subquery for each row. The query is expected \
        to return a single row and column, the value of which is returned by \
        the expression."
    );
    let expression = expressions::Expression::BigFunction(String::from("scalar_subquery"));
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a containment subquery.
fn parse_in_predicate(
    x: &substrait::expression::subquery::InPredicate,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse the needles.
    let needle_types = proto_required_repeated_field!(x, y, needles, expressions::parse_expression)
        .0
        .iter()
        .map(|x| x.data_type())
        .collect::<Vec<_>>();

    // Parse the relation and get its schema.
    let schema = y.enter_relation_root(|y| {
        proto_boxed_required_field!(x, y, haystack, relations::parse_rel)
            .0
            .data_type()
    });

    // Match data types of needles and haystack.
    if let Some(field_types) = schema.unwrap_struct() {
        if needle_types.len() != field_types.len() {
            diagnostic!(
                y,
                Error,
                TypeMismatch,
                "column count mismatch between needle and haystack"
            );
        } else {
            for (index, (field_type, needle_type)) in
                field_types.iter().zip(needle_types.iter()).enumerate()
            {
                types::assert_equal(
                    y,
                    field_type,
                    needle_type,
                    format!(
                        "haystack field type does not match needle type for column {}",
                        index + 1
                    ),
                );
            }
        }
    } else {
        assert!(schema.is_unresolved());
    }

    // Describe node.
    y.set_data_type(data_type::DataType::new_predicate(false));
    summary!(
        y,
        "Executes the contained subquery for each row. Returns true \
        if and only if the needle expressions match the fields of at \
        least one of the rows returned by the subquery."
    );
    let expression = expressions::Expression::BigFunction(String::from("in_subquery"));
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a set predicate subquery.
fn parse_set_predicate(
    x: &substrait::expression::subquery::SetPredicate,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    use substrait::expression::subquery::set_predicate::PredicateOp;

    // Parse the relation.
    y.enter_relation_root(|y| proto_boxed_required_field!(x, y, tuples, relations::parse_rel));

    // Parse the operation type.
    let operation = proto_required_enum_field!(x, y, predicate_op, PredicateOp)
        .1
        .unwrap_or_default();

    // Describe node.
    y.set_data_type(data_type::DataType::new_predicate(false));
    let expression = match operation {
        PredicateOp::Unspecified => {
            expressions::Expression::BigFunction(String::from("invalid_subquery"))
        }
        PredicateOp::Exists => {
            summary!(
                y,
                "Executes the contained subquery for each row. Returns true \
                if and only if at least one row is returned by the subquery."
            );
            expressions::Expression::BigFunction(String::from("subquery_exists"))
        }
        PredicateOp::Unique => {
            summary!(
                y,
                "Executes the contained subquery for each row. Returns true \
                if and only if no duplicate rows are returned."
            );
            expressions::Expression::BigFunction(String::from("subquery_unique"))
        }
    };
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a set comparison subquery.
fn parse_set_comparison(
    x: &substrait::expression::subquery::SetComparison,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    use substrait::expression::subquery::set_comparison::ComparisonOp;
    use substrait::expression::subquery::set_comparison::ReductionOp;

    // Parse the left-hand side.
    let (n, e) = proto_boxed_required_field!(x, y, left, expressions::parse_expression);
    let left_type = n.data_type();
    let left_expression = e.unwrap_or_default();

    // Parse the operation type.
    let comparison_op = proto_required_enum_field!(x, y, comparison_op, ComparisonOp)
        .1
        .unwrap_or_default();
    let reduction_op = proto_required_enum_field!(x, y, reduction_op, ReductionOp)
        .1
        .unwrap_or_default();

    // Parse the right-hand side.
    let right_schema = y.enter_relation_root(|y| {
        proto_boxed_required_field!(x, y, right, relations::parse_rel)
            .0
            .data_type()
    });

    // Right-hand side must return a single column.
    let right_type = if let Some(right_type) = right_schema.unwrap_singular_struct() {
        right_type
    } else {
        if !right_schema.is_unresolved() {
            diagnostic!(
                y,
                Error,
                ExpressionIllegalSubquery,
                "subquery must return a single column"
            );
        }
        Arc::default()
    };

    // Check that the data types match.
    types::assert_equal(
        y,
        &right_type,
        &left_type,
        "subquery field type does not match expression type",
    );

    // Describe node.
    y.set_data_type(data_type::DataType::new_predicate(false));
    let expression = expressions::Expression::BigFunction(format!(
        "{}_{}_subquery",
        match comparison_op {
            ComparisonOp::Unspecified => "invalid",
            ComparisonOp::Eq => "equal",
            ComparisonOp::Ne => "not_equal",
            ComparisonOp::Lt => "less_than",
            ComparisonOp::Gt => "greater_than",
            ComparisonOp::Le => "less_equal",
            ComparisonOp::Ge => "greater_equal",
        },
        match reduction_op {
            ReductionOp::Unspecified => "invalid",
            ReductionOp::Any => "any",
            ReductionOp::All => "all",
        },
    ));
    summary!(
        y,
        "Executes the contained subquery for each row. Returns true if"
    );
    summary!(
        y,
        "{}",
        match reduction_op {
            ReductionOp::Unspecified => "<invalid>",
            ReductionOp::Any => "any",
            ReductionOp::All => "all",
        }
    );
    summary!(
        y,
        "rows returned are {}",
        match comparison_op {
            ComparisonOp::Unspecified => "<invalid>",
            ComparisonOp::Eq => "equal to",
            ComparisonOp::Ne => "not equal to",
            ComparisonOp::Lt => "less than",
            ComparisonOp::Gt => "greater than",
            ComparisonOp::Le => "less than or equal to",
            ComparisonOp::Ge => "greater than or equal to",
        }
    );
    summary!(y, "{:#}.", left_expression);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}

/// Parse a particular subquery type.
fn parse_subquery_type(
    x: &substrait::expression::subquery::SubqueryType,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    match x {
        substrait::expression::subquery::SubqueryType::Scalar(x) => parse_scalar(x, y),
        substrait::expression::subquery::SubqueryType::InPredicate(x) => parse_in_predicate(x, y),
        substrait::expression::subquery::SubqueryType::SetPredicate(x) => parse_set_predicate(x, y),
        substrait::expression::subquery::SubqueryType::SetComparison(x) => {
            parse_set_comparison(x, y)
        }
    }
}

/// Parse a subquery.
pub fn parse_subquery(
    x: &substrait::expression::Subquery,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse fields.
    let (n, e) = proto_required_field!(x, y, subquery_type, parse_subquery_type);
    let return_type = n.data_type();
    let expression = e.unwrap_or_default();

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    Ok(expression)
}
