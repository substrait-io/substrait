// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for fetch relations.
//!
//! The fetch operation eliminates records outside a desired window. Typically
//! corresponds to a fetch/offset SQL clause.
//!
//! See <https://substrait.io/relations/logical_relations/#fetch-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse fetch relation.
pub fn parse_fetch_rel(
    x: &substrait::FetchRel,
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
