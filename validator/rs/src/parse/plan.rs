// SPDX-License-Identifier: Apache-2.0

//! Module providing toplevel parse/validation functions for plans.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::extensions;
use crate::parse::relations;

// Parse a relation root, i.e. a toplevel relation that includes field name
// information.
fn parse_rel_root(x: &substrait::RelRoot, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse the fields.
    let schema = proto_required_field!(x, y, input, relations::parse_rel)
        .0
        .data_type();
    proto_repeated_field!(x, y, names);

    // Relate the names to the schema.
    let schema = schema
        .apply_field_names(&x.names)
        .map_err(|x| diagnostic!(y, Error, x))
        .unwrap_or_default();
    y.set_schema(schema);

    // Describe the node.
    describe!(y, Misc, "Relation root");
    summary!(y, "Attaches names to result schema");
    Ok(())
}

// Parse a relation type.
fn parse_rel_type(
    x: &substrait::plan_rel::RelType,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    match x {
        substrait::plan_rel::RelType::Rel(x) => relations::parse_rel(x, y),
        substrait::plan_rel::RelType::Root(x) => parse_rel_root(x, y),
    }
}

/// Parse a PlanRel node.
fn parse_plan_rel(x: &substrait::PlanRel, y: &mut context::Context) -> diagnostic::Result<()> {
    y.enter_relation_root(|y| {
        proto_required_field!(x, y, rel_type, parse_rel_type);
    });
    Ok(())
}

/// Toplevel parse function for a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) -> diagnostic::Result<()> {
    // Handle extensions first, because we'll need their declarations to
    // correctly interpret the relations.
    extensions::parse_plan(x, y);

    // Handle the relations.
    let num_relations = proto_repeated_field!(x, y, relations, parse_plan_rel)
        .0
        .len();
    if num_relations == 0 {
        diagnostic!(
            y,
            Error,
            RelationRootMissing,
            "a plan must have at least one relation"
        );
    }

    // Generate an Info diagnostic for every extension definition that wasn't
    // used at any point, and can thus be safely removed.
    extensions::check_unused_definitions(y);

    Ok(())
}
