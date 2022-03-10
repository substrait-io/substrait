// SPDX-License-Identifier: Apache-2.0

//! Module providing the types containing contextual information for parse
//! functions.
//!
//! Refer to the documentation for [`parse`](mod@crate::parse) for more
//! information.

// FIXME: remove once validation code is finished.
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::input::config;
use crate::output::data_type;
use crate::output::extension;
use crate::output::path;
use crate::output::tree;
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
    pub config: &'a config::Config,
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

    /// The YAML data object under construction, if any.
    pub yaml_data: Option<extension::YamlData>,
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

/// Convenience/shorthand macro for the with_context function. Intended for use
/// within tests, where a temporary parse context is needed.
macro_rules! with_context {
    ($function:expr, ()) => {
        with_context!(&mut crate::parse::context::State::default(), $function, ())
    };
    ($function:expr, ($($args:expr),*)) => {
        with_context!(&mut crate::parse::context::State::default(), $function, ($($args),*))
    };
    (config = $config:expr, $function:expr, ()) => {
        with_context!(&mut crate::parse::context::State::default(), $config, $function, ())
    };
    (config = $config:expr, $function:expr, ($($args:expr),*)) => {
        with_context!(&mut crate::parse::context::State::default(), $config, $function, ($($args),*))
    };
    ($state:expr, $function:expr, ()) => {
        with_context!($state, &crate::parse::context::Config::default(), $function, ())
    };
    ($state:expr, $function:expr, ($($args:expr),*)) => {
        with_context!($state, &crate::parse::context::Config::default(), $function, ($($args),*))
    };
    ($state:expr, $config:expr, $function:expr, ()) => {
        crate::parse::context::with_context(
            $state,
            $config,
            $function,
        )
    };
    ($state:expr, $config:expr, $function:expr, ($($args:expr),*)) => {
        crate::parse::context::with_context(
            $state,
            $config,
            |y| $function($($args),*, y),
        )
    };
}

// Creates a temporary context and calls a function with it.
pub fn with_context<R, F: FnOnce(&mut Context) -> R>(
    state: &mut State,
    config: &config::Config,
    function: F,
) -> (R, tree::Node) {
    // Create the root node for the output.
    let mut output = tree::NodeType::ProtoMessage("temp").into();

    // Create a temporary context.
    let mut context = Context {
        output: &mut output,
        state,
        breadcrumb: &mut Breadcrumb::new("temp"),
        config,
    };

    // Call the function.
    let result = function(&mut context);

    // Return the results.
    (result, output)
}
