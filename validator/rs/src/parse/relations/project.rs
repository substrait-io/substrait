// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for project relations.
//!
//! The project operation will produce one or more additional expressions based
//! on the inputs of the dataset.
//!
//! See <https://substrait.io/relations/logical_relations/#project-operation>

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse project relation.
pub fn parse_project_rel(
    x: &substrait::ProjectRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let _in_type = handle_rel_input!(x, y);

    // TODO: derive schema.
    y.set_schema(data_type::DataType::default());

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
