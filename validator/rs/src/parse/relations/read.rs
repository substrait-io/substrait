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
use crate::parse::expressions::literals;
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
    x: &substrait::read_rel::VirtualTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    let mut data_type: Arc<data_type::DataType> = Arc::default();

    // Parse rows, ensuring that they all have the same type.
    proto_repeated_field!(x, y, values, |x, y| {
        let result = literals::parse_struct(x, y, false);
        data_type = types::assert_equal(
            y,
            &y.data_type(),
            &data_type,
            "virtual table rows must have the same type",
        );
        result
    });

    // Describe the node.
    describe!(y, Misc, "Virtual table");
    Ok(SourceInfo {
        name: String::from("virtual table"),
        data_type: Some(data_type),
    })
}

/// Parse file entry. Returns whether this matches multiple files.
fn parse_path_type(
    x: &substrait::read_rel::local_files::file_or_files::PathType,
    y: &mut context::Context,
) -> diagnostic::Result<bool> {
    // FIXME: I'm not sure these paths should even be URIs. These are supposed
    // to be local files after all, so shouldn't they just be paths? But they
    // really shouldn't be called URIs if they're not going to conform to the
    // standard governing them, and if they're paths, there should still be
    // some specification about what kind of paths they can be (POSIX? Windows
    // with slashes? UNC? etc).
    //
    // Note that the diagnostics for this have their own code, so if a user
    // disagrees with the syntax they can just downgrade these warnings to
    // infos.
    use substrait::read_rel::local_files::file_or_files::PathType;
    match x {
        PathType::UriPath(x) => {
            if !string_util::is_uri(x) {
                diagnostic!(
                    y,
                    Warning,
                    IllegalUri,
                    "this URI may not be valid according to RFC 3986"
                );
            }
            Ok(false)
        }
        PathType::UriPathGlob(x) => {
            if !string_util::is_uri_glob(x) {
                diagnostic!(
                    y,
                    Warning,
                    IllegalUri,
                    "this URI may not be valid according to RFC 3986 + globs for paths"
                );
            }
            Ok(true)
        }
        PathType::UriFile(x) => {
            if !string_util::is_uri(x) {
                diagnostic!(
                    y,
                    Warning,
                    IllegalUri,
                    "this URI may not be valid according to RFC 3986"
                );
            }
            Ok(false)
        }
        PathType::UriFolder(x) => {
            if !string_util::is_uri(x) {
                diagnostic!(
                    y,
                    Warning,
                    IllegalUri,
                    "this URI may not be valid according to RFC 3986"
                );
            }
            Ok(true)
        }
    }
}

/// Parse file entry.
fn parse_file_or_files(
    x: &substrait::read_rel::local_files::FileOrFiles,
    y: &mut context::Context,
    extension_present: bool,
) -> diagnostic::Result<()> {
    // Parse path.
    let multiple = proto_required_field!(x, y, path_type, parse_path_type)
        .1
        .unwrap_or_default();

    // Parse read configuration.
    let format = proto_enum_field!(
        x,
        y,
        format,
        substrait::read_rel::local_files::file_or_files::FileFormat,
        |x, y| {
            if !extension_present
                && matches!(
                    x,
                    substrait::read_rel::local_files::file_or_files::FileFormat::Unspecified
                )
            {
                diagnostic!(
                    y,
                    Error,
                    IllegalValue,
                    "file format must be specified when no enhancement extension is present"
                );
            }
            Ok(*x)
        }
    )
    .1
    .unwrap_or_default();
    proto_primitive_field!(x, y, partition_index);
    proto_primitive_field!(x, y, start);
    proto_primitive_field!(x, y, length);

    // Having nonzero file offsets makes no sense when this entry refers to
    // multiple files.
    if multiple && (x.start > 0 || x.length > 0) {
        diagnostic!(
            y,
            Error,
            IllegalValue,
            "file offsets are not allowed in conjunction with multiple files"
        );
    }

    // Describe the node.
    if multiple {
        describe!(y, Misc, "Multiple files");
    } else {
        describe!(y, Misc, "Single file");
    }
    summary!(y, "Read");
    if x.partition_index != 0 {
        summary!(y, "partition {}", x.partition_index);
    }
    summary!(y, "from");
    if multiple {
        summary!(y, "multiple");
    } else {
        if x.start > 0 {
            if x.length > 0 {
                summary!(y, "byte offset {} to {} of", x.start, x.start + x.length);
            } else {
                summary!(y, "byte offset {} to the end of", x.start);
            }
        } else if x.length > 0 {
            summary!(y, "the first {} byte(s) of", x.length);
        }
        summary!(y, "a single");
    }
    match format {
        substrait::read_rel::local_files::file_or_files::FileFormat::Unspecified => {}
        substrait::read_rel::local_files::file_or_files::FileFormat::Parquet => {
            summary!(y, "Parquet");
        }
    }
    if multiple {
        summary!(y, "files");
    } else {
        summary!(y, "file");
    }

    Ok(())
}

/// Parse local files.
fn parse_local_files(
    x: &substrait::read_rel::LocalFiles,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // Parse fields.
    let extension_present = x
        .advanced_extension
        .as_ref()
        .and_then(|x| x.enhancement.as_ref())
        .is_some();
    proto_required_repeated_field!(
        x,
        y,
        items,
        parse_file_or_files,
        |_, _, _, _, _| (),
        extension_present
    );
    proto_field!(
        x,
        y,
        advanced_extension,
        extensions::advanced::parse_advanced_extension
    );

    // Describe the node.
    describe!(y, Misc, "Table from file(s)");
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
    proto_required_repeated_field!(x, y, names);
    proto_field!(
        x,
        y,
        advanced_extension,
        extensions::advanced::parse_advanced_extension
    );

    // Determine and check consistency of the table name.
    let name = if x.names.is_empty() {
        String::from("?")
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
        string_util::as_ident_or_string(x.names.first().unwrap())
    };

    // Describe the node.
    describe!(
        y,
        Misc,
        "Named table {}",
        string_util::as_ident_or_string(&name)
    );
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

    // Describe the node.
    describe!(
        y,
        Misc,
        "{} extension",
        x.detail
            .as_ref()
            .map(|x| x.type_url.clone())
            .unwrap_or_else(|| String::from("Unknown"))
    );
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
            types::assert_equal(y, &schema, &data_type, "data differs from schema")
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
