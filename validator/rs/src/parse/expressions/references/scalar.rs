// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validation scalar references.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions::literals;
use crate::string_util;
use std::sync::Arc;

/// Parse a struct field reference. Returns a string describing the nested
/// reference.
fn parse_struct_field(
    x: &substrait::expression::reference_segment::StructField,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<String> {
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

    // Handle the struct index field.
    let data_type = proto_primitive_field!(x, y, field, super::parse_struct_field_index, root)
        .1
        .unwrap_or_default();

    // Create description.
    let mut description = format!(".{}", x.field);

    // Set resulting data type.
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the struct field.
    if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Update description.
        if let Some(s) = result {
            description += &s;
        } else {
            description += ".?";
        }
    }

    // Describe node.
    describe!(y, Expression, "Selects {}", &description);
    Ok(description)
}

/// Parse a list element reference. Returns a string describing the nested
/// reference.
fn parse_list_element(
    x: &substrait::expression::reference_segment::ListElement,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<String> {
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
    let mut description = format!(".[{}]", x.offset);

    // Set resulting data type.
    let data_type = root.unwrap_list().unwrap_or_default();
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the list element.
    if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Update description.
        if let Some(s) = result {
            description += &s;
        } else {
            description += ".?";
        }
    }

    // Describe node.
    describe!(y, Expression, "Selects {}", &description);
    Ok(description)
}

/// Parse a map key reference. Returns a string describing the nested
/// reference.
fn parse_map_key(
    x: &substrait::expression::reference_segment::MapKey,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<String> {
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

    // Create description.
    let mut description = format!(".[{}]", key);

    // Set resulting data type.
    let data_type = root.unwrap_map().unwrap_or_default();
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively select elements from
    // the map value.
    if x.child.is_some() {
        let (node, result) =
            proto_boxed_required_field!(x, y, child, parse_reference_segment, &data_type);

        // Update data type.
        y.set_data_type(node.data_type());

        // Update description.
        if let Some(s) = result {
            description += &s;
        } else {
            description += ".?";
        }
    }

    // Describe node.
    describe!(y, Expression, "Selects {}", &description);
    Ok(description)
}

/// Parse a reference segment type. Returns a string describing the nested
/// reference.
fn parse_reference_type(
    x: &substrait::expression::reference_segment::ReferenceType,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<String> {
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
/// structure of type root. Returns a string describing the nested reference.
pub fn parse_reference_segment(
    x: &substrait::expression::ReferenceSegment,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<String> {
    // Parse the selection.
    let (node, result) = proto_required_field!(x, y, reference_type, parse_reference_type, root);

    // Set the data type.
    y.set_data_type(node.data_type());

    // Describe node.
    let description = result.unwrap_or_else(|| String::from("?"));
    describe!(y, Expression, "Selects {}", &description);
    Ok(description)
}
