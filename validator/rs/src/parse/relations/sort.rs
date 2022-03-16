// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for sort relations.
//!
//! The sort operator reorders a dataset based on one or more identified
//! sort fields as well as a sorting function.
//!
//! See <https://substrait.io/relations/logical_relations/#sort-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse sort relation.
pub fn parse_sort_rel(x: &substrait::SortRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse input.
    let _in_type = handle_rel_input!(x, y);

    // TODO: derive schema.
    y.set_schema(Arc::default());

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
