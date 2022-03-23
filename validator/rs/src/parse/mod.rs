// SPDX-License-Identifier: Apache-2.0

//! Parsing/validation module.
//!
//! Roughly speaking, this module takes a Substrait plan represented using the
//! types provided by the [`input`](crate::input) module, and transforms it to
//! an equivalent plan represented using the types provided by the
//! [`output`](crate::output) module. In doing so, it parses and validates the
//! plan.
//!
//! # Traversal
//!
//! Most of the boilerplate code for tree traversal is handled by the
//! [`traversal`] module. What remains are "parse functions" of the form
//! `(x: &T, y: &mut Context) -> Result<R>`, where:
//!
//!  - `x` is a reference to the the JSON/YAML value or the prost wrapper for
//!    the protobuf message that is to be parsed and validated;
//!  - `y` is the parse context ([`context::Context`], see next section); and
//!  - `R` is any desired return type.
//!
//! The body of the parse function can use a wide variety of function-like
//! macros from [`traversal`] to traverse the children of `x` in the
//! appropriate order and with the appropriate parse functions. The macros
//! return a tuple of a reference to the created
//! [`Node`](crate::output::tree::Node) and the `R` returned by the parse
//! function (depending on the macro, these may be wrapped in [`Option`]s or
//! [`Vec`]s). Note that any children not traversed by the parse function will
//! automatically be traversed by [`traversal`] (along with a warning
//! diagnostic that these children were not validated), and that traversing a
//! child twice is illegal (this will panic).
//!
//! # Parser context
//!
//! The mutable [`context::Context`] reference that is passed into every parse
//! function and is needed for every traversal macro stores all contextual
//! information needed for parsing, except for the input. Any and all results
//! of the parse process need to eventually end up in here, and as such it has
//! quite a few functions defined on it. It also has a reference to the
//! configuration structure; it's kind of the odd one out here since the
//! configuration is more of an input than output or state; it's simply
//! convenient to pass it along with the context object to save on some typing
//! when defining parse functions.
//!
//! Besides macros strictly intended for traversal, the [`traversal`] module
//! also defines some convenience macros for pushing things other than child
//! nodes into the context, particularly for things that regularly involve
//! [format!].
//!
//! ## Diagnostics
//!
//! Rather than just passing `Result`s around, diagnostics are used to
//! communicate whether a plan is valid or not. This solves two problems:
//!
//!  - distinguishing between messages signalling provable invalidity
//!    (errors), messages signalling inability to determine validity
//!    (warnings), and messages that are just intended to provide extra
//!    information to the user;
//!  - returning as many diagnostics as possible, rather than just stopping
//!    at the first sight of trouble.
//!
//! Diagnostics can be pushed into the parser context using the [`diagnostic!`]
//! and [`ediagnostic!`] macros. The latter allows third-party `Err` types to
//! be pushed as the message, the former uses a [format!] syntax. However,
//! sometimes it also very useful to just use the `?` operator for something.
//! Therefore, parse functions also return
//! [`diagnostic::Result<T>`](crate::output::diagnostic::Result). This result
//! is taken care of by the traversal macros; when `Err`, the diagnostic cause
//! is simply pushed as an error. This also suppresses the usual "unknown
//! field" warning emitted when a parse function failed to traverse all its
//! children; after all, it probably exited early.
//!
//! More information about all the information recorded in a diagnostic can be
//! found in the docs for the [diagnostic](crate::output::diagnostic) module.
//!
//! Beyond diagnostics, it's also possible to push comments into the context.
//! This can be done using the [`comment!`] and [`link!`] macros, or, for more
//! control, by pushing a []
//!
//! ## Data types
//!
//! Data type information gets some special treatment, because it is so
//! important for validation. It's also very useful to have when debugging a
//! tree. It's considered so important that each
//! [`Node`](crate::output::tree::Node) has a place where it can store its
//! "return type". What this type actually represents depends on the type of
//! node:
//!
//!  - type nodes: the represented type;
//!  - expression nodes: the returned type;
//!  - relation nodes: the schema (automatically set by
//!    [`set_schema()`](context::Context::set_schema())).
//!
//! The data type can be set using the
//! [`set_data_type()`](context::Context::set_data_type()) method. Note that
//! all of the parsers for the above node types should call
//! [`set_data_type()`](context::Context::set_data_type()) at
//! least once, even if they're unable to determine what the actual type is;
//! in the latter case they can just push an unresolved type (for example
//! using `Default`, but additional information can be attached using
//! [`new_unresolved()`](crate::output::data_type::DataType::new_unresolved()).
//!
//! [`set_data_type()`](context::Context::set_data_type()) may be called more
//! than once for a single node. The data type of the node will simply be the
//! last one that was set when parsing for that node completes. However, each
//! call also records the data type as a special type of child of the node,
//! making the complete history of
//! [`set_data_type()`](context::Context::set_data_type()) calls visible in the
//! resulting parse tree.
//!
//! ## Schemas
//!
//! Perhaps even more important than data types in general are schemas; in
//! general, in order to be able to determine the data type returned by an
//! expression, contextual information about the schema(s) of the data
//! stream(s) being operated on needs to be known. Moreover, the context in
//! which an expression is evaluated may contain more than one schema when
//! subqueries get involved.
//!
//! This information is tracked in the schema stack. The stack can be
//! manipulated using the following functions.
//!
//!  - The root node of a relation tree must be parsed within the context
//!    created by
//!    [`enter_relation_root()`](context::Context::enter_relation_root()). This
//!    macro ensures that a schema is pushed onto the stack prior to traversal
//!    of the relation tree, and popped after traversal completes. Initially,
//!    the schema is set to an unresolved type, but the actual type should not
//!    matter at this stage, because it semantically doesn't exist until the
//!    first leaf in the relation tree is parsed.
//!  - All relations call [`clear_schema()`](context::Context::clear_schema())
//!    prior to any relation-specific logic (this is done by the RelType parse
//!    function), because semantically, no schema exists prior to parsing a
//!    relation.
//!  - [`set_schema()`](context::Context::set_schema()) sets or updates the
//!    current schema. It must be called every time the data stream is
//!    functionally updated, and just after the data stream is first created
//!    by leaf relations. Relations that combine data streams should call it
//!    just after traversal of its data sources completes (otherwise the
//!    active schema will be whatever the schema of the most recently parsed
//!    data source turned out to be). Doing so will also push the data type
//!    corresponding to the schema to the node, such that the final tree
//!    contains a type node for every semantic change of the data stream for
//!    debugging/documentation purposes.
//!
//! The current schema information can be retrieved using
//! [`schema()`](context::Context::schema()). Its integer argument specifies
//! how many subqueries to break out of; 0 is used to refer to the schema of
//! the current (sub)query, 1 is its parent query, 2 is its grandparent, and
//! so on.
//!
//! ## How the parser context works
//!
//! A context object contains the following things:
//!
//!  - [`output: &mut tree::Node`](crate::output::tree::Node), a mutable
//!    reference to the node in the output tree that we're writing to. Note
//!    that the [`traversal`] macros create a
//!    [`Node`](crate::output::tree::Node) already populated with the default
//!    [`NodeType`](crate::output::tree::NodeType) before calling the parse
//!    function, including a copy of the primitive data element for leaf nodes,
//!    and almost everything else can be added using the [`traversal`] macros,
//!    so you shouldn't normally have to mutate this. Exceptions exist, however,
//!    for example when an integer primitive needs to be upgraded to an anchor
//!    reference.
//!  - [`state: &mut context::State`](context::State), a mutable reference to a
//!    global state structure for the parser. This includes, for instance,
//!    lookup tables for things previously defined in the plan, such as
//!    function declarations. The state object is initially constructed by
//!    [`traversal`] using [`Default`], and is then just recursively passed to
//!    every parse function.
//!  - [`breadcrumb: &mut context::Breadcrumb`](context::Breadcrumb). This
//!    fulfills a similar purpose as `state`, but using a stack-like structure:
//!    for every child node, a new [`Breadcrumb`](context::Breadcrumb) is
//!    pushed onto the stack. Note that only the top of the stack is mutable.
//!    This is mostly used for keeping track of the current
//!    [`Path`](crate::output::path::Path) and internally by the [`traversal`]
//!    module; the parse functions can and should just use local variables when
//!    they need to store something this way.
//!  - [`config: &config::Config`](config::Config), a reference to the
//!    configuration structure that the validator was called with.

#[macro_use]
pub mod traversal;

#[macro_use]
pub mod context;

mod expressions;
mod extensions;
mod plan;
mod relations;
mod sorts;
mod types;

use crate::input::config;
use crate::input::proto;
use crate::output::parse_result;

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf>(
    buffer: B,
    config: &config::Config,
) -> parse_result::ParseResult {
    traversal::parse_proto::<proto::substrait::Plan, _, _>(
        buffer,
        "plan",
        plan::parse_plan,
        &mut context::State::default(),
        config,
    )
}
