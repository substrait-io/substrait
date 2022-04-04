// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validation scalar references.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions::literals;
use crate::parse::expressions::references;
use crate::parse::types;
use crate::string_util;
use std::sync::Arc;

/// Parse a struct field reference. Returns a description of the nested
/// reference.
fn parse_struct_field(
    x: &substrait::expression::reference_segment::StructField,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<references::ReferencePath> {
    // Struct selections can only be applied to structs.
    if !root.is_unresolved() && !root.is_struct() {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "struct selection requires a struct type, but got a {}",
            root.class()
        );
    }

    // Create description.
    let description = format!(".{}", x.field);

    // Determine result data type.
    let data_type = proto_primitive_field!(x, y, field, super::parse_struct_field_index, root)
        .1
        .unwrap_or_default();

    // If the struct is nullable, the field must also be nullable.
    let data_type = if root.nullable() {
        data_type.make_nullable()
    } else {
        data_type
    };

    // Set resulting data type.
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the struct field.
    let reference = if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Generate reference.
        result.unwrap_or_default().prefix(description)
    } else {
        references::ReferencePath::new().prefix(description)
    };

    // Describe node.
    describe!(y, Expression, "Selects {}", &reference);
    summary!(y, "Full reference path: {:#}", &reference);
    Ok(reference)
}

/// Parse a list element reference. Returns a description of the nested
/// reference.
fn parse_list_element(
    x: &substrait::expression::reference_segment::ListElement,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<references::ReferencePath> {
    // Struct selections can only be applied to lists.
    if !root.is_unresolved() && !root.is_list() {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "list selection requires a list type, but got a {}",
            root.class()
        );
    }

    // Handle the list index field.
    proto_primitive_field!(x, y, offset, |x, y| {
        describe!(
            y,
            Misc,
            "Selects {} list element",
            string_util::describe_index(*x)
        );
        Ok(())
    });

    // Create description.
    let description = format!(".[{}]", x.offset);

    // Determine result data type.
    let data_type = root.unwrap_list().unwrap_or_default();

    // If the list is nullable, the selection must also be nullable.
    let data_type = if root.nullable() {
        data_type.make_nullable()
    } else {
        data_type
    };

    // FIXME: what is the runtime behavior for index out of range, throw or
    // yield null? In the latter case, the return type would always need to
    // be nullable.

    // Set resulting data type.
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the list element.
    let reference = if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Generate reference.
        result.unwrap_or_default().prefix(description)
    } else {
        references::ReferencePath::new().prefix(description)
    };

    // Describe node.
    describe!(y, Expression, "Selects {}", &reference);
    summary!(y, "Full reference path: {:#}", &reference);
    Ok(reference)
}

/// Parse a map key reference. Returns a description of the nested
/// reference.
fn parse_map_key(
    x: &substrait::expression::reference_segment::MapKey,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<references::ReferencePath> {
    // Map selections can only be applied to maps.
    if !root.is_unresolved() && !root.is_map() {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "map selection requires a map type, but got a {}",
            root.class()
        );
    }

    // Handle the map key primitive.
    let key = proto_required_field!(x, y, map_key, literals::parse_literal)
        .1
        .unwrap_or_default();

    // Check the key type.
    types::assert_equal(
        y,
        key.data_type(),
        &root.unwrap_map_key().unwrap_or_default(),
        "map key type mismatch",
    );

    // Create description.
    let description = format!(".[{}]", key);

    // Determine result data type.
    let data_type = root.unwrap_map().unwrap_or_default();

    // If the map is nullable, the selection must also be nullable.
    let data_type = if root.nullable() {
        data_type.make_nullable()
    } else {
        data_type
    };

    // FIXME: what is the runtime behavior for index out of range, throw or
    // yield null? In the latter case, the return type would always need to
    // be nullable.

    // Set resulting data type.
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the map value.
    let reference = if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Generate reference.
        result.unwrap_or_default().prefix(description)
    } else {
        references::ReferencePath::new().prefix(description)
    };

    // Describe node.
    describe!(y, Expression, "Selects {}", &reference);
    summary!(y, "Full reference path: {:#}", &reference);
    Ok(reference)
}

/// Parse a reference segment type. Returns a description of the nested
/// reference.
fn parse_reference_type(
    x: &substrait::expression::reference_segment::ReferenceType,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<references::ReferencePath> {
    match x {
        substrait::expression::reference_segment::ReferenceType::StructField(x) => {
            parse_struct_field(x, y, root)
        }
        substrait::expression::reference_segment::ReferenceType::ListElement(x) => {
            parse_list_element(x, y, root)
        }
        substrait::expression::reference_segment::ReferenceType::MapKey(x) => {
            parse_map_key(x, y, root)
        }
    }
}

/// Parse a reference segment, i.e. a scalar reference into some nested
/// structure of type root. Returns a description of the nested reference.
pub fn parse_reference_segment(
    x: &substrait::expression::ReferenceSegment,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<references::ReferencePath> {
    // Parse the selection.
    let (node, result) = proto_required_field!(x, y, reference_type, parse_reference_type, root);

    // Set the data type.
    y.set_data_type(node.data_type());

    // Describe node.
    let reference = result.unwrap_or_default();
    describe!(y, Expression, "Selects {}", &reference);
    summary!(y, "Full reference path: {:#}", &reference);
    Ok(reference)
}
