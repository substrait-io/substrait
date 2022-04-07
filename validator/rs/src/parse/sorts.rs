// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for sort fields.

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::input::traits::ProtoEnum;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::expressions::functions;
use crate::parse::extensions;

/// Parse a sort direction.
fn parse_sort_direction(x: &i32, y: &mut context::Context) -> diagnostic::Result<&'static str> {
    use substrait::sort_field::SortDirection;
    match SortDirection::proto_enum_from_i32(*x) {
        None => {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "unknown value {x} for {}",
                SortDirection::proto_enum_type()
            );
            Ok("Invalid sort by")
        }
        Some(SortDirection::Unspecified) => {
            diagnostic!(y, Error, ProtoMissingField, "direction");
            Ok("Invalid sort by")
        }
        Some(SortDirection::AscNullsFirst) => {
            describe!(y, Misc, "Sort ascending, nulls first");
            Ok("Ascending sort by")
        }
        Some(SortDirection::AscNullsLast) => {
            describe!(y, Misc, "Sort ascending, nulls last");
            Ok("Ascending sort by")
        }
        Some(SortDirection::DescNullsFirst) => {
            describe!(y, Misc, "Sort descending, nulls first");
            Ok("Descending sort by")
        }
        Some(SortDirection::DescNullsLast) => {
            describe!(y, Misc, "Sort descending, nulls last");
            Ok("Descending sort by")
        }
        Some(SortDirection::Clustered) => {
            describe!(y, Misc, "Coalesce equal values");
            summary!(
                y,
                "Equal values are grouped together, but no ordering is defined between clusters."
            );
            Ok("Coalesce")
        }
    }
}

/// Parse a function reference that should resolve to a comparison function
/// (i.e. one usable for sorts) for the given type.
fn parse_comparison_function_reference(
    x: &u32,
    y: &mut context::Context,
    data_type: &Arc<data_type::DataType>,
) -> diagnostic::Result<&'static str> {
    // Resolve the reference as normal.
    let function = extensions::simple::parse_function_reference(x, y)?;

    // Check the function.
    if let Some(function) = &function.definition {
        let return_type =
            functions::check_function(y, function, &[], &[data_type.clone(), data_type.clone()]);
        if !matches!(
            return_type.class(),
            data_type::Class::Simple(data_type::Simple::Boolean)
                | data_type::Class::Simple(data_type::Simple::I8)
                | data_type::Class::Simple(data_type::Simple::I16)
                | data_type::Class::Simple(data_type::Simple::I32)
                | data_type::Class::Simple(data_type::Simple::I64)
                | data_type::Class::Unresolved
        ) {
            diagnostic!(
                y,
                Error,
                TypeMismatch,
                "comparison functions must yield booleans (a < b) or integers (a ?= b), but found {}",
                return_type
            );
        }
    } else {
        diagnostic!(
            y,
            Warning,
            ExpressionFunctionDefinitionUnavailable,
            "cannot check validity of comparison function"
        );
    }

    // Describe how the function is to be interpreted.
    y.push_summary(
        comment::Comment::new()
            .plain("Comparison function for sorting. Taking two elements as input,")
            .plain("it must determine the correct sort order. Comparison functions")
            .plain("may return booleans or integers, interpreted as follows:")
            .lo()
            .plain("f(a, b) => true or negative: a sorts before b;")
            .li()
            .plain("f(a, b) => false or positive: b sorts before a;")
            .li()
            .plain("f(a, b) => 0 or null: a and b have no defined sort order.")
            .lc()
            .plain("This corresponds to f: a < b or f: a ?= b."),
    );

    Ok("Custom sort")
}

/// Parse a sort kind, applicable to elements of the given data type.
fn parse_sort_kind(
    x: &substrait::sort_field::SortKind,
    y: &mut context::Context,
    data_type: &Arc<data_type::DataType>,
) -> diagnostic::Result<&'static str> {
    match x {
        substrait::sort_field::SortKind::Direction(x) => parse_sort_direction(x, y),
        substrait::sort_field::SortKind::ComparisonFunctionReference(x) => {
            parse_comparison_function_reference(x, y, data_type)
        }
    }
}

/// Parse a sort field.
pub fn parse_sort_field(
    x: &substrait::SortField,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse fields.
    let (n, e) = proto_required_field!(x, y, expr, expressions::parse_expression);
    let expression = e.unwrap_or_default();
    let method = proto_required_field!(x, y, sort_kind, parse_sort_kind, &n.data_type())
        .1
        .unwrap_or("Invalid sort by");

    // Describe node.
    describe!(y, Misc, "{method} {expression}");
    summary!(y, "{method} {expression:#}.");
    Ok(expression)
}
