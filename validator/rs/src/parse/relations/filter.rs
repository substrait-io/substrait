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
use crate::parse::expressions;

/// Parse filter relation.
pub fn parse_filter_rel(
    x: &substrait::FilterRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let in_type = handle_rel_input!(x, y);

    // Filters pass through their input schema unchanged.
    y.set_schema(in_type);

    // Check the filter predicate.
    let (n, e) = proto_boxed_required_field!(x, y, condition, expressions::parse_predicate);
    let predicate = e.unwrap_or_default();
    let nullable = n.data_type().nullable();

    // Describe the relation.
    describe!(y, Relation, "Filter by {}", &predicate);
    summary!(
        y,
        "This relation discards all rows for which {} yields false.",
        &predicate
    );
    if nullable {
        // FIXME: what's the behavior when a filter condition is nullable and
        // yields null? Same applies for all other usages of parse_predicate().
        summary!(y, "Behavior for a null condition is unspecified.");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
