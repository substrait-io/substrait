// FIXME: remove once validation code is finished. Also re-evaluate the
// usefulness of the validator functions at that stage, i.e. remove them if
// they weren't useful anywhere.
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::comment;
use crate::context;
use crate::data_type;
use crate::diagnostic;
use crate::path;
use crate::primitives;
use crate::proto::meta::*;
use crate::tree;
use crate::yaml;
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
        crate::parsing::push_diagnostic($context, crate::diagnostic::Level::$level, $cause)
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
        diagnostic::Diagnostic {
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
        crate::parsing::push_comment($context, format!($($fmts),*), None)
    };
}

/// Convenience/shorthand macro for pushing comments to a node.
macro_rules! link {
    ($context:expr, $link:expr, $($fmts:expr),*) => {
        crate::parsing::push_comment($context, format!($($fmts),*), Some($link))
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
        crate::parsing::push_data_type($context, $typ)
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
        crate::parsing::push_proto_field(
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
        crate::parsing::push_proto_field(
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
    TF: ProtoDatum,
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
        let mut field_output = field_input.proto_data_to_node();

        // Create the path element for referring to the child node.
        let path_element = if let Some(variant) = field_input.proto_data_variant() {
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
        crate::parsing::push_proto_required_field(
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
        crate::parsing::push_proto_required_field(
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
        crate::parsing::push_proto_required_field(
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
    TF: ProtoDatum,
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
        (Arc::new(TF::proto_type_to_node()), None)
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
        crate::parsing::push_proto_repeated_field(
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
    TF: ProtoDatum,
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
            let mut field_output = field_input.proto_data_to_node();

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
pub fn handle_unknown_proto_fields<T: ProtoDatum>(
    input: &T,
    context: &mut context::Context,
    unknown_subtree: bool,
) {
    if input.proto_parse_unknown(context) && !unknown_subtree {
        let mut fields = vec![];
        for data in context.output.data.iter() {
            if let tree::NodeData::Child(child) = data {
                if !child.recognized {
                    fields.push(child.path_element.to_string());
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
        crate::parsing::push_yaml_field($input, $context, $field, false, $parser, $validator)
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
        let mut field_output = yaml::yaml_to_node(field_input);

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
        crate::parsing::push_yaml_element($input, $context, $element, false, $parser, $validator)
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
        let mut field_output = yaml::yaml_to_node(field_input);

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
        crate::parsing::push_yaml_required_field(
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
        diagnostic!(context, Error, YamlMissingField, field_name);
        (
            Arc::new(tree::NodeType::YamlPrimitive(primitives::PrimitiveData::Null).into()),
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
        crate::parsing::push_yaml_required_element(
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
            Arc::new(tree::NodeType::YamlPrimitive(primitives::PrimitiveData::Null).into()),
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
    match input {
        yaml::Value::Array(array) => {
            let mut unknown_indices = vec![];
            for (index, _) in array.iter().enumerate() {
                if context.breadcrumb.fields_parsed.insert(index.to_string()) {
                    unknown_indices.push(index);
                    push_yaml_element(array, context, index, true, |_, _| Ok(()), |_, _, _| Ok(()));
                }
            }
            if !unknown_subtree && !unknown_indices.is_empty() {
                let elements: String = itertools::Itertools::intersperse(
                    unknown_indices.iter().map(|x| x.to_string()),
                    ", ".to_string(),
                )
                .collect();
                diagnostic!(context, Warning, YamlUnknownElement, elements);
            }
        }
        yaml::Value::Object(object) => {
            let mut all_fields: Vec<_> = object.keys().collect();
            all_fields.sort();
            let mut unknown_fields = vec![];
            for field_name in all_fields.iter() {
                if context
                    .breadcrumb
                    .fields_parsed
                    .insert((*field_name).clone())
                {
                    unknown_fields.push((*field_name).clone());
                    push_yaml_field(
                        object,
                        context,
                        field_name,
                        true,
                        |_, _| Ok(()),
                        |_, _, _| Ok(()),
                    );
                }
            }
            if !unknown_subtree && !unknown_fields.is_empty() {
                let fields: String =
                    itertools::Itertools::intersperse(unknown_fields.into_iter(), ", ".to_string())
                        .collect();
                diagnostic!(context, Warning, YamlUnknownField, fields);
            }
        }
        _ => {}
    }
}

//=============================================================================
// Utilities for tests
//=============================================================================

/// Convenience/shorthand macro for the with_context function.
macro_rules! with_context {
    ($function:expr, ()) => {
        with_context!(&mut crate::context::State::default(), $function, ())
    };
    ($function:expr, ($($args:expr),*)) => {
        with_context!(&mut crate::context::State::default(), $function, ($($args),*))
    };
    (config = $config:expr, $function:expr, ()) => {
        with_context!(&mut crate::context::State::default(), $config, $function, ())
    };
    (config = $config:expr, $function:expr, ($($args:expr),*)) => {
        with_context!(&mut crate::context::State::default(), $config, $function, ($($args),*))
    };
    ($state:expr, $function:expr, ()) => {
        with_context!($state, &crate::context::Config::default(), $function, ())
    };
    ($state:expr, $function:expr, ($($args:expr),*)) => {
        with_context!($state, &crate::context::Config::default(), $function, ($($args),*))
    };
    ($state:expr, $config:expr, $function:expr, ()) => {
        crate::parsing::with_context(
            $state,
            $config,
            $function,
        )
    };
    ($state:expr, $config:expr, $function:expr, ($($args:expr),*)) => {
        crate::parsing::with_context(
            $state,
            $config,
            |y| $function($($args),*, y),
        )
    };
}

// Creates a temporary context and calls a function with it.
pub fn with_context<R, F: FnOnce(&mut context::Context) -> R>(
    state: &mut context::State,
    config: &context::Config,
    function: F,
) -> (R, tree::Node) {
    // Create the root node for the output.
    let mut output = tree::NodeType::ProtoMessage("temp").into();

    // Create a temporary context.
    let mut context = context::Context {
        output: &mut output,
        state,
        breadcrumb: &mut context::Breadcrumb::new("temp"),
        config,
    };

    // Call the function.
    let result = function(&mut context);

    // Return the results.
    (result, output)
}
