// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for read relations.
//!
//! The read operator is an operator that produces one output. A simple example
//! would be the reading of a Parquet file.
//!
//! See <https://substrait.io/relations/logical_relations/#read-operator>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::expressions::references::mask;
use crate::parse::extensions;
use crate::parse::types;
use crate::string_util;

/// Information about a data source.
struct SourceInfo {
    /// Short description of the data source, used in the brief of the read
    /// relation.
    pub name: String,

    /// The schema of the data, if not context-sensitive.
    pub data_type: Option<Arc<data_type::DataType>>,
}

/// Parse virtual table.
fn parse_virtual_table(
    _x: &substrait::read_rel::VirtualTable,
    _y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // TODO
    Ok(SourceInfo {
        name: String::from("virtual table"),
        data_type: None,
    })
}

/// Parse local files.
fn parse_local_files(
    _x: &substrait::read_rel::LocalFiles,
    _y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // TODO
    Ok(SourceInfo {
        name: String::from("local files"),
        data_type: None,
    })
}

/// Parse named table.
fn parse_named_table(
    x: &substrait::read_rel::NamedTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // Parse fields.
    proto_repeated_field!(x, y, names);
    proto_field!(
        x,
        y,
        advanced_extension,
        extensions::advanced::parse_advanced_extension
    );

    // Determine and check consistency of the table name.
    let name = if x.names.is_empty() {
        diagnostic!(y, Error, ProtoMissingField, "names");
        String::from("unknown table")
    } else {
        if x.names.len() > 1 {
            // FIXME: what does this mean?
            diagnostic!(
                y,
                Warning,
                NotYetImplemented,
                "named tables with multiple names"
            );
        }
        format!(
            "table {}",
            string_util::as_ident_or_string(x.names.first().unwrap())
        )
    };

    Ok(SourceInfo {
        name,
        data_type: None,
    })
}

/// Parse extension table.
fn parse_extension_table(
    x: &substrait::read_rel::ExtensionTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    proto_required_field!(x, y, detail, extensions::advanced::parse_functional_any);
    Ok(SourceInfo {
        name: x
            .detail
            .as_ref()
            .map(|x| x.type_url.to_string())
            .unwrap_or_else(|| String::from("extension")),
        data_type: None,
    })
}

/// Parse read type.
fn parse_read_type(
    x: &substrait::read_rel::ReadType,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    match x {
        substrait::read_rel::ReadType::VirtualTable(x) => parse_virtual_table(x, y),
        substrait::read_rel::ReadType::LocalFiles(x) => parse_local_files(x, y),
        substrait::read_rel::ReadType::NamedTable(x) => parse_named_table(x, y),
        substrait::read_rel::ReadType::ExtensionTable(x) => parse_extension_table(x, y),
    }
}

/// Parse read relation.
pub fn parse_read_rel(x: &substrait::ReadRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Handle read type field.
    let source = proto_required_field!(x, y, read_type, parse_read_type)
        .1
        .unwrap_or(SourceInfo {
            name: String::from("unknown source"),
            data_type: None,
        });

    // Handle schema field.
    let schema = proto_required_field!(x, y, base_schema, types::parse_named_struct)
        .0
        .data_type
        .clone();

    // If both data_type and schema are known, verify that they are the same.
    let mut schema = match (source.data_type, schema) {
        (Some(data_type), Some(schema)) => {
            types::assert_equal(y, schema, data_type, "data differs from schema")
        }
        (Some(data_type), None) => data_type,
        (None, Some(schema)) => schema,
        (None, None) => Arc::default(),
    };

    // Set the schema to the merged data type.
    y.set_schema(schema.clone());

    // Handle filter.
    proto_boxed_field!(x, y, filter, expressions::parse_predicate);

    // Handle projection.
    if x.projection.is_some() {
        schema =
            proto_required_field!(x, y, projection, mask::parse_mask_expression, &schema, true)
                .0
                .data_type();
        y.set_schema(schema.clone());
    }

    // Describe the relation.
    match (x.filter.is_some(), x.projection.is_some()) {
        (false, false) => describe!(y, Relation, "Read from {}", source.name),
        (false, true) => describe!(y, Relation, "Partial read from {}", source.name),
        (true, false) => describe!(y, Relation, "Filtered read from {}", source.name),
        (true, true) => describe!(y, Relation, "Filtered partial read from {}", source.name),
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
