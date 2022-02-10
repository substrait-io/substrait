use std::fmt::Formatter;

/// Element of a path to some field of a protobuf message and/or YAML file.
#[derive(Clone, Debug, PartialEq)]
pub enum PathElement {
    /// Refers to an optional protobuf field with the given name within the
    /// message, or a YAML map entry with the given key.
    Field(&'static str),

    /// Refers to one of the elements of a repeated field with the given
    /// name within the message referred to by the parent path.
    Repeated(&'static str, usize),

    /// Refers to the selected variant of a OneOf field with the given name
    /// within the message referred to by the parent path. The first str is
    /// the field name, the second is the variant name.
    Variant(&'static str, &'static str),

    /// Refers to an indexed element within a YAML array.
    Index(usize),
}

impl std::fmt::Display for PathElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Field(field) => write!(f, ".{}", field),
            PathElement::Repeated(field, index) => write!(f, ".{}[{}]", field, index),
            PathElement::Variant(field, variant) => write!(f, ".{}<{}>", field, variant),
            PathElement::Index(index) => write!(f, "[{}]", index),
        }
    }
}

/// Refers to a location within a protobuf message.
#[derive(Clone, Debug, PartialEq)]
pub struct PathBuf {
    root: &'static str,
    elements: Vec<PathElement>,
}

impl std::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Path<'_> {
    /// Returns a new Path that references an optional field with the
    /// given name within the protobuf message referred to by the current
    /// path, or likewise for the key within a YAML map.
    pub fn with_field(&self, name: &'static str) -> Path {
        Path::Select(self, PathElement::Field(name))
    }

    /// Returns a new Path that references an element of a repeated field
    /// with the given name within the message referred to by the current
    /// path.
    pub fn with_repeated(&self, name: &'static str, index: usize) -> Path {
        Path::Select(self, PathElement::Repeated(name, index))
    }

    /// Returns a new Path that references a particular variant of a
    /// OneOf field with the given name within the message referred to
    /// by the current path.
    pub fn with_variant(&self, name: &'static str, variant: &'static str) -> Path {
        Path::Select(self, PathElement::Variant(name, variant))
    }

    /// Returns a new Path that references a YAML array element.
    pub fn with_index(&self, index: usize) -> Path {
        Path::Select(self, PathElement::Index(index))
    }
}

impl std::fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Root(name) => write!(f, "{}", name),
            Path::Select(parent, element) => write!(f, "{}{}", parent, element),
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
}
