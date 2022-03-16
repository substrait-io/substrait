// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for read relations.
//!
//! The read operator is an operator that produces one output. A simple example
//! would be the reading of a Parquet file.
//!
//! See <https://substrait.io/relations/logical_relations/#read-operator>

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::types;

/// Parse virtual table.
fn parse_virtual_table(
    _x: &substrait::read_rel::VirtualTable,
    _y: &mut context::Context,
) -> diagnostic::Result<Option<data_type::DataType>> {
    // TODO
    Ok(None)
}

/// Parse local files.
fn parse_local_files(
    _x: &substrait::read_rel::LocalFiles,
    _y: &mut context::Context,
) -> diagnostic::Result<Option<data_type::DataType>> {
    // TODO
    Ok(None)
}

/// Parse named table.
fn parse_named_table(
    _x: &substrait::read_rel::NamedTable,
    _y: &mut context::Context,
) -> diagnostic::Result<Option<data_type::DataType>> {
    // TODO
    Ok(None)
}

/// Parse extension table.
fn parse_extension_table(
    _x: &substrait::read_rel::ExtensionTable,
    _y: &mut context::Context,
) -> diagnostic::Result<Option<data_type::DataType>> {
    // TODO
    Ok(None)
}

/// Parse read type.
fn parse_read_type(
    x: &substrait::read_rel::ReadType,
    y: &mut context::Context,
) -> diagnostic::Result<Option<data_type::DataType>> {
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
    let data_type = proto_required_field!(x, y, read_type, parse_read_type)
        .1
        .flatten();

    // Handle schema field.
    let schema = proto_required_field!(x, y, base_schema, types::parse_named_struct)
        .0
        .data_type
        .clone();

    // If both data_type and schema are known, verify that they are the same.
    let schema = match (data_type, schema) {
        (Some(data_type), Some(schema)) => {
            types::assert_equal(y, schema, data_type, "data differs from schema")
        }
        (Some(data_type), None) => data_type,
        (None, Some(schema)) => schema,
        (None, None) => data_type::DataType::default(),
    };

    // Set the schema.
    schema!(y, schema);

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
