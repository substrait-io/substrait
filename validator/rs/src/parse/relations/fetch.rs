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
use crate::string_util;

/// Parse fetch relation.
pub fn parse_fetch_rel(
    x: &substrait::FetchRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let in_type = handle_rel_input!(x, y);

    // Filters pass through their input schema unchanged.
    y.set_schema(in_type);

    // Parse offset and count.
    proto_primitive_field!(x, y, offset, |x, y| {
        if *x < 0 {
            diagnostic!(y, Error, IllegalValue, "offsets cannot be negative");
        }
        Ok(())
    });
    proto_primitive_field!(x, y, count, |x, y| {
        if *x < 0 {
            diagnostic!(y, Error, IllegalValue, "count cannot be negative");
        }
        Ok(())
    });

    // Describe the relation.
    if x.count == 1 {
        describe!(
            y,
            Relation,
            "Propagate only the {} row",
            (x.offset + 1)
                .try_into()
                .map(string_util::describe_nth)
                .unwrap_or_else(|_| String::from("?"))
        );
    } else if x.count > 1 {
        describe!(
            y,
            Relation,
            "Propagate only {} rows, starting from the {}",
            x.count,
            (x.offset + 1)
                .try_into()
                .map(string_util::describe_nth)
                .unwrap_or_else(|_| String::from("?"))
        );
    } else if x.offset == 1 {
        describe!(y, Relation, "Discard the first row");
    } else if x.offset > 1 {
        describe!(y, Relation, "Discard the first {} rows", x.offset);
    } else {
        describe!(y, Relation, "Invalid fetch relation");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
