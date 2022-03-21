// SPDX-License-Identifier: Apache-2.0

//! Module for primitive data elements.
//!
//! The [`PrimitiveData`] enum is used to represent primitive data in the
//! input, for use in the leaf nodes of the tree.

/// Enumeration for representing any type of primitive data that can be stored
/// in YAML or protobuf.
#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveData {
    /// Used for nulls (YAML only).
    Null,

    /// Used for booleans.
    Bool(bool),

    /// Used for unsigned integers.
    Unsigned(u64),

    /// Used for signed integers.
    Signed(i64),

    /// Used for floating-point values.
    Float(f64),

    /// Used for UTF-8 strings.
    String(String),

    /// Used for bytestrings.
    Bytes(Vec<u8>),

    /// Used for enumerations (protobuf only).
    Enum(&'static str),

    /// Used for Any messages (protobuf only).
    Any(prost_types::Any),
}

fn hexdump(f: &mut std::fmt::Formatter<'_>, x: &[u8]) -> std::fmt::Result {
    for (i, b) in x.iter().enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }
        write!(f, "{:02X}", b)?;
    }
    Ok(())
}

impl std::fmt::Display for PrimitiveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveData::Null => write!(f, "null"),
            PrimitiveData::Bool(true) => write!(f, "true"),
            PrimitiveData::Bool(false) => write!(f, "false"),
            PrimitiveData::Unsigned(x) => write!(f, "{x}"),
            PrimitiveData::Signed(x) => write!(f, "{x}"),
            PrimitiveData::Float(x) => write!(f, "{x}"),
            PrimitiveData::String(x) => write!(f, "{x:?}"),
            PrimitiveData::Bytes(x) => hexdump(f, x),
            PrimitiveData::Enum(x) => write!(f, "{x}"),
            PrimitiveData::Any(x) => {
                write!(f, "{}(", x.type_url)?;
                hexdump(f, &x.value)?;
                write!(f, ")")
            }
        }
    }
}
