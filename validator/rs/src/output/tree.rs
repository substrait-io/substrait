// SPDX-License-Identifier: Apache-2.0

//! Module for the output tree structure.
//!
//! This module provides the types for the tree structure that constitutes
//! the output of the validator. The nodes in the tree are intended to
//! correspond exactly to the protobuf messages, primitives, and YAML values
//! (the latter actually using the JSON object model) that constitute the
//! incoming plan. Likewise, the structure of the tree is the same as the
//! input. However, unlike the input:
//!
//!  - All nodes and the relations between them are encapsulated in generic
//!    types, independent from the corresponding messages/values in the
//!    original tree. This allows the tree to be traversed by generic code
//!    with no understanding of Substrait.
//!  - Additional information can be attached to the nodes, edges, and
//!    between the edges, such as diagnostic messages and data type
//!    information.
//!
//! The node type for the output trees is [`Node`]. This structure contains
//! a single [`NodeType`] enum variant and zero or more [`NodeData`] enum
//! variants in an ordered sequence to form the tree structure; [`NodeType`]
//! includes information about the node itself, while the [`NodeData`]
//! elements represent edges to other nodes ([`Child`]) or contextual
//! information. A subtree might look something like this:
//!
//! ```text
//!                 Node ---> ProtoMessage                   } Parent node
//!                  |
//!   .--------------'--------------.
//!   |         |         |         |
//!   v         v         v         v
//! Child  Diagnostic  Comment    Child                      } Edges
//!   |                             |
//!   v                             v
//! Node ---> ProtoPrimitive      Node ---> ProtoMessage     } Child nodes
//!            |                    |
//!            '-> PrimitiveData    :
//! ```
//!
//! Note that the [`Child`] struct includes information about how the child
//! node relates to its parent (which field, array element, etc) via
//! [`PathElement`](path::PathElement), such that the original tree structure
//! could in theory be completely reconstructed.
//!
//! Nevertheless, the conversion from protobuf/YAML to this tree structure is
//! only intended to be a one-way street; indeed, the output tree is not
//! intended to ever be treated as some executable query plan by a computer at
//! all. It serves only as an intermediate format for documentation, debug,
//! and/or validation output. The [export](mod@crate::export) module deals with
//! breaking this internal representation down further, into (file) formats
//! that are not specific to the Substrait validator.

use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::path;
use crate::output::primitive_data;
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
    /// The type of a node in terms of plan semantics.
    pub class: Class,

    /// An optional brief description of the node. This can be regarded as
    /// a comment placed at the start of the data vector, but it is usually
    /// only set at the end of the parse function.
    pub brief: Option<comment::Brief>,

    /// An optional comment summarizing what this node does. This can be
    /// regarded as a comment placed at the start of the data vector (just
    /// after brief, if brief is also defined), but it is usually only set
    /// at the end of the parse function.
    pub summary: Option<comment::Comment>,

    /// The type of node in terms of what it represents in the original
    /// data structure.
    pub node_type: NodeType,

    /// The type of data returned by this node, if any. Depending on the
    /// message and context, this may represent a table schema or scalar
    /// data.
    pub data_type: Option<Arc<data_type::DataType>>,

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
            class: Class::Misc,
            brief: None,
            summary: None,
            node_type,
            data_type: None,
            data: vec![],
        }
    }
}

impl Node {
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
    pub fn iter_diagnostics(&self) -> impl Iterator<Item = &diagnostic::Diagnostic> + '_ {
        self.iter_flattened_node_data().filter_map(|x| match x {
            NodeData::Diagnostic(d) => Some(d),
            _ => None,
        })
    }

    /// Returns the first diagnostic of the highest severity level in the tree.
    pub fn get_diagnostic(&self) -> Option<&diagnostic::Diagnostic> {
        let mut result: Option<&diagnostic::Diagnostic> = None;
        for diag in self.iter_diagnostics() {
            // We can return immediately for error diagnostics, since this is the
            // highest level.
            if diag.adjusted_level == diagnostic::Level::Error {
                return Some(diag);
            }

            // For other levels, update only if the incoming diagnostic is of a
            // higher level/severity than the current one.
            if let Some(cur) = result.as_mut() {
                if diag.adjusted_level > (*cur).adjusted_level {
                    *cur = diag;
                }
            } else {
                result = Some(diag);
            }
        }
        result
    }

    /// Returns a reference to the data type that this node returns at runtime
    /// or (for type nodes) represents. If no type information is attached, a
    /// reference to a default-generated unresolved type is returned.
    pub fn data_type(&self) -> Arc<data_type::DataType> {
        self.data_type.clone().unwrap_or_default()
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
    ProtoPrimitive(&'static str, primitive_data::PrimitiveData),

    /// The associated node represents an unpopulated oneof field. This is used
    /// for an error recovery node when a required oneof field is not
    /// populated.
    ProtoMissingOneOf,

    /// Used for anchor/reference-based references to other nodes.
    NodeReference(u64, NodeReference),

    /// Used for resolved YAML URIs, in order to include the parse result and
    /// documentation for the referenced YAML (if available), in addition to
    /// the URI itself.
    YamlReference(Arc<extension::YamlData>),

    /// The associated node represents a YAML map. The contents of the map are
    /// described using Field and UnknownField.
    YamlMap,

    /// The associated node represents a YAML array. The contents of the array
    /// are described using ArrayElement datums.
    YamlArray,

    /// The associated node represents a YAML primitive.
    YamlPrimitive(primitive_data::PrimitiveData),
}

/// Semantical information about a node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Class {
    /// Used for nodes for which no better classification exists.
    Misc,

    /// Used for nodes that define a type. The data_type field signifies this
    /// data type.
    Type,

    /// Used for nodes that represent scalar expressions or literals. The
    /// data_type field signifies the type of the value returned by the
    /// expression.
    Expression,

    /// Used for nodes that represent relations. The data_type field signifies
    /// the schema for the data returned by the relation.
    Relation,
}

/// Information nodes for a parsed protobuf message.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeData {
    /// A reference to a child node in the tree.
    Child(Child),

    /// Indicates that parsing/validating this message resulted in some
    /// diagnostic message being emitted. The secondary error level is the
    /// modified level via
    Diagnostic(diagnostic::Diagnostic),

    /// Provides (intermediate) type information for this node. Depending on
    /// the message, this may be a struct or named struct representing a
    /// schema, or it may represent the type of some scalar expression.
    /// Multiple TypeInfo nodes may be present, in particular for relations
    /// that perform multiple operations in one go (for example read, project,
    /// emit). The TypeInfo and operation description *Field nodes are then
    /// ordered by data flow. In particular, the last TypeInfo node always
    /// represents the type of the final result of a node.
    DataType(Arc<data_type::DataType>),

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
