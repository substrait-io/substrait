use crate::data_type;
use crate::diagnostic;
use crate::path;
use crate::proto::meta::*;
use std::rc::Rc;

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
#[derive(Clone, Debug)]
pub struct Node {
    /// The protobuf type name for the message that this object represents.
    pub node_type: NodeType,

    /// The information gathered about the message.
    ///
    /// This normally includes all the populated fields of the message that
    /// the validator knows about (note the implication here: fields that the
    /// validator does NOT know about are NOT represented, so this is not
    /// necessarily a complete representation of the incoming Substrait
    /// message) via OptionalField, RepeatedField, and OneOfField elements.
    /// These elements are however interspersed with diagnostics, type
    /// information, and unstructured comment nodes to provide context, and
    /// the validator will try to order nodes in a reasonable way.
    pub data: Vec<NodeData>,
}

impl From<NodeType> for Node {
    fn from(node_type: NodeType) -> Self {
        Node {
            node_type,
            data: vec![],
        }
    }
}

impl Node {
    /// Pushes an error message to the node information list.
    pub fn push_error(&mut self, context: &crate::Context, cause: diagnostic::Cause) {
        self.data.push(NodeData::Diagnostic(diagnostic::Diagnostic {
            cause,
            level: diagnostic::Level::Error,
            path: context.path.to_path_buf(),
        }))
    }

    /// Pushes a warning message to the node information list.
    pub fn push_warning(&mut self, context: &crate::Context, cause: diagnostic::Cause) {
        self.data.push(NodeData::Diagnostic(diagnostic::Diagnostic {
            cause,
            level: diagnostic::Level::Warning,
            path: context.path.to_path_buf(),
        }))
    }

    /// Pushes an info message to the node information list.
    pub fn push_info(&mut self, context: &crate::Context, cause: diagnostic::Cause) {
        self.data.push(NodeData::Diagnostic(diagnostic::Diagnostic {
            cause,
            level: diagnostic::Level::Info,
            path: context.path.to_path_buf(),
        }))
    }

    /// Pushes a comment to the node information list.
    pub fn push_comment<T: AsRef<str>>(&mut self, comment: T) {
        self.data
            .push(NodeData::Comment(comment.as_ref().to_string()))
    }

    /// Pushes a data type to the node information list, and saves it in the
    /// current context.
    pub fn push_type(&mut self, context: &mut crate::Context, data_type: data_type::DataType) {
        self.data.push(NodeData::DataType(data_type.clone()));
        context.data_type = Some(data_type);
    }

    /// Parse and push a protobuf optional field.
    pub fn push_proto_field<T, F, FV>(
        &mut self,
        context: &mut crate::Context,
        input: &Option<T>,
        field_name: &'static str,
        parser: F,
        validator: FV,
    ) -> Option<Rc<Node>>
    where
        T: ProtoDatum,
        F: FnOnce(&T, &mut crate::Context, &mut Node) -> crate::Result<()>,
        FV: Fn(&Node, &mut crate::Context, &mut Node) -> crate::Result<()>,
    {
        if let Some(field_input) = input {
            // Create the context for the child message.
            let mut field_context = crate::Context {
                parent: Some(context),
                path: context.path.with_field(field_name),
                data_type: None,
            };

            // Create the node for the child message.
            let mut field_output = field_input.proto_data_to_node();

            // Call the provided parser function.
            if let Err(cause) = parser(field_input, &mut field_context, &mut field_output) {
                field_output.push_error(&field_context, cause);
            }

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
            if let Err(cause) = validator(&field_output, context, self) {
                self.push_error(context, cause);
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
        context: &mut crate::Context,
        input: &Option<T>,
        field_name: &'static str,
        parser: F,
        validator: FV,
    ) -> Rc<Node>
    where
        T: ProtoDatum,
        F: FnOnce(&T, &mut crate::Context, &mut Node) -> crate::Result<()>,
        FV: Fn(&Node, &mut crate::Context, &mut Node) -> crate::Result<()>,
    {
        if let Some(node) = self.push_proto_field(context, input, field_name, parser, validator) {
            node
        } else {
            self.push_error(
                context,
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
        context: &mut crate::Context,
        input: &[T],
        field_name: &'static str,
        parser: F,
        validator: FV,
    ) -> Vec<Rc<Node>>
    where
        T: ProtoDatum,
        F: Fn(&T, &mut crate::Context, &mut Node) -> crate::Result<()>,
        FV: Fn(usize, &Node, &mut crate::Context, &mut Node) -> crate::Result<()>,
    {
        input
            .iter()
            .enumerate()
            .map(|(index, field_input)| {
                // Create the context for the child message.
                let mut field_context = crate::Context {
                    parent: Some(context),
                    path: context.path.with_repeated(field_name, index),
                    data_type: None,
                };

                // Create the node for the child message.
                let mut field_output = field_input.proto_data_to_node();

                // Call the provided parser function.
                if let Err(cause) = parser(field_input, &mut field_context, &mut field_output) {
                    field_output.push_error(&field_context, cause);
                }

                // Push the completed node.
                let field_output = Rc::new(field_output);
                self.data.push(NodeData::RepeatedField(
                    field_name,
                    index,
                    field_output.clone(),
                ));

                // Run the validator.
                if let Err(cause) = validator(index, &field_output, context, self) {
                    self.push_error(context, cause);
                }

                field_output
            })
            .collect()
    }
}

/// The original data type that the node represents, to (in theory) allow the
/// original structure of the plan to be recovered from the documentation tree.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum NodeData {
    /// For protobuf nodes, indicates that the node has a populated optional
    /// field (optional in the protobuf sense, may be mandatory in Substrait
    /// context) with the given field name and data. For YAML maps, represents
    /// a key-value pair.
    Field(&'static str, Rc<Node>),

    /// For YAML maps, indicates that a field was specified that the validator
    /// doesn't know about.
    UnknownField(String, Rc<Node>),

    /// For protobuf nodes, indicates that the node has a populated repeated
    /// field with the given field name, index, and data. The elements of
    /// repeated nodes are always stored in-order and without gaps, but the
    /// elements may be interspersed with metadata (diagnostics, comments,
    /// etc).
    RepeatedField(&'static str, usize, Rc<Node>),

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

    /// Provides type information for this message. Depending on the message,
    /// this may be a struct or named struct representing a schema, or it may
    /// represent the type of some scalar expression. Multiple TypeInfo nodes
    /// may be present, in particular for relations that perform multiple
    /// operations in one go (for example read, project, emit). The TypeInfo
    /// and operation description *Field nodes are then ordered by data flow.
    /// In particular, the last TypeInfo node always represents the type of the
    /// final result of a node.
    DataType(data_type::DataType),

    /// Used for adding unstructured additional information to a message,
    /// wherever this may aid human understanding of a message.
    Comment(String),
}

/// A reference to a node elsewhere in the tree.
#[derive(Clone, Debug)]
pub struct NodeReference {
    /// Absolute path to the node.
    pub path: path::PathBuf,

    /// Link to the node.
    pub node: Rc<Node>,
}
