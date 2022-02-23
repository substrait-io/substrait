use crate::path;
use thiserror::Error;

/// Enumeration for the particular types of diagnostics we might encounter.
#[derive(Clone, Debug, PartialEq, Error)]
pub enum Cause {
    #[error("failed to parse proto format: {0}")]
    ProtoParseFailed(#[from] prost::DecodeError),
    #[error("did not attempt to resolve YAML: {0}")]
    YamlResolutionDisabled(String),
    #[error("failed to resolve YAML: {0}")]
    YamlResolutionFailed(#[from] curl::Error),
    #[error("failed to parse YAML: {0}")]
    YamlParseFailed(String),
    #[error("YAML is invalid: {0}")]
    YamlSchemaValidationFailed(String),
    #[error("unknown type {0}")]
    UnknownType(String),
    #[error("mismatched type parameters: {0}")]
    MismatchedTypeParameters(String),
    #[error("missing required field {0}")]
    MissingField(String),
    #[error("encountered values for field(s) not yet understood by the validator: {0}")]
    UnknownField(String),
    #[error("encountered a protobuf \"any\": {0}")]
    ProtoAny(String),
    #[error("redundant protobuf \"any\" declaration: {0}")]
    RedundantProtoAnyDeclaration(String),
    #[error("missing protobuf \"any\" declaration: {0}")]
    MissingProtoAnyDeclaration(String),
    #[error("not yet implemented: {0}")]
    NotYetImplemented(String),
    #[error("illegal value: {0}")]
    IllegalValue(String),
    #[error("missing anchor for reference: {0}")]
    MissingAnchor(String),
    #[error("failed to resolve name: {0}")]
    NameResolutionFailed(String),
}

/// Result type for diagnostic causes.
pub type Result<T> = std::result::Result<T, Cause>;

/// Error level for a diagnostic message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Level used for diagnostics that don't point out anything wrong with
    /// the plan, and merely provide additional information.
    Info,

    /// Level used for diagnostics that may or may not indicate that there
    /// is something wrong with the plan, i.e. the plan *could* be valid,
    /// but the validator isn't sure.
    Warning,

    /// Level used for diagnostics that indicate that there is definitely
    /// something wrong with the plan.
    Error,
}

/// A complete diagnostic message.
#[derive(Clone, Debug, PartialEq, Error)]
pub struct Diagnostic {
    /// The cause of the diagnostic.
    pub cause: Cause,

    /// The severity of the diagnostic.
    pub level: Level,

    /// The path within the protobuf message where the diagnostic occurred.
    pub path: path::PathBuf,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({}): {}", self.level, self.path, self.cause)
    }
}

/// A list of diagnostics.
pub type Diagnostics = Vec<Diagnostic>;

/// Result type for complete diagnostics, including path.
pub type DiagResult<T> = std::result::Result<T, Diagnostic>;

/// Convenience/shorthand macro for creating error diagnostics.
macro_rules! error {
    ($path:expr, $cause:ident, $($fmts:expr),*) => {
        error!($path, crate::diagnostic::Cause::$cause(format!($($fmts),*)))
    };
    ($path:expr, $cause:expr) => {
        crate::diagnostic::Diagnostic {
            cause: $cause,
            level: crate::diagnostic::Level::Error,
            path: $path
        }
    };
}
