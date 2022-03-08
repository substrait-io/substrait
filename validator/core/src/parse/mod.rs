//! Parsing/validation module.
//!
//! Roughly speaking, this module takes a Substrait plan represented using the
//! types provided by the [`input`](crate::input) module, and transforms it to
//! an equivalent plan represented using the types provided by the
//! [`output`](crate::output) module. In doing so, it parses and validates the
//! plan.
//!
//! TODO: document approach
//!
//! TODO: put this somewhere:
//!
//! Most functions return [`Result<T, Cause>`] results. This type is abbreviated
//! to just [`diagnostic::Result<T>`] in this module. [`diagnostic::Cause`]
//! structs are normally constructed using the [`cause!`] macro for convenience.
//! This macro takes the name of the [`diagnostic::Classification`] variant as
//! its first argument, and either something that converts into a supported error
//! type or a set of [`format!`] arguments as subsequent arguments.
//!
//! Some functions need a bit more control, and return
//! [`Result<T, diagnostic::RawDiagnostic>`] instead, a.k.a.
//! [`diagnostic::DiagResult<T>`]. This already includes error level and path
//! information. The [`diagnostic::RawDiagnostic`] is constructed using the
//! [`diag!`] macro, which works the same as [`cause!`], but includes a path
//! and level argument before the [`cause!`] arguments. You can also pass a
//! preconstructed [`diagnostic::Cause`] into it directly, as returned by an
//! inner function call, for instance.
//!
//! Ultimately, the above [`diagnostic::Result`] and [`diagnostic::DiagResult`]
//! enums end up in a parse function. The Err variant should then be handled by
//! pushing the diagnostic into the [`Node`](crate::Node) that is being emitted
//! by the parse function. For raw [`diagnostic::Cause`] structs (as returned
//! via [`diagnostic::Result`]), the path will be set to that of the node that
//! is being parsed, and the level will (usually) be set to `Error` (but this
//! depends on context; for YAML resolution errors for example the level will
//! be `Warning`). During this push operation, the [`diagnostic::RawDiagnostic`]
//! is also converted into an [`diagnostic::Diagnostic`], with its
//! `adjusted_level` set based on its classification and the configuration
//! passed to the validator by the user. This is all handled by the
//! [`diagnostic!`] macro and [`push_diagnostic()`](traversal::push_diagnostic)
//! function, defined in the [`traversal`]module.
//!
//! Note that parse functions themselves also return [`diagnostic::Result<T>`],
//! such that the `?` operator can be used in them. This error is handled by
//! the boilerplate code in [`traversal`].

#[macro_use]
pub mod traversal;

#[macro_use]
pub mod context;

mod extensions;

use crate::input::config;
use crate::input::proto;
use crate::output::diagnostic;
use crate::output::tree;

/// Toplevel parse function for a plan.
fn parse_plan(x: &proto::substrait::Plan, y: &mut context::Context) -> diagnostic::Result<()> {
    extensions::parse_extensions_before_relations(x, y);
    // TODO
    extensions::parse_extensions_after_relations(x, y);
    Ok(())
}

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf>(buffer: B, config: &config::Config) -> tree::Node {
    traversal::parse_proto::<proto::substrait::Plan, _, _>(
        buffer,
        "plan",
        parse_plan,
        &mut context::State::default(),
        config,
    )
}
