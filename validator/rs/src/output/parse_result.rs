// SPDX-License-Identifier: Apache-2.0

//! Module for the toplevel type representing a parse/validation result.

use crate::export;
use crate::output::diagnostic;
use crate::output::tree;

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

impl From<diagnostic::Level> for Validity {
    fn from(level: diagnostic::Level) -> Self {
        match level {
            diagnostic::Level::Info => Validity::Valid,
            diagnostic::Level::Warning => Validity::MaybeValid,
            diagnostic::Level::Error => Validity::Invalid,
        }
    }
}

impl From<Validity> for diagnostic::Level {
    fn from(validity: Validity) -> Self {
        match validity {
            Validity::Valid => diagnostic::Level::Info,
            Validity::MaybeValid => diagnostic::Level::Warning,
            Validity::Invalid => diagnostic::Level::Error,
        }
    }
}

/// Representation of a parse/validation result.
pub struct ParseResult {
    /// The root node of the tree.
    pub root: tree::Node,
}

impl ParseResult {
    /// Iterates over all diagnostic messages in the tree.
    pub fn iter_diagnostics(&self) -> impl Iterator<Item = &diagnostic::Diagnostic> + '_ {
        self.root.iter_diagnostics()
    }

    /// Returns the first diagnostic of the highest severity level in the tree.
    pub fn get_diagnostic(&self) -> Option<&diagnostic::Diagnostic> {
        self.root.get_diagnostic()
    }

    /// Returns whether the plan represented by the given parse tree is valid.
    pub fn check(&self) -> Validity {
        if let Some(diag) = self.get_diagnostic() {
            diag.adjusted_level.into()
        } else {
            Validity::Valid
        }
    }

    /// Exports a parse tree to a file or other output device using the specified
    /// data format.
    pub fn export<T: std::io::Write>(
        &self,
        out: &mut T,
        format: export::Format,
    ) -> std::io::Result<()> {
        export::export(out, format, "plan", self)
    }
}
