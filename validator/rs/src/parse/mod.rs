// SPDX-License-Identifier: Apache-2.0

//! Parsing/validation module.
//!
//! Roughly speaking, this module takes a Substrait plan represented using the
//! types provided by the [`input`](crate::input) module, and transforms it to
//! an equivalent plan represented using the types provided by the
//! [`output`](crate::output) module. In doing so, it parses and validates the
//! plan.
//!
//! Most of the boilerplate code for tree traversal is handled by the
//! [`traversal`] module. What remains are "parse functions" of the form
//! `(x: &T, y: &mut Context) -> Result<R>`, where:
//!
//!  - `x` is a reference to the the JSON/YAML value or the prost wrapper for
//!    the protobuf message that is to be parsed and validated;
//!  - `y` is the parse context; and
//!  - `R` is any desired return type.
//!
//! The body of the parse function can use a wide variety of function-like
//! macros from [`traversal`] to traverse the children of `x` in the
//! appropriate order and with the appropriate parse functions. The macros
//! return a tuple of a reference to the created
//! [Node](crate::output::tree::Node) and the `R` returned by the parse
//! function (depending on the macro, these may be wrapped in [`Option`]s or
//! [`Vec`]s). Note that any children not traversed by the parse function will
//! automatically be traversed by [`traversal`] (along with a warning
//! diagnostic that these children were not validated), and that traversing a
//! child twice is illegal (this will panic).
//!
//! If the parse function fails in an unrecoverable way, it can return Err via
//! `?`. When it does this, the [`traversal`] macros takes care of pushing an
//! appropriate diagnostic into the output tree. For recoverable errors or other
//! diagnostics, [`diagnostic!`] can be used. Miscellaneous information can be
//! pushed into the output tree via [`comment!`], [`link!`], and
//! [`data_type!`].
//!
//! The reference to the [`context::Context`] object can also be used directly.
//! It contains the following things:
//!
//!  - [`output: &mut tree::Node`](crate::output::tree::Node), a mutable
//!    reference to the node in the output tree that we're writing to. Note
//!    that the [`traversal`] macros create a
//!    [`Node`](crate::output::tree::Node) already populated with the default
//!    [`NodeType`](crate::output::tree::NodeType) before calling the parse
//!    function, including a copy of the primitive data element for leaf nodes,
//!    and everything else can be added using the [`traversal`] macros, so you
//!    shouldn't normally need to access this. Exceptions exist, however, for
//!    example when an integer primitive needs to be upgraded to an anchor
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

mod extensions;
mod plan;
mod relations;
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
