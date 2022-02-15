use crate::data_type;
use crate::doc_tree;
use crate::extension;
use crate::path;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Context<'a, T> {
    /// The to-be-validated input node. Fields in the node can be read directly
    /// if needed for validation of this node, but should always also be
    /// traversed by a *_field!() macro for the output tree structure to be
    /// completed.
    pub input: &'a T,

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
    pub breadcrumbs: &'a mut Breadcrumb<'a>,

    /// Configuration structure, created before validation starts and immutable
    /// afterwards.
    pub config: &'a Config,
}

/// Global state information tracked by the validation logic.
pub struct State {
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

/// Configuration structure.
pub struct Config {
    // Placeholder; nothing here yet.
}
