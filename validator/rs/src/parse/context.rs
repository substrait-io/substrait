// SPDX-License-Identifier: Apache-2.0

//! Module providing the types containing contextual information for parse
//! functions.
//!
//! Refer to the documentation for [`parse`](mod@crate::parse) for more
//! information.

use crate::input::config;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
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
    output: &'a mut tree::Node,

    /// State object. This is tracked between nodes as they are traversed, and
    /// is always mutable for the node currently being validated.
    state: &'a mut State,

    /// "Breadcrumbs" with information about the ancestors of the current node.
    /// Essentially a stack structure, where only the top of the stack is
    /// mutable.
    breadcrumb: Breadcrumb<'a>,

    /// Configuration structure, created before validation starts and immutable
    /// afterwards.
    pub config: &'a config::Config,
}

impl<'a> Context<'a> {
    /// Creates a root parse context.
    ///
    /// root_name is the prefix used for all paths, normally just "plan" (if
    /// different tree parsers are ever created, this can be used to
    /// disambiguate between tree types). output is the root node that the
    /// children will be added to as parsing progresses. state is the state
    /// object used for tracking parser state. config is the configuration for
    /// the parser.
    pub fn new(
        root_name: &'static str,
        output: &'a mut tree::Node,
        state: &'a mut State,
        config: &'a config::Config,
    ) -> Self {
        Self {
            output,
            state,
            breadcrumb: Breadcrumb::new(root_name),
            config,
        }
    }

    /// Creates a parse context for a child of the node corresponding to this
    /// context. output is its node. path_element specifies its relation to
    /// the node corresponding to the current context.
    pub fn child<'b>(
        &'b mut self,
        output: &'b mut tree::Node,
        path_element: path::PathElement,
    ) -> Context<'b> {
        Context {
            output,
            state: self.state,
            breadcrumb: self.breadcrumb.next(path_element),
            config: self.config,
        }
    }

    /// Returns the node type of the associated node.
    pub fn node_type(&self) -> &tree::NodeType {
        &self.output.node_type
    }

    /// Replaces the node type of the associated node.
    ///
    /// This should only be needed to upgrade primitive nodes to more specific
    /// types, for instance references or resolved URIs.
    pub fn replace_node_type(&mut self, node_type: tree::NodeType) -> tree::NodeType {
        std::mem::replace(&mut self.output.node_type, node_type)
    }

    /// Returns the data type currently associated with the current node. If no
    /// data type was associated yet, this silently returns a reference to an
    /// unresolved type object.
    pub fn data_type(&self) -> &data_type::DataType {
        self.output.data_type.as_ref().unwrap_or_default()
    }

    /// Replaces the data type associated with this node, including the
    /// information indicating whether this node semantically *has* a data
    /// type.
    ///
    /// This is only used by the traversal macros. Don't use it directly.
    pub fn replace_data_type(
        &mut self,
        data_type: Option<data_type::DataType>,
    ) -> Option<data_type::DataType> {
        std::mem::replace(&mut self.output.data_type, data_type)
    }

    /// Pushes data into the current node.
    ///
    /// This is primarily intended for use by the traversal macros and the more
    /// specific functions defined here, like set_data_type().
    pub fn push(&mut self, node_data: tree::NodeData) {
        self.output.data.push(node_data);
    }

    /// Pushes a diagnostic into the node. This also evaluates its adjusted
    /// error level.
    pub fn push_diagnostic(&mut self, diag: diagnostic::RawDiagnostic) {
        // Get the configured level limits for this diagnostic. First try the
        // classification of the diagnostic itself, then its group, and then
        // finally Unclassified. If no entries exist, simply yield
        // (Info, Error), which is no-op.
        let (min, max) = self
            .config
            .diagnostic_level_overrides
            .get(&diag.cause.classification)
            .or_else(|| {
                self.config
                    .diagnostic_level_overrides
                    .get(&diag.cause.classification.group())
            })
            .or_else(|| {
                self.config
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

        // Actually push the data item.
        self.output.data.push(tree::NodeData::Diagnostic(adjusted));
    }

    /// Pushes a comment into the node.
    pub fn push_comment<C: Into<comment::Comment>>(&mut self, comment: C) {
        self.push(tree::NodeData::Comment(comment.into()))
    }

    /// Sets the data type "returned" by this node. Specifically:
    ///
    ///  - for type nodes, this should be used to specify the type;
    ///  - for expression nodes, this should be used to specify the type of the
    ///    data returned by the expression;
    ///
    /// Can be called multiple times; only the data type specified for the
    /// final call attached to the node's "return type", but each time a
    /// NodeData::DataType is pushed into the node data as well.
    pub fn set_data_type(&mut self, data_type: data_type::DataType) {
        self.push(tree::NodeData::DataType(data_type.clone()));
        self.output.data_type = Some(data_type);
    }

    /// Updates the current schema. This also pushes the data type to the
    /// current node. Relation parsers *must* use this after traversing their
    /// inputs, but before they start to parse any expressions based on that
    /// schema; after all, the schema defines how (column) references behave.
    /// If the schema isn't known, it may be set to an unresolved type.
    pub fn set_schema(&mut self, schema: data_type::DataType) {
        *self
            .state
            .schema_stack
            .last_mut()
            .expect("no schema present on schema stack") = Some(schema.clone());
        self.set_data_type(schema);
    }

    /// Clears the current schema, requiring schema!() to be called before
    /// expressions can be parsed again.
    pub fn clear_schema(&mut self) {
        *self
            .state
            .schema_stack
            .last_mut()
            .expect("no schema present on schema stack") = None;
    }

    /// Returns the current schema. depth specifies for which subquery the
    /// schema should be selected; depth 0 is the current query, depth 1 would
    /// be its parent query, 2 would be its grandparent, etc. Returns Err when
    /// the referenced schema semantically doesn't exist; returns Ok(unresolved
    /// type) when it does but the actual type isn't known.
    pub fn schema(&self, depth: usize) -> diagnostic::Result<data_type::DataType> {
        let len = self.state.schema_stack.len();
        if depth >= len {
            Err(cause!(
                ExpressionFieldRefMissingStream,
                "indexing query beyond current query depth ({len})"
            ))
        } else if let Some(Some(schema)) = self.state.schema_stack.get(len - depth - 1) {
            Ok(schema.clone())
        } else {
            Err(cause!(
                ExpressionFieldRefMissingStream,
                "query data stream has not yet been instantiated"
            ))
        }
    }

    /// Pushes an empty slot for the schema of the relation tree onto the
    /// schema stack, allowing schema!() to be used. This must be used when
    /// traversing into the root of a relation tree; i.e., the root must be
    /// parsed within the context of the provided function.
    pub fn enter_relation_root<R, F: FnOnce(&mut Context) -> R>(&mut self, f: F) -> R {
        // Push a schema slot onto the stack for the relation tree to fill
        // in.
        self.state.schema_stack.push(None);

        // Ensure that return statements can't break out of the context
        // early by wrapping the block in a closure first.
        let result = f(self);

        // Pop the schema again.
        self.state
            .schema_stack
            .pop()
            .expect("no schema present on schema stack");

        result
    }

    /// Returns all data that has thus far been pushed into the current node.
    pub fn node_data(&self) -> &[tree::NodeData] {
        &self.output.data
    }

    /// Registers a URI anchor. If the anchor index was already defined, this
    /// returns the conflicting entry.
    pub fn register_uri(
        &mut self,
        anchor: u32,
        uri: Arc<extension::YamlInfo>,
    ) -> Option<Arc<extension::YamlInfo>> {
        self.state.uris.insert(anchor, uri)
    }

    /// Resolves a URI reference.
    pub fn resolve_uri(&self, reference: u32) -> Option<&Arc<extension::YamlInfo>> {
        self.state.uris.get(&reference)
    }

    /// Registers a function anchor. If the anchor index was already defined,
    /// this returns the conflicting entry.
    pub fn register_fn(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::Function>>,
    ) -> Option<Arc<extension::Reference<extension::Function>>> {
        self.state.functions.insert(anchor, uri)
    }

    /// Resolves a function reference.
    pub fn resolve_fn(
        &self,
        reference: u32,
    ) -> Option<&Arc<extension::Reference<extension::Function>>> {
        self.state.functions.get(&reference)
    }

    /// Registers a type anchor. If the anchor index was already defined, this
    /// returns the conflicting entry.
    pub fn register_type(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::DataType>>,
    ) -> Option<Arc<extension::Reference<extension::DataType>>> {
        self.state.types.insert(anchor, uri)
    }

    /// Resolves a type reference.
    pub fn resolve_type(
        &self,
        reference: u32,
    ) -> Option<&Arc<extension::Reference<extension::DataType>>> {
        self.state.types.get(&reference)
    }

    /// Registers a type variation anchor. If the anchor index was already
    /// defined, this returns the conflicting entry.
    pub fn register_tvar(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::TypeVariation>>,
    ) -> Option<Arc<extension::Reference<extension::TypeVariation>>> {
        self.state.type_variations.insert(anchor, uri)
    }

    /// Resolves a type variation reference.
    pub fn resolve_tvar(
        &self,
        reference: u32,
    ) -> Option<&Arc<extension::Reference<extension::TypeVariation>>> {
        self.state.type_variations.get(&reference)
    }

    /// Returns a mutable reference to the YAML data object under construction,
    /// if any.
    pub fn yaml_data(&mut self) -> &mut Option<extension::YamlData> {
        &mut self.state.yaml_data
    }

    /// Resolves a protobuf "any" message. Returns whether the user has
    /// explicitly allowed this message type for use.
    pub fn resolve_any(&mut self, x: &prost_types::Any) -> bool {
        self.state
            .pending_proto_url_dependencies
            .entry(x.type_url.clone())
            .or_insert_with(|| self.breadcrumb.path.to_path_buf());
        self.config
            .allowed_proto_any_urls
            .iter()
            .any(|p| p.matches(&x.type_url))
    }

    /// Protobuf "any" URLs depended on, that we have not encountered a
    /// declaration for yet (we check the declarations at the end). The
    /// path refers to the first use of that URL.
    pub fn pending_proto_url_dependencies(&mut self) -> &mut HashMap<String, path::PathBuf> {
        &mut self.state.pending_proto_url_dependencies
    }

    /// Protobuf "any" URLs that have been declared in the plan. The path
    /// refers to the declaration.
    pub fn proto_url_declarations(&mut self) -> &mut HashMap<String, path::PathBuf> {
        &mut self.state.proto_url_declarations
    }

    /// Returns the path leading up to the current node.
    pub fn path(&self) -> &path::Path<'a> {
        &self.breadcrumb.path
    }

    /// Returns the path leading up to the current node.
    pub fn path_buf(&self) -> path::PathBuf {
        self.breadcrumb.path.to_path_buf()
    }

    /// Returns the path leading up to the parent node, if any.
    pub fn parent_path_buf(&self) -> Option<path::PathBuf> {
        self.breadcrumb.parent.map(|x| x.path.to_path_buf())
    }

    /// Indicates that the field with the given name has been parsed. See also
    /// is_field_parsed().
    pub fn set_field_parsed<S: ToString>(&mut self, field: S) -> bool {
        self.breadcrumb.fields_parsed.insert(field.to_string())
    }

    /// Returns whether the field with the given name has been parsed yet.
    ///
    /// This is primarily intended for use by the traversal macros. They use it
    /// to ensure that:
    ///
    ///  - a field is only parsed once;
    ///  - fields not parsed by the parse function are parsed using a generic
    ///    method, along with emission of a warning message.
    pub fn field_parsed<S: AsRef<str>>(&mut self, field: S) -> bool {
        self.breadcrumb.fields_parsed.contains(field.as_ref())
    }
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
    /// None is used only for the top of the stack, and only when we're inside
    /// a relation tree, but no schema is known yet (in terms of dataflow,
    /// we're still in the time before the input relation has created a
    /// stream).
    pub schema_stack: Vec<Option<data_type::DataType>>,

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
