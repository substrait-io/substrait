// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for join relations.
//!
//! The join operation will combine two separate inputs into a single output,
//! based on a join expression. A common subtype of joins is a equality join
//! where the join expression is constrained to a list of equality (or
//! equality + null equality) conditions between the two inputs of the join.
//!
//! See <https://substrait.io/relations/logical_relations/#join-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse join relation.
pub fn parse_join_rel(x: &substrait::JoinRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse input.
    let _left_type = handle_rel_input!(x, y, left);
    let _right_type = handle_rel_input!(x, y, right);

    // TODO

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
