use crate::path;
use std::fmt::Formatter;
use thiserror::Error;

/// Enumeration for the particular types of diagnostics we might encounter.
#[derive(Clone, Debug, Error)]
pub enum Cause {
    #[error("failed to parse proto format")]
    ProtoParseFailure(#[from] prost::DecodeError),
    #[error("unknown type {0}")]
    UnknownType(String),
    #[error("mismatched type parameters: {0}")]
    MismatchedTypeParameters(String),
    #[error("missing required field {0}")]
    MissingField(String),
}

/// Result type for diagnostics.
pub type Result<T> = std::result::Result<T, Cause>;

/// Error level for a diagnostic message.
#[derive(Clone, Debug)]
pub enum Level {
    /// Level used for diagnostics that indicate that there is definitely
    /// something wrong with the plan.
    Error,

    /// Level used for diagnostics that may or may not indicate that there
    /// is something wrong with the plan, i.e. the plan *could* be valid,
    /// but the validator isn't sure.
    Warning,

    /// Level used for diagnostics that don't point out anything wrong with
    /// the plan, and merely provide additional information.
    Info,
}

/// A complete diagnostic message.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// The cause of the diagnostic.
    pub cause: Cause,

    /// The severity of the diagnostic.
    pub level: Level,

    /// The path within the protobuf message where the diagnostic occurred.
    pub path: path::PathBuf,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({}): {}", self.level, self.path, self.cause)
    }
}

/// A list of diagnostics.
pub type Diagnostics = Vec<Diagnostic>;
