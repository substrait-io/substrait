use crate::doc_tree;
use crate::path;
use std::rc::Rc;

/// Information about a YAML extension.
#[derive(Clone, Debug, PartialEq)]
pub struct YamlData {
    /// URI for the YAML file.
    pub uri: String,

    /// The path to the node that defined the anchor, if any.
    pub anchor_path: Option<path::PathBuf>,

    /// Reference to the parsed YAML data, if any.
    pub data: Option<doc_tree::NodeReference>,
}

impl YamlData {
    pub fn unresolved() -> Self {
        YamlData {
            uri: "<unknown>".to_string(),
            anchor_path: None,
            data: None,
        }
    }
}

impl Default for YamlData {
    fn default() -> Self {
        Self::unresolved()
    }
}

impl std::fmt::Display for YamlData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

/// Extension information.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionInfo {
    /// The name of the type, type variation, or function.
    pub name: String,

    /// Information about the YAML that this extension is defined in, if any.
    pub yaml_data: Option<Rc<YamlData>>,

    /// The path to the node that defined the anchor for this extension, if
    /// any.
    pub anchor_path: Option<path::PathBuf>,

    /// The path to the YAML node that defined this extension, if any.
    pub definition_path: Option<Rc<YamlData>>,
    // TODO: link to the data record for the definition
}

impl ExtensionInfo {
    pub fn unresolved() -> Self {
        ExtensionInfo {
            name: "<unknown>".to_string(),
            yaml_data: None,
            anchor_path: None,
            definition_path: None,
        }
    }
}

impl Default for ExtensionInfo {
    fn default() -> Self {
        Self::unresolved()
    }
}

impl std::fmt::Display for ExtensionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.yaml_data {
            Some(ref data) => write!(f, "{:?}", data.uri),
            None => write!(f, "<unknown>"),
        }?;
        write!(f, ".{}", self.name)
    }
}
