use crate::doc_tree;

/// Extension information.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionInfo {
    /// The URI of the YAML file that's defining the extension.
    pub uri: String,

    /// The name of the type, type variation, or function.
    pub name: String,

    /// Reference to the documentation node that defined the type.
    pub reference: doc_tree::NodeReference,
}
