// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for cross relations.
//!
//! The cross product operation will combine two separate inputs into a single
//! output. It pairs every record from the left input with every record of the
//! right input.
//!
//! See <https://substrait.io/relations/logical_relations/#cross-product-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse cross relation.
pub fn parse_cross_rel(
    x: &substrait::CrossRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let _left_type = handle_rel_input!(x, y, left);
    let _right_type = handle_rel_input!(x, y, right);

    // TODO: derive schema.
    y.set_schema(Arc::default());

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
