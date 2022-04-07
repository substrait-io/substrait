// SPDX-License-Identifier: Apache-2.0

//! Module for parsing logic common to all relation types.

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse a stats node.
fn parse_stats(
    x: &substrait::rel_common::hint::Stats,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    proto_primitive_field!(x, y, row_count, |x, y| {
        if *x < 0.0 {
            diagnostic!(
                y,
                Error,
                IllegalValueInHint,
                "negative row counts are nonsensical"
            );
        }
        Ok(())
    });
    proto_primitive_field!(x, y, record_size, |x, y| {
        if *x < 0.0 {
            diagnostic!(
                y,
                Error,
                IllegalValueInHint,
                "negative record sizes are nonsensical"
            );
        }
        Ok(())
    });
    proto_field!(
        x,
        y,
        advanced_extension,
        crate::parse::extensions::advanced::parse_advanced_extension
    );
    Ok(())
}

/// Parse a constraints node.
fn parse_runtime_constraint(
    x: &substrait::rel_common::hint::RuntimeConstraint,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    proto_field!(
        x,
        y,
        advanced_extension,
        crate::parse::extensions::advanced::parse_advanced_extension
    );
    Ok(())
}

/// Parse a hint node.
fn parse_hint(x: &substrait::rel_common::Hint, y: &mut context::Context) -> diagnostic::Result<()> {
    proto_field!(x, y, stats, parse_stats);
    proto_field!(x, y, constraint, parse_runtime_constraint);
    proto_field!(
        x,
        y,
        advanced_extension,
        crate::parse::extensions::advanced::parse_advanced_extension
    );
    Ok(())
}

/// Parse emit mapping. Takes the data type of the schema thus far as argument.
fn parse_emit_mapping(
    x: &i32,
    _: &mut context::Context,
    data_type: Arc<data_type::DataType>,
) -> diagnostic::Result<Arc<data_type::DataType>> {
    let x: usize = (*x)
        .try_into()
        .map_err(|_| cause!(TypeInvalidSwizzle, "index cannot be negative"))?;
    data_type
        .index_struct(x)
        .ok_or_else(|| cause!(TypeInvalidSwizzle, "index out of range"))
}

/// Parse emit kind. Takes the data type of the schema thus far as argument.
fn parse_emit_kind(
    x: &substrait::rel_common::EmitKind,
    y: &mut context::Context,
    data_type: Arc<data_type::DataType>,
) -> diagnostic::Result<Arc<data_type::DataType>> {
    match x {
        substrait::rel_common::EmitKind::Direct(_) => Ok(data_type),
        substrait::rel_common::EmitKind::Emit(x) => {
            let fields = proto_repeated_field!(
                x,
                y,
                output_mapping,
                parse_emit_mapping,
                |_, _, _, _, _| (),
                data_type.clone()
            )
            .1
            .into_iter()
            .map(|x| x.unwrap_or_default())
            .collect::<Vec<_>>();
            Ok(data_type::DataType::new_struct(fields, false))
        }
    }
}

/// Parse RelCommon node. This should be processed after the rest of the
/// relation has processed, as it can transmute the data type.
pub fn parse_rel_common(
    x: &substrait::RelCommon,
    y: &mut context::Context,
    data_type: Arc<data_type::DataType>,
) -> diagnostic::Result<Arc<data_type::DataType>> {
    // Handle hint.
    proto_field!(x, y, hint, parse_hint);

    // Handle advanced extension.
    let data_type = if proto_field!(
        x,
        y,
        advanced_extension,
        crate::parse::extensions::advanced::parse_advanced_extension
    )
    .1
    .unwrap_or_default()
    {
        data_type::DataType::new_unresolved()
    } else {
        data_type
    };

    // Parse emit kind.
    let data_type = proto_field!(x, y, emit_kind, parse_emit_kind, data_type.clone())
        .1
        .unwrap_or(data_type);

    Ok(data_type)
}

/// Handle the common field for a relation. This should be processed after the
/// rest of the relation has processed, as it can transmute the data type.
macro_rules! handle_rel_common {
    ($input:expr, $context:expr) => {
        let data_type = $context.data_type();

        // Call the parser.
        let result = proto_field!(
            $input,
            $context,
            common,
            crate::parse::relations::common::parse_rel_common,
            data_type
        )
        .1;

        // If common was populated and its parser succeeded (it should always
        // do that), update the type information.
        if let Some(data_type) = result {
            $context.set_schema(data_type);
        }
    };
}

/// Handle the advanced extension field for a builtin relation.
macro_rules! handle_advanced_extension {
    ($input:expr, $context:expr) => {
        if proto_field!(
            $input,
            $context,
            advanced_extension,
            crate::parse::extensions::advanced::parse_advanced_extension
        )
        .1
        .unwrap_or_default()
        {
            $context.set_schema(std::sync::Arc::default());
        }
    };
}

/// Shorthand for handling the input field of a relation. Returns a the data
/// type corresponding to the schema returned by the relation.
macro_rules! handle_rel_input {
    ($input:expr, $context:expr) => {
        handle_rel_input!($input, $context, input)
    };
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_required_field!($input, $context, $field, crate::parse::relations::parse_rel)
            .0
            .data_type()
    };
}

/// Shorthand for handling the input fields of a relation that takes a flexible
/// amount of inputs. Returns an iterator to references to the data types
/// corresponding to the schemas returned by the relations. Each data type can
/// be None if schema type deduction failed.
macro_rules! handle_rel_inputs {
    ($input:expr, $context:expr) => {
        handle_rel_inputs!($input, $context, inputs)
    };
    ($input:expr, $context:expr, $field:ident) => {
        proto_repeated_field!($input, $context, $field, crate::parse::relations::parse_rel)
            .0
            .iter()
            .map(|x| x.data_type())
    };
}
