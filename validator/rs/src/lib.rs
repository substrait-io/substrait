// SPDX-License-Identifier: Apache-2.0

//! Crate for validating [Substrait](https://substrait.io/).
//!
//! The usage pattern is roughly as follows.
//!
//!  1) Build a [`Config`] structure to configure the validator. You can also
//!     just use [`std::default::Default`] if you don't need to configure
//!     anything, but you might want to at least call
//!     [`WithCurlResolver::add_curl_yaml_uri_resolver()`].
//!  2) Parse the incoming `substrait.Plan` message using [`parse()`]. This
//!     creates a [tree](substrait_validator_core::output::tree) structure
//!     corresponding to the query plan that also contains diagnostics and
//!     other annotations added by the validator.
//!  3) If you don't want to traverse the tree yourself, use
//!     [`check()`], [`get_diagnostic()`], [`iter_diagnostics()`], or
//!     [`export()`] to obtain the validation results you need.
//!
//! Note that only the binary protobuf serialization format is supported at the
//! input; the JSON format is *not* supported. This is a limitation of `prost`,
//! the crate that was used for protobuf deserialization. If you're looking for
//! a library (or CLI) that supports more human-friendly input, check out the
//! Python bindings.

#[macro_use]
pub mod output;

#[macro_use]
mod parse;

pub mod export;
pub mod input;

// Aliases for common types used on the crate interface.
pub use input::config::glob::Pattern;
pub use input::config::Config;
pub use output::diagnostic::Classification;
pub use output::diagnostic::Diagnostic;
pub use output::diagnostic::Level;
pub use output::tree::Node;

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf>(buffer: B, config: &Config) -> Node {
    parse::parse(buffer, config)
}

/// Iterates over all diagnostic messages in the tree.
pub fn iter_diagnostics(root: &Node) -> impl Iterator<Item = &Diagnostic> + '_ {
    root.iter_diagnostics()
}

/// Returns the first diagnostic of the highest severity level in the tree.
pub fn get_diagnostic(root: &Node) -> Option<&Diagnostic> {
    let mut result: Option<&Diagnostic> = None;
    for diag in iter_diagnostics(root) {
        // We can return immediately for error diagnostics, since this is the
        // highest level.
        if diag.adjusted_level == Level::Error {
            return Some(diag);
        }

        // For other levels, update only if the incoming diagnostic is of a
        // higher level/severity than the current one.
        if let Some(cur) = result.as_mut() {
            if diag.adjusted_level > (*cur).adjusted_level {
                *cur = diag;
            }
        } else {
            result = Some(diag);
        }
    }
    result
}

/// Validity of a plan.
///
/// Note that there is a one-to-one correspondence with Level. The only
/// difference between Level and Validity is that the variant names for Level
/// are more sensible in the context of a diagnostic, while the names for
/// Validity are more sensible when talking about a validation result as a
/// whole.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Validity {
    /// The plan is valid.
    Valid,

    /// The plan may or may not be valid; the validator was not able to prove
    /// or disprove validity.
    MaybeValid,

    /// The plan is invalid.
    Invalid,
}

impl From<Level> for Validity {
    fn from(level: Level) -> Self {
        match level {
            Level::Info => Validity::Valid,
            Level::Warning => Validity::MaybeValid,
            Level::Error => Validity::Invalid,
        }
    }
}

impl From<Validity> for Level {
    fn from(validity: Validity) -> Self {
        match validity {
            Validity::Valid => Level::Info,
            Validity::MaybeValid => Level::Warning,
            Validity::Invalid => Level::Error,
        }
    }
}

/// Returns whether the plan represented by the given parse tree is valid.
pub fn check(root: &Node) -> Validity {
    if let Some(diag) = get_diagnostic(root) {
        diag.adjusted_level.into()
    } else {
        Validity::Valid
    }
}

/// Exports a parse tree to a file or other output device using the specified
/// data format.
pub fn export<T: std::io::Write>(
    out: &mut T,
    format: export::Format,
    root: &Node,
) -> std::io::Result<()> {
    export::export(out, format, "plan", root)
}
