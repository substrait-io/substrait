// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for aggregate relations.
//!
//! The aggregate operation groups input data on one or more sets of grouping
//! keys, calculating each measure for each combination of grouping key.
//!
//! See https://substrait.io/relations/logical_relations/#aggregate-operation

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse aggregate relation.
pub fn parse_aggregate_rel(
    x: &substrait::AggregateRel,
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
