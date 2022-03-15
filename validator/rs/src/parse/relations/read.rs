// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for read relations.
//!
//! The read operator is an operator that produces one output. A simple example
//! would be the reading of a Parquet file.
//!
//! See https://substrait.io/relations/logical_relations/#read-operator

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::types;

/// Parse read relation.
pub fn parse_read_rel(x: &substrait::ReadRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // TODO
    proto_required_field!(x, y, base_schema, types::parse_named_struct);

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
