// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with YAML-based Substrait extensions.

use crate::output::data_type;
use crate::output::path;
use crate::output::tree;
use crate::string_util;
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a named reference to something.
#[derive(Clone, Debug, Default)]
pub struct NamedReference {
    /// The name of the type, type variation, or function.
    name: Option<String>,

    /// The path to the node that defined the anchor for this extension, if
    /// any.
    anchor_path: Option<path::PathBuf>,
}

impl PartialEq for NamedReference {
    /// Named references are equal if both references have a known name and
    /// those names are the same.
    fn eq(&self, other: &Self) -> bool {
        self.name.is_some() && other.name.is_some() && self.name == other.name
    }
}

impl Eq for NamedReference {}

impl std::fmt::Display for NamedReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", string_util::as_ident_or_string(name))
        } else {
            write!(f, "?")
        }
    }
}

impl NamedReference {
    /// Create a new anchor-based reference.
    pub fn new<S: ToString>(
        name: Option<S>,
        anchor_path: Option<path::PathBuf>,
    ) -> Arc<NamedReference> {
        Arc::new(NamedReference {
            name: name.map(|x| x.to_string()),
            anchor_path,
        })
    }

    /// Create a new named reference.
    pub fn new_by_name<S: ToString>(name: S) -> Arc<NamedReference> {
        Arc::new(NamedReference {
            name: Some(name.to_string()),
            anchor_path: None,
        })
    }

    /// Create a new unknown reference.
    pub fn new_unknown() -> Arc<NamedReference> {
        Arc::default()
    }

    /// Returns the name, if known.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| &s[..])
    }

    /// Returns the path to the anchor, if known.
    pub fn anchor_path(&self) -> Option<&path::PathBuf> {
        self.anchor_path.as_ref()
    }
}

/// Named/namespaced reference to a particular extension definition.
#[derive(Clone, Debug, Default)]
pub struct Reference<T> {
    /// The name of the type, type variation, or function.
    pub name: Arc<NamedReference>,

    /// The URI of the YAML file that defined this extension.
    pub uri: Arc<NamedReference>,

    /// Extension definition information, specific to this type of extension,
    /// if we managed to resolve the reference.
    pub definition: Option<Arc<T>>,
}

impl<T> PartialEq for Reference<T> {
    /// References are equal if they refer to the same thing, regardless of how
    /// they refer to it. If we're not sure4 because either reference is
    /// (partially) unresolved, return false pessimistically.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.uri == other.uri
    }
}

impl<T> Eq for Reference<T> {}

impl<T> std::fmt::Display for Reference<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.uri, self.name)
    }
}

/// User-defined base data type.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataType {
    /// The underlying structure of the type.
    pub structure: Vec<(String, data_type::Simple)>,
}

/// The base type of a type variation.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeVariationBase {
    /// The type variation is immediately based in a physical type.
    Physical(data_type::Class),

    /// The type variation is based in another logical type variation.
    Logical(Arc<TypeVariation>),

    /// The base type is unknown.
    Unresolved,
}

impl Default for TypeVariationBase {
    fn default() -> Self {
        TypeVariationBase::Unresolved
    }
}

/// Type variation extension.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TypeVariation {
    /// The base type for this variation.
    pub base: TypeVariationBase,

    /// Function behavior for this variation.
    pub function_behavior: FunctionBehavior,
}

impl TypeVariation {
    /// Return the base class for this type variation, if known.
    pub fn get_base_class(&self) -> data_type::Class {
        match &self.base {
            TypeVariationBase::Physical(x) => x.clone(),
            TypeVariationBase::Logical(x) => x.get_base_class(),
            TypeVariationBase::Unresolved => data_type::Class::Unresolved,
        }
    }
}

/// Type variation function behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBehavior {
    Inherits,
    Separate,
}

impl Default for FunctionBehavior {
    fn default() -> Self {
        FunctionBehavior::Inherits
    }
}

/// Function extension.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Function {
    // TODO: need much more information here to do type checking.
}

/// Information about a YAML extension, which may or may not be resolved.
#[derive(Clone, Debug, PartialEq)]
pub enum YamlInfo {
    Unresolved(Arc<NamedReference>),
    Resolved(Arc<YamlData>),
}

impl YamlInfo {
    pub fn data(&self) -> Option<&YamlData> {
        match self {
            YamlInfo::Unresolved(_) => None,
            YamlInfo::Resolved(x) => Some(x),
        }
    }

    pub fn uri(&self) -> &Arc<NamedReference> {
        match self {
            YamlInfo::Unresolved(x) => x,
            YamlInfo::Resolved(x) => &x.uri,
        }
    }
}

impl Default for YamlInfo {
    fn default() -> Self {
        YamlInfo::Unresolved(Arc::default())
    }
}

impl std::fmt::Display for YamlInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri())
    }
}

/// Data for a resolved YAML file.
#[derive(Clone, Debug, PartialEq)]
pub struct YamlData {
    /// URI for the YAML file.
    pub uri: Arc<NamedReference>,

    /// Reference to the parsed YAML data, if any.
    pub data: tree::NodeReference,

    /// Functions defined in this YAML file. Names are stored in lower case
    /// (Substrait's name resolution is case-insensitive).
    pub functions: HashMap<String, Arc<Function>>,

    /// Types defined in this YAML file. Names are stored in lower case
    /// (Substrait's name resolution is case-insensitive).
    pub types: HashMap<String, Arc<DataType>>,

    /// Type variations defined in this YAML file. Names are stored in lower
    /// case (Substrait's name resolution is case-insensitive).
    pub type_variations: HashMap<String, Arc<TypeVariation>>,
}

impl YamlData {
    /// Constructs an empty YamlData object with an invalid reference to the
    /// data node. Everything still needs to be populated for this to become
    /// valid.
    pub fn new(uri: Arc<NamedReference>) -> YamlData {
        YamlData {
            uri,
            data: tree::NodeReference {
                path: path::Path::Root("").to_path_buf(),
                node: Arc::new(tree::NodeType::YamlMap.into()),
            },
            functions: HashMap::default(),
            types: HashMap::default(),
            type_variations: HashMap::default(),
        }
    }

    /// Helper function for the various resolvers.
    fn local_reference<S: ToString, T>(
        &self,
        name: S,
        definition: Option<Arc<T>>,
    ) -> Arc<Reference<T>> {
        Arc::new(Reference {
            name: NamedReference::new_by_name(name),
            uri: self.uri.clone(),
            definition,
        })
    }

    /// Resolves a function defined in this YAML data block by name. Returns an
    /// unresolved reference if it does not exist.
    pub fn resolve_function<S: ToString>(&self, name: S) -> Arc<Reference<Function>> {
        let name = name.to_string();
        let maybe_def = self.functions.get(&name).cloned();
        self.local_reference(name, maybe_def)
    }

    /// Resolves a type defined in this YAML data block by name. Returns an
    /// unresolved reference if it does not exist.
    pub fn resolve_type<S: ToString>(&self, name: S) -> Arc<Reference<DataType>> {
        let name = name.to_string();
        let maybe_def = self.types.get(&name).cloned();
        self.local_reference(name, maybe_def)
    }

    /// Resolves a type variation defined in this YAML data block by name.
    /// Returns an unresolved reference if it does not exist.
    pub fn resolve_type_variation<S: ToString>(&self, name: S) -> Arc<Reference<TypeVariation>> {
        let name = name.to_string();
        let maybe_def = self.type_variations.get(&name).cloned();
        self.local_reference(name, maybe_def)
    }
}
