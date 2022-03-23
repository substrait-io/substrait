// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for relational algebra.

#[macro_use]
mod common;
mod aggregate;
mod cross;
mod extension;
mod fetch;
mod filter;
mod join;
mod project;
mod read;
mod set;
mod sort;

use crate::input::proto::substrait;
use crate::input::traits::ProtoOneOf;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse a relation type.
fn parse_rel_type(x: &substrait::rel::RelType, y: &mut context::Context) -> diagnostic::Result<()> {
    // Ensure that the top of the schema stack exists and it set to an
    // unresolved type.
    y.clear_schema();

    // Set a basic description, to ensure that these nodes are always marked
    // as relations.
    describe!(y, Relation, "{} relation", x.proto_oneof_variant());

    // NOTE: if you're here because you added a relation type and now CI is
    // failing, you can just add "_ => Ok(())," to the end of this list. The
    // validator will then automatically throw a "not yet implemented" warning
    // if it finds that relation type in a plan.
    match x {
        substrait::rel::RelType::Read(x) => read::parse_read_rel(x, y),
        substrait::rel::RelType::Filter(x) => filter::parse_filter_rel(x, y),
        substrait::rel::RelType::Fetch(x) => fetch::parse_fetch_rel(x, y),
        substrait::rel::RelType::Aggregate(x) => aggregate::parse_aggregate_rel(x, y),
        substrait::rel::RelType::Sort(x) => sort::parse_sort_rel(x, y),
        substrait::rel::RelType::Join(x) => join::parse_join_rel(x, y),
        substrait::rel::RelType::Project(x) => project::parse_project_rel(x, y),
        substrait::rel::RelType::Set(x) => set::parse_set_rel(x, y),
        substrait::rel::RelType::ExtensionSingle(x) => extension::parse_extension_single_rel(x, y),
        substrait::rel::RelType::ExtensionMulti(x) => extension::parse_extension_multi_rel(x, y),
        substrait::rel::RelType::ExtensionLeaf(x) => extension::parse_extension_leaf_rel(x, y),
        substrait::rel::RelType::Cross(x) => cross::parse_cross_rel(x, y),
        // _ => Ok(()),
    }
}

/// Parse a relation root, i.e. a toplevel relation that includes field name
/// information.
pub fn parse_rel(x: &substrait::Rel, y: &mut context::Context) -> diagnostic::Result<()> {
    let schema = proto_required_field!(x, y, rel_type, parse_rel_type)
        .0
        .data_type();
    y.set_schema(schema);
    Ok(())
}
