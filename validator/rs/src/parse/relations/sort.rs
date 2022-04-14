// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for sort relations.
//!
//! The sort operator reorders a dataset based on one or more identified
//! sort fields as well as a sorting function.
//!
//! See <https://substrait.io/relations/logical_relations/#sort-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::sorts;

/// Parse sort relation.
pub fn parse_sort_rel(x: &substrait::SortRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse input.
    let in_type = handle_rel_input!(x, y);

    // Sorts pass through their input schema unchanged.
    y.set_schema(in_type);

    // Check the sorts.
    let keys = proto_required_repeated_field!(x, y, sorts, sorts::parse_sort_field).1;

    // Describe the relation.
    describe!(
        y,
        Relation,
        "Order by {}",
        keys.first().cloned().flatten().unwrap_or_default()
    );
    if x.sorts.len() > 1 {
        summary!(
            y,
            "This relation reorders or coalesces a dataset based on {} keys. \
            For sorts, the first key has greatest priority; only if the first \
            key is equivalent for two rows will the next key be checked.",
            x.sorts.len()
        );
    } else {
        summary!(
            y,
            "This relation reorders or coalesces a dataset based on the value of {}.",
            keys.first().cloned().flatten().unwrap_or_default()
        );
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
