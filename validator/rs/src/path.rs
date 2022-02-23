use crate::primitives;

/// Element of a path to some field of a protobuf message and/or YAML file.
#[derive(Clone, Debug, PartialEq)]
pub enum PathElement {
    /// Refers to an optional protobuf field with the given name within the
    /// message, or a YAML map entry with the given key.
    Field(String),

    /// Refers to one of the elements of a repeated field with the given
    /// name within the message referred to by the parent path.
    Repeated(String, usize),

    /// Refers to the selected variant of a OneOf field with the given name
    /// within the message referred to by the parent path. The first str is
    /// the field name, the second is the variant name.
    Variant(String, String),

    /// Refers to an indexed element within a YAML array.
    Index(usize),
}

impl std::fmt::Display for PathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Field(field) => write!(f, ".{}", primitives::as_ident_or_string(field)),
            PathElement::Repeated(field, index) => {
                write!(f, ".{}[{}]", primitives::as_ident_or_string(field), index)
            }
            PathElement::Variant(field, variant) => write!(
                f,
                ".{}<{}>",
                primitives::as_ident_or_string(field),
                primitives::as_ident_or_string(variant)
            ),
            PathElement::Index(index) => write!(f, "[{}]", index),
        }
    }
}

/// Refers to a location within a protobuf message.
#[derive(Clone, Debug, PartialEq)]
pub struct PathBuf {
    pub root: &'static str,
    pub elements: Vec<PathElement>,
}

impl std::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)?;
        for element in self.elements.iter() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

/// Used to track a location within a protobuf message. The owned version
/// is PathBuf.
#[derive(Clone, Debug, PartialEq)]
pub enum Path<'a> {
    /// Refers to the root message.
    Root(&'static str),

    /// Refers to an optional field with the given name within the message
    /// referred to by the given parent path.
    Select(&'a Path<'a>, PathElement),
}

impl Default for Path<'_> {
    fn default() -> Self {
        Path::Root("")
    }
}

impl Path<'_> {
    /// Returns a new Path that references an optional field with the
    /// given name within the protobuf message referred to by the current
    /// path, or likewise for the key within a YAML map.
    pub fn with(&self, element: PathElement) -> Path {
        Path::Select(self, element)
    }

    /// Returns a new Path that references an optional field with the
    /// given name within the protobuf message referred to by the current
    /// path, or likewise for the key within a YAML map.
    pub fn with_field<S: Into<String>>(&self, name: S) -> Path {
        self.with(PathElement::Field(name.into()))
    }

    /// Returns a new Path that references an element of a repeated field
    /// with the given name within the message referred to by the current
    /// path.
    pub fn with_repeated<S: Into<String>>(&self, name: S, index: usize) -> Path {
        self.with(PathElement::Repeated(name.into(), index))
    }

    /// Returns a new Path that references a particular variant of a
    /// OneOf field with the given name within the message referred to
    /// by the current path.
    pub fn with_variant<S: Into<String>, V: Into<String>>(&self, name: S, variant: V) -> Path {
        self.with(PathElement::Variant(name.into(), variant.into()))
    }

    /// Returns a new Path that references a YAML array element.
    pub fn with_index(&self, index: usize) -> Path {
        self.with(PathElement::Index(index))
    }
}

impl std::fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Root(name) => write!(f, "{}", name),
            Path::Select(parent, element) => write!(f, "{}{}", parent, element),
        }
    }
}

impl Path<'_> {
    pub fn end_to_string(&self) -> String {
        match self {
            Path::Root(name) => name.to_string(),
            Path::Select(_, element) => element.to_string(),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        let a = Path::Root("a");
        let b = a.with_field("b");
        let c = b.with_repeated("c", 42);
        let d = c.with_variant("d", "e");
        let e = d.with_index(33);
        let buf: PathBuf = e.to_path_buf();
        assert_eq!(format!("{}", e), "a.b.c[42].d<e>[33]");
        assert_eq!(format!("{}", buf), "a.b.c[42].d<e>[33]");
    }

    #[test]
    fn non_ident_paths() {
        let a = Path::Root("a");
        let b = a.with_field("4");
        let c = b.with_repeated("8", 15);
        let d = c.with_variant("16", "23");
        let e = d.with_index(42);
        let buf: PathBuf = e.to_path_buf();
        assert_eq!(format!("{}", e), "a.\"4\".\"8\"[15].\"16\"<\"23\">[42]");
        assert_eq!(format!("{}", buf), "a.\"4\".\"8\"[15].\"16\"<\"23\">[42]");
    }
}
