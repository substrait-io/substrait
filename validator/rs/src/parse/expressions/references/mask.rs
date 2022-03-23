// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validation mask expressions.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::string_util;
use std::sync::Arc;

/// Parse a struct item.
fn parse_struct_item(
    x: &substrait::expression::mask_expression::StructItem,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    // Handle the struct index field.
    let data_type = proto_primitive_field!(x, y, field, super::parse_struct_field_index, root)
        .1
        .unwrap_or_default();

    // Set resulting data type.
    y.set_data_type(data_type.clone());

    // Handle child selection, if any, to recursively project the field type
    // of the selected struct field.
    if x.child.is_some() {
        let data_type = proto_required_field!(x, y, child, parse_select, &data_type)
            .0
            .data_type();

        // Update data type.
        y.set_data_type(data_type);

        // Describe node.
        describe!(y, Expression, "Struct item selection and sub-selection");
    } else {
        describe!(y, Expression, "Struct item selection");
    }

    Ok(())
}

/// Parse a struct selection, a filter/swizzle for a struct type.
fn parse_struct_select(
    x: &substrait::expression::mask_expression::StructSelect,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
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

    // Parse fields.
    let fields = proto_repeated_field!(
        x,
        y,
        struct_items,
        parse_struct_item,
        |_, _, _, _, _| (),
        root
    )
    .0
    .iter()
    .map(|x| x.data_type())
    .collect::<Vec<_>>();

    // Create struct.
    y.set_data_type(data_type::DataType::new_struct(fields, root.nullable()));

    // Describe node.
    describe!(y, Expression, "Struct selection");
    Ok(())
}

/// Parse a list element selection.
fn parse_list_select_item_element(
    x: &substrait::expression::mask_expression::list_select::list_select_item::ListElement,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    proto_primitive_field!(x, y, field);
    describe!(
        y,
        Expression,
        "Select {} element",
        string_util::describe_index(x.field)
    );
    Ok(())
}

/// Parse a list slice selection.
fn parse_list_select_item_slice(
    x: &substrait::expression::mask_expression::list_select::list_select_item::ListSlice,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    proto_primitive_field!(x, y, start);
    proto_primitive_field!(x, y, end);

    // Raise a diagnostic if the slice is always null, and describe the slice.
    let description = if (x.start >= 0) == (x.end >= 0) && x.start < x.end {
        diagnostic!(y, Info, RedundantListSlice, "slice is always null");
        String::from("Selects an empty list slice")
    } else if x.start == 0 {
        match x.end {
            i32::MIN..=-3 => format!("Selects all but the last {} elements", -x.end - 1),
            -2 => String::from("Selects all but the last element"),
            -1 => String::from("Selects the complete list"),
            0 => String::from("Selects the first element"),
            1..=i32::MAX => format!("Selects the first {} elements", x.end + 1),
        }
    } else if x.end == -1 {
        match x.start {
            i32::MIN..=-2 => format!("Selects the last {} elements", -x.start),
            -1 => String::from("Selects the last element"),
            0 => String::from("Selects the complete list"),
            1 => String::from("Selects all but the first element"),
            2..=i32::MAX => format!("Selects all but the first {} elements", x.start),
        }
    } else {
        format!(
            "Select {} until {} element (inclusive)",
            string_util::describe_index(x.start),
            string_util::describe_index(x.end)
        )
    };
    describe!(y, Expression, "{}", description);

    // Describe the node.
    Ok(())
}

/// Parse a list selection item type.
fn parse_list_select_item_type(
    x: &substrait::expression::mask_expression::list_select::list_select_item::Type,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    match x {
        substrait::expression::mask_expression::list_select::list_select_item::Type::Item(x) => {
            parse_list_select_item_element(x, y)
        }
        substrait::expression::mask_expression::list_select::list_select_item::Type::Slice(x) => {
            parse_list_select_item_slice(x, y)
        }
    }
}

/// Parse a list selection item.
fn parse_list_select_item(
    x: &substrait::expression::mask_expression::list_select::ListSelectItem,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    proto_required_field!(x, y, r#type, parse_list_select_item_type);
    Ok(())
}

/// Parse a list selection, a filter/swizzle for a list type.
fn parse_list_select(
    x: &substrait::expression::mask_expression::ListSelect,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    // List selections can only be applied to lists.
    if !root.is_unresolved() && !root.is_list() {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "list selection requires a list type, but got a {}",
            root.class()
        );
    }

    // Parse fields.
    proto_repeated_field!(x, y, selection, parse_list_select_item);

    // Set resulting data type.
    y.set_data_type(root.clone());

    // Handle child selection, if any, to recursively project the list element
    // type.
    if x.child.is_some() {
        // Get the list element type.
        let data_type = root.unwrap_list().unwrap_or_default();

        // Apply selection logic recursively.
        let data_type = proto_boxed_required_field!(x, y, child, parse_select, &data_type)
            .0
            .data_type();

        // Create the new type.
        y.set_data_type(data_type::DataType::new_list(data_type, root.nullable()));

        // Describe node.
        describe!(y, Expression, "List selection and sub-selection");
    } else {
        describe!(y, Expression, "List selection");
    }

    Ok(())
}

/// Parse a map single-key selection.
fn parse_map_select_key(
    _x: &substrait::expression::mask_expression::map_select::MapKey,
    y: &mut context::Context,
    _key_type: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    // FIXME: map keys are not necessarily strings. Why is this not a
    // primitive?
    diagnostic!(
        y,
        Error,
        NotYetImplemented,
        "map key remappings are not yet specified"
    );
    describe!(y, Expression, "Single-key map selection");
    Ok(())
}

/// Parse a map selection by means of an expression.
fn parse_map_select_expression(
    _x: &substrait::expression::mask_expression::map_select::MapKeyExpression,
    y: &mut context::Context,
    _key_type: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    // FIXME: in Rust vernacular, need an Fn(K) -> Option<K> here. I suppose
    // there is no structure for that yet?
    diagnostic!(
        y,
        Error,
        NotYetImplemented,
        "map key remappings are not yet specified"
    );
    describe!(y, Expression, "Map key remapping");
    Ok(())
}

/// Parse a map selection type.
fn parse_map_select_type(
    x: &substrait::expression::mask_expression::map_select::Select,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    match x {
        substrait::expression::mask_expression::map_select::Select::Key(x) => {
            parse_map_select_key(x, y, root)
        }
        substrait::expression::mask_expression::map_select::Select::Expression(x) => {
            parse_map_select_expression(x, y, root)
        }
    }
}

/// Parse a map selection.
fn parse_map_select(
    x: &substrait::expression::mask_expression::MapSelect,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
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

    // Parse selection field.
    if x.select.is_some() {
        proto_required_field!(
            x,
            y,
            select,
            parse_map_select_type,
            &root.unwrap_map_key().unwrap_or_default()
        );
    } else {
        comment!(y, "No select key specified: mapping is left unchanged.");
    }

    // Set resulting data type.
    y.set_data_type(root.clone());

    // Handle child selection, if any, to recursively project the map value
    // type.
    if x.child.is_some() {
        // Get the map types.
        let value_type = root.unwrap_map().unwrap_or_default();
        let key_type = root.unwrap_map_key().unwrap_or_default();

        // Apply selection logic recursively.
        let value_type = proto_boxed_required_field!(x, y, child, parse_select, &value_type)
            .0
            .data_type();

        // Create the new type.
        y.set_data_type(data_type::DataType::new_map(
            key_type,
            value_type,
            root.nullable(),
        ));

        // Describe node.
        describe!(y, Expression, "Map selection and sub-selection");
    } else {
        describe!(y, Expression, "Map selection");
    }

    Ok(())
}

/// Parse a selection type.
fn parse_select_type(
    x: &substrait::expression::mask_expression::select::Type,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    match x {
        substrait::expression::mask_expression::select::Type::Struct(x) => {
            parse_struct_select(x, y, root)
        }
        substrait::expression::mask_expression::select::Type::List(x) => {
            parse_list_select(x.as_ref(), y, root)
        }
        substrait::expression::mask_expression::select::Type::Map(x) => {
            parse_map_select(x.as_ref(), y, root)
        }
    }
}

fn parse_select(
    x: &substrait::expression::mask_expression::Select,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<()> {
    proto_required_field!(x, y, r#type, parse_select_type, root);
    Ok(())
}

/// Parses the maintain_singular_struct field of a mask expression. is_singular
/// must specify whether the data type is actually a singular struct, while
/// struct_required must specify whether the context of the mask expression
/// requires a struct type. Returns whether the data type is a singular struct
/// and should be unwrapped.
fn parse_maintain_singular_struct(
    x: &bool,
    y: &mut context::Context,
    is_singular: bool,
    struct_required: bool,
) -> diagnostic::Result<bool> {
    let maintain = *x;
    match (is_singular, maintain, struct_required) {
        (true, true, _) => {
            // Okay: maintain struct.
            summary!(
                y,
                "Mask expression yields a singular struct, which is maintained as-is."
            );
            Ok(false)
        }
        (true, false, true) => {
            // Error: request to remove struct, but context requires a struct.
            summary!(y, "Mask expression yields a singular struct, which would be reduced to its element type, but its context does not allow this.");
            diagnostic!(y, Error, TypeStructRequired, "context requires a struct type and type is a singular struct, maintain_singular_struct must be set");
            Ok(false)
        }
        (true, false, false) => {
            // Okay: remove singular struct wrapper.
            summary!(
                y,
                "Mask expression yields a singular struct, which is reduced to its element type."
            );
            Ok(true)
        }
        (false, true, _) => {
            // Okay: not a singular struct, so there is nothing to strip.
            summary!(y, "Data type of mask expression is not a singular struct, so there is nothing to strip or maintain. The explicit true is redundant.");
            Ok(false)
        }
        (false, false, _) => {
            // Okay: not a singular struct, so there is nothing to strip.
            summary!(y, "Data type of mask expression is not a singular struct, so there is nothing to strip or maintain.");
            Ok(false)
        }
    }
}

/// Parse a mask expression; that is, a field selection that can output a
/// nested structure. root specifies the data type being indexed, while
/// struct_required must specify whether the context of the mask expression
/// requires a struct type.
pub fn parse_mask_expression(
    x: &substrait::expression::MaskExpression,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
    struct_required: bool,
) -> diagnostic::Result<()> {
    // Parse the struct selection and get its data type.
    let data_type = proto_required_field!(x, y, select, parse_struct_select, root)
        .0
        .data_type();

    // Determine if the data type is a singular struct (i.e. a struct with only
    // one item) and its element type if so.
    let singular_type = data_type.unwrap_singular_struct();

    // Handle the maintain_singular_struct field.
    let unwrap = proto_primitive_field!(
        x,
        y,
        maintain_singular_struct,
        parse_maintain_singular_struct,
        singular_type.is_some(),
        struct_required
    )
    .1
    .unwrap_or_default();

    // Set the data type.
    y.set_data_type(if unwrap {
        singular_type.unwrap()
    } else {
        data_type
    });

    // Describe node.
    describe!(
        y,
        Expression,
        "References fields into a new nested structure"
    );
    Ok(())
}
