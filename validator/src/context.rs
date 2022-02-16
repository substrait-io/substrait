use crate::data_type;
use crate::doc_tree;
use crate::extension;
use crate::path;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Parse/validation context and output node, passed to parser functions along
/// with a reference to the to-be-parsed input node.
pub struct Context<'a> {
    /// The node in the documentation tree that should reflect the input node.
    /// The structure of the documentation tree will be the same as the input
    /// tree, but represented in a more generic way, and with annotations like
    /// comments and diagnostics attached to each node. The output tree is not
    /// intended to be read back by the validator.
    pub output: &'a mut doc_tree::Node,

    /// State object. This is tracked between nodes as they are traversed, and
    /// is always mutable for the node currently being validated.
    pub state: &'a mut State,

    /// "Breadcrumbs" with information about the ancestors of the current node.
    /// Essentially a stack structure, where only the top of the stack is
    /// mutable.
    pub breadcrumb: &'a mut Breadcrumb<'a>,

    /// Configuration structure, created before validation starts and immutable
    /// afterwards.
    pub config: &'a Config,
}

/// Global state information tracked by the validation logic.
#[derive(Default)]
pub struct State {
    /// YAML extension URI map.
    pub uris: HashMap<u32, extension::YamlData>,

    /// YAML-defined function set, indexed by anchor.
    pub functions: HashMap<u32, extension::ExtensionInfo>,

    /// YAML-defined function set, indexed by anchor.
    pub types: HashMap<u32, Rc<data_type::UserDefined>>,

    /// YAML-defined function set, indexed by anchor.
    pub type_variations: HashMap<u32, Rc<data_type::Variation>>,

    /// Schema stack. This is what the validator for FieldRefs uses to
    /// determine the return type of the FieldRef. The back of the vector
    /// represents the innermost query, while entries further to the front
    /// of the vector are used to break out of correlated subqueries.
    pub schema: Vec<data_type::DataType>,
}

/// Breadcrumbs structure. Each breadcrumb is associated with a node, and
/// immutably links to the breadcrumb for its parent node (except for the
/// root). Used for two things: tracking the path leading up to the current
/// node from the root, and keeping track of mutable state information that
/// belongs to a specific node.
pub struct Breadcrumb<'a> {
    /// Breadcrumb for the parent node, unless this is the root.
    pub parent: Option<&'a Breadcrumb<'a>>,

    /// The path leading up to the node associated with this breadcrumb. Used
    /// primarily for attaching information to diagnostic messages.
    pub path: path::Path<'a>,

    /// The set of field names of the associated node that we've already
    /// parsed. This is used to automatically search through message subtrees
    /// that the validator doesn't yet implement: after all normal validation
    /// for a node is done, the generic tree-walking logic checks whether there
    /// are fields with non-default data associated with them of which the
    /// field name hasn't been added to this set yet. It's also used to assert
    /// that the same subtree isn't traversed twice.
    pub fields_parsed: HashSet<String>,
}

impl Breadcrumb<'_> {
    /// Creates a breadcrumb for the root node.
    pub fn new(root_name: &'static str) -> Self {
        Self {
            parent: None,
            path: path::Path::Root(root_name),
            fields_parsed: HashSet::new(),
        }
    }

    /// Creates the next breadcrumb.
    pub fn next(&self, element: path::PathElement) -> Breadcrumb {
        Breadcrumb {
            parent: Some(self),
            path: self.path.with(element),
            fields_parsed: HashSet::new(),
        }
    }
}

/// Configuration structure.
#[derive(Default)]
pub struct Config {
    /// When set, so not generate warnings for unknown protobuf fields that are
    /// set to their protobuf-defined default value.
    pub ignore_unknown_fields_set_to_default: bool,
}
