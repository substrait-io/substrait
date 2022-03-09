//! Module for the boilerplate code involved with traversing an input
//! protobuf/YAML tree to form the output [tree](tree::Node).
//!
//! Refer to the documentation for [`parse`](mod@crate::parse) for more
//! information.

// FIXME: remove once validation code is finished. Also re-evaluate the
// usefulness of the validator functions at that stage, i.e. remove them if
// they weren't useful anywhere.
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::input::config;
use crate::input::traits::InputNode;
use crate::input::yaml;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::path;
use crate::output::primitive_data;
use crate::output::tree;
use crate::parse::context;
use std::sync::Arc;

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
        $context.output.push_diagnostic($diag, &$context.config)
    };
}

/// Pushes a diagnostic message to the node information list.
pub fn push_diagnostic(
    context: &mut context::Context,
    level: diagnostic::Level,
    cause: diagnostic::Cause,
) {
    context.output.push_diagnostic(
        diagnostic::RawDiagnostic {
            cause,
            level,
            path: context.breadcrumb.path.to_path_buf(),
        },
        context.config,
    );
}

/// Convenience/shorthand macro for pushing comments to a node.
macro_rules! comment {
    ($context:expr, $($fmts:expr),*) => {
        crate::parse::traversal::push_comment($context, format!($($fmts),*), None)
    };
}

/// Convenience/shorthand macro for pushing comments to a node.
macro_rules! link {
    ($context:expr, $link:expr, $($fmts:expr),*) => {
        crate::parse::traversal::push_comment($context, format!($($fmts),*), Some($link))
    };
}

/// Pushes a comment to the node information list.
pub fn push_comment<S: AsRef<str>>(
    context: &mut context::Context,
    text: S,
    path: Option<path::PathBuf>,
) {
    let text = text.as_ref().to_string();
    let comment = comment::Comment::new();
    let comment = if let Some(path) = path {
        comment.with_link_to_path(text, path)
    } else {
        comment.with_plain(text)
    };
    context.output.data.push(tree::NodeData::Comment(comment))
}

/// Convenience/shorthand macro for pushing type information to a node. Note
/// that this macro isn't shorter than just using push_data_type() directly; it
/// exists for symmetry.
macro_rules! data_type {
    ($context:expr, $typ:expr) => {
        crate::parse::traversal::push_data_type($context, $typ)
    };
}

/// Pushes a data type to the node information list, and saves it in the
/// current context.
pub fn push_data_type(context: &mut context::Context, data_type: data_type::DataType) {
    context
        .output
        .data
        .push(tree::NodeData::DataType(data_type.clone()));
    context.output.data_type = Some(data_type);
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
        proto_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_field(
            $input,
            $context,
            &$input.$field.as_ref(),
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Convenience/shorthand macro for parsing optional protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_boxed_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_field(
            $input,
            $context,
            &$input.$field,
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Parse and push a protobuf optional field.
pub fn push_proto_field<TP, TF, TR, FP, FV>(
    input: &TP,
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Option<Arc<tree::Node>>, Option<TR>)
where
    TF: InputNode,
    FP: Fn(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&TP, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    if !context
        .breadcrumb
        .fields_parsed
        .insert(field_name.to_string())
    {
        panic!("field {} was parsed multiple times", field_name);
    }

    if let Some(field_input) = field {
        let field_input = field_input.deref();

        // Create the node for the child message.
        let mut field_output = field_input.data_to_node();

        // Create the path element for referring to the child node.
        let path_element = if let Some(variant) = field_input.oneof_variant() {
            path::PathElement::Variant(field_name.to_string(), variant.to_string())
        } else {
            path::PathElement::Field(field_name.to_string())
        };

        // Create the context for the child message.
        let mut field_context = context::Context {
            output: &mut field_output,
            state: context.state,
            breadcrumb: &mut context.breadcrumb.next(path_element.clone()),
            config: context.config,
        };

        // Call the provided parser function.
        let result = parser(field_input, &mut field_context)
            .map_err(|cause| {
                diagnostic!(&mut field_context, Error, cause);
            })
            .ok();

        // Handle any fields not handled by the provided parse function.
        handle_unknown_proto_fields(field_input, &mut field_context, unknown_subtree);

        // Push and return the completed node.
        let field_output = Arc::new(field_output);
        context.output.data.push(tree::NodeData::Child(tree::Child {
            path_element,
            node: field_output.clone(),
            recognized: !unknown_subtree,
        }));

        // Run the validator.
        if let Err(cause) = validator(input, context, &field_output) {
            diagnostic!(context, Error, cause);
        }

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
        proto_required_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $input,
            $context,
            &$input.$field.as_ref(),
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Convenience/shorthand macro for parsing required protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_required_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_boxed_required_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $input,
            $context,
            &$input.$field,
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Convenience/shorthand macro for parsing primitive protobuf fields.
macro_rules! proto_primitive_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_primitive_field!($input, $context, $field, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_primitive_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $input,
            $context,
            &Some(&$input.$field),
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Parse and push a required field of some message type. If the field is
/// not populated, a MissingField diagnostic is pushed automatically, and
/// an empty node is returned as an error recovery placeholder.
pub fn push_proto_required_field<TP, TF, TR, FP, FV>(
    input: &TP,
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Arc<tree::Node>, Option<TR>)
where
    TF: InputNode,
    FP: Fn(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&TP, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    if let (Some(node), result) = push_proto_field(
        input,
        context,
        field,
        field_name,
        unknown_subtree,
        parser,
        validator,
    ) {
        (node, result)
    } else {
        diagnostic!(context, Error, ProtoMissingField, field_name);
        (Arc::new(TF::type_to_node()), None)
    }
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
        proto_repeated_field!($input, $context, $field, $parser, |_, _, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_repeated_field(
            $input,
            $context,
            &$input.$field,
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Parse and push a repeated field of some message type. If specified, the
/// given validator function will be called in the current context
/// immediately after each repetition of the field is handled, allowing
/// field-specific validation to be done.
pub fn push_proto_repeated_field<TP, TF, TR, FP, FV>(
    input: &TP,
    context: &mut context::Context,
    field: &[TF],
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Vec<Arc<tree::Node>>, Vec<Option<TR>>)
where
    TF: InputNode,
    FP: Fn(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&TP, &mut context::Context, &tree::Node, usize) -> diagnostic::Result<()>,
{
    if !context
        .breadcrumb
        .fields_parsed
        .insert(field_name.to_string())
    {
        panic!("field {} was parsed multiple times", field_name);
    }

    field
        .iter()
        .enumerate()
        .map(|(index, field_input)| {
            // Create the node for the child message.
            let mut field_output = field_input.data_to_node();

            // Create the path element for referring to the child node.
            let path_element = path::PathElement::Repeated(field_name.to_string(), index);

            // Create the context for the child message.
            let mut field_context = context::Context {
                output: &mut field_output,
                state: context.state,
                breadcrumb: &mut context.breadcrumb.next(path_element.clone()),
                config: context.config,
            };

            // Call the provided parser function.
            let result = parser(field_input, &mut field_context)
                .map_err(|cause| {
                    diagnostic!(&mut field_context, Error, cause);
                })
                .ok();

            // Handle any fields not handled by the provided parse function.
            handle_unknown_proto_fields(field_input, &mut field_context, unknown_subtree);

            // Push the completed node.
            let field_output = Arc::new(field_output);
            context.output.data.push(tree::NodeData::Child(tree::Child {
                path_element,
                node: field_output.clone(),
                recognized: !unknown_subtree,
            }));

            // Run the validator.
            if let Err(cause) = validator(input, context, &field_output, index) {
                diagnostic!(context, Error, cause);
            }

            (field_output, result)
        })
        .unzip()
}

//=============================================================================
// Unknown protobuf field handling
//=============================================================================

/// Handle all fields that haven't already been handled. If unknown_subtree
/// is false, this also generates a diagnostic message if there were
/// populated/non-default unhandled fields.
pub fn handle_unknown_proto_fields<T: InputNode>(
    input: &T,
    context: &mut context::Context,
    unknown_subtree: bool,
) {
    if input.parse_unknown(context) && !unknown_subtree {
        let mut fields = vec![];
        for data in context.output.data.iter() {
            if let tree::NodeData::Child(child) = data {
                if !child.recognized {
                    fields.push(child.path_element.to_string_without_dot());
                }
            }
        }
        if !fields.is_empty() {
            let fields: String =
                itertools::Itertools::intersperse(fields.into_iter(), ", ".to_string()).collect();
            diagnostic!(context, Warning, ProtoUnknownField, fields);
        }
    }
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
) -> tree::Node
where
    T: prost::Message + InputNode + Default,
    F: FnOnce(&T, &mut context::Context) -> diagnostic::Result<()>,
    B: prost::bytes::Buf,
{
    match T::decode(buffer) {
        Err(err) => {
            // Create a minimal root node with just the decode error
            // diagnostic.
            let mut output = T::type_to_node();
            output.push_diagnostic(
                diagnostic::RawDiagnostic {
                    cause: cause!(ProtoParseFailed, err),
                    level: diagnostic::Level::Error,
                    path: path::PathBuf {
                        root: root_name,
                        elements: vec![],
                    },
                },
                config,
            );
            output
        }
        Ok(input) => {
            // Create the root node.
            let mut output = input.data_to_node();

            // Create the root context.
            let mut context = context::Context {
                output: &mut output,
                state,
                breadcrumb: &mut context::Breadcrumb::new(root_name),
                config,
            };

            // Call the provided parser function.
            if let Err(cause) = root_parser(&input, &mut context) {
                diagnostic!(&mut context, Error, cause);
            }

            // Handle any fields not handled by the provided parse function.
            handle_unknown_proto_fields(&input, &mut context, false);

            output
        }
    }
}

//=============================================================================
// YAML optional field handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional YAML fields.
macro_rules! yaml_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_yaml_field(
            $input, $context, $field, false, $parser, $validator,
        )
    };
}

/// Parse and push an optional YAML field.
pub fn push_yaml_field<TS, TR, FP, FV>(
    input: &yaml::Map,
    context: &mut context::Context,
    field_name: TS,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Option<Arc<tree::Node>>, Option<TR>)
where
    TS: AsRef<str>,
    FP: Fn(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&yaml::Map, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    let field_name = field_name.as_ref();
    if !context
        .breadcrumb
        .fields_parsed
        .insert(field_name.to_string())
    {
        panic!("field {} was parsed multiple times", field_name);
    }

    if let Some(field_input) = input.get(field_name) {
        // Create the node for the child message.
        let mut field_output = field_input.data_to_node();

        // Create the path element for referring to the child node.
        let path_element = path::PathElement::Field(field_name.to_string());

        // Create the context for the child message.
        let mut field_context = context::Context {
            output: &mut field_output,
            state: context.state,
            breadcrumb: &mut context.breadcrumb.next(path_element.clone()),
            config: context.config,
        };

        // Call the provided parser function.
        let result = parser(field_input, &mut field_context)
            .map_err(|cause| {
                diagnostic!(&mut field_context, Error, cause);
            })
            .ok();

        // Handle any fields not handled by the provided parse function.
        handle_unknown_yaml_items(field_input, &mut field_context, unknown_subtree);

        // Push and return the completed node.
        let field_output = Arc::new(field_output);
        context.output.data.push(tree::NodeData::Child(tree::Child {
            path_element,
            node: field_output.clone(),
            recognized: !unknown_subtree,
        }));

        // Run the validator.
        if let Err(cause) = validator(input, context, &field_output) {
            diagnostic!(context, Error, cause);
        }

        (Some(field_output), result)
    } else {
        (None, None)
    }
}

/// Convenience/shorthand macro for parsing optional YAML array elements.
macro_rules! yaml_element {
    ($input:expr, $context:expr, $element:expr) => {
        yaml_element!($input, $context, $element, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $element:expr, $parser:expr) => {
        yaml_element!($input, $context, $element, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $element:expr, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_yaml_element(
            $input, $context, $element, false, $parser, $validator,
        )
    };
}

/// Parse and push an optional YAML array element.
pub fn push_yaml_element<TR, FP, FV>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Option<Arc<tree::Node>>, Option<TR>)
where
    FP: Fn(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&yaml::Array, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    if !context.breadcrumb.fields_parsed.insert(index.to_string()) {
        panic!("element {} was parsed multiple times", index);
    }

    if let Some(field_input) = input.get(index) {
        // Create the node for the child message.
        let mut field_output = field_input.data_to_node();

        // Create the path element for referring to the child node.
        let path_element = path::PathElement::Index(index);

        // Create the context for the child message.
        let mut field_context = context::Context {
            output: &mut field_output,
            state: context.state,
            breadcrumb: &mut context.breadcrumb.next(path_element.clone()),
            config: context.config,
        };

        // Call the provided parser function.
        let result = parser(field_input, &mut field_context)
            .map_err(|cause| {
                diagnostic!(&mut field_context, Error, cause);
            })
            .ok();

        // Handle any fields not handled by the provided parse function.
        handle_unknown_yaml_items(field_input, &mut field_context, unknown_subtree);

        // Push and return the completed node.
        let field_output = Arc::new(field_output);
        context.output.data.push(tree::NodeData::Child(tree::Child {
            path_element,
            node: field_output.clone(),
            recognized: !unknown_subtree,
        }));

        // Run the validator.
        if let Err(cause) = validator(input, context, &field_output) {
            diagnostic!(context, Error, cause);
        }

        (Some(field_output), result)
    } else {
        (None, None)
    }
}

//=============================================================================
// YAML required field handling
//=============================================================================

/// Convenience/shorthand macro for parsing required YAML fields.
macro_rules! yaml_required_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_required_field!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_yaml_required_field(
            $input, $context, $field, false, $parser, $validator,
        )
    };
}

/// Parse and push a required field of a YAML object. If the field does not
/// exist, a MissingField diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_field<TS, TR, FP, FV>(
    input: &yaml::Map,
    context: &mut context::Context,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Arc<tree::Node>, Option<TR>)
where
    TS: AsRef<str>,
    FP: Fn(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&yaml::Map, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    if let (Some(node), result) = push_yaml_field(
        input,
        context,
        field_name,
        unknown_subtree,
        parser,
        validator,
    ) {
        (node, result)
    } else {
        diagnostic!(context, Error, YamlMissingKey, field_name);
        (
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        )
    }
}

/// Convenience/shorthand macro for parsing required YAML array elements.
macro_rules! yaml_required_element {
    ($input:expr, $context:expr, $index:expr) => {
        yaml_required_element!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $index:expr, $parser:expr) => {
        yaml_required_element!($input, $context, $field, $parser, |_, _, _| Ok(()))
    };
    ($input:expr, $context:expr, $index:expr, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_yaml_required_element(
            $input, $context, $index, false, $parser, $validator,
        )
    };
}

/// Parse and push a required element of a YAML array. If the element does not
/// exist, a MissingElement diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_element<TR, FP, FV>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> (Arc<tree::Node>, Option<TR>)
where
    FP: Fn(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
    FV: Fn(&yaml::Array, &mut context::Context, &tree::Node) -> diagnostic::Result<()>,
{
    if let (Some(node), result) =
        push_yaml_element(input, context, index, unknown_subtree, parser, validator)
    {
        (node, result)
    } else {
        diagnostic!(context, Error, YamlMissingElement, "index {}", index);
        (
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        )
    }
}

//=============================================================================
// Unknown YAML field handling
//=============================================================================

/// Handle all fields that haven't already been handled. If unknown_subtree
/// is false, this also generates a diagnostic message if there were
/// populated/non-default unhandled fields.
pub fn handle_unknown_yaml_items(
    input: &yaml::Value,
    context: &mut context::Context,
    unknown_subtree: bool,
) {
    if input.parse_unknown(context) && !unknown_subtree {
        let mut fields = vec![];
        for data in context.output.data.iter() {
            if let tree::NodeData::Child(child) = data {
                if !child.recognized {
                    fields.push(child.path_element.to_string_without_dot());
                }
            }
        }
        if !fields.is_empty() {
            let fields: String =
                itertools::Itertools::intersperse(fields.into_iter(), ", ".to_string()).collect();

            if input.as_array().is_some() {
                diagnostic!(context, Warning, YamlUnknownElement, fields);
            } else {
                diagnostic!(context, Warning, YamlUnknownKey, fields);
            }
        }
    }
}

//=============================================================================
// YAML root handling
//=============================================================================

/// Attempts to resolve a URI.
fn resolve_uri(uri: &str, config: &config::Config) -> diagnostic::Result<config::BinaryData> {
    // Apply yaml_uri_overrides configuration.
    let remapped_uri = config
        .yaml_uri_overrides
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
        remapped_uri
    } else {
        return Err(cause!(
            YamlResolutionDisabled,
            "YAML resolution for {} was disabled",
            uri
        ));
    };

    // If a custom download function is specified, use it to resolve.
    if let Some(ref resolver) = config.yaml_uri_resolver {
        return resolver(remapped_uri)
            .map_err(|x| cause!(YamlResolutionFailed, x.as_ref().to_string()));
    }

    // Parse as a URL.
    let url = match url::Url::parse(remapped_uri) {
        Ok(url) => url,
        Err(e) => {
            return Err(if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "configured URI remapping ({}) did not parse as URL: {}",
                    remapped_uri,
                    e
                )
            } else {
                cause!(
                    YamlResolutionFailed,
                    "failed to parse {} as URL: {}",
                    remapped_uri,
                    e
                )
            });
        }
    };

    // Reject anything that isn't file://-based.
    if url.scheme() != "file" {
        return Err(if is_remapped {
            cause!(
                YamlResolutionFailed,
                "configured URI remapping ({}) does not use file:// scheme",
                remapped_uri
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
                    "configured URI remapping ({}) could not be converted to file path",
                    remapped_uri
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
                    "failed to file remapping for URI ({}): {}",
                    remapped_uri,
                    e
                )
            } else {
                cause!(YamlResolutionFailed, e)
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
    let binary_data = match resolve_uri(uri, context.config) {
        Err(e) => {
            diagnostic!(context, Warning, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as UTF-8.
    let string_data = match std::str::from_utf8(binary_data.as_ref().as_ref()) {
        Err(e) => {
            diagnostic!(context, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as YAML.
    let yaml_data = match yaml_rust::YamlLoader::load_from_str(string_data) {
        Err(e) => {
            diagnostic!(context, Error, YamlParseFailed, e);
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
    let json_data = match yaml::yaml_to_json(yaml_data, &context.breadcrumb.path) {
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
                diagnostic!(context, Error, YamlSchemaValidationFailed, e);
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

    // Resolve the YAML file.
    if let Some(root_input) = load_yaml(uri, context, schema) {
        // Create an empty YamlData object.
        context.state.yaml_data = Some(extension::YamlData::default());

        // Create the node for the YAML data root.
        let mut root_output = root_input.data_to_node();

        // Create the path element for referring to the YAML data root.
        let path_element = path::PathElement::Field("data".to_string());

        // Create the context for the YAML data root.
        let mut root_context = context::Context {
            output: &mut root_output,
            state: context.state,
            breadcrumb: &mut context.breadcrumb.next(path_element.clone()),
            config: context.config,
        };

        // Create a PathBuf for the root node.
        let root_path = root_context.breadcrumb.path.to_path_buf();

        // Call the provided root parser.
        if let Err(cause) = parser(&root_input, &mut root_context) {
            diagnostic!(&mut root_context, Error, cause);
        }

        // Handle any fields not handled by the provided parse function.
        handle_unknown_yaml_items(&root_input, &mut root_context, false);

        // Push and return the completed node.
        let root_output = Arc::new(root_output);
        context.output.data.push(tree::NodeData::Child(tree::Child {
            path_element,
            node: root_output.clone(),
            recognized: true,
        }));

        // Configure the reference to the root node in the YamlData object.
        let mut node_ref = context.state.yaml_data.as_mut().unwrap();
        node_ref.data.path = root_path;
        node_ref.data.node = root_output;
    }

    // Construct the YAML data object.
    let yaml_info = Arc::new(extension::YamlInfo {
        uri: uri.to_string(),
        anchor_path: context.breadcrumb.parent.map(|x| x.path.to_path_buf()),
        data: context.state.yaml_data.take(),
    });

    // The node type will have been set as if this is a normal string
    // primitive. We want extra information though, namely the contents of the
    // YAML file. So we change the node type.
    context.output.node_type = tree::NodeType::YamlReference(yaml_info.clone());

    yaml_info
}
