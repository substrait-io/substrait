// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for join relations.
//!
//! The join operation will combine two separate inputs into a single output,
//! based on a join expression. A common subtype of joins is a equality join
//! where the join expression is constrained to a list of equality (or
//! equality + null equality) conditions between the two inputs of the join.
//!
//! See <https://substrait.io/relations/logical_relations/#join-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;

/// Parse join relation.
pub fn parse_join_rel(x: &substrait::JoinRel, y: &mut context::Context) -> diagnostic::Result<()> {
    use substrait::join_rel::JoinType;

    // Parse input.
    let left = handle_rel_input!(x, y, left);
    let right = handle_rel_input!(x, y, right);

    // Derive schema with which the join expression is evaluated.
    if let (Some(mut fields), Some(additional_fields)) =
        (left.unwrap_struct(), right.unwrap_struct())
    {
        fields.extend(additional_fields.into_iter());
        let schema = data_type::DataType::new_struct(fields, false);
        y.set_schema(schema);
    } else {
        y.set_schema(Arc::default());
    }

    // Parse join expression.
    let join_expression =
        proto_boxed_required_field!(x, y, expression, expressions::parse_predicate)
            .1
            .unwrap_or_default();

    // Parse join type.
    let join_type = proto_required_enum_field!(x, y, r#type, JoinType)
        .1
        .unwrap_or_default();

    // Determine whether the join can null the left and/or right side, and
    // whether the right side is returned at all.
    let (left_nullable, right_nullable) = match join_type {
        JoinType::Unspecified => (false, Some(false)),
        JoinType::Inner => (false, Some(false)),
        JoinType::Outer => (true, Some(true)),
        JoinType::Left => (false, Some(true)),
        JoinType::Right => (true, Some(false)),
        JoinType::Semi => (false, None),
        JoinType::Anti => (false, None),
        JoinType::Single => (false, Some(true)),
    };

    // Derive final schema.
    if let (Some(left_fields), Some(right_fields)) = (left.unwrap_struct(), right.unwrap_struct()) {
        let mut fields = Vec::with_capacity(left_fields.len() + right_fields.len());
        if left_nullable {
            fields.extend(left_fields.into_iter().map(|x| x.make_nullable()))
        } else {
            fields.extend(left_fields.into_iter())
        }
        if let Some(right_nullable) = right_nullable {
            if right_nullable {
                fields.extend(right_fields.into_iter().map(|x| x.make_nullable()))
            } else {
                fields.extend(right_fields.into_iter())
            }
        }
        let schema = data_type::DataType::new_struct(fields, false);
        y.set_schema(schema);
    } else {
        y.set_schema(Arc::default());
    }

    // Handle optional post-join filter.
    let filter_expression =
        proto_boxed_field!(x, y, post_join_filter, expressions::parse_predicate).1;

    // Describe the relation.
    let prefix = match (join_type, x.post_join_filter.is_some()) {
        (JoinType::Unspecified, _) => "Unknown",
        (JoinType::Inner, true) => "Filtered inner",
        (JoinType::Inner, false) => "Inner",
        (JoinType::Outer, true) => "Filtered outer",
        (JoinType::Outer, false) => "Outer",
        (JoinType::Left, true) => "Filtered left",
        (JoinType::Left, false) => "Left",
        (JoinType::Right, true) => "Filtered right",
        (JoinType::Right, false) => "Right",
        (JoinType::Semi, true) => "Filtered semi",
        (JoinType::Semi, false) => "Semi",
        (JoinType::Anti, true) => "Filtered anti",
        (JoinType::Anti, false) => "Anti",
        (JoinType::Single, true) => "Filtered single",
        (JoinType::Single, false) => "Single",
    };
    describe!(y, Relation, "{prefix} join by {join_expression}");
    summary!(y, "{prefix} join by {join_expression:#}.");
    y.push_summary(comment::Comment::new().nl().plain(match join_type {
        JoinType::Unspecified => "",
        JoinType::Inner => concat!(
            " Returns rows combining the row from the left and right ",
            "input for each pair where the join expression yields true.",
        ),
        JoinType::Outer => concat!(
            " Returns rows combining the row from the left and right ",
            "input for each pair where the join expression yields true. ",
            "If the join expression never yields true for any left or ",
            "right row, this returns a row anyway, with the fields ",
            "corresponding to the other input set to null.",
        ),
        JoinType::Left => concat!(
            " Returns rows combining the row from the left and right ",
            "input for each pair where the join expression yields true. ",
            "If the join expression never yields true for a row from the ",
            "left, this returns a row anyway, with the fields corresponding ",
            "to the right input set to null.",
        ),
        JoinType::Right => concat!(
            " Returns rows combining the row from the left and right ",
            "input for each pair where the join expression yields true. ",
            "If the join expression never yields true for a row from the ",
            "right, this returns a row anyway, with the fields corresponding ",
            "to the left input set to null.",
        ),
        JoinType::Semi => concat!(
            " Filters rows from the left input, propagating a row only if ",
            "the join expression yields true for that row combined with ",
            "any row from the right input.",
        ),
        JoinType::Anti => concat!(
            " Filters rows from the left input, propagating a row only if ",
            "the join expression does not yield true for that row combined ",
            "with any row from the right input.",
        ),
        JoinType::Single => concat!(
            " Returns a row for each row from the left input, concatenating ",
            "it with the row from the right input for which the join ",
            "expression yields true. If the expression never yields true for ",
            "a left input, the fields corresponding to the right input are ",
            "set to null. If the expression yields true for a left row and ",
            "multiple right rows, this may return the first pair encountered ",
            "or throw an error."
        ),
    }));
    if let Some(filter_expression) = filter_expression {
        y.push_summary(
            comment::Comment::new()
                .nl()
                .plain(format!("The result is filtered by {filter_expression:#}.")),
        );
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
