// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating literals.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::types;
use crate::string_util;
use std::sync::Arc;

/// The value of a literal, not including type information.
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
#[derive(Default)]
pub struct Literal {
    /// The value of the literal.
    value: LiteralValue,

    /// The data type of the literal. LiteralValue must be a valid instance of
    /// this.
    data_type: Arc<data_type::DataType>,
}

/// If only N items of a sequence longer than N can be printed, print this many
/// elements from the front and this many from the back. The two returned
/// values always sum to the input. For small values, returns:
///
///  - 0 -> (0, 0) because it's the only valid option;
///  - 1 -> (1, 0) because the start of a sequence is usually more interesting
///    than the end;
///  - 2 -> (1, 1) because the start and end of a sequence are probably the
///    most interesting values.
///
/// For N -> infinity, the 2x as many elements are printed on the left as on
/// the right.
///
/// Note that this is all just heuristics. Anything goes for where the split is
/// made.
fn heuristic_split(n: usize) -> (usize, usize) {
    let n_right = (n + 1) / 3;
    let n_left = n - n_right;
    (n_left, n_right)
}

/// Represent data as a quoted string. If the string is too long, abbreviate
/// it. limit specifies the rough resulting string length that is considered
/// to be "too long."
fn repr_string(f: &mut std::fmt::Formatter<'_>, data: &str, limit: usize) -> std::fmt::Result {
    // ~1 characters per character (limiting case), so print limit characters.
    let n = limit;

    if data.len() > n {
        let (l, r) = heuristic_split(n);
        write!(
            f,
            "{}..{}",
            string_util::as_quoted_string(&data[..l]),
            string_util::as_quoted_string(&data[data.len() - r..])
        )
    } else {
        write!(f, "{}", string_util::as_quoted_string(data))
    }
}

/// Represent data as a complete hexdump.
fn repr_binary_all(f: &mut std::fmt::Formatter<'_>, data: &[u8]) -> std::fmt::Result {
    for byte in data {
        write!(f, "{byte:08X}")?;
    }
    Ok(())
}

/// Represent data as a hexdump. If the resulting dump is too long, abbreviate
/// it. limit specifies the rough resulting string length that is considered
/// to be "too long."
fn repr_binary(f: &mut std::fmt::Formatter<'_>, data: &[u8], limit: usize) -> std::fmt::Result {
    // 2 characters per byte, so divide limit by 2 to get number of bytes to
    // be printed.
    let n = limit / 2;

    if data.len() > n {
        let (l, r) = heuristic_split(n);
        repr_binary_all(f, &data[..l])?;
        write!(f, "..")?;
        repr_binary_all(f, &data[data.len() - r..])
    } else {
        repr_binary_all(f, data)
    }
}

/// Represent the given sequence completely.
fn repr_sequence_all<T, F>(
    f: &mut std::fmt::Formatter<'_>,
    values: &[T],
    offset: usize,
    el_limit: usize,
    repr: &F,
) -> std::fmt::Result
where
    F: Fn(&mut std::fmt::Formatter<'_>, &T, usize, usize) -> std::fmt::Result,
{
    let mut first = true;
    for (index, value) in values.iter().enumerate() {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        repr(f, value, index + offset, el_limit)?;
    }
    Ok(())
}

/// Represent the given sequence with heuristic length limits.
fn repr_sequence<T, F>(
    f: &mut std::fmt::Formatter<'_>,
    values: &[T],
    limit: usize,
    repr: F,
) -> std::fmt::Result
where
    F: Fn(&mut std::fmt::Formatter<'_>, &T, usize, usize) -> std::fmt::Result,
{
    // We get to decide how many characters we want to devote to each element.
    // 20 seems like a nice number.
    const TARGET_ELEMENT_LIMIT: usize = 20;
    let n = limit / TARGET_ELEMENT_LIMIT;

    if values.len() > n {
        let (l, r) = heuristic_split(n);
        let el_limit = limit / n;
        repr_sequence_all(f, &values[..l], 0, el_limit, &repr)?;
        if l > 0 {
            write!(f, ", ")?;
        }
        write!(f, "..")?;
        if r > 0 {
            write!(f, ", ")?;
        }
        let offset = values.len() - r;
        repr_sequence_all(f, &values[offset..], offset, el_limit, &repr)?;
    } else {
        let el_limit = limit / values.len();
        repr_sequence_all(f, values, 0, el_limit, &repr)?;
    }
    Ok(())
}

/// Converts a value in microseconds since the epoch to a chrono::NaiveDateTime.
fn to_date_time(micros: i64) -> chrono::NaiveDateTime {
    let secs = micros / 1_000_000;
    let nsecs = ((micros % 1_000_000) * 1000) as u32;
    chrono::NaiveDateTime::from_timestamp(secs, nsecs)
}

impl Literal {
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

    /// Represents the value of this literal with some size limit. The size
    /// limit very roughly corresponds to a number of characters, but this is
    /// purely a heuristic thing.
    pub fn fmt2(&self, f: &mut std::fmt::Formatter<'_>, limit: usize) -> std::fmt::Result {
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
                    write!(f, "{}", to_date_time(*i).format("%Y-%m-%d %H:%M:%S"))
                }
                data_type::Class::Simple(data_type::Simple::TimestampTz) => {
                    write!(f, "{} UTC", to_date_time(*i).format("%Y-%m-%d %H:%M:%S"))
                }
                data_type::Class::Simple(data_type::Simple::Date) => {
                    write!(
                        f,
                        "{}",
                        to_date_time(*i * 24 * 60 * 60 * 1_000_000).format("%Y-%m-%d")
                    )
                }
                data_type::Class::Simple(data_type::Simple::Time) => {
                    write!(f, "{}", to_date_time(*i).format("%H:%M:%S"))
                }
                _ => write!(f, "{i}"),
            },
            LiteralValue::Float(v) => match self.data_type.class() {
                data_type::Class::Simple(data_type::Simple::Fp32) => write!(f, "{v}f32"),
                data_type::Class::Simple(data_type::Simple::Fp64) => write!(f, "{v}f64"),
                _ => write!(f, "{v}"),
            },
            LiteralValue::Data16(d) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Decimal) => {
                    if let Some(scale) = self.data_type.int_parameter(1) {
                        if d < &0 {
                            write!(f, "-")?;
                        }
                        let d = d.abs() as u128;
                        let s = 10u128.pow(scale as u32);
                        write!(f, "{0}.{1:02$}", d / s, d % s, scale as usize)
                    } else {
                        repr_binary(f, &d.to_le_bytes(), limit)
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
                _ => repr_binary(f, &d.to_le_bytes(), limit),
            },
            LiteralValue::String(s) => repr_string(f, s, limit),
            LiteralValue::Binary(b) => repr_binary(f, b, limit),
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
                    repr_sequence(f, x, limit, |f, value, index, limit| {
                        write!(f, ".{index}: ")?;
                        value.fmt2(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::NamedStruct) => {
                    write!(f, "(")?;
                    repr_sequence(f, x, limit, |f, value, index, limit| {
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
                        value.fmt2(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::List) => {
                    write!(f, "[")?;
                    repr_sequence(f, x, limit, |f, value, _, limit| value.fmt2(f, limit))?;
                    write!(f, "]")
                }
                _ => {
                    write!(f, "(")?;
                    repr_sequence(f, x, limit, |f, value, _, limit| value.fmt2(f, limit))?;
                    write!(f, ")")
                }
            },
            LiteralValue::Pairs(x) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Map) => {
                    write!(f, "{{")?;
                    repr_sequence(f, x, limit, |f, (key, value), _, limit| {
                        let (key_limit, value_limit) = heuristic_split(limit);
                        key.fmt2(f, key_limit)?;
                        write!(f, ": ")?;
                        value.fmt2(f, value_limit)
                    })?;
                    write!(f, "}}")
                }
                _ => {
                    write!(f, "(")?;
                    repr_sequence(f, x, limit, |f, (key, value), _, limit| {
                        write!(f, "(")?;
                        let (key_limit, value_limit) = heuristic_split(limit);
                        key.fmt2(f, key_limit)?;
                        write!(f, ": ")?;
                        value.fmt2(f, value_limit)?;
                        write!(f, ")")
                    })?;
                    write!(f, ")")
                }
            },
        }
    }
}

impl std::fmt::Display for Literal {
    /// Represents the literal. The default representation strives to print
    /// about 100 characters that represent data, while the alternate
    /// representation prints (basically) everything. Use repr() for more
    /// control.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.fmt2(f, usize::MAX)
        } else {
            self.fmt2(f, 100)
        }
    }
}

/// Parse a literal value. Returns the parsed literal.
pub fn parse_literal_type(
    x: &substrait::expression::literal::LiteralType,
    y: &mut context::Context,
    nullable: bool,
) -> diagnostic::Result<Literal> {
    match x {
        substrait::expression::literal::LiteralType::Boolean(x) => Literal::new_simple(
            LiteralValue::Boolean(*x),
            data_type::Simple::Boolean,
            nullable,
        ),
        substrait::expression::literal::LiteralType::I8(x) => {
            let x = i8::try_from(*x)
                .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i8 value out of range"))?;
            Literal::new_simple(
                LiteralValue::Integer(x as i64),
                data_type::Simple::I8,
                nullable,
            )
        }
        substrait::expression::literal::LiteralType::I16(x) => {
            let x = i16::try_from(*x)
                .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i16 value out of range"))?;
            Literal::new_simple(
                LiteralValue::Integer(x as i64),
                data_type::Simple::I16,
                nullable,
            )
        }
        substrait::expression::literal::LiteralType::I32(x) => Literal::new_simple(
            LiteralValue::Integer(*x as i64),
            data_type::Simple::I32,
            nullable,
        ),
        substrait::expression::literal::LiteralType::I64(x) => {
            Literal::new_simple(LiteralValue::Integer(*x), data_type::Simple::I64, nullable)
        }
        substrait::expression::literal::LiteralType::Fp32(x) => Literal::new_simple(
            LiteralValue::Float(*x as f64),
            data_type::Simple::Fp32,
            nullable,
        ),
        substrait::expression::literal::LiteralType::Fp64(x) => {
            Literal::new_simple(LiteralValue::Float(*x), data_type::Simple::Fp64, nullable)
        }
        substrait::expression::literal::LiteralType::String(x) => Literal::new_simple(
            LiteralValue::String(x.clone()),
            data_type::Simple::String,
            nullable,
        ),
        substrait::expression::literal::LiteralType::Binary(x) => Literal::new_simple(
            LiteralValue::Binary(x.clone()),
            data_type::Simple::Binary,
            nullable,
        ),
        substrait::expression::literal::LiteralType::Timestamp(x) => {
            let dt = to_date_time(*x);
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
        substrait::expression::literal::LiteralType::TimestampTz(x) => {
            let dt = to_date_time(*x);
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
        substrait::expression::literal::LiteralType::Date(x) => {
            let dt = to_date_time((*x as i64) * 24 * 60 * 60 * 1_000_000);
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
        substrait::expression::literal::LiteralType::Time(x) => {
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
        substrait::expression::literal::LiteralType::IntervalYearToMonth(x) => {
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
            let months = x.months + 12 * x.years;
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
        substrait::expression::literal::LiteralType::IntervalDayToSecond(x) => {
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
        substrait::expression::literal::LiteralType::Uuid(x) => {
            let uuid = if let Ok(x) = (&x[..]).try_into() {
                i128::from_ne_bytes(x)
            } else {
                0
            };
            Literal::new_simple(
                LiteralValue::Data16(uuid),
                data_type::Simple::Uuid,
                nullable,
            )
        }
        substrait::expression::literal::LiteralType::FixedChar(x) => Literal::new_compound(
            LiteralValue::String(x.clone()),
            data_type::Compound::FixedChar,
            nullable,
            vec![x.len() as u64],
        ),
        substrait::expression::literal::LiteralType::VarChar(x) => {
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
        substrait::expression::literal::LiteralType::FixedBinary(x) => Literal::new_compound(
            LiteralValue::Binary(x.clone()),
            data_type::Compound::FixedBinary,
            nullable,
            vec![x.len() as u64],
        ),
        substrait::expression::literal::LiteralType::Decimal(x) => {
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
            proto_primitive_field!(x, y, value, |x, _| {
                if x.len() != 16 {
                    Err(cause!(
                        ExpressionIllegalLiteralValue,
                        "decimal literal value must be 16 bytes in length"
                    ))
                } else {
                    Ok(())
                }
            });
            let mut val = 0i128;
            for byte in x.value.iter() {
                val <<= 8;
                val |= *byte as i128;
            }
            let precision = u64::try_from(x.precision).unwrap_or_default();
            let scale = u64::try_from(x.scale).unwrap_or_default();
            Literal::new_compound(
                LiteralValue::Data16(val),
                data_type::Compound::Decimal,
                nullable,
                vec![precision, scale],
            )
        }
        substrait::expression::literal::LiteralType::Struct(x) => {
            let (values, types): (Vec<_>, Vec<_>) =
                proto_repeated_field!(x, y, fields, parse_literal)
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
        substrait::expression::literal::LiteralType::List(x) => {
            let (values, types): (Vec<_>, Vec<_>) =
                proto_repeated_field!(x, y, values, parse_literal)
                    .1
                    .into_iter()
                    .map(|x| {
                        let x = x.unwrap_or_default();
                        let data_type = x.data_type.clone();
                        (x, data_type)
                    })
                    .unzip();
            if x.values.is_empty() {
                diagnostic!(
                    y,
                    Error,
                    ExpressionIllegalLiteralValue,
                    "need at least one list element to derive type (use EmptyList instead)"
                );
            }
            let mut data_type = Arc::default();
            for (index, field_type) in types.into_iter().enumerate() {
                data_type = types::assert_equal(
                    y,
                    field_type,
                    data_type,
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
        substrait::expression::literal::LiteralType::Map(x) => todo!(),
        substrait::expression::literal::LiteralType::EmptyList(x) => todo!(),
        substrait::expression::literal::LiteralType::EmptyMap(x) => todo!(),
        substrait::expression::literal::LiteralType::Null(x) => todo!(),
    }
}

/// Parse a literal value. Returns the parsed literal.
pub fn parse_literal(
    x: &substrait::expression::Literal,
    y: &mut context::Context,
) -> diagnostic::Result<Literal> {
    // Parse type parameters that apply to all literals.
    proto_primitive_field!(x, y, nullable);

    // FIXME: why would literals not support type variations? Feels like there
    // should be a type variation reference here.

    // Parse the literal value.
    let literal = proto_required_field!(x, y, literal_type, parse_literal_type, x.nullable)
        .1
        .unwrap_or_default();

    // Describe node.
    describe!(y, Expression, "{}", literal);
    summary!(y, "Literal with value {:#}", literal);
    Ok(literal)
}
