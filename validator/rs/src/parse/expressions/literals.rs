// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating literals.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::types;
use crate::string_util;
use crate::string_util::Describe;
use std::sync::Arc;

/// The value of a literal, not including type information.
#[derive(Clone)]
enum LiteralValue {
    /// May be used for any nullable type.
    Null,

    /// May be used only for booleans.
    Boolean(bool),

    /// May be used only for I8, I16, I32, I64, Timestamp, TimestampTz, Date, and Time.
    Integer(i64),

    /// May be used only for Fp32 and Fp64.
    Float(f64),

    /// May be used only for decimals and UUIDs.
    Data16(i128),

    /// May be used only for strings, FixedChars, and VarChars.
    String(String),

    /// May be used only for binary and FixedBinary.
    Binary(Vec<u8>),

    /// May be used only for IntervalYearToMonth and IntervalDayToSecond.
    Interval(i32, i32),

    /// May be used only for structs and lists.
    Items(Vec<Literal>),

    /// May be used only for maps.
    Pairs(Vec<(Literal, Literal)>),
}

impl Default for LiteralValue {
    fn default() -> Self {
        LiteralValue::Null
    }
}

/// A complete literal, including type information.
#[derive(Clone, Default)]
pub struct Literal {
    /// The value of the literal.
    value: LiteralValue,

    /// The data type of the literal. LiteralValue must be a valid instance of
    /// this.
    data_type: Arc<data_type::DataType>,
}

/// Converts a value in microseconds since the epoch to a chrono::NaiveDateTime.
fn to_date_time(micros: i64) -> diagnostic::Result<chrono::NaiveDateTime> {
    let secs = micros.div_euclid(1_000_000);
    let nsecs = ((micros.rem_euclid(1_000_000)) * 1000) as u32;
    chrono::NaiveDateTime::from_timestamp_opt(secs, nsecs).ok_or(ecause!(
        ExpressionIllegalLiteralValue,
        "timestamp out of range"
    ))
}

/// Converts a value in microseconds since the epoch to a string.
fn to_date_time_str(micros: i64, fmt: &str) -> String {
    to_date_time(micros)
        .map(|x| x.format(fmt).to_string())
        .unwrap_or_else(|_| String::from("?"))
}

impl Literal {
    /// Shorthand for a new null literal.
    pub fn new_null(data_type: Arc<data_type::DataType>) -> Literal {
        Literal {
            value: LiteralValue::Null,
            data_type,
        }
    }

    /// Shorthand for a new simple literal.
    fn new_simple(
        value: LiteralValue,
        simple: data_type::Simple,
        nullable: bool,
    ) -> diagnostic::Result<Literal> {
        Ok(Literal {
            value,
            data_type: data_type::DataType::new(
                data_type::Class::Simple(simple),
                nullable,
                None,
                vec![],
            )?,
        })
    }

    /// Shorthand for a new compound literal.
    fn new_compound<T: Into<data_type::Parameter>>(
        value: LiteralValue,
        compound: data_type::Compound,
        nullable: bool,
        args: Vec<T>,
    ) -> diagnostic::Result<Literal> {
        Ok(Literal {
            value,
            data_type: data_type::DataType::new(
                data_type::Class::Compound(compound),
                nullable,
                None,
                args.into_iter().map(|x| x.into()).collect(),
            )?,
        })
    }

    /// Returns the data type of this literal.
    pub fn data_type(&self) -> &Arc<data_type::DataType> {
        &self.data_type
    }
}

impl Describe for Literal {
    /// Represents the value of this literal with some size limit. The size
    /// limit very roughly corresponds to a number of characters, but this is
    /// purely a heuristic thing.
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        match &self.value {
            LiteralValue::Null => {
                if self.data_type.is_unresolved() {
                    write!(f, "!")
                } else {
                    write!(f, "null")
                }
            }
            LiteralValue::Boolean(true) => write!(f, "true"),
            LiteralValue::Boolean(false) => write!(f, "false"),
            LiteralValue::Integer(i) => match self.data_type.class() {
                data_type::Class::Simple(data_type::Simple::I8) => write!(f, "{i}i8"),
                data_type::Class::Simple(data_type::Simple::I16) => write!(f, "{i}i16"),
                data_type::Class::Simple(data_type::Simple::I32) => write!(f, "{i}i32"),
                data_type::Class::Simple(data_type::Simple::I64) => write!(f, "{i}i64"),
                data_type::Class::Simple(data_type::Simple::Timestamp) => {
                    write!(f, "{}", to_date_time_str(*i, "%Y-%m-%d %H:%M:%S%.6f"))
                }
                data_type::Class::Simple(data_type::Simple::TimestampTz) => {
                    write!(f, "{} UTC", to_date_time_str(*i, "%Y-%m-%d %H:%M:%S%.6f"))
                }
                data_type::Class::Simple(data_type::Simple::Date) => {
                    write!(
                        f,
                        "{}",
                        to_date_time_str(i.saturating_mul(24 * 60 * 60 * 1_000_000), "%Y-%m-%d")
                    )
                }
                data_type::Class::Simple(data_type::Simple::Time) => {
                    write!(f, "{}", to_date_time_str(*i, "%H:%M:%S%.6f"))
                }
                _ => write!(f, "{i}"),
            },
            LiteralValue::Float(v) => {
                let max = std::cmp::min(std::cmp::max(3, limit.chars()), 10);
                write!(f, "{:3.1$}", float_pretty_print::PrettyPrintFloat(*v), max)
            }
            LiteralValue::Data16(d) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Decimal) => {
                    if let Some(scale) = self.data_type.int_parameter(1) {
                        if d < &0 {
                            write!(f, "-")?;
                        }
                        let d = d.abs() as u128;
                        let s = 10u128.pow(scale as u32);
                        if self
                            .data_type
                            .int_parameter(0)
                            .map(|precision| scale < precision)
                            .unwrap_or(true)
                        {
                            write!(f, "{0}", d.div_euclid(s))?;
                        }
                        write!(f, ".")?;
                        if scale > 0 {
                            write!(f, "{0:01$}", d.rem_euclid(s), scale as usize)?;
                        }
                        Ok(())
                    } else {
                        string_util::describe_binary(f, &d.to_le_bytes(), limit)
                    }
                }
                data_type::Class::Simple(data_type::Simple::Uuid) => {
                    let b = d.to_ne_bytes();
                    write!(
                        f,
                        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
                    )
                }
                _ => string_util::describe_binary(f, &d.to_le_bytes(), limit),
            },
            LiteralValue::String(s) => string_util::describe_string(f, s, limit),
            LiteralValue::Binary(b) => string_util::describe_binary(f, b, limit),
            LiteralValue::Interval(a, b) => match self.data_type.class() {
                data_type::Class::Simple(data_type::Simple::IntervalYear) => {
                    write!(f, "{a}y{b:+}m")
                }
                data_type::Class::Simple(data_type::Simple::IntervalDay) => write!(f, "{a}d{b:+}s"),
                _ => write!(f, "({a}, {b})"),
            },
            LiteralValue::Items(x) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Struct) => {
                    write!(f, "(")?;
                    string_util::describe_sequence(f, x, limit, 20, |f, value, index, limit| {
                        write!(f, ".{index}: ")?;
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::NamedStruct) => {
                    write!(f, "(")?;
                    string_util::describe_sequence(f, x, limit, 20, |f, value, index, limit| {
                        if let Some(name) = self
                            .data_type
                            .parameters()
                            .get(index)
                            .and_then(|x| x.get_name())
                        {
                            write!(f, ".{}: ", string_util::as_ident_or_string(name))?;
                        } else {
                            write!(f, ".{index}: ")?;
                        }
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::List) => {
                    write!(f, "[")?;
                    string_util::describe_sequence(f, x, limit, 20, |f, value, _, limit| {
                        value.describe(f, limit)
                    })?;
                    write!(f, "]")
                }
                _ => {
                    write!(f, "(")?;
                    string_util::describe_sequence(f, x, limit, 20, |f, value, _, limit| {
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
            },
            LiteralValue::Pairs(x) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Map) => {
                    write!(f, "{{")?;
                    string_util::describe_sequence(
                        f,
                        x,
                        limit,
                        40,
                        |f, (key, value), _, limit| {
                            let (key_limit, value_limit) = limit.split(20);
                            key.describe(f, key_limit)?;
                            write!(f, ": ")?;
                            value.describe(f, value_limit)
                        },
                    )?;
                    write!(f, "}}")
                }
                _ => {
                    write!(f, "(")?;
                    string_util::describe_sequence(
                        f,
                        x,
                        limit,
                        40,
                        |f, (key, value), _, limit| {
                            write!(f, "(")?;
                            let (key_limit, value_limit) = limit.split(20);
                            key.describe(f, key_limit)?;
                            write!(f, ": ")?;
                            value.describe(f, value_limit)?;
                            write!(f, ")")
                        },
                    )?;
                    write!(f, ")")
                }
            },
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Parses a boolean literal.
fn parse_boolean(
    x: &bool,
    _y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Boolean(*x),
        data_type::Simple::Boolean,
        nullable,
    )
}

/// Parses an i8 literal.
fn parse_i8(x: &i32, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    let x = i8::try_from(*x)
        .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i8 value out of range"))?;
    Literal::new_simple(
        LiteralValue::Integer(x as i64),
        data_type::Simple::I8,
        nullable,
    )
}

/// Parses an i16 literal.
fn parse_i16(x: &i32, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    let x = i16::try_from(*x)
        .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i16 value out of range"))?;
    Literal::new_simple(
        LiteralValue::Integer(x as i64),
        data_type::Simple::I16,
        nullable,
    )
}

/// Parses an i32 literal.
fn parse_i32(x: &i32, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Integer(*x as i64),
        data_type::Simple::I32,
        nullable,
    )
}

/// Parses an i64 literal.
fn parse_i64(x: &i64, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    Literal::new_simple(LiteralValue::Integer(*x), data_type::Simple::I64, nullable)
}

/// Parses an fp32 literal.
fn parse_fp32(x: &f32, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Float(*x as f64),
        data_type::Simple::Fp32,
        nullable,
    )
}

/// Parses an fp64 literal.
fn parse_fp64(x: &f64, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    Literal::new_simple(LiteralValue::Float(*x), data_type::Simple::Fp64, nullable)
}

/// Parses a string literal.
fn parse_string(x: &str, _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::String(x.to_string()),
        data_type::Simple::String,
        nullable,
    )
}

/// Parses a binary literal.
fn parse_binary(
    x: &[u8],
    _y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Binary(x.to_owned()),
        data_type::Simple::Binary,
        nullable,
    )
}

/// Parses a timestamp literal.
fn parse_timestamp(
    x: &i64,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let dt = to_date_time(*x)?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "timestamp out of range 1000-01-01 to 9999-12-31"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::Timestamp,
        nullable,
    )
}

/// Parses a UTC timestamp literal.
fn parse_timestamp_tz(
    x: &i64,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let dt = to_date_time(*x)?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "timestamp out of range 1000-01-01 UTC to 9999-12-31 UTC"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::TimestampTz,
        nullable,
    )
}

/// Parses a date literal.
fn parse_date(x: &i32, y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    let dt = to_date_time((*x as i64).saturating_mul(24 * 60 * 60 * 1_000_000))?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "date out of range 1000-01-01 UTC to 9999-12-31 UTC"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x as i64),
        data_type::Simple::Date,
        nullable,
    )
}

/// Parses a time literal.
fn parse_time(x: &i64, y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    if *x < 0 || *x >= 24 * 60 * 60 * 1_000_000 {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "time of day out of range 00:00:00.000000 to 23:59:59.999999"
        );
    }
    Literal::new_simple(LiteralValue::Integer(*x), data_type::Simple::Time, nullable)
}

/// Parses a year to month interval literal.
fn parse_interval_year_to_month(
    x: &substrait::expression::literal::IntervalYearToMonth,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    // FIXME: see FIXME for associated type.
    proto_primitive_field!(x, y, years, |x, _| {
        if *x < -10000 || *x > 10000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "year count out of range -10000 to 10000"
            ))
        } else {
            Ok(())
        }
    });
    proto_primitive_field!(x, y, months, |x, _| {
        if *x < -120000 || *x > 120000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "month count out of range -120000 to 120000"
            ))
        } else {
            Ok(())
        }
    });
    let months = x.months.saturating_add(x.years.saturating_mul(12));
    if months < -120000 || months > 120000 {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "combined interval out of range -10000 to 10000 years"
        );
    }
    Literal::new_simple(
        LiteralValue::Interval(x.years, x.months),
        data_type::Simple::IntervalYear,
        nullable,
    )
}

/// Parses a day to second interval literal.
fn parse_interval_day_to_second(
    x: &substrait::expression::literal::IntervalDayToSecond,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    // FIXME: see FIXME for associated type.
    proto_primitive_field!(x, y, days, |x, _| {
        if *x < -3650000 || *x > 3650000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "day count out of range -3_650_000 to 3_650_000"
            ))
        } else {
            Ok(())
        }
    });

    // FIXME: according to the docs, day to second supports microsecond
    // precision. The literal doesn't. The i32 seconds also doesn't
    // support the full specified range (but that range is weird
    // anyway).
    proto_primitive_field!(x, y, seconds);
    Literal::new_simple(
        LiteralValue::Interval(x.days, x.seconds),
        data_type::Simple::IntervalDay,
        nullable,
    )
}

/// Parses a UUID literal.
fn parse_uuid(x: &[u8], _y: &mut context::Context, nullable: bool) -> diagnostic::Result<Literal> {
    if let Ok(x) = x.try_into() {
        Literal::new_simple(
            LiteralValue::Data16(i128::from_ne_bytes(x)),
            data_type::Simple::Uuid,
            nullable,
        )
    } else {
        Err(cause!(
            ExpressionIllegalLiteralValue,
            "uuid literals must be 16 bytes in length, got {}",
            x.len()
        ))
    }
}

/// Parses a fixed-length string literal.
fn parse_fixed_char(
    x: &str,
    _y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    Literal::new_compound(
        LiteralValue::String(x.to_string()),
        data_type::Compound::FixedChar,
        nullable,
        vec![x.len() as u64],
    )
}

/// Parses a variable-length string literal.
fn parse_var_char(
    x: &substrait::expression::literal::VarChar,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    proto_primitive_field!(x, y, length);
    let len = x.length as usize;
    proto_primitive_field!(x, y, value, |x, _| {
        if x.len() > len {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "varchar literal value is longer than specified length"
            ))
        } else {
            Ok(())
        }
    });
    Literal::new_compound(
        LiteralValue::String(x.value.clone()),
        data_type::Compound::VarChar,
        nullable,
        vec![len as u64],
    )
}

/// Parses a fixed-length binary literal.
fn parse_fixed_binary(
    x: &[u8],
    _y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    Literal::new_compound(
        LiteralValue::Binary(x.to_owned()),
        data_type::Compound::FixedBinary,
        nullable,
        vec![x.len() as u64],
    )
}

/// Parses a decimal literal.
fn parse_decimal(
    x: &substrait::expression::literal::Decimal,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    proto_primitive_field!(x, y, precision, |x, _| {
        if *x < 0 {
            Err(cause!(
                IllegalValue,
                "negative type parameters are not supported"
            ))
        } else {
            Ok(())
        }
    });
    proto_primitive_field!(x, y, scale);
    let val = proto_primitive_field!(x, y, value, |x, _| {
        if let Ok(x) = (&x[..]).try_into() {
            Ok(i128::from_le_bytes(x))
        } else {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "decimal literals must be 16 bytes in length, got {}",
                x.len()
            ))
        }
    })
    .1;
    let precision = u64::try_from(x.precision).unwrap_or_default();
    let scale = u64::try_from(x.scale).unwrap_or_default();

    if let Some(val) = val {
        let range = 10i128.saturating_pow(precision.try_into().unwrap_or_default());
        if val >= range || val <= -range {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "decimal value is out of range for specificied precision and scale"
            ))
        } else {
            Literal::new_compound(
                LiteralValue::Data16(val),
                data_type::Compound::Decimal,
                nullable,
                vec![precision, scale],
            )
        }
    } else {
        Ok(Literal::default())
    }
}

/// Parses a struct literal.
fn parse_struct_int(
    x: &substrait::expression::literal::Struct,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let (values, types): (Vec<_>, Vec<_>) = proto_repeated_field!(x, y, fields, parse_literal)
        .1
        .into_iter()
        .map(|x| {
            let x = x.unwrap_or_default();
            let data_type = x.data_type.clone();
            (x, data_type)
        })
        .unzip();
    Literal::new_compound(
        LiteralValue::Items(values),
        data_type::Compound::Struct,
        nullable,
        types,
    )
}

/// Parses a struct literal.
pub fn parse_struct(
    x: &substrait::expression::literal::Struct,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let literal = parse_struct_int(x, y, nullable)?;
    y.set_data_type(literal.data_type().clone());
    Ok(literal)
}

/// Parses a list literal.
fn parse_list(
    x: &substrait::expression::literal::List,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let values: Vec<_> = proto_required_repeated_field!(x, y, values, parse_literal)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    if values.is_empty() {
        comment!(
            y,
            "At least one list element is required to derive type. Use EmptyList instead."
        );
    }
    let mut data_type = Arc::default();
    for (index, value) in values.iter().enumerate() {
        data_type = types::assert_equal(
            y,
            value.data_type(),
            &data_type,
            format!("unexpected type for index {index}"),
        );
    }
    Literal::new_compound(
        LiteralValue::Items(values),
        data_type::Compound::List,
        nullable,
        vec![data_type],
    )
}

/// Parses a map literal.
fn parse_map(
    x: &substrait::expression::literal::Map,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    let values: Vec<_> = proto_required_repeated_field!(x, y, key_values, |x, y| {
        let key = proto_required_field!(x, y, key, parse_literal)
            .1
            .unwrap_or_default();
        let value = proto_required_field!(x, y, value, parse_literal)
            .1
            .unwrap_or_default();
        Ok((key, value))
    })
    .1
    .into_iter()
    .map(|x| x.unwrap_or_default())
    .collect();
    if values.is_empty() {
        comment!(
            y,
            "At least one key-value pair is required to derive types. Use EmptyMap instead."
        );
    }
    let mut key_type = Arc::default();
    let mut value_type = Arc::default();
    for (index, value) in values.iter().enumerate() {
        key_type = types::assert_equal(
            y,
            value.0.data_type(),
            &key_type,
            format!("unexpected key type for index {index}"),
        );
        value_type = types::assert_equal(
            y,
            value.1.data_type(),
            &value_type,
            format!("unexpected value type for index {index}"),
        );
    }
    Literal::new_compound(
        LiteralValue::Pairs(values),
        data_type::Compound::Map,
        nullable,
        vec![key_type, value_type],
    )
}

/// Parses an empty list literal.
fn parse_empty_list(
    x: &substrait::r#type::List,
    y: &mut context::Context,
    _nullable: bool,
) -> diagnostic::Result<Literal> {
    // FIXME: nullability is redundantly specified, and the type
    // variation reference would be if it had gotten the same
    // treatment as nullability. Why doesn't EmptyList just map to only
    // the element data type?
    types::parse_list(x, y)?;
    Ok(Literal {
        value: LiteralValue::Items(vec![]),
        data_type: y.data_type(),
    })
}

/// Parses an empty map literal.
fn parse_empty_map(
    x: &substrait::r#type::Map,
    y: &mut context::Context,
    _nullable: bool,
) -> diagnostic::Result<Literal> {
    // FIXME: same note as for EmptyList.
    types::parse_map(x, y)?;
    Ok(Literal {
        value: LiteralValue::Pairs(vec![]),
        data_type: y.data_type(),
    })
}

/// Parses a null literal.
fn parse_null(
    x: &substrait::Type,
    y: &mut context::Context,
    _nullable: bool,
) -> diagnostic::Result<Literal> {
    // FIXME: same note as for EmptyList.
    types::parse_type(x, y)?;
    let data_type = y.data_type();
    if !data_type.nullable() && !data_type.is_unresolved() {
        Err(cause!(
            TypeMismatch,
            "type of null literal must be nullable"
        ))
    } else {
        Ok(Literal {
            value: LiteralValue::Null,
            data_type: y.data_type(),
        })
    }
}

/// Parse a literal value. Returns the parsed literal.
fn parse_literal_type(
    x: &substrait::expression::literal::LiteralType,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    use substrait::expression::literal::LiteralType;
    match x {
        LiteralType::Boolean(x) => parse_boolean(x, y, nullable),
        LiteralType::I8(x) => parse_i8(x, y, nullable),
        LiteralType::I16(x) => parse_i16(x, y, nullable),
        LiteralType::I32(x) => parse_i32(x, y, nullable),
        LiteralType::I64(x) => parse_i64(x, y, nullable),
        LiteralType::Fp32(x) => parse_fp32(x, y, nullable),
        LiteralType::Fp64(x) => parse_fp64(x, y, nullable),
        LiteralType::String(x) => parse_string(x, y, nullable),
        LiteralType::Binary(x) => parse_binary(x, y, nullable),
        LiteralType::Timestamp(x) => parse_timestamp(x, y, nullable),
        LiteralType::TimestampTz(x) => parse_timestamp_tz(x, y, nullable),
        LiteralType::Date(x) => parse_date(x, y, nullable),
        LiteralType::Time(x) => parse_time(x, y, nullable),
        LiteralType::IntervalYearToMonth(x) => parse_interval_year_to_month(x, y, nullable),
        LiteralType::IntervalDayToSecond(x) => parse_interval_day_to_second(x, y, nullable),
        LiteralType::Uuid(x) => parse_uuid(x, y, nullable),
        LiteralType::FixedChar(x) => parse_fixed_char(x, y, nullable),
        LiteralType::VarChar(x) => parse_var_char(x, y, nullable),
        LiteralType::FixedBinary(x) => parse_fixed_binary(x, y, nullable),
        LiteralType::Decimal(x) => parse_decimal(x, y, nullable),
        LiteralType::Struct(x) => parse_struct_int(x, y, nullable),
        LiteralType::List(x) => parse_list(x, y, nullable),
        LiteralType::Map(x) => parse_map(x, y, nullable),
        LiteralType::EmptyList(x) => parse_empty_list(x, y, nullable),
        LiteralType::EmptyMap(x) => parse_empty_map(x, y, nullable),
        LiteralType::Null(x) => parse_null(x, y, nullable),
    }
}

/// Parse a literal value. Returns the parsed literal.
pub fn parse_literal(
    x: &substrait::expression::Literal,
    y: &mut context::Context,
) -> diagnostic::Result<Literal> {
    // Parse type parameters that apply to all literals (except empty objects
    // and null...).
    if !matches!(
        x.literal_type,
        Some(substrait::expression::literal::LiteralType::EmptyList(_))
            | Some(substrait::expression::literal::LiteralType::EmptyMap(_))
            | Some(substrait::expression::literal::LiteralType::Null(_))
    ) {
        // FIXME: why isn't the nullability enum used here? Especially
        // considering nullability here actually should be unspecified when
        // above match yields false, while it must be specified everywhere
        // else. Better yet, change the semantics as described in the other
        // fixmes such that it is always mandatory everywhere, and then use
        // a boolean everywhere? If the point of the enum is to allow types
        // to be "partially unresolved," then the type system is pretty
        // fundamentally broken, since overload resolution depends on it.
        proto_primitive_field!(x, y, nullable);

        // FIXME: why would literals not support type variations? Feels like
        // there should be a type variation reference here.
    } else {
        // FIXME: this is all very ugly. Since all types can be made nullable
        // anyway, why isn't the nullability field taken out of the type kind
        // for types as well? Then the "empty" values can just refer to the
        // type kind rather than the whole type message, and the problem would
        // be solved. Likewise, I don't see why type variations should get
        // special treatment in the sense that (currently) user-defined types
        // can't also have variations. Why explicitly disallow that?
        proto_primitive_field!(x, y, nullable, |x, y| {
            // Send diagnostic only when x is not set to its default value,
            // since the default value is indistinguishable from unspecified.
            if *x {
                diagnostic!(
                    y,
                    Info,
                    RedundantField,
                    "this field is inoperative for empty lists, empty maps, and null."
                );
            } else {
                comment!(
                    y,
                    "This field is inoperative for empty lists, empty maps, and null."
                );
            }
            Ok(())
        });
    }

    // Parse the literal value.
    let literal = proto_required_field!(x, y, literal_type, parse_literal_type, x.nullable)
        .1
        .unwrap_or_default();

    // Describe node.
    y.set_data_type(literal.data_type().clone());
    describe!(y, Expression, "{}", literal);
    summary!(
        y,
        "Literal of type {:#} with value {:#}",
        literal.data_type(),
        literal
    );
    Ok(literal)
}
