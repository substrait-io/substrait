// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for project relations.
//!
//! The project operation will produce one or more additional expressions based
//! on the inputs of the dataset.
//!
//! See <https://substrait.io/relations/logical_relations/#project-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;

/// Parse project relation.
pub fn parse_project_rel(
    x: &substrait::ProjectRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let mut schema = handle_rel_input!(x, y);

    // Start with the input schema.
    y.set_schema(schema.clone());

    // Parse the expressions that are to be appended to the schema.
    let expressions = proto_required_repeated_field!(
        x,
        y,
        expressions,
        expressions::parse_expression,
        |_x, y, _i, n, _r| {
            // Update the schema.
            if let Some(mut fields) = schema.unwrap_struct() {
                fields.push(n.data_type());
                schema = data_type::DataType::new_struct(fields, false);
                y.set_schema(schema.clone());
            } else {
                y.set_schema(Arc::default());
            }
        }
    )
    .1;

    // Describe the relation.
    describe!(y, Relation, "Projection");
    if expressions.len() > 1 {
        summary!(
            y,
            "This relation generates {} new columns by projecting the existing columns using scalar expressions.",
            expressions.len()
        );
    } else {
        summary!(
            y,
            "This relation generates a new column by projecting the existing columns using a scalar expression."
        );
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
