// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for relational algebra
//! extensions.

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::extensions;

/// Parse one to one extension.
pub fn parse_extension_single_rel(
    x: &substrait::ExtensionSingleRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse input.
    let _in_type = handle_rel_input!(x, y);

    // Set schema to an unresolved type.
    y.set_schema(Arc::default());

    // Parse the extension data.
    proto_required_field!(x, y, detail, extensions::advanced::parse_functional_any);

    // Describe the relation.
    if let Some(x) = &x.detail {
        describe!(y, Relation, "{} extension", x.type_url);
    } else {
        describe!(y, Relation, "Unknown extension");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    Ok(())
}

/// Parse many to one extension.
pub fn parse_extension_multi_rel(
    x: &substrait::ExtensionMultiRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse inputs.
    let _in_types: Vec<_> = handle_rel_inputs!(x, y).collect();

    // Set schema to an unresolved type.
    y.set_schema(Arc::default());

    // Parse the extension data.
    proto_required_field!(x, y, detail, extensions::advanced::parse_functional_any);

    // Describe the relation.
    if let Some(x) = &x.detail {
        describe!(y, Relation, "{} extension", x.type_url);
    } else {
        describe!(y, Relation, "Unknown extension");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    Ok(())
}

/// Parse input extension.
pub fn parse_extension_leaf_rel(
    x: &substrait::ExtensionLeafRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Set schema to an unresolved type.
    y.set_schema(Arc::default());

    // Parse the extension data.
    proto_required_field!(x, y, detail, extensions::advanced::parse_functional_any);

    // Describe the relation.
    if let Some(x) = &x.detail {
        describe!(y, Relation, "{} extension", x.type_url);
    } else {
        describe!(y, Relation, "Unknown extension");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    Ok(())
}
