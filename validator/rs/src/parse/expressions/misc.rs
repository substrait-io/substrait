// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating miscellaneous expression types.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::types;
use crate::string_util;

/// Parse an enum expression. Returns a description of said expression.
pub fn parse_enum(
    x: &substrait::expression::Enum,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse variant.
    let variant = proto_required_field!(x, y, enum_kind, |x, y| {
        match x {
            substrait::expression::r#enum::EnumKind::Specified(x) => {
                if x.is_empty() {
                    diagnostic!(y, Error, IllegalValue, "enum variant name cannot be empty");
                }
                Ok(Some(x.clone()))
            }
            substrait::expression::r#enum::EnumKind::Unspecified(_) => Ok(None),
        }
    })
    .1
    .flatten();

    // Describe node.
    if let Some(variant) = &variant {
        describe!(
            y,
            Misc,
            "Function option variant {}",
            string_util::as_ident_or_string(variant)
        );
    } else {
        describe!(y, Misc, "Default function option variant");
    }

    Ok(expressions::Expression::EnumVariant(variant))
}

/// Parse a typecast expression. Returns a description of said expression.
pub fn parse_cast(
    x: &substrait::expression::Cast,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse fields.
    let data_type = proto_required_field!(x, y, r#type, types::parse_type)
        .0
        .data_type();
    let input = proto_boxed_required_field!(x, y, input, expressions::parse_expression)
        .1
        .unwrap_or_default();
    let expression = expressions::Expression::Cast(data_type, Box::new(input));

    // TODO: check if this is a valid typecast.
    // FIXME: how?
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "typecast validation rules are not yet implemented"
    );

    // Describe node.
    describe!(y, Expression, "{}", expression);
    summary!(y, "Type conversion: {:#}", expression);
    Ok(expression)
}
