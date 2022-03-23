// SPDX-License-Identifier: Apache-2.0

//! Module for representing Substrait protobuf input.
//!
//! The structures here are generated using [`prost`], but have a bunch of
//! extra traits from [`traits`](crate::input::traits) associated with them,
//! for which the implementations are generated using
//! [`substrait_validator_derive`]. The purpose of these traits is to add basic
//! introspection capabilities to the prost structures. One of the use cases
//! for this is to let the parsing code automatically detect when the
//! validation code ignored a subtree while validating, which implies that the
//! validator hasn't checked everything and thus should not warrant that the
//! received plan is valid.

use crate::input::traits;
use crate::output::primitive_data;

use heck::ToUpperCamelCase;

#[allow(clippy::large_enum_variant)]
pub mod substrait {
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
    pub mod validator {
        include!(concat!(env!("OUT_DIR"), "/substrait.validator.rs"));
    }
}

/// Converts a Rust module path and name (the latter already processed by
/// cook_ident()) to a protobuf type path.
pub fn cook_path(module_path: &str, type_name: &str) -> String {
    let mut iter = module_path
        .split("::")
        .skip(module_path!().split("::").count())
        .map(cook_ident)
        .chain(::std::iter::once(type_name))
        .peekable();
    let mut items = vec![];
    if matches!(iter.peek(), Some(&"substrait")) {
        items.push(iter.next().unwrap().to_string());
        if matches!(iter.peek(), Some(&"extensions") | Some(&"validator")) {
            items.push(iter.next().unwrap().to_string());
        }
    }
    items.extend(iter.map(|x| x.to_upper_camel_case()));
    ::itertools::Itertools::intersperse(items.iter().map(|x| x.as_ref()), ".").collect()
}

/// Converts a Rust identifier string generated via stringify!() to the
/// original identifier by "cooking" raw identifiers.
pub fn cook_ident(ident: &str) -> &str {
    if let Some((_, keyword)) = ident.split_once('#') {
        keyword
    } else {
        ident
    }
}

impl traits::ProtoPrimitive for bool {
    fn proto_primitive_type() -> &'static str {
        "bool"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bool(false)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bool(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        !*self
    }
}

impl traits::ProtoPrimitive for u32 {
    fn proto_primitive_type() -> &'static str {
        "uint32"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for u64 {
    fn proto_primitive_type() -> &'static str {
        "uint64"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for i32 {
    fn proto_primitive_type() -> &'static str {
        "int32"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for i64 {
    fn proto_primitive_type() -> &'static str {
        "int64"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for f32 {
    fn proto_primitive_type() -> &'static str {
        "float"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(0.0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0.0
    }
}

impl traits::ProtoPrimitive for f64 {
    fn proto_primitive_type() -> &'static str {
        "double"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(0.0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0.0
    }
}

impl traits::ProtoPrimitive for String {
    fn proto_primitive_type() -> &'static str {
        "string"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::String(String::new())
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::String(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.is_empty()
    }
}

impl traits::ProtoPrimitive for Vec<u8> {
    fn proto_primitive_type() -> &'static str {
        "bytes"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bytes(vec![])
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bytes(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.is_empty()
    }
}

impl traits::ProtoPrimitive for prost_types::Any {
    fn proto_primitive_type() -> &'static str {
        "any"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Any(prost_types::Any::default())
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Any(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.type_url.is_empty()
    }
}
