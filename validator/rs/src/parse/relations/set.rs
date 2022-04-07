// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for set relations.
//!
//! The set operation encompasses several set level operations that support
//! combining datasets based, possibly excluding records based on various
//! types of record level matching.
//!
//! See <https://substrait.io/relations/logical_relations/#set-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::types;

enum Operation {
    Invalid,
    Subtract,
    SubtractByUnion,
    SubtractByIntersection,
    Intersect,
    IntersectWithUnion,
    Union,
    Merge,
}

/// Parse set relation.
pub fn parse_set_rel(x: &substrait::SetRel, y: &mut context::Context) -> diagnostic::Result<()> {
    use substrait::set_rel::SetOp;

    // Parse inputs.
    let in_types: Vec<_> = handle_rel_inputs!(x, y).collect();

    // Check inputs and derive schema.
    if in_types.len() < 2 {
        diagnostic!(
            y,
            Error,
            RelationMissing,
            "set operations require at least two input relations"
        );
    }
    let mut schema = Arc::default();
    for in_type in in_types.iter() {
        schema = types::assert_equal(
            y,
            &in_type.strip_field_names(),
            &schema,
            "all set inputs must have matching schemas",
        );
    }
    y.set_schema(schema);

    // Check set operation.
    let op = proto_required_enum_field!(x, y, op, SetOp)
        .1
        .unwrap_or_default();
    let op = match (op, in_types.len() > 2) {
        (SetOp::Unspecified, _) => Operation::Invalid,
        (SetOp::MinusPrimary, true) => Operation::SubtractByUnion,
        (SetOp::MinusPrimary, false) => Operation::Subtract,
        (SetOp::MinusMultiset, true) => Operation::SubtractByIntersection,
        (SetOp::MinusMultiset, false) => Operation::Subtract,
        (SetOp::IntersectionPrimary, true) => Operation::IntersectWithUnion,
        (SetOp::IntersectionPrimary, false) => Operation::Intersect,
        (SetOp::IntersectionMultiset, _) => Operation::Intersect,
        (SetOp::UnionDistinct, _) => Operation::Union,
        (SetOp::UnionAll, _) => Operation::Merge,
    };

    // Describe the relation.
    match op {
        Operation::Invalid => {
            describe!(y, Relation, "Invalid set operation");
        }
        Operation::Subtract => {
            describe!(y, Relation, "Set subtraction");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist \
                in the second dataset."
            );
        }
        Operation::SubtractByUnion => {
            describe!(y, Relation, "Set subtract by union");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist \
                in any of the other datasets."
            );
        }
        Operation::SubtractByIntersection => {
            describe!(y, Relation, "Set subtract by intersection");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist in \
                all of the other datasets."
            );
        }
        Operation::Intersect => {
            describe!(y, Relation, "Set intersection");
            summary!(
                y,
                "Yields all rows from the first dataset that exist in all \
                datasets."
            );
        }
        Operation::IntersectWithUnion => {
            describe!(y, Relation, "Set intersect with union");
            summary!(
                y,
                "Yields all rows from the first dataset that exist in any of \
                the other datasets."
            );
        }
        Operation::Union => {
            describe!(y, Relation, "Set union");
            summary!(
                y,
                "Yields all rows that exist in any dataset, removing duplicates."
            );
        }
        Operation::Merge => {
            describe!(y, Relation, "Merge");
            summary!(y, "Yields all rows from all incoming datasets.");
        }
    };

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
