// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for types.

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::comment;
use crate::output::data_type;
use crate::output::data_type::ParameterInfo;
use crate::output::diagnostic;
use crate::output::extension;
use crate::parse::context;
use crate::parse::extensions;
use crate::string_util;

/// Parses a required nullability enum.
fn parse_required_nullability(
    x: &substrait::r#type::Nullability,
    _: &mut context::Context,
) -> diagnostic::Result<bool> {
    match x {
        substrait::r#type::Nullability::Nullable => Ok(true),
        substrait::r#type::Nullability::Required => Ok(false),
        substrait::r#type::Nullability::Unspecified => Err(cause!(
            IllegalValue,
            "nullability information is required in this context"
        )),
    }
}

/// Parses an optional type variation reference.
fn parse_type_variation_reference(
    x: &u32,
    y: &mut context::Context,
) -> diagnostic::Result<data_type::Variation> {
    if *x == 0 {
        Ok(None)
    } else {
        Some(extensions::simple::parse_type_variation_reference(x, y)).transpose()
    }
}

/// Parses an unsigned integer type parameter.
fn parse_integral_type_parameter(
    x: &i32,
    _: &mut context::Context,
) -> diagnostic::Result<data_type::Parameter> {
    Ok(u64::try_from(*x)
        .map_err(|_| cause!(IllegalValue, "integral type parameters cannot be negative"))?
        .into())
}

/// Macro for simple types, since they're all the same.
macro_rules! parse_simple_type {
    ($input:expr, $context:expr, $typ:ident) => {{
        // Parse fields.
        let nullable = proto_required_enum_field!(
            $input,
            $context,
            nullability,
            substrait::r#type::Nullability,
            parse_required_nullability
        )
        .1;
        let variation = proto_primitive_field!(
            $input,
            $context,
            type_variation_reference,
            parse_type_variation_reference
        )
        .1;

        // Convert to internal type object.
        let data_type = if let (Some(nullable), Some(variation)) = (nullable, variation) {
            data_type::DataType::new(
                data_type::Class::Simple(data_type::Simple::$typ),
                nullable,
                variation,
                vec![],
            )
            .map_err(|e| diagnostic!($context, Error, e))
            .unwrap_or_default()
        } else {
            Arc::default()
        };

        // Attach the type to the node.
        $context.set_data_type(data_type);

        Ok(())
    }};
}

/// Parses a boolean type.
pub fn parse_boolean(
    x: &substrait::r#type::Boolean,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Boolean)
}

/// Parses a i8 type.
pub fn parse_i8(x: &substrait::r#type::I8, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, I8)
}

/// Parses a i16 type.
pub fn parse_i16(x: &substrait::r#type::I16, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, I16)
}

/// Parses a i32 type.
pub fn parse_i32(x: &substrait::r#type::I32, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, I32)
}

/// Parses a i64 type.
pub fn parse_i64(x: &substrait::r#type::I64, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, I64)
}

/// Parses a fp32 type.
pub fn parse_fp32(x: &substrait::r#type::Fp32, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Fp32)
}

/// Parses a fp64 type.
pub fn parse_fp64(x: &substrait::r#type::Fp64, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Fp64)
}

/// Parses a string type.
pub fn parse_string(
    x: &substrait::r#type::String,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, String)
}

/// Parses a binary type.
pub fn parse_binary(
    x: &substrait::r#type::Binary,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Binary)
}

/// Parses a timestamp type.
pub fn parse_timestamp(
    x: &substrait::r#type::Timestamp,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Timestamp)
}

/// Parses a date type.
pub fn parse_date(x: &substrait::r#type::Date, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Date)
}

/// Parses a time type.
pub fn parse_time(x: &substrait::r#type::Time, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Time)
}

/// Parses a interval-year type.
pub fn parse_interval_year(
    x: &substrait::r#type::IntervalYear,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, IntervalYear)
}

/// Parses a interval-day type.
pub fn parse_interval_day(
    x: &substrait::r#type::IntervalDay,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, IntervalDay)
}

/// Parses a timestamp-tz type.
pub fn parse_timestamp_tz(
    x: &substrait::r#type::TimestampTz,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, TimestampTz)
}

/// Parses a UUID type.
pub fn parse_uuid(x: &substrait::r#type::Uuid, y: &mut context::Context) -> diagnostic::Result<()> {
    parse_simple_type!(x, y, Uuid)
}

/// Macro for compound types with just a length, since they're all the same.
macro_rules! parse_compound_type_with_length {
    ($input:expr, $context:expr, $typ:ident) => {{
        // Parse fields.
        let length =
            proto_primitive_field!($input, $context, length, parse_integral_type_parameter).1;
        let nullable = proto_required_enum_field!(
            $input,
            $context,
            nullability,
            substrait::r#type::Nullability,
            parse_required_nullability
        )
        .1;
        let variation = proto_primitive_field!(
            $input,
            $context,
            type_variation_reference,
            parse_type_variation_reference
        )
        .1;

        // Convert to internal type object.
        let data_type = if let (Some(length), Some(nullable), Some(variation)) =
            (length, nullable, variation)
        {
            data_type::DataType::new(
                data_type::Class::Compound(data_type::Compound::$typ),
                nullable,
                variation,
                vec![length],
            )
            .map_err(|e| diagnostic!($context, Error, e))
            .unwrap_or_default()
        } else {
            Arc::default()
        };

        // Attach the type to the node.
        $context.set_data_type(data_type);

        Ok(())
    }};
}

/// Parses a fixed-char type.
pub fn parse_fixed_char(
    x: &substrait::r#type::FixedChar,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_compound_type_with_length!(x, y, FixedChar)
}

/// Parses a varchar type.
pub fn parse_var_char(
    x: &substrait::r#type::VarChar,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_compound_type_with_length!(x, y, VarChar)
}

/// Parses a fixed-binary type.
pub fn parse_fixed_binary(
    x: &substrait::r#type::FixedBinary,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    parse_compound_type_with_length!(x, y, FixedBinary)
}

/// Parses a decimal type.
pub fn parse_decimal(
    x: &substrait::r#type::Decimal,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse fields.
    let precision = proto_primitive_field!(x, y, precision, parse_integral_type_parameter).1;
    let scale = proto_primitive_field!(x, y, scale, parse_integral_type_parameter).1;
    let nullable = proto_required_enum_field!(
        x,
        y,
        nullability,
        substrait::r#type::Nullability,
        parse_required_nullability
    )
    .1;
    let variation = proto_primitive_field!(
        x,
        y,
        type_variation_reference,
        parse_type_variation_reference
    )
    .1;

    // Convert to internal type object.
    let data_type = if let (Some(precision), Some(scale), Some(nullable), Some(variation)) =
        (precision, scale, nullable, variation)
    {
        data_type::DataType::new(
            data_type::Class::Compound(data_type::Compound::Decimal),
            nullable,
            variation,
            vec![precision, scale],
        )
        .map_err(|e| diagnostic!(y, Error, e))
        .unwrap_or_default()
    } else {
        Arc::default()
    };

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a struct type.
pub fn parse_struct(
    x: &substrait::r#type::Struct,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse fields.
    let types = proto_repeated_field!(x, y, types, parse_type)
        .0
        .iter()
        .map(|n| n.data_type.clone().unwrap_or_default().into())
        .collect();
    let nullable = proto_required_enum_field!(
        x,
        y,
        nullability,
        substrait::r#type::Nullability,
        parse_required_nullability
    )
    .1;
    let variation = proto_primitive_field!(
        x,
        y,
        type_variation_reference,
        parse_type_variation_reference
    )
    .1;

    // Convert to internal type object.
    let data_type = if let (Some(nullable), Some(variation)) = (nullable, variation) {
        data_type::DataType::new(
            data_type::Class::Compound(data_type::Compound::Struct),
            nullable,
            variation,
            types,
        )
        .map_err(|e| diagnostic!(y, Error, e))
        .unwrap_or_default()
    } else {
        Arc::default()
    };

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a list type.
pub fn parse_list(x: &substrait::r#type::List, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse fields.
    let element_type = proto_boxed_required_field!(x, y, r#type, parse_type)
        .0
        .data_type
        .clone()
        .unwrap_or_default();
    let nullable = proto_required_enum_field!(
        x,
        y,
        nullability,
        substrait::r#type::Nullability,
        parse_required_nullability
    )
    .1;
    let variation = proto_primitive_field!(
        x,
        y,
        type_variation_reference,
        parse_type_variation_reference
    )
    .1;

    // Convert to internal type object.
    let data_type = if let (Some(nullable), Some(variation)) = (nullable, variation) {
        data_type::DataType::new(
            data_type::Class::Compound(data_type::Compound::List),
            nullable,
            variation,
            vec![element_type.into()],
        )
        .map_err(|e| diagnostic!(y, Error, e))
        .unwrap_or_default()
    } else {
        Arc::default()
    };

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a map type.
pub fn parse_map(x: &substrait::r#type::Map, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse fields.
    let key_type = proto_boxed_required_field!(x, y, key, parse_type)
        .0
        .data_type
        .clone()
        .unwrap_or_default();
    let value_type = proto_boxed_required_field!(x, y, value, parse_type)
        .0
        .data_type
        .clone()
        .unwrap_or_default();
    let nullable = proto_required_enum_field!(
        x,
        y,
        nullability,
        substrait::r#type::Nullability,
        parse_required_nullability
    )
    .1;
    let variation = proto_primitive_field!(
        x,
        y,
        type_variation_reference,
        parse_type_variation_reference
    )
    .1;

    // Convert to internal type object.
    let data_type = if let (Some(nullable), Some(variation)) = (nullable, variation) {
        data_type::DataType::new(
            data_type::Class::Compound(data_type::Compound::Map),
            nullable,
            variation,
            vec![key_type.into(), value_type.into()],
        )
        .map_err(|e| diagnostic!(y, Error, e))
        .unwrap_or_default()
    } else {
        Arc::default()
    };

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a user-defined type.
pub fn parse_user_defined(x: &u32, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse fields.
    let user_type = extensions::simple::parse_type_reference(x, y)
        .map_err(|e| diagnostic!(y, Error, e))
        .ok();

    // Convert to internal type object.
    let data_type = if let Some(user_type) = user_type {
        data_type::DataType::new(
            data_type::Class::UserDefined(user_type),
            false,
            None,
            vec![],
        )
        .map_err(|e| diagnostic!(y, Error, e))
        .unwrap_or_default()
    } else {
        Arc::default()
    };

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a type kind.
pub fn parse_type_kind(
    x: &substrait::r#type::Kind,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    match x {
        substrait::r#type::Kind::Bool(x) => parse_boolean(x, y),
        substrait::r#type::Kind::I8(x) => parse_i8(x, y),
        substrait::r#type::Kind::I16(x) => parse_i16(x, y),
        substrait::r#type::Kind::I32(x) => parse_i32(x, y),
        substrait::r#type::Kind::I64(x) => parse_i64(x, y),
        substrait::r#type::Kind::Fp32(x) => parse_fp32(x, y),
        substrait::r#type::Kind::Fp64(x) => parse_fp64(x, y),
        substrait::r#type::Kind::String(x) => parse_string(x, y),
        substrait::r#type::Kind::Binary(x) => parse_binary(x, y),
        substrait::r#type::Kind::Timestamp(x) => parse_timestamp(x, y),
        substrait::r#type::Kind::Date(x) => parse_date(x, y),
        substrait::r#type::Kind::Time(x) => parse_time(x, y),
        substrait::r#type::Kind::IntervalYear(x) => parse_interval_year(x, y),
        substrait::r#type::Kind::IntervalDay(x) => parse_interval_day(x, y),
        substrait::r#type::Kind::TimestampTz(x) => parse_timestamp_tz(x, y),
        substrait::r#type::Kind::Uuid(x) => parse_uuid(x, y),
        substrait::r#type::Kind::FixedChar(x) => parse_fixed_char(x, y),
        substrait::r#type::Kind::Varchar(x) => parse_var_char(x, y),
        substrait::r#type::Kind::FixedBinary(x) => parse_fixed_binary(x, y),
        substrait::r#type::Kind::Decimal(x) => parse_decimal(x, y),
        substrait::r#type::Kind::Struct(x) => parse_struct(x, y),
        substrait::r#type::Kind::List(x) => parse_list(x, y),
        substrait::r#type::Kind::Map(x) => parse_map(x, y),
        substrait::r#type::Kind::UserDefinedTypeReference(x) => parse_user_defined(x, y),
    }
}

fn describe_type(y: &mut context::Context, data_type: &Arc<data_type::DataType>) {
    let mut brief = match &data_type.class() {
        data_type::Class::Simple(data_type::Simple::Boolean) => {
            summary!(y, "Values of this type can be either true or false.");
            String::from("boolean type")
        }
        data_type::Class::Simple(data_type::Simple::I8) => {
            summary!(
                y,
                "Implementations of this type must support all integers in \
                the range [-2^7, 2^7)."
            );
            String::from("8-bit signed integer type")
        }
        data_type::Class::Simple(data_type::Simple::I16) => {
            summary!(
                y,
                "Implementations of this type must support all integers in \
                the range [-2^15, 2^15)."
            );
            String::from("16-bit signed integer type")
        }
        data_type::Class::Simple(data_type::Simple::I32) => {
            summary!(
                y,
                "Implementations of this type must support all integers in \
                the range [-2^31, 2^31)."
            );
            String::from("32-bit signed integer type")
        }
        data_type::Class::Simple(data_type::Simple::I64) => {
            summary!(
                y,
                "Implementations of this type must support all integers in \
                the range [-2^63, 2^63)."
            );
            String::from("64-bit signed integer type")
        }
        data_type::Class::Simple(data_type::Simple::Fp32) => {
            summary!(
                y,
                "Implementations of this type must support a superset of the \
                values representable using IEEE 754 binary32."
            );
            String::from("single-precision float type")
        }
        data_type::Class::Simple(data_type::Simple::Fp64) => {
            summary!(
                y,
                "Implementations of this type must support a superset of the \
                values representable using IEEE 754 binary64."
            );
            String::from("double-precision float type")
        }
        data_type::Class::Simple(data_type::Simple::String) => {
            summary!(
                y,
                "Implementations of this type must support all strings \
                representable using UTF-8 encoding and up to 2^31-1 bytes of \
                storage."
            );
            String::from("Unicode string type")
        }
        data_type::Class::Simple(data_type::Simple::Binary) => {
            summary!(
                y,
                "Implementations of this type must support all byte strings \
                of up to 2^31-1 bytes in length."
            );
            String::from("Binary string type")
        }
        data_type::Class::Simple(data_type::Simple::Timestamp) => {
            summary!(
                y,
                "Implementations of this type must support all timestamps \
                within the range [1000-01-01 00:00:00.000000, \
                9999-12-31 23:59:59.999999] with microsecond precision. \
                Timezone information is however not encoded, so contextual \
                information would be needed to map the timestamp to a fixed \
                point in time."
            );
            String::from("Timezone-naive timestamp type")
        }
        data_type::Class::Simple(data_type::Simple::TimestampTz) => {
            summary!(
                y,
                "Implementations of this type must support all timestamps \
                within the range [1000-01-01 00:00:00.000000 UTC, \
                9999-12-31 23:59:59.999999 UTC] with microsecond precision."
            );
            String::from("Timezone-aware timestamp type")
        }
        data_type::Class::Simple(data_type::Simple::Date) => {
            summary!(
                y,
                "Implementations of this type must support all dates within \
                the range [1000-01-01, 9999-12-31]."
            );
            String::from("Date type")
        }
        data_type::Class::Simple(data_type::Simple::Time) => {
            summary!(
                y,
                "Implementations of this type must support all times of day \
                with microsecond precision, not counting leap seconds; that \
                is, any integer number of microseconds since the start of a \
                day in the range [0, 24*60*60*10^6]."
            );
            String::from("Time-of-day type")
        }
        data_type::Class::Simple(data_type::Simple::IntervalYear) => {
            // FIXME: the way this type is defined makes no sense; its
            // definition conflicts with the analog representations of at least
            // Arrow as specified on the website (assuming INTERVAL_MONTHS was
            // intended), and intuitively does not make sense either. The way
            // it's written, for example [10000y, -120000m] necessarily encodes
            // a semantically different value [0y, 0m], rather than that these
            // can just be aliases of each other. Wouldn't it be better to
            // define it as needing to represent all integer numbers of months
            // in the range [-120000, 120000]? If someone then really wants the
            // current semantics, they can just use
            //
            //   NSTRUCT<years: interval_year, months: interval_year>
            //
            // with some additional constraints. However, an implementation
            // that wants to encode this interval type as an integer number of
            // years plus an integer number of months still complies with the
            // [-120000, 120000] months requirement just fine.
            //
            // Renaming it to interval_month makes a lot more sense then too,
            // i.e. a signed interval with at least month precision and
            // +/- 10000 year range, and that's it.
            summary!(
                y,
                "Implementations of this type must support a range of any \
                combination of years and months that total less than or equal \
                to 10000 years. Each component can be specified as positive or \
                negative."
            );
            String::from("Year/month interval type")
        }
        data_type::Class::Simple(data_type::Simple::IntervalDay) => {
            // FIXME: see note for IntervalYear, making this
            // interval_microsecond, i.e. a signed interval with at least
            // microsecond precision and +/- 10000 year range.
            //
            // Worth noting in addition that 2^63 nanoseconds is a lot more
            // than 10000 years. It doesn't make much sense to me to use
            // I64 limits (for a different precision to boot) when all the
            // other limits are based around +/- 10000 years.
            summary!(
                y,
                "Implementations of this type must support a range of any \
                combination of [-365*10000, 365*10000] days and \
                [ceil(-2^63/1000), floor(2^63/1000)] integer microseconds."
            );
            String::from("Day/microsecond interval type")
        }
        data_type::Class::Simple(data_type::Simple::Uuid) => {
            summary!(
                y,
                "Implementations of this type must support 2^128 different \
                values, typically represented using the following hex format: \
                c48ffa9e-64f4-44cb-ae47-152b4e60e77b."
            );
            String::from("128-bit identifier type")
        }
        data_type::Class::Compound(data_type::Compound::FixedChar) => {
            let length = data_type
                .parameters()
                .get(0)
                .map(|x| x.to_string())
                .unwrap_or_else(|| String::from("?"));
            summary!(
                y,
                "Implementations of this type must support all unicode \
                strings with exactly {length} characters (i.e. code points). \
                Values shorter than that must be right-padded with spaces."
            );
            format!("Fixed-length ({length}) unicode string type")
        }
        data_type::Class::Compound(data_type::Compound::VarChar) => {
            let length = data_type
                .parameters()
                .get(0)
                .map(|x| x.to_string())
                .unwrap_or_else(|| String::from("?"));
            summary!(
                y,
                "Implementations of this type must support all unicode \
                strings with 0 to {length} characters (i.e. code points)."
            );
            format!("Variable-length ({length}) unicode string type")
        }
        data_type::Class::Compound(data_type::Compound::FixedBinary) => {
            let length = data_type
                .parameters()
                .get(0)
                .map(|x| x.to_string())
                .unwrap_or_else(|| String::from("?"));
            summary!(
                y,
                "Implementations of this type must support all binary \
                strings of exactly {length} bytes in length. Values shorter \
                than that must be right-padded with zero bytes."
            );
            format!("Fixed-length ({length}) binary string type")
        }
        data_type::Class::Compound(data_type::Compound::Decimal) => {
            let precision = data_type.int_parameter(0);
            let scale = data_type.int_parameter(1);
            let (p, i, s) = if let (Some(precision), Some(scale)) = (precision, scale) {
                (
                    precision.to_string(),
                    (precision - scale).to_string(),
                    scale.to_string(),
                )
            } else {
                (String::from("?"), String::from("?"), String::from("?"))
            };
            summary!(
                y,
                "Implementations of this type must support all decimal \
                numbers with {i} integer digits and {s} fractional digits \
                (precision = {p}, scale = {s})."
            );
            format!("Decimal number type with {i} integer and {s} fractional digits")
        }
        data_type::Class::Compound(data_type::Compound::Struct)
        | data_type::Class::Compound(data_type::Compound::NamedStruct) => {
            let n = data_type.parameters().len();
            if n == 1 {
                summary!(y, "Structure with one field.");
                String::from("Structure with one field")
            } else {
                summary!(y, "Structure with {n} fields.");
                format!("Structure with {n} fields")
            }
        }
        data_type::Class::Compound(data_type::Compound::List) => {
            let e = data_type
                .type_parameter(0)
                .map(|t| t.to_string())
                .unwrap_or_else(|| String::from("?"));
            summary!(
                y,
                "Implementations of this type must support all sequences of \
                0 to 2^31-1 {e} elements."
            );
            String::from("List type")
        }
        data_type::Class::Compound(data_type::Compound::Map) => {
            // FIXME: the definition in the spec is technically a multimap,
            // because it says nothing about key uniqueness, but that's
            // probably not intentional (how would references work, then?).
            // Also, unlike all the other types, there's no specified size
            // limit here. Assuming the other size limits are 2^31-1 for
            // Java compatibility, the same would need to apply here.
            let k = data_type
                .type_parameter(0)
                .map(|t| t.to_string())
                .unwrap_or_else(|| String::from("?"));
            let v = data_type
                .type_parameter(1)
                .map(|t| t.to_string())
                .unwrap_or_else(|| String::from("?"));
            summary!(
                y,
                "Implementations of this type must support any mapping from \
                {k} keys to {v} values, consisting of up to 2^31-1 key-value \
                pairs. No key uniqueness check is required on insertion, but \
                resolving the mapping for a key for which multiple values are \
                defined is undefined behavior."
            );
            String::from("Map type")
        }
        data_type::Class::UserDefined(u) => {
            summary!(y, "Extension type {u}.");
            if let Some(x) = &u.definition {
                y.push_summary(
                    comment::Comment::new()
                        .plain("Internal structure corresponds to:")
                        .lo(),
                );
                let mut first = true;
                for (name, class) in &x.structure {
                    if first {
                        first = false;
                    } else {
                        y.push_summary(comment::Comment::new().li());
                    }
                    summary!(y, "{}: {}", string_util::as_ident_or_string(name), class);
                }
                y.push_summary(comment::Comment::new().lc());
            }
            format!("Extension type {}", u.name)
        }
        data_type::Class::Unresolved => {
            summary!(
                y,
                "Failed to resolve information about this type due to \
                validation errors."
            );
            String::from("Unresolved type")
        }
    };
    if data_type.nullable() {
        brief += ", nullable";
        summary!(
            y,
            "Values of this type are optional, i.e. this type is nullable."
        );
    } else {
        summary!(
            y,
            "Values of this type are required, i.e. the type is not nullable."
        );
    }
    let variation = if let Some(u) = data_type.variation() {
        let mut variation = format!("This is the {u} variation of this type");
        if let Some(tv) = &u.definition {
            if tv.function_behavior == extension::FunctionBehavior::Inherits {
                variation +=
                    ", which behaves the same as the base type w.r.t. overload resolution.";
            } else {
                variation += ", which behaves as a separate type w.r.t. overload resolution.";
            }
        } else {
            variation += ".";
        }
        variation
    } else {
        String::from("This is the base variation of this type.")
    };
    summary!(y, "{}", variation);
    describe!(y, Type, "{}", brief);
}

/// Parses a type.
pub fn parse_type(x: &substrait::Type, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse fields.
    let data_type = proto_required_field!(x, y, kind, parse_type_kind)
        .0
        .data_type();

    // Describe the data type.
    describe_type(y, &data_type);

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Parses a named struct.
pub fn parse_named_struct(
    x: &substrait::NamedStruct,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse fields.
    proto_repeated_field!(x, y, names);
    let node = proto_required_field!(x, y, r#struct, parse_struct).0;

    // Try to apply the names to the data type.
    let data_type = match node.data_type().apply_field_names(&x.names) {
        Err(e) => {
            diagnostic!(y, Error, e);
            node.data_type()
        }
        Ok(data_type) => data_type,
    };

    // Describe the data type.
    describe_type(y, &data_type);

    // Attach the type to the node.
    y.set_data_type(data_type);

    Ok(())
}

/// Asserts that two types are equal, and returns the combined type, pushing
/// diagnostics if there is a mismatch. Warnings are used for field name
/// mismatches, errors are used for any other difference. If either type is
/// unresolved at any point in the tree, the other is returned. If both are
/// unresolved, base is returned.
fn assert_equal_internal(
    context: &mut context::Context,
    other: &Arc<data_type::DataType>,
    promote_other: bool,
    base: &Arc<data_type::DataType>,
    promote_base: bool,
    message: &str,
    path: &str,
) -> Arc<data_type::DataType> {
    if other.is_unresolved() {
        base.clone()
    } else if base.is_unresolved() {
        other.clone()
    } else {
        // Match base types.
        let base_types_match = match (other.class(), base.class()) {
            (
                data_type::Class::Compound(data_type::Compound::Struct),
                data_type::Class::Compound(data_type::Compound::NamedStruct),
            ) => true,
            (
                data_type::Class::Compound(data_type::Compound::NamedStruct),
                data_type::Class::Compound(data_type::Compound::Struct),
            ) => true,
            (a, b) => a == b,
        };
        if !base_types_match {
            diagnostic!(
                context,
                Error,
                TypeMismatch,
                "{message}: {} vs. {}{path}",
                other.class(),
                base.class()
            );

            // No sense in comparing parameters if the base type is already
            // different, so just return here.
            return base.clone();
        }

        // Match nullability.
        let nullable = match (other.nullable(), base.nullable()) {
            (true, false) => {
                if promote_base {
                    true
                } else {
                    diagnostic!(
                        context,
                        Error,
                        TypeMismatch,
                        "{message}: nullable vs. required{path}"
                    );
                    false
                }
            }
            (false, true) => {
                if !promote_other {
                    diagnostic!(
                        context,
                        Error,
                        TypeMismatch,
                        "{message}: required vs. nullable{path}"
                    );
                }
                true
            }
            (_, x) => x,
        };

        // Match variations.
        match (other.variation(), base.variation()) {
            (Some(other), Some(base)) => {
                if base != other {
                    diagnostic!(
                        context,
                        Error,
                        TypeMismatch,
                        "{message}: variation {other} vs. {base}{path}"
                    );
                }
            }
            (Some(other), None) => diagnostic!(
                context,
                Error,
                TypeMismatch,
                "{message}: variation {other} vs. no variation{path}"
            ),
            (None, Some(base)) => diagnostic!(
                context,
                Error,
                TypeMismatch,
                "{message}: no variation vs. variation {base}{path}"
            ),
            (None, None) => {}
        }

        // Match parameter count.
        let other_len = other.parameters().len();
        let base_len = base.parameters().len();
        if other_len != base_len {
            diagnostic!(
                context,
                Error,
                TypeMismatch,
                "{message}: {other_len} parameters vs. {base_len} parameters{path}"
            );
            return base.clone();
        }

        // Now match the parameters. We call ourselves recursively for each
        // type parameter, using the combined type to form the new type
        // parameter, such that information present in only one of the
        // parameters ends up in the final parameter, regardless of which
        // it is.
        let parameters = other
            .parameters()
            .iter()
            .zip(base.parameters().iter())
            .enumerate()
            .map(|(index, (other_param, base_param))| {
                let path_element = base_param
                    .get_name()
                    .or_else(|| other_param.get_name())
                    .map(String::from)
                    .or_else(|| base.class().parameter_name(index))
                    .unwrap_or_else(|| String::from("!"));
                let path = if path.is_empty() {
                    format!(" on parameter path {path_element}")
                } else {
                    format!("{path}.{path_element}")
                };
                match (other_param, base_param) {
                    (data_type::Parameter::Type(other), data_type::Parameter::Type(base)) => {
                        data_type::Parameter::Type(assert_equal_internal(
                            context,
                            other,
                            promote_other,
                            base,
                            promote_base,
                            message,
                            &path,
                        ))
                    }
                    (
                        data_type::Parameter::Type(other),
                        data_type::Parameter::NamedType(name, base),
                    ) => data_type::Parameter::NamedType(
                        name.clone(),
                        assert_equal_internal(
                            context,
                            other,
                            promote_other,
                            base,
                            promote_base,
                            message,
                            &path,
                        ),
                    ),
                    (
                        data_type::Parameter::NamedType(name, other),
                        data_type::Parameter::Type(base),
                    ) => data_type::Parameter::NamedType(
                        name.clone(),
                        assert_equal_internal(
                            context,
                            other,
                            promote_other,
                            base,
                            promote_base,
                            message,
                            &path,
                        ),
                    ),
                    (
                        data_type::Parameter::NamedType(other_name, other),
                        data_type::Parameter::NamedType(base_name, base),
                    ) => {
                        if other_name != base_name {
                            diagnostic!(
                                context,
                                Warning,
                                TypeMismatch,
                                "{message}: field name {} vs. {}{path}",
                                string_util::as_ident_or_string(&other_name),
                                string_util::as_ident_or_string(&base_name)
                            );
                        }
                        data_type::Parameter::NamedType(
                            base_name.clone(),
                            assert_equal_internal(
                                context,
                                other,
                                promote_other,
                                base,
                                promote_base,
                                message,
                                &path,
                            ),
                        )
                    }
                    (other, base) => {
                        if other != base {
                            diagnostic!(
                                context,
                                Error,
                                TypeMismatch,
                                "{message}: {other} vs. {base}{path}"
                            );
                        }
                        base.clone()
                    }
                }
            })
            .collect();

        // If either type is a named struct, the result should be a named
        // struct, since we'll have taken the field names from the type that
        // has them in the loop above.
        let class = match (other.class(), base.class()) {
            (
                data_type::Class::Compound(data_type::Compound::Struct),
                data_type::Class::Compound(data_type::Compound::NamedStruct),
            ) => data_type::Class::Compound(data_type::Compound::NamedStruct),
            (
                data_type::Class::Compound(data_type::Compound::NamedStruct),
                data_type::Class::Compound(data_type::Compound::Struct),
            ) => data_type::Class::Compound(data_type::Compound::NamedStruct),
            (a, _) => a.clone(),
        };

        data_type::DataType::new(class, nullable, base.variation().clone(), parameters)
            .expect("assert_equal() failed to correctly combine types")
    }
}

/// Asserts that two types are equal, and returns the combined type, pushing
/// diagnostics if there is a mismatch. Warnings are used for field name
/// mismatches, errors are used for any other difference. If either type is
/// unresolved at any point in the tree, the other is returned. If both are
/// unresolved, base is returned.
pub fn assert_equal<S: AsRef<str>>(
    context: &mut context::Context,
    other: &Arc<data_type::DataType>,
    base: &Arc<data_type::DataType>,
    message: S,
) -> Arc<data_type::DataType> {
    assert_equal_internal(context, other, false, base, false, message.as_ref(), "")
}

/// Like assert_equal, but will first promote either input to try to make them
/// match.
pub fn promote_and_assert_equal<S: AsRef<str>>(
    context: &mut context::Context,
    other: &Arc<data_type::DataType>,
    base: &Arc<data_type::DataType>,
    message: S,
) -> Arc<data_type::DataType> {
    assert_equal_internal(context, other, true, base, true, message.as_ref(), "")
}
