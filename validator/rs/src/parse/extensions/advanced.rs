// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for advanced extensions, i.e.
//! those based around protobuf Any values.

use crate::input::proto::substrait;
use crate::output::diagnostic::Result;
use crate::parse::context;

/// Parse a protobuf "any" type declaration.
#[allow(clippy::ptr_arg)]
fn parse_expected_type_url(x: &String, y: &mut context::Context) -> Result<()> {
    if let Err(path) = y.define_proto_any_type(x) {
        diagnostic!(
            y,
            Info,
            RedundantProtoAnyDeclaration,
            "message type {x} redeclared"
        );
        link!(y, path, "Previous declaration was here.");
    }
    Ok(())
}

/// Parse a protobuf "any" message that consumers may ignore.
pub fn parse_hint_any(x: &prost_types::Any, y: &mut context::Context) -> Result<()> {
    let (allowed, path) = y.resolve_proto_any(x);
    if allowed {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "explicitly allowed hint of type {}",
            x.type_url
        );
    } else {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "ignoring unknown hint of type {}",
            x.type_url
        );
    }
    if let Some(path) = path {
        link!(y, path, "Type URL declaration is here.");
    }
    Ok(())
}

/// Parse a protobuf "any" message that consumers are not allowed to ignore.
pub fn parse_functional_any(x: &prost_types::Any, y: &mut context::Context) -> Result<()> {
    let (allowed, path) = y.resolve_proto_any(x);
    if allowed {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "explicitly allowed enhancement of type {}",
            x.type_url
        );
    } else {
        diagnostic!(
            y,
            Warning,
            ProtoAny,
            "unknown enhancement of type {}; plan is only valid \
            for consumers recognizing this enhancement",
            x.type_url
        );
    }
    if let Some(path) = path {
        link!(y, path, "Type URL declaration is here.");
    }
    Ok(())
}

/// Parse an advanced extension message (based on protobuf "any" messages).
/// Returns whether an enhancement was specified.
pub fn parse_advanced_extension(
    x: &substrait::extensions::AdvancedExtension,
    y: &mut context::Context,
) -> Result<bool> {
    proto_field!(x, y, optimization, parse_hint_any);
    Ok(proto_field!(x, y, enhancement, parse_functional_any)
        .0
        .is_some())
}

/// Parses the advanced extension information in a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) {
    proto_repeated_field!(x, y, expected_type_urls, parse_expected_type_url);
    proto_field!(x, y, advanced_extensions, parse_advanced_extension);
}

/// Generate Info diagnostics for any extension definitions that weren't used.
pub fn check_unused_definitions(y: &mut context::Context) {
    for (uri, _, path) in y
        .proto_any_types()
        .iter_unused()
        .collect::<Vec<_>>()
        .into_iter()
    {
        diagnostic!(
            y,
            Info,
            RedundantProtoAnyDeclaration,
            "message type {uri} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }
}
