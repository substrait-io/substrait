use crate::context;
use crate::data_type;
use crate::diagnostic;
use crate::path;
use crate::proto::meta::*;
use std::collections::HashSet;
use std::rc::Rc;

/// Convenience/shorthand macro for pushing diagnostic messages to a node.
macro_rules! diagnostic {
    ($output:expr, $context:expr, $level:ident, $cause:expr) => {
        $output.push_diagnostic($context, diagnostic::Level::$level, $cause)
    };
    ($output:expr, $context:expr, $level:ident, $cause:ident, $($fmts:expr),*) => {
        $output.push_diagnostic($context, diagnostic::Level::$level, diagnostic::Cause::$cause(format!($($fmts),*)))
    };
}

/// Convenience/shorthand macro for pushing comments to a node.
macro_rules! comment {
    ($output:expr, $($fmts:expr),*) => {
        $output.push_comment(format!($($fmts),*))
    };
}

/// Convenience/shorthand macro for pushing type information to a node. Note
/// that this macro isn't shorter than just using push_type() directly; it
/// exists for symmetry.
macro_rules! set_type {
    ($output:expr, $typ:expr) => {
        $output.push_type($typ)
    };
}

/// Convenience/shorthand macro for parsing optional protobuf fields.
macro_rules! proto_field {
    ($output:expr, $context:expr, $input:expr, $field:ident) => {
        proto_field!($output, $context, $input, $field, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr) => {
        proto_field!($output, $context, $input, $field, parser, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr, $validator:expr) => {
        $output.push_proto_field(
            $context,
            &$input.$field.as_ref(),
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

macro_rules! proto_boxed_field {
    ($output:expr, $context:expr, $input:expr, $field:ident) => {
        proto_boxed_field!($output, $context, $input, $field, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr) => {
        proto_boxed_field!($output, $context, $input, $field, parser, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr, $validator:expr) => {
        $output.push_proto_field(
            $context,
            &$input.$field,
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Convenience/shorthand macro for parsing required protobuf fields.
macro_rules! proto_required_field {
    ($output:expr, $context:expr, $input:expr, $field:ident) => {
        proto_required_field!($output, $context, $input, $field, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr) => {
        proto_required_field!($output, $context, $input, $field, parser, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr, $validator:expr) => {
        $output.push_proto_required_field(
            $context,
            &$input.$field.as_ref(),
            stringify!($field),
            $parser,
            $validator,
        )
    };
}

#[allow(unused_macros)]
macro_rules! proto_boxed_required_field {
    ($output:expr, $context:expr, $input:expr, $field:ident) => {
        proto_boxed_required_field!($output, $context, $input, $field, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr) => {
        proto_boxed_required_field!($output, $context, $input, $field, parser, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr, $validator:expr) => {
        $output.push_proto_required_field(
            $context,
            &$input.$field,
            stringify!($field),
            $parser,
            $validator,
        )
    };
}

/// Convenience/shorthand macro for parsing repeated protobuf fields.
macro_rules! proto_repeated_field {
    ($output:expr, $context:expr, $input:expr, $field:ident) => {
        proto_repeated_field!($output, $context, $input, $field, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr) => {
        proto_repeated_field!($output, $context, $input, $field, parser, |_, _, _| Ok(()))
    };
    ($output:expr, $context:expr, $input:expr, $field:ident, $parser:expr, $validator:expr) => {
        $output.push_proto_repeated_field(
            $context,
            &$input.$field,
            stringify!($field),
            false,
            $parser,
            $validator,
        )
    };
}

/// Node for a semi-structured documentation-like tree representation of a
/// parsed Substrait plan. The intention is for this to be serialized into
/// some human-readable format.
///
/// Note: although it should be possible to reconstruct the entire plan from
/// the information contained in the tree, the tree is only intended to be
/// converted to structured human-readable documentation for the plan. It is
/// expressly NOT intended to be read as a form of AST by a downstream
/// process, and therefore isn't nearly as strictly-typed as you would
/// otherwise want it to be. Protobuf itself is already a reasonable format
/// for this!
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// The type of node.
    pub node_type: NodeType,

    /// The type of data returned by this node, if any. Depending on the
    /// message and context, this may represent a table schema or scalar
    /// data.
    pub data_type: Option<data_type::DataType>,

    /// The information gathered about the message.
    ///
    /// This normally includes all child nodes for this message, possibly
    /// interspersed with diagnostics, type information, and unstructured
    /// comment nodes to provide context, all ordered in a reasonable way.
    /// Note however that this information is intended to be understood by
    /// a human, not by the validator itself (aside from serialization to a
    /// human-readable notation).
    pub data: Vec<NodeData>,
}

impl From<NodeType> for Node {
    fn from(node_type: NodeType) -> Self {
        Node {
            node_type,
            data_type: None,
            data: vec![],
        }
    }
}

impl Node {
    /// Pushes a diagnostic message to the node information list.
    pub fn push_diagnostic(
        &mut self,
        breadcrumb: &context::Breadcrumb,
        level: diagnostic::Level,
        cause: diagnostic::Cause,
    ) {
        self.data.push(NodeData::Diagnostic(diagnostic::Diagnostic {
            cause,
            level,
            path: breadcrumb.path.to_path_buf(),
        }))
    }

    /// Pushes a comment to the node information list.
    pub fn push_comment<T: AsRef<str>>(&mut self, comment: T) {
        self.data
            .push(NodeData::Comment(comment.as_ref().to_string()))
    }

    /// Pushes a data type to the node information list, and saves it in the
    /// current context.
    pub fn push_type(&mut self, data_type: data_type::DataType) {
        self.data.push(NodeData::DataType(data_type.clone()));
        self.data_type = Some(data_type);
    }

    /// Parse and push a protobuf optional field.
    pub fn push_proto_field<T, F, FV>(
        &mut self,
        breadcrumb: &mut context::Breadcrumb,
        input: &Option<impl std::ops::Deref<Target = T>>,
        field_name: &'static str,
        unknown_subtree: bool,
        parser: F,
        validator: FV,
    ) -> Option<Rc<Node>>
    where
        T: ProtoDatum,
        F: FnOnce(&T, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
        FV: Fn(&Node, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
    {
        if !breadcrumb.fields_parsed.insert(field_name.to_string()) {
            panic!("field {} was parsed multiple times", field_name);
        }

        if let Some(field_input) = input {
            let field_input = field_input.deref();

            // Create the breadcrumb for the child message.
            let mut field_breadcrumb = context::Breadcrumb {
                parent: Some(breadcrumb),
                path: breadcrumb.path.with_field(field_name),
                fields_parsed: HashSet::new(),
            };

            // Create the node for the child message.
            let mut field_output = field_input.proto_data_to_node();

            // Call the provided parser function.
            if let Err(cause) = parser(field_input, &mut field_breadcrumb, &mut field_output) {
                field_output.push_diagnostic(&field_breadcrumb, diagnostic::Level::Error, cause);
            }

            // Handle any fields not handled by the provided parse function.
            field_output.handle_unknown_fields(&mut field_breadcrumb, field_input, unknown_subtree);

            // Push and return the completed node.
            let field_output = Rc::new(field_output);
            self.data.push(
                if let Some(variant_name) = field_input.proto_data_variant() {
                    NodeData::OneOfField(field_name, variant_name, field_output.clone())
                } else {
                    NodeData::Field(field_name, field_output.clone())
                },
            );

            // Run the validator.
            if let Err(cause) = validator(&field_output, breadcrumb, self) {
                self.push_diagnostic(breadcrumb, diagnostic::Level::Error, cause);
            }

            Some(field_output)
        } else {
            None
        }
    }

    /// Parse and push a required field of some message type. If the field is
    /// not populated, a MissingField diagnostic is pushed automatically, and
    /// an empty node is returned as an error recovery placeholder.
    pub fn push_proto_required_field<T, F, FV>(
        &mut self,
        breadcrumb: &mut context::Breadcrumb,
        input: &Option<impl std::ops::Deref<Target = T>>,
        field_name: &'static str,
        parser: F,
        validator: FV,
    ) -> Rc<Node>
    where
        T: ProtoDatum,
        F: FnOnce(&T, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
        FV: Fn(&Node, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
    {
        if let Some(node) =
            self.push_proto_field(breadcrumb, input, field_name, false, parser, validator)
        {
            node
        } else {
            self.push_diagnostic(
                breadcrumb,
                diagnostic::Level::Error,
                diagnostic::Cause::MissingField(field_name.to_string()),
            );
            Rc::new(T::proto_type_to_node())
        }
    }

    /// Parse and push a repeated field of some message type. If specified, the
    /// given validator function will be called in the current context
    /// immediately after each repetition of the field is handled, allowing
    /// field-specific validation to be done.
    pub fn push_proto_repeated_field<T, F, FV>(
        &mut self,
        breadcrumb: &mut context::Breadcrumb,
        input: &[T],
        field_name: &'static str,
        unknown_subtree: bool,
        parser: F,
        validator: FV,
    ) -> Vec<Rc<Node>>
    where
        T: ProtoDatum,
        F: Fn(&T, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
        FV: Fn(usize, &Node, &mut context::Breadcrumb, &mut Node) -> crate::Result<()>,
    {
        if !breadcrumb.fields_parsed.insert(field_name.to_string()) {
            panic!("field {} was parsed multiple times", field_name);
        }

        input
            .iter()
            .enumerate()
            .map(|(index, field_input)| {
                // Create the context for the child message.
                let mut field_breadcrumb = context::Breadcrumb {
                    parent: Some(breadcrumb),
                    path: breadcrumb.path.with_repeated(field_name, index),
                    fields_parsed: HashSet::new(),
                };

                // Create the node for the child message.
                let mut field_output = field_input.proto_data_to_node();

                // Call the provided parser function.
                if let Err(cause) = parser(field_input, &mut field_breadcrumb, &mut field_output) {
                    field_output.push_diagnostic(
                        &field_breadcrumb,
                        diagnostic::Level::Error,
                        cause,
                    );
                }

                // Handle any fields not handled by the provided parse function.
                field_output.handle_unknown_fields(
                    &mut field_breadcrumb,
                    field_input,
                    unknown_subtree,
                );

                // Push the completed node.
                let field_output = Rc::new(field_output);
                self.data.push(NodeData::RepeatedField(
                    field_name,
                    index,
                    field_output.clone(),
                ));

                // Run the validator.
                if let Err(cause) = validator(index, &field_output, breadcrumb, self) {
                    self.push_diagnostic(breadcrumb, diagnostic::Level::Error, cause);
                }

                field_output
            })
            .collect()
    }

    /// Handle all fields that haven't already been handled. If unknown_subtree
    /// is false, this also generates a diagnostic message if there were
    /// populated/non-default unhandled fields.
    pub fn handle_unknown_fields<T: ProtoDatum>(
        &mut self,
        breadcrumb: &mut context::Breadcrumb,
        input: &T,
        unknown_subtree: bool,
    ) {
        if input.proto_parse_unknown(breadcrumb, self) && !unknown_subtree {
            let mut fields = HashSet::new();
            for data in self.data.iter() {
                match data {
                    NodeData::UnknownField(field, _) => {
                        fields.insert(field.clone());
                    }
                    NodeData::UnknownRepeatedField(field, _, _) => {
                        fields.insert(field.clone());
                    }
                    _ => {}
                }
            }
            if !fields.is_empty() {
                let fields: String =
                    itertools::Itertools::intersperse(fields.into_iter(), ", ".to_string())
                        .collect();
                self.push_diagnostic(
                    breadcrumb,
                    diagnostic::Level::Warning,
                    diagnostic::Cause::UnknownField(fields),
                );
            }
        }
    }
}

/// The original data type that the node represents, to (in theory) allow the
/// original structure of the plan to be recovered from the documentation tree.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    /// The associated node represents a protobuf message of the given type
    /// (full protobuf path). The contents of the message are described using
    /// Field, RepeatedField, and OneOfField.
    ProtoMessage(&'static str),

    /// The associated node represents a protobuf primitive value of the given
    /// type and with the given data.
    ProtoPrimitive(&'static str, ProtoPrimitiveData),

    /// The associated node represents an unpopulated oneof field.
    ProtoMissingOneOf,

    /// Used for anchor/reference-based references to other nodes.
    Reference(u64, NodeReference),

    /// Used for resolved YAML URIs, in order to include the resolution result,
    /// parse result, and documentation for the referenced YAML, in addition to
    /// the URI itself.
    YamlUri(String, diagnostic::Result<Rc<Node>>),

    /// The associated node represents a YAML map. The contents of the map are
    /// described using Field and UnknownField.
    YamlMap,

    /// The associated node represents a YAML array. The contents of the array
    /// are described using ArrayElement datums.
    YamlArray,

    /// The associated node represents a YAML primitive
    YamlPrimitive(ProtoPrimitiveData),
}

/// Information nodes for a parsed protobuf message.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeData {
    /// For protobuf nodes, indicates that the node has a populated optional
    /// field (optional in the protobuf sense, may be mandatory in Substrait
    /// context) with the given field name and data. For YAML maps, represents
    /// a key-value pair.
    Field(&'static str, Rc<Node>),

    /// For protobuf nodes and YAML maps, indicates that a field exists/was
    /// specified that the validator doesn't know about.
    UnknownField(String, Rc<Node>),

    /// For protobuf nodes, indicates that the node has a populated repeated
    /// field with the given field name, index, and data. The elements of
    /// repeated nodes are always stored in-order and without gaps, but the
    /// elements may be interspersed with metadata (diagnostics, comments,
    /// etc).
    RepeatedField(&'static str, usize, Rc<Node>),

    /// Combination of UnknownField and RepeatedField.
    UnknownRepeatedField(String, usize, Rc<Node>),

    /// For protobuf nodes, indicates that the node as a populated OneOf field
    /// with the given field name, variant name, and data.
    OneOfField(&'static str, &'static str, Rc<Node>),

    /// For YAML arrays, provides information for the given index in the array.
    /// The array elements are always stored in-order and without gaps, but the
    /// elements may be interspersed with metadata (diagnostics, comments,
    /// etc).
    ArrayElement(usize, Rc<Node>),

    /// Indicates that parsing/validating this message resulted in some
    /// diagnostic message being emitted.
    Diagnostic(diagnostic::Diagnostic),

    /// Provides (intermediate) type information for this node. Depending on
    /// the message, this may be a struct or named struct representing a
    /// schema, or it may represent the type of some scalar expression.
    /// Multiple TypeInfo nodes may be present, in particular for relations
    /// that perform multiple operations in one go (for example read, project,
    /// emit). The TypeInfo and operation description *Field nodes are then
    /// ordered by data flow. In particular, the last TypeInfo node always
    /// represents the type of the final result of a node.
    DataType(data_type::DataType),

    /// Used for adding unstructured additional information to a message,
    /// wherever this may aid human understanding of a message.
    Comment(String),
}

/// A reference to a node elsewhere in the tree.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeReference {
    /// Absolute path to the node.
    pub path: path::PathBuf,

    /// Link to the node.
    pub node: Rc<Node>,
}
