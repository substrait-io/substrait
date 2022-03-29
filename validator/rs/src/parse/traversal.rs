// SPDX-License-Identifier: Apache-2.0

//! Module for the boilerplate code involved with traversing an input
//! protobuf/YAML tree to form the output [tree](tree::Node).
//!
//! Refer to the documentation for [`parse`](mod@crate::parse) for more
//! information.

// TODO: remove once validation code is finished.
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::input::config;
use crate::input::traits::InputNode;
use crate::input::traits::ProtoEnum;
use crate::input::yaml;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::parse_result;
use crate::output::path;
use crate::output::primitive_data;
use crate::output::tree;
use crate::parse::context;
use std::sync::Arc;

//=============================================================================
// Type definitions
//=============================================================================

// Return value for parse macros for optional fields. The first element refers
// to the node for the field, if the field was present. The second is the
// return value of the supplied parse function, if it was called and didn't
// fail.
type OptionalResult<T> = (Option<Arc<tree::Node>>, Option<T>);

// Return value for parse macros for required fields. The first element refers
// to the node for the field; if the required field wasn't actually specified,
// a dummy node would have been made, so this is not an Option. The second is
// the return value of the supplied parse function, if it was called and didn't
// fail, just like for OptionalResult<T>.
type RequiredResult<T> = (Arc<tree::Node>, Option<T>);

// Return value for parse macros for repeated fields. Same as RequiredResult,
// but with each tuple entry wrapped in a vector. Both vectors will have equal
// length.
type RepeatedResult<T> = (Vec<Arc<tree::Node>>, Vec<Option<T>>);

//=============================================================================
// Macros for pushing annotations
//=============================================================================

/// Convenience/shorthand macro for pushing diagnostic messages to a node.
macro_rules! diagnostic {
    ($context:expr, $level:ident, $class:ident, $($args:expr),*) => {
        diagnostic!($context, $level, cause!($class, $($args),*))
    };
    ($context:expr, $level:ident, $cause:expr) => {
        crate::parse::traversal::push_diagnostic($context, crate::output::diagnostic::Level::$level, $cause)
    };
    ($context:expr, $diag:expr) => {
        $context.push_diagnostic($diag)
    };
}
macro_rules! ediagnostic {
    ($context:expr, $level:ident, $class:ident, $err:expr) => {
        diagnostic!($context, $level, ecause!($class, $err))
    };
}

/// Pushes a diagnostic message to the node information list.
pub fn push_diagnostic(
    context: &mut context::Context,
    level: diagnostic::Level,
    cause: diagnostic::Cause,
) {
    context.push_diagnostic(diagnostic::RawDiagnostic {
        cause,
        level,
        path: context.path_buf(),
    });
}

/// Convenience/shorthand macro for pushing formatted comments to a node.
macro_rules! comment {
    ($context:expr, $($fmts:expr),*) => {
        $context.push_comment(format!($($fmts),*))
    };
}

/// Convenience/shorthand macro for pushing formatted comments that link to
/// some path to a node.
macro_rules! link {
    ($context:expr, $path:expr, $($fmts:expr),*) => {
        $context.push_comment(crate::output::comment::Comment::new().link(format!($($fmts),*), $path))
    };
}

/// Convenience/shorthand macro for setting descriptive information for a node.
macro_rules! describe {
    ($context:expr, $class:ident, $($fmts:expr),*) => {
        $context.set_description(crate::output::tree::Class::$class, Some(format!($($fmts),*)))
    };
}

/// Convenience/shorthand macro for appending plain text to the summary of a
/// node.
macro_rules! summary {
    ($context:expr, $($fmts:expr),*) => {
        $context.push_summary(format!($($fmts),*))
    };
}

//=============================================================================
// Generic code for field handling
//=============================================================================

/// Parses a child node and pushes it into the provided parent context.
fn push_child<TF, TR, FP>(
    context: &mut context::Context,
    child: &TF,
    path_element: path::PathElement,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    // Create the node for the child.
    let mut field_output = child.data_to_node();

    // Create the context for calling the parse function for the child.
    let mut field_context = context.child(&mut field_output, path_element.clone());

    // Call the provided parser function.
    let result = parser(child, &mut field_context)
        .map_err(|cause| {
            diagnostic!(&mut field_context, Error, cause);
        })
        .ok();

    // Handle any fields not handled by the provided parse function. Only
    // generate a warning diagnostic for unhandled children if the parse
    // function succeeded and we're not already in an unknown subtree.
    handle_unknown_children(
        child,
        &mut field_context,
        result.is_some() && !unknown_subtree,
    );

    // Push and return the completed node.
    let field_output = Arc::new(field_output);
    context.push(tree::NodeData::Child(tree::Child {
        path_element,
        node: field_output.clone(),
        recognized: !unknown_subtree,
    }));

    (field_output, result)
}

/// Handle all children that haven't already been handled. If with_diagnostic
/// is set, this also generates a diagnostic message if there were
/// populated/non-default unhandled fields.
fn handle_unknown_children<T: InputNode>(
    input: &T,
    context: &mut context::Context,
    with_diagnostic: bool,
) {
    if input.parse_unknown(context) && with_diagnostic {
        let mut fields = vec![];
        for data in context.node_data().iter() {
            if let tree::NodeData::Child(child) = data {
                if !child.recognized {
                    fields.push(child.path_element.to_string_without_dot());
                }
            }
        }
        if !fields.is_empty() {
            let fields: String =
                itertools::Itertools::intersperse(fields.into_iter(), ", ".to_string()).collect();
            diagnostic!(
                context,
                Warning,
                NotYetImplemented,
                "the following child nodes were not recognized by the validator: {fields}"
            );
        }
    }
}

//=============================================================================
// Protobuf optional field handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional protobuf fields.
macro_rules! proto_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_field(
            $context,
            &$input.$field.as_ref(),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing optional protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_boxed_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push a protobuf optional field.
pub fn push_proto_field<TF, TR, FP>(
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> OptionalResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if !context.set_field_parsed(field_name) {
        panic!("field {field_name} was parsed multiple times");
    }

    if let Some(field_input) = field {
        let path_element = if let Some(variant) = field_input.oneof_variant() {
            path::PathElement::Variant(field_name.to_string(), variant.to_string())
        } else {
            path::PathElement::Field(field_name.to_string())
        };
        let (field_output, result) = push_child(
            context,
            field_input.deref(),
            path_element,
            unknown_subtree,
            parser,
        );
        (Some(field_output), result)
    } else {
        (None, None)
    }
}

//=============================================================================
// Protobuf required and primitive field handling
//=============================================================================

/// Convenience/shorthand macro for parsing required protobuf fields.
macro_rules! proto_required_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &$input.$field.as_ref(),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_required_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing required protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_required_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_boxed_required_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing primitive protobuf fields.
macro_rules! proto_primitive_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_primitive_field!($input, $context, $field, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &Some(&$input.$field),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_primitive_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push a required field of some message type. If the field is
/// not populated, a MissingField diagnostic is pushed automatically, and
/// an empty node is returned as an error recovery placeholder.
pub fn push_proto_required_field<TF, TR, FP>(
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let (Some(node), result) =
        push_proto_field(context, field, field_name, unknown_subtree, parser)
    {
        (node, result)
    } else {
        ediagnostic!(context, Error, ProtoMissingField, field_name);
        (Arc::new(TF::type_to_node()), None)
    }
}

/// Convenience/shorthand macro for parsing enumeration protobuf fields.
macro_rules! proto_enum_field {
    ($input:expr, $context:expr, $field:ident, $typ:ty) => {
        proto_enum_field!($input, $context, $field, $typ, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr) => {
        crate::parse::traversal::push_proto_enum_field::<$typ, _, _>(
            $context,
            $input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr, $($args:expr),*) => {
        proto_enum_field!($input, $context, $field, $typ, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push an enumeration field of some message type. The i32 in the
/// struct generated by prost is automatically converted to the enum; if the
/// value is out of range, an error is generated.
pub fn push_proto_enum_field<TF, TR, FP>(
    context: &mut context::Context,
    field: i32,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: ProtoEnum,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let Some(field) = TF::proto_enum_from_i32(field) {
        push_proto_required_field(context, &Some(&field), field_name, unknown_subtree, parser)
    } else {
        (
            push_proto_required_field(
                context,
                &Some(&field),
                field_name,
                unknown_subtree,
                |x, y| {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "unknown value {x} for {}",
                        TF::proto_enum_type()
                    );
                    Ok(())
                },
            )
            .0,
            None,
        )
    }
}

/// Convenience/shorthand macro for parsing enumeration protobuf fields of
/// which the value must be specified.
macro_rules! proto_required_enum_field {
    ($input:expr, $context:expr, $field:ident, $typ:ty) => {
        proto_required_enum_field!($input, $context, $field, $typ, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr) => {
        crate::parse::traversal::push_proto_required_enum_field::<$typ, _, _>(
            $context,
            $input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr, $($args:expr),*) => {
        proto_required_enum_field!($input, $context, $field, $typ, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push an enumeration field of some message type. The i32 in the
/// struct generated by prost is automatically converted to the enum; if the
/// value is out of range, an error is generated.
pub fn push_proto_required_enum_field<TF, TR, FP>(
    context: &mut context::Context,
    field: i32,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: ProtoEnum,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    push_proto_enum_field(context, field, field_name, unknown_subtree, |x, y| {
        if field == 0 {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "this enum may not be left unspecified"
            );
        }
        parser(x, y)
    })
}

//=============================================================================
// Protobuf repeated field handling
//=============================================================================

/// Convenience/shorthand macro for parsing repeated protobuf fields.
macro_rules! proto_repeated_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_repeated_field!($input, $context, $field, $parser, |_, _, _, _, _| ())
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_repeated_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
            $validator,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr, $($args:expr),*) => {
        proto_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $validator)
    };
}

/// Parse and push a repeated field of some message type.
pub fn push_proto_repeated_field<TF, TR, FP, FV>(
    context: &mut context::Context,
    field: &[TF],
    field_name: &'static str,
    unknown_subtree: bool,
    mut parser: FP,
    mut validator: FV,
) -> RepeatedResult<TR>
where
    TF: InputNode,
    FP: FnMut(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: FnMut(&TF, &mut context::Context, usize, &Arc<tree::Node>, Option<&TR>),
{
    if !context.set_field_parsed(field_name) {
        panic!("field {field_name} was parsed multiple times");
    }

    field
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let (node, result) = push_child(
                context,
                child,
                path::PathElement::Repeated(field_name.to_string(), index),
                unknown_subtree,
                &mut parser,
            );
            validator(child, context, index, &node, result.as_ref());
            (node, result)
        })
        .unzip()
}

/// Convenience/shorthand macro for parsing repeated protobuf fields for which
/// at least one element must exist.
macro_rules! proto_required_repeated_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_required_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_required_repeated_field!($input, $context, $field, $parser, |_, _, _, _, _| ())
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_required_repeated_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
            $validator,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr, $($args:expr),*) => {
        proto_required_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $validator)
    };
}

/// Parse and push a repeated field of some message type, and check that at
/// least one element exists.
pub fn push_proto_required_repeated_field<TF, TR, FP, FV>(
    context: &mut context::Context,
    field: &[TF],
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> RepeatedResult<TR>
where
    TF: InputNode,
    FP: FnMut(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: FnMut(&TF, &mut context::Context, usize, &Arc<tree::Node>, Option<&TR>),
{
    if field.is_empty() {
        ediagnostic!(context, Error, ProtoMissingField, field_name);
    }
    push_proto_repeated_field(
        context,
        field,
        field_name,
        unknown_subtree,
        parser,
        validator,
    )
}

//=============================================================================
// Protobuf root message handling
//=============================================================================

/// Parses a serialized protobuf message using the given root parse function,
/// initial state, and configuration.
pub fn parse_proto<T, F, B>(
    buffer: B,
    root_name: &'static str,
    root_parser: F,
    state: &mut context::State,
    config: &config::Config,
) -> parse_result::ParseResult
where
    T: prost::Message + InputNode + Default,
    F: FnOnce(&T, &mut context::Context) -> diagnostic::Result<()>,
    B: prost::bytes::Buf,
{
    match T::decode(buffer) {
        Err(err) => {
            // Create a minimal root node with just the decode error
            // diagnostic.
            let mut root = T::type_to_node();

            // Create a root context for it.
            let mut context = context::Context::new(root_name, &mut root, state, config);

            // Push the diagnostic using the context.
            context.push_diagnostic(diagnostic::RawDiagnostic {
                cause: ecause!(ProtoParseFailed, err),
                level: diagnostic::Level::Error,
                path: path::PathBuf {
                    root: root_name,
                    elements: vec![],
                },
            });

            parse_result::ParseResult { root }
        }
        Ok(input) => {
            // Create the root node.
            let mut root = input.data_to_node();

            // Create the root context.
            let mut context = context::Context::new(root_name, &mut root, state, config);

            // Call the provided parser function.
            let success = root_parser(&input, &mut context)
                .map_err(|cause| {
                    diagnostic!(&mut context, Error, cause);
                })
                .is_ok();

            // Handle any fields not handled by the provided parse function.
            // Only generate a warning diagnostic for unhandled children if the
            // parse function succeeded.
            handle_unknown_children(&input, &mut context, success);

            parse_result::ParseResult { root }
        }
    }
}

//=============================================================================
// YAML object handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional YAML fields.
macro_rules! yaml_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        crate::parse::traversal::push_yaml_field($input, $context, $field, false, $parser)
    };
}

/// Parse and push an optional YAML field.
pub fn push_yaml_field<TS, TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: TS,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<OptionalResult<TR>>
where
    TS: AsRef<str>,
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let serde_json::Value::Object(input) = input {
        let field_name = field_name.as_ref();
        if !context.set_field_parsed(field_name) {
            panic!("field {field_name} was parsed multiple times");
        }

        if let Some(child) = input.get(field_name) {
            let (field_output, result) = push_child(
                context,
                child,
                path::PathElement::Field(field_name.to_string()),
                unknown_subtree,
                parser,
            );
            Ok((Some(field_output), result))
        } else {
            Ok((None, None))
        }
    } else {
        Err(cause!(YamlInvalidType, "object expected"))
    }
}

/// Convenience/shorthand macro for parsing required YAML fields.
macro_rules! yaml_required_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        crate::parse::traversal::push_yaml_required_field($input, $context, $field, false, $parser)
    };
}

/// Parse and push a required field of a YAML object. If the field does not
/// exist, a MissingField diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_field<TS, TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: TS,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<RequiredResult<TR>>
where
    TS: AsRef<str>,
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    let field_name = field_name.as_ref();
    if let (Some(node), result) =
        push_yaml_field(input, context, field_name, unknown_subtree, parser)?
    {
        Ok((node, result))
    } else {
        ediagnostic!(context, Error, YamlMissingKey, field_name);
        Ok((
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        ))
    }
}

//=============================================================================
// YAML array handling
//=============================================================================

/// Convenience/shorthand macro for parsing a YAML array that may be empty.
macro_rules! yaml_array {
    ($input:expr, $context:expr) => {
        yaml_array!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $parser:expr) => {
        yaml_array!($input, $context, $field, $parser, 0)
    };
    ($input:expr, $context:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_array($input, $context, $min_size, false, $parser)
    };
}

/// Convenience/shorthand macro for parsing a YAML array that must have at
/// least one value.
macro_rules! yaml_required_array {
    ($input:expr, $context:expr) => {
        yaml_required_array!($input, $context, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $parser:expr) => {
        yaml_array!($input, $context, $parser, 1)
    };
}

/// Parse and push an optional YAML array element.
pub fn push_yaml_element<TR, FP>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
) -> OptionalResult<TR>
where
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if !context.set_field_parsed(index) {
        panic!("element {index} was parsed multiple times");
    }

    if let Some(child) = input.get(index) {
        let (field_output, result) = push_child(
            context,
            child,
            path::PathElement::Index(index),
            unknown_subtree,
            parser,
        );
        (Some(field_output), result)
    } else {
        (None, None)
    }
}

/// Parse and push a required element of a YAML array. If the element does not
/// exist, a MissingElement diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_element<TR, FP>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let (Some(node), result) = push_yaml_element(input, context, index, unknown_subtree, parser)
    {
        (node, result)
    } else {
        diagnostic!(context, Error, YamlMissingElement, "index {index}");
        (
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        )
    }
}

/// Parse and push a complete YAML array. If a required element does not exist,
/// a MissingElement diagnostic is pushed automatically, and an empty node is
/// returned as an error recovery placeholder.
pub fn push_yaml_array<TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    min_size: usize,
    unknown_subtree: bool,
    mut parser: FP,
) -> diagnostic::Result<RepeatedResult<TR>>
where
    FP: FnMut(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let serde_json::Value::Array(input) = input {
        let size = std::cmp::max(min_size, input.len());
        Ok((0..size)
            .into_iter()
            .map(|index| {
                push_yaml_required_element(input, context, index, unknown_subtree, &mut parser)
            })
            .unzip())
    } else {
        Err(cause!(YamlInvalidType, "array expected"))
    }
}

/// Shorthand for fields that must be arrays if specified.
macro_rules! yaml_repeated_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_repeated_field!($input, $context, $field, $parser, 0)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_repeated_field(
            $input, $context, $field, false, $min_size, false, $parser,
        )
    };
}

/// Shorthand for fields that must be arrays.
macro_rules! yaml_required_repeated_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_required_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_required_repeated_field!($input, $context, $field, $parser, 1)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_repeated_field(
            $input, $context, $field, true, $min_size, false, $parser,
        )
    };
}

/// Parse and push a complete YAML array. If a required element does not exist,
/// a MissingElement diagnostic is pushed automatically, and an empty node is
/// returned as an error recovery placeholder.
pub fn push_yaml_repeated_field<TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: &'static str,
    field_required: bool,
    min_size: usize,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<RepeatedResult<TR>>
where
    FP: FnMut(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    Ok(if field_required {
        push_yaml_required_field(input, context, field_name, unknown_subtree, |x, y| {
            yaml_array!(x, y, parser, min_size)
        })?
        .1
    } else {
        push_yaml_field(input, context, field_name, unknown_subtree, |x, y| {
            yaml_array!(x, y, parser, min_size)
        })?
        .1
    }
    .unwrap_or_else(|| (vec![], vec![])))
}

//=============================================================================
// YAML primitive handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional YAML fields.
macro_rules! yaml_prim {
    ($typ:ident) => {
        |x, y| crate::parse::traversal::yaml_primitive_parsers::$typ(x, y, |x, _| Ok(x.to_owned()))
    };
    ($typ:ident, $parser:expr) => {
        |x, y| crate::parse::traversal::yaml_primitive_parsers::$typ(x, y, $parser)
    };
}

pub mod yaml_primitive_parsers {
    use super::*;

    /// Boolean primitive helper.
    pub fn bool<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&bool, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Bool(x) = x {
            parser(x, y)
        } else {
            Err(cause!(YamlInvalidType, "string expected"))
        }
    }

    /// Signed integer primitive helper.
    pub fn i64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&i64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_i64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "signed integer expected"))
    }

    /// Unsigned integer primitive helper.
    pub fn u64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&u64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_u64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "unsigned integer expected"))
    }

    /// Float primitive helper.
    pub fn f64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&f64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_f64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "floating point number expected"))
    }

    /// String primitive helper.
    pub fn str<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&str, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::String(x) = x {
            parser(x, y)
        } else {
            Err(cause!(YamlInvalidType, "string expected"))
        }
    }
}

//=============================================================================
// YAML root handling
//=============================================================================

/// Attempts to resolve a URI.
fn resolve_uri(
    uri: &str,
    context: &mut context::Context,
) -> diagnostic::Result<config::BinaryData> {
    // Apply yaml_uri_overrides configuration.
    let remapped_uri = context
        .config
        .uri_overrides
        .iter()
        .find_map(|(pattern, mapping)| {
            if pattern.matches(uri) {
                Some(mapping.as_ref().map(|x| &x[..]))
            } else {
                None
            }
        });
    let is_remapped = remapped_uri.is_some();
    let remapped_uri = remapped_uri.unwrap_or(Some(uri));

    let remapped_uri = if let Some(remapped_uri) = remapped_uri {
        remapped_uri.to_owned()
    } else {
        return Err(cause!(
            YamlResolutionDisabled,
            "YAML resolution for {uri} was disabled"
        ));
    };
    if is_remapped {
        diagnostic!(context, Info, Yaml, "URI was remapped to {remapped_uri}");
    }

    // If a custom download function is specified, use it to resolve.
    if let Some(ref resolver) = context.config.uri_resolver {
        return resolver(&remapped_uri)
            .map_err(|x| ecause!(YamlResolutionFailed, x.as_ref().to_string()));
    }

    // Parse as a URL.
    let url = match url::Url::parse(&remapped_uri) {
        Ok(url) => url,
        Err(e) => {
            return Err(if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "configured URI remapping ({remapped_uri}) did not parse as URL: {e}"
                )
            } else {
                cause!(
                    YamlResolutionFailed,
                    "failed to parse {remapped_uri} as URL: {e}"
                )
            });
        }
    };

    // Reject anything that isn't file://-based.
    if url.scheme() != "file" {
        return Err(if is_remapped {
            cause!(
                YamlResolutionFailed,
                "configured URI remapping ({remapped_uri}) does not use file:// scheme"
            )
        } else {
            cause!(YamlResolutionFailed, "URI does not use file:// scheme")
        });
    }

    // Convert to path.
    let path = match url.to_file_path() {
        Ok(path) => path,
        Err(_) => {
            return Err(if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "configured URI remapping ({remapped_uri}) could not be converted to file path"
                )
            } else {
                cause!(
                    YamlResolutionFailed,
                    "URI could not be converted to file path"
                )
            });
        }
    };

    // Read the file.
    std::fs::read(path)
        .map_err(|e| {
            if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "failed to file remapping for URI ({remapped_uri}): {e}"
                )
            } else {
                ecause!(YamlResolutionFailed, e)
            }
        })
        .map(|d| -> config::BinaryData { Box::new(d) })
}

/// Resolves a URI to a YAML file, parses the YAML syntax, and optionally
/// validates it using the given JSON schema.
fn load_yaml(
    uri: &str,
    context: &mut context::Context,
    schema: Option<&jsonschema::JSONSchema>,
) -> Option<yaml::Value> {
    // Try to resolve the YAML file. Note that failure to resolve is a warning,
    // not an error; it means the plan isn't valid in the current environment,
    // but it might still be valid in another one, in particular for consumers
    // that don't need to be able to resolve the YAML files to use the plan.
    let binary_data = match resolve_uri(uri, context) {
        Err(e) => {
            diagnostic!(context, Warning, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as UTF-8.
    let string_data = match std::str::from_utf8(binary_data.as_ref().as_ref()) {
        Err(e) => {
            ediagnostic!(context, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as YAML.
    let yaml_data = match yaml_rust::YamlLoader::load_from_str(string_data) {
        Err(e) => {
            ediagnostic!(context, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => {
            if x.len() > 1 {
                diagnostic!(
                    context,
                    Warning,
                    YamlParseFailed,
                    "YAML file contains multiple documents; ignoring all but the first"
                );
            }
            match x.into_iter().next() {
                None => {
                    diagnostic!(
                        context,
                        Error,
                        YamlParseFailed,
                        "YAML file contains zero documents"
                    );
                    return None;
                }
                Some(x) => x,
            }
        }
    };

    // Convert to JSON DOM.
    let json_data = match yaml::yaml_to_json(yaml_data, context.path()) {
        Err(e) => {
            diagnostic!(context, e);
            return None;
        }
        Ok(x) => x,
    };

    // Validate with schema.
    if let Some(schema) = schema {
        if let Err(es) = schema.validate(&json_data) {
            for e in es {
                ediagnostic!(context, Error, YamlSchemaValidationFailed, e);
            }
            return None;
        }
    }

    Some(json_data)
}

/// Attempt to load and parse a YAML file using the given root parse function,
/// initial state, and configuration.
pub fn parse_yaml<TS, FP>(
    uri: TS,
    context: &mut context::Context,
    schema: Option<&jsonschema::JSONSchema>,
    parser: FP,
) -> Arc<extension::YamlInfo>
where
    TS: AsRef<str>,
    FP: Fn(&yaml::Value, &mut context::Context) -> diagnostic::Result<()>,
{
    let uri = uri.as_ref();
    let uri_reference = extension::NamedReference::new(Some(uri), context.parent_path_buf());

    // Resolve the YAML file.
    let yaml_info = Arc::new(if let Some(root_input) = load_yaml(uri, context, schema) {
        // Create an empty YamlData object.
        *context.yaml_data_opt() = Some(extension::YamlData::new(uri_reference));

        // Create the node for the YAML data root.
        let mut root_output = root_input.data_to_node();

        // Create the path element for referring to the YAML data root.
        let path_element = path::PathElement::Field("data".to_string());

        // Create the context for the YAML data root.
        let mut root_context = context.child(&mut root_output, path_element.clone());

        // Create a PathBuf for the root node.
        let root_path = root_context.path_buf();

        // Call the provided root parser.
        let success = parser(&root_input, &mut root_context)
            .map_err(|cause| {
                diagnostic!(&mut root_context, Error, cause);
            })
            .is_ok();

        // Handle any fields not handled by the provided parse function.
        handle_unknown_children(&root_input, &mut root_context, success);

        // Push and return the completed node.
        let root_output = Arc::new(root_output);
        context.push(tree::NodeData::Child(tree::Child {
            path_element,
            node: root_output.clone(),
            recognized: true,
        }));

        // Take the constructed YAML data object from the context.
        let mut yaml_data = context.yaml_data_opt().take().unwrap();

        // Configure the reference to the root node in the YamlData object.
        yaml_data.data.path = root_path;
        yaml_data.data.node = root_output;

        // Wrap the completed YAML data object in an Arc.
        let yaml_data = Arc::new(yaml_data);

        // The node type will have been set as if this is a normal string
        // primitive. We want extra information though, namely the contents of
        // the YAML file. So we change the node type.
        context.replace_node_type(tree::NodeType::YamlReference(yaml_data.clone()));

        // Construct the YAML information object.
        extension::YamlInfo::Resolved(yaml_data)
    } else {
        extension::YamlInfo::Unresolved(uri_reference)
    });

    yaml_info
}
