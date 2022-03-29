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
use std::fmt::Debug;
use std::hash::Hash;
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
    pub fn data_type(&self) -> Arc<data_type::DataType> {
        self.output.data_type.clone().unwrap_or_default()
    }

    /// Sets the semantic description of the current node.
    pub fn set_description<B: Into<comment::Brief>>(
        &mut self,
        class: tree::Class,
        brief: Option<B>,
    ) {
        self.output.class = class;
        self.output.brief = brief.map(|c| c.into());
    }

    /// Appends to the summary of this node.
    pub fn push_summary<C: Into<comment::Comment>>(&mut self, comment: C) {
        if let Some(summary) = self.output.summary.as_mut() {
            summary.extend(comment.into())
        } else {
            self.output.summary = Some(comment.into())
        }
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
    pub fn set_data_type(&mut self, data_type: Arc<data_type::DataType>) {
        if !data_type.is_unresolved() {
            self.push(tree::NodeData::DataType(data_type.clone()));
        }
        self.output.data_type = Some(data_type);
    }

    /// Updates the current schema. This also pushes the data type to the
    /// current node. Relation parsers *must* use this after traversing their
    /// inputs, but before they start to parse any expressions based on that
    /// schema; after all, the schema defines how (column) references behave.
    /// If the schema isn't known, it may be set to an unresolved type.
    pub fn set_schema(&mut self, schema: Arc<data_type::DataType>) {
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
    pub fn schema(&self, depth: usize) -> diagnostic::Result<Arc<data_type::DataType>> {
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

    /// Returns the resolver for URI anchors and references.
    pub fn extension_uris(&mut self) -> &mut Resolver<u32, Arc<extension::YamlInfo>> {
        &mut self.state.extension_uris
    }

    /// Registers an extension URI definition. Shorthand for uris().define(),
    /// using the current path as the registration path.
    pub fn define_extension_uri(
        &mut self,
        anchor: u32,
        uri: Arc<extension::YamlInfo>,
    ) -> Result<(), (Arc<extension::YamlInfo>, path::PathBuf)> {
        self.state
            .extension_uris
            .define(anchor, uri, self.breadcrumb.path.to_path_buf())
    }

    /// Returns the resolver for function anchors and references.
    pub fn fns(&mut self) -> &mut Resolver<u32, Arc<extension::Reference<extension::Function>>> {
        &mut self.state.functions
    }

    /// Registers a function definition. Shorthand for fns().define(), using
    /// the current path as the registration path.
    pub fn define_fn(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::Function>>,
    ) -> Result<
        (),
        (
            Arc<extension::Reference<extension::Function>>,
            path::PathBuf,
        ),
    > {
        self.state
            .functions
            .define(anchor, uri, self.breadcrumb.path.to_path_buf())
    }

    /// Returns the resolver for type anchors and references.
    pub fn types(&mut self) -> &mut Resolver<u32, Arc<extension::Reference<extension::DataType>>> {
        &mut self.state.types
    }

    /// Registers a type definition. Shorthand for fns().define(), using the
    /// current path as the registration path.
    pub fn define_type(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::DataType>>,
    ) -> Result<
        (),
        (
            Arc<extension::Reference<extension::DataType>>,
            path::PathBuf,
        ),
    > {
        self.state
            .types
            .define(anchor, uri, self.breadcrumb.path.to_path_buf())
    }

    /// Returns the resolver for type variation anchors and references.
    pub fn tvars(
        &mut self,
    ) -> &mut Resolver<u32, Arc<extension::Reference<extension::TypeVariation>>> {
        &mut self.state.type_variations
    }

    /// Registers a type definition. Shorthand for fns().define(), using the
    /// current path as the registration path.
    pub fn define_tvar(
        &mut self,
        anchor: u32,
        uri: Arc<extension::Reference<extension::TypeVariation>>,
    ) -> Result<
        (),
        (
            Arc<extension::Reference<extension::TypeVariation>>,
            path::PathBuf,
        ),
    > {
        self.state
            .type_variations
            .define(anchor, uri, self.breadcrumb.path.to_path_buf())
    }

    /// Returns the resolver for protobuf Any types present in the
    /// `expected_type_urls` manifest.
    pub fn proto_any_types(&mut self) -> &mut Resolver<String, ()> {
        &mut self.state.proto_any_types
    }

    /// Defines a protobuf Any type URL, allowing it for use within the plan.
    /// If the type was already declared, this returns the path that defined
    /// it in the form of an Err result.
    pub fn define_proto_any_type<S: ToString>(&mut self, url: S) -> Result<(), path::PathBuf> {
        self.state
            .proto_any_types
            .define(url.to_string(), (), self.breadcrumb.path.to_path_buf())
            .map_err(|(_, p)| p)
    }

    /// Resolves a protobuf "any" message. The first return value specifies
    /// whether usage of the type was explicitly allowed in the validator
    /// configuration. The second return value specifies the path to the
    /// manifest entry for the type, if it was defined. If the type URL does
    /// not exist in the manifest, a suitable error is generated automatically.
    pub fn resolve_proto_any(&mut self, x: &prost_types::Any) -> (bool, Option<path::PathBuf>) {
        let path = self
            .state
            .proto_any_types
            .resolve(&x.type_url)
            .map(|(_, path)| path.clone());
        if path.is_none() {
            diagnostic!(self, Error, ProtoMissingAnyDeclaration, "{}", x.type_url);
        }
        let allowed = self
            .config
            .allowed_proto_any_urls
            .iter()
            .any(|p| p.matches(&x.type_url));
        (allowed, path)
    }

    /// Returns a mutable reference to the Option that possibly contains the
    /// YAML data object under construction.
    pub fn yaml_data_opt(&mut self) -> &mut Option<extension::YamlData> {
        &mut self.state.yaml_data
    }

    /// Returns a mutable reference to the YAML data object under construction.
    /// Panics if we're not currently constructing YAML data.
    pub fn yaml_data(&mut self) -> &mut extension::YamlData {
        self.state.yaml_data.as_mut().unwrap()
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

#[derive(Clone, Debug, Default)]
pub struct Resolver<K, V>
where
    K: Clone + Debug + Default + Eq + Hash,
    V: Clone + Debug + Default,
{
    /// Map of keys that have been registered thus far to their value and to
    /// the path from which they were registered.
    map: HashMap<K, (V, path::PathBuf)>,

    /// The set of keys for which resolve() was called at least once. Used to
    /// detect unused keys.
    used: HashSet<K>,
}

impl<K, V> Resolver<K, V>
where
    K: Clone + Debug + Default + Eq + Hash,
    V: Clone + Debug + Default,
{
    /// Creates a new resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines a key-value-path triplet. If a key was previously defined, its
    /// entry is overridden, and the previous value-path pair is returned
    /// in the form of an Err result.
    pub fn define(
        &mut self,
        key: K,
        value: V,
        path: path::PathBuf,
    ) -> Result<(), (V, path::PathBuf)> {
        if let Some(previous) = self.map.insert(key, (value, path)) {
            Err(previous)
        } else {
            Ok(())
        }
    }

    /// Resolves the given key to its value-path pair. If no value was
    /// registered for the given key, None is returned. If this was the first
    /// use of this key (regardless of whether or not a value was registered
    /// for it yet), it is recorded in the set of used keys.
    pub fn resolve(&mut self, key: &K) -> Option<&(V, path::PathBuf)> {
        self.used.insert(key.clone());
        self.map.get(key)
    }

    /// Iterates over all key-value-path triplets corresponding to def
    pub fn iter_unused(&self) -> impl Iterator<Item = (K, V, path::PathBuf)> + '_ {
        self.map.iter().filter_map(|(k, (v, p))| {
            if self.used.contains(k) {
                None
            } else {
                Some((k.clone(), v.clone(), p.clone()))
            }
        })
    }
}

/// Global state information tracked by the validation logic.
#[derive(Default)]
pub struct State {
    /// URI anchor resolver.
    pub extension_uris: Resolver<u32, Arc<extension::YamlInfo>>,

    /// YAML-defined function anchor resolver.
    pub functions: Resolver<u32, Arc<extension::Reference<extension::Function>>>,

    /// YAML-defined data type anchor resolver.
    pub types: Resolver<u32, Arc<extension::Reference<extension::DataType>>>,

    /// YAML-defined type variation anchor resolver.
    pub type_variations: Resolver<u32, Arc<extension::Reference<extension::TypeVariation>>>,

    /// Protobuf Any type URL resolver.
    pub proto_any_types: Resolver<String, ()>,

    /// Schema stack. This is what the validator for FieldRefs uses to
    /// determine the return type of the FieldRef. The back of the vector
    /// represents the innermost query, while entries further to the front
    /// of the vector are used to break out of correlated subqueries.
    /// None is used only for the top of the stack, and only when we're inside
    /// a relation tree, but no schema is known yet (in terms of dataflow,
    /// we're still in the time before the input relation has created a
    /// stream).
    pub schema_stack: Vec<Option<Arc<data_type::DataType>>>,

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
