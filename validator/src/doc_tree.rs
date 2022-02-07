use crate::data_type;
use crate::diagnostic;
use crate::path;
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

/// The original data type that the node represents, to (in theory) allow the
/// original structure of the plan to be recovered from the documentation tree.
#[derive(Clone, Debug)]
pub enum NodeType {
    /// The associated node represents a protobuf message of the given type
    /// (full protobuf path). The contents of the message are described using
    /// Field, RepeatedField, and OneOfField.
    ProtoMessage(String),

    /// The associated node represents a YAML map. The contents of the map are
    /// described using Field and UnknownField.
    YamlMap,

    /// The associated node represents a YAML array. The contents of the array
    /// are described using ArrayElement datums.
    YamlArray,
}

/// Information nodes for a parsed protobuf message.
#[derive(Clone, Debug)]
pub enum NodeData {
    /// For protobuf nodes, indicates that the node has a populated optional
    /// field (optional in the protobuf sense, may be mandatory in Substrait
    /// context) with the given field name and data. For YAML maps, represents
    /// a key-value pair.
    Field(String, FieldData),

    /// For YAML maps, indicates that a field was specified that the validator
    /// doesn't know about.
    UnknownField(String, FieldData),

    /// For protobuf nodes, indicates that the node has a populated repeated
    /// field with the given field name, index, and data. The elements of
    /// repeated nodes are always stored in-order and without gaps, but the
    /// elements may be interspersed with metadata (diagnostics, comments,
    /// etc).
    RepeatedField(String, usize, FieldData),

    /// For protobuf nodes, indicates that the node as a populated OneOf field
    /// with the given field name, variant name, and data.
    VariantField(String, String, FieldData),

    /// For YAML arrays, provides information for the given index in the array.
    /// The array elements are always stored in-order and without gaps, but the
    /// elements may be interspersed with metadata (diagnostics, comments,
    /// etc).
    ArrayElement(usize, FieldData),

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

/// Enumeration of the different kinds of data that may be associated with a
/// protobuf field.
#[derive(Clone, Debug)]
pub enum FieldData {
    /// Used for non-leaf nodes.
    Edge(Rc<Node>),

    /// Used for boolean scalar fields.
    Bool(bool),

    /// Used for anchor/reference-based references to other nodes.
    Reference(u32, NodeReference),

    /// Used for unsigned integer scalar fields.
    Unsigned(u64),

    /// Used for signed integer scalar fields.
    Signed(i64),

    /// Used for floating-point scalar fields.
    Float(f64),

    /// Used for UTF-8 strings, except for resolved YAML URIs.
    String(String),

    /// Used for bytestrings.
    Bytes(Vec<u8>),

    /// Used for resolved YAML URIs, in order to include the resolution result,
    /// parse result, and documentation for the referenced YAML, in addition to
    /// the URI itself.
    YamlUri(String, diagnostic::Result<Rc<Node>>),
}

/// A reference to a node elsewhere in the tree.
#[derive(Clone, Debug)]
pub struct NodeReference {
    /// Absolute path to the node.
    pub path: path::PathBuf,

    /// Link to the node.
    pub node: Rc<Node>,
}
