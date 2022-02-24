use crate::data_type;
use crate::extension;
use crate::path;
use crate::tree;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

/// Parse/validation context and output node, passed to parser functions along
/// with a reference to the to-be-parsed input node.
pub struct Context<'a> {
    /// The node in the documentation tree that should reflect the input node.
    /// The structure of the documentation tree will be the same as the input
    /// tree, but represented in a more generic way, and with annotations like
    /// comments and diagnostics attached to each node. The output tree is not
    /// intended to be read back by the validator.
    pub output: &'a mut tree::Node,

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
    pub uris: HashMap<u32, Arc<extension::YamlInfo>>,

    /// YAML-defined function set, indexed by anchor.
    pub functions: HashMap<u32, Arc<extension::Reference<extension::Function>>>,

    /// YAML-defined function set, indexed by anchor.
    pub types: HashMap<u32, Arc<extension::Reference<extension::DataType>>>,

    /// YAML-defined function set, indexed by anchor.
    pub type_variations: HashMap<u32, Arc<extension::Reference<extension::TypeVariation>>>,

    /// Protobuf "any" URLs depended on, that we have not encountered a
    /// declaration for yet (we check the declarations at the end). The
    /// path refers to the first use of that URL.
    pub pending_proto_url_dependencies: HashMap<String, path::PathBuf>,

    /// Protobuf "any" URLs that have been declared in the plan. The path
    /// refers to the declaration.
    pub proto_url_declarations: HashMap<String, path::PathBuf>,

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

/// Callback function type for resolving/downloading URIs.
type UriResolver = Box<dyn Fn(&str) -> std::result::Result<Vec<u8>, String>>;

/// Configuration structure.
#[derive(Default)]
pub struct Config {
    /// When set, so not generate warnings for unknown protobuf fields that are
    /// set to their protobuf-defined default value.
    pub ignore_unknown_fields_set_to_default: bool,

    /// Protobuf message URLs that are whitelisted for use in "any" messages,
    /// i.e. that the caller warrants the existence of in the consumer that
    /// the plan is validated for.
    pub whitelisted_any_urls: HashSet<String>,

    /// Allows URIs from the plan to be remapped (Some(mapping)) or ignored
    /// (None). All resolution can effectively be disabled by just adding a
    /// rule that maps * to None. Furthermore, in the absence of a custom
    /// yaml_uri_resolver function, this can be used to remap URIs to
    /// pre-downloaded files.
    pub yaml_uri_overrides: Vec<(glob::Pattern, Option<String>)>,

    /// Optional callback function for resolving YAML URIs. If specified, all
    /// URIs (after processing yaml_uri_overrides) are resolved using this
    /// function. The function takes the URI as its argument, and should either
    /// return the download contents as a Vec<u8> or return a String-based
    /// error. If no downloader is specified, only file:// URLs with an
    /// absolute path are supported.
    pub yaml_uri_resolver: Option<UriResolver>,
}
