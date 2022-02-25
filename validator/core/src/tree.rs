use crate::comment;
use crate::context;
use crate::data_type;
use crate::diagnostic;
use crate::extension;
use crate::parsing;
use crate::path;
use crate::primitives;
use crate::proto::meta::*;
use std::collections::VecDeque;
use std::sync::Arc;

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
    /// Pushes a diagnostic into the node. This also evaluates its adjusted
    /// error level.
    pub fn push_diagnostic(&mut self, diag: diagnostic::Diagnostic, config: &context::Config) {
        // Get the configured level limits for this diagnostic. First try the
        // classification of the diagnostic itself, then its group, and then
        // finally Unclassified. If no entries exist, simply yield
        // (Info, Error), which is no-op.
        let (min, max) = config
            .diagnostic_level_overrides
            .get(&diag.cause.classification)
            .or_else(|| {
                config
                    .diagnostic_level_overrides
                    .get(&diag.cause.classification.group())
            })
            .or_else(|| {
                config
                    .diagnostic_level_overrides
                    .get(&diagnostic::Classification::Unclassified)
            })
            .unwrap_or(&(diagnostic::Level::Info, diagnostic::Level::Error));

        // Adjust the level.
        let adjusted_level = if diag.level < *min {
            *min
        } else if diag.level > *max {
            *max
        } else {
            diag.level
        };
        let adjusted = diag.adjust_level(adjusted_level);

        self.data.push(NodeData::Diagnostic(adjusted));
    }

    /// Parses/validates the given binary serialization of a protobuffer using
    /// the given (root) parser/validator.
    pub fn parse_proto<T, F, B>(
        buffer: B,
        root_name: &'static str,
        root_parser: F,
        state: &mut context::State,
        config: &context::Config,
    ) -> Self
    where
        T: prost::Message + ProtoDatum + Default,
        F: FnOnce(&T, &mut context::Context) -> diagnostic::Result<()>,
        B: prost::bytes::Buf,
    {
        match T::decode(buffer) {
            Err(err) => {
                // Create a minimal root node with just the prot

                let mut output = T::proto_type_to_node();
                output.push_diagnostic(
                    diagnostic::Diagnostic {
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
                let mut output = input.proto_data_to_node();

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
                parsing::handle_unknown_proto_fields(&input, &mut context, false);

                output
            }
        }
    }

    /// Returns an iterator that iterates over all nodes depth-first.
    pub fn iter_flattened_nodes(&self) -> FlattenedNodeIter {
        FlattenedNodeIter {
            remaining: VecDeque::from(vec![self]),
        }
    }

    /// Returns an iterator that iterates over all NodeData objects in the
    /// order in which they were defined.
    pub fn iter_flattened_node_data(&self) -> FlattenedNodeDataIter {
        FlattenedNodeDataIter {
            remaining: self.data.iter().rev().collect(),
        }
    }

    /// Iterates over all diagnostics in the tree.
    pub fn iter_diagnostics(&self) -> impl Iterator<Item = &diagnostic::AdjustedDiagnostic> + '_ {
        self.iter_flattened_node_data().filter_map(|x| match x {
            NodeData::Diagnostic(d) => Some(d),
            _ => None,
        })
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
    ProtoPrimitive(&'static str, primitives::PrimitiveData),

    /// The associated node represents an unpopulated oneof field. This is used
    /// for an error recovery node when a required oneof field is not
    /// populated.
    ProtoMissingOneOf,

    /// Used for anchor/reference-based references to other nodes.
    NodeReference(u64, NodeReference),

    /// Used for resolved YAML URIs, in order to include the parse result and
    /// documentation for the referenced YAML (if available), in addition to
    /// the URI itself.
    YamlReference(Arc<extension::YamlInfo>),

    /// The associated node represents a YAML map. The contents of the map are
    /// described using Field and UnknownField.
    YamlMap,

    /// The associated node represents a YAML array. The contents of the array
    /// are described using ArrayElement datums.
    YamlArray,

    /// The associated node represents a YAML primitive.
    YamlPrimitive(primitives::PrimitiveData),
}

/// Information nodes for a parsed protobuf message.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeData {
    /// A reference to a child node in the tree.
    Child(Child),

    /// Indicates that parsing/validating this message resulted in some
    /// diagnostic message being emitted. The secondary error level is the
    /// modified level via
    Diagnostic(diagnostic::AdjustedDiagnostic),

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
    Comment(comment::Comment),
}

/// Reference to a child node in the tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Child {
    /// Path element identifying the relation of this child node to its parent.
    pub path_element: path::PathElement,

    /// The child node.
    pub node: Arc<Node>,

    /// Whether the validator recognized/expected the field or element that
    /// this child represents. Fields/elements may be unrecognized simply
    /// because validation is not implemented for them yet. In any case, this
    /// flag indicates that the subtree represented by this node could not be
    /// validated.
    pub recognized: bool,
}

/// A reference to a node elsewhere in the tree.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeReference {
    /// Absolute path to the node.
    pub path: path::PathBuf,

    /// Link to the node.
    pub node: Arc<Node>,
}

pub struct FlattenedNodeIter<'a> {
    remaining: VecDeque<&'a Node>,
}
impl<'a> Iterator for FlattenedNodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_node = self.remaining.pop_back();
        if let Some(node) = maybe_node {
            self.remaining
                .extend(node.data.iter().rev().filter_map(|x| -> Option<&Node> {
                    if let NodeData::Child(child) = x {
                        Some(&child.node)
                    } else {
                        None
                    }
                }));
        }
        maybe_node
    }
}

pub struct FlattenedNodeDataIter<'a> {
    remaining: VecDeque<&'a NodeData>,
}

impl<'a> Iterator for FlattenedNodeDataIter<'a> {
    type Item = &'a NodeData;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_node_data = self.remaining.pop_back();
        if let Some(NodeData::Child(child)) = maybe_node_data {
            self.remaining.extend(child.node.data.iter().rev())
        }
        maybe_node_data
    }
}
