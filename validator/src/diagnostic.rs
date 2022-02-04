use std::fmt::Formatter;
use thiserror::Error;

/// Enumeration for the particular types of diagnostics we might encounter.
#[derive(Error, Debug, Clone)]
pub enum Cause {
    #[error("failed to parse proto format")]
    ProtoParseFailure(#[from] prost::DecodeError),
}

/// Result type for diagnostics.
pub type Result<T> = std::result::Result<T, Cause>;

/// Error level for a diagnostic message.
#[derive(Debug)]
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

/// Element of a path to some field of a protobuf message.
#[derive(Debug, Clone)]
pub enum PathElement {
    /// Refers to an optional field with the given name within the message
    /// referred to by the parent path.
    Optional(&'static str),

    /// Refers to one of the elements of a repeated field with the given
    /// name within the message referred to by the parent path.
    Repeated(&'static str, usize),

    /// Refers to the selected variant of a OneOf field with the given name
    /// within the message referred to by the parent path. The first str is
    /// the field name, the second is the variant name.
    Variant(&'static str, &'static str),
}

impl std::fmt::Display for PathElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Optional(field) => write!(f, "{}", field),
            PathElement::Repeated(field, index) => write!(f, "{}[{}]", field, index),
            PathElement::Variant(field, variant) => write!(f, "{}<{}>", field, variant),
        }
    }
}

/// Refers to a location within a protobuf message.
#[derive(Debug)]
pub struct PathBuf {
    root: &'static str,
    elements: Vec<PathElement>,
}

impl std::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)?;
        for element in self.elements.iter() {
            write!(f, ".{}", element)?;
        }
        Ok(())
    }
}

/// Used to track a location within a protobuf message. The owned version
/// is PathBuf.
#[derive(Debug)]
pub enum Path<'a> {
    /// Refers to the root message.
    Root(&'static str),

    /// Refers to an optional field with the given name within the message
    /// referred to by the given parent path.
    Select(&'a Path<'a>, PathElement),
}

impl Path<'_> {
    /// Returns a new Path that references an optional field with the
    /// given name within the message referred to by the current path.
    pub fn select_optional(&self, name: &'static str) -> Path {
        Path::Select(self, PathElement::Optional(name))
    }

    /// Returns a new Path that references an element of a repeated field
    /// with the given name within the message referred to by the current
    /// path.
    pub fn select_repeated(&self, name: &'static str, index: usize) -> Path {
        Path::Select(self, PathElement::Repeated(name, index))
    }

    /// Returns a new Path that references a particular variant of a
    /// OneOf field with the given name within the message referred to
    /// by the current path.
    pub fn select_variant(&self, name: &'static str, variant: &'static str) -> Path {
        Path::Select(self, PathElement::Variant(name, variant))
    }
}

impl std::fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Root(name) => write!(f, "{}", name),
            Path::Select(parent, element) => write!(f, "{}.{}", parent, element),
        }
    }
}

impl Path<'_> {
    /// Creates an owned version of this Path.
    pub fn to_path_buf(&self) -> PathBuf {
        match self {
            Path::Root(name) => PathBuf {
                root: name,
                elements: vec![],
            },
            Path::Select(parent, element) => {
                let mut parent = parent.to_path_buf();
                parent.elements.push(element.clone());
                parent
            }
        }
    }
}

impl From<Path<'_>> for PathBuf {
    fn from(path: Path<'_>) -> Self {
        path.to_path_buf()
    }
}

/// A complete diagnostic message.
#[derive(Debug)]
pub struct Diagnostic {
    /// The cause of the diagnostic.
    pub cause: Cause,

    /// The severity of the diagnostic.
    pub level: Level,

    /// The path within the protobuf message where the diagnostic occurred.
    pub path: PathBuf,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({}): {}", self.level, self.path, self.cause)
    }
}

/// A list of diagnostics.
pub type Diagnostics = Vec<Diagnostic>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        let a = Path::Root("a");
        let b = a.select_optional("b");
        let c = b.select_repeated("c", 42);
        let d = c.select_variant("d", "e");
        let buf: PathBuf = d.to_path_buf();
        assert_eq!(format!("{}", d), "a.b.c[42].d<e>");
        assert_eq!(format!("{}", buf), "a.b.c[42].d<e>");
    }
}
