// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for filter relations.
//!
//! The filter operator eliminates one or more records from the input data
//! based on a boolean filter expression.
//!
//! See <https://substrait.io/relations/logical_relations/#filter-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse filter relation.
pub fn parse_filter_rel(
    x: &substrait::FilterRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let _in_type = handle_rel_input!(x, y);

    // TODO

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
