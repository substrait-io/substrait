// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for set relations.
//!
//! The set operation encompasses several set level operations that support
//! combining datasets based, possibly excluding records based on various
//! types of record level matching.
//!
//! See <https://substrait.io/relations/logical_relations/#set-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse set relation.
pub fn parse_set_rel(x: &substrait::SetRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse inputs.
    let _in_types: Vec<_> = handle_rel_inputs!(x, y).collect();

    // TODO

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
