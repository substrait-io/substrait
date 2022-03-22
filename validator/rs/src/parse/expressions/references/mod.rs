// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating references.

use crate::input::proto::substrait;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::expressions;
use crate::string_util;
use crate::string_util::Describe;
use std::sync::Arc;

pub mod mask;
pub mod scalar;

/// Description of the root of a reference.
enum Root {
    Unresolved,
    Expression(expressions::Expression),
    Schema(usize),
}

impl From<expressions::Expression> for Root {
    fn from(e: expressions::Expression) -> Self {
        Root::Expression(e)
    }
}

impl Default for Root {
    fn default() -> Self {
        Root::Unresolved
    }
}

/// Description of a reference path.
pub struct ReferencePath {
    // *Reversed* list of segments.
    segments: Vec<String>,
}

impl Default for ReferencePath {
    fn default() -> Self {
        Self {
            segments: vec![String::from(".?")],
        }
    }
}

impl ReferencePath {
    fn new() -> Self {
        Self { segments: vec![] }
    }

    fn prefix(mut self, s: String) -> Self {
        self.segments.push(s);
        self
    }

    /// Returns the length of the complete path string.
    pub fn len(&self) -> usize {
        self.segments.iter().map(String::len).sum()
    }
}

impl Describe for ReferencePath {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        let lens = self.segments.iter().map(String::len).collect::<Vec<_>>();
        let (n_left, n_right) = limit.split_ns(&lens);
        for i in 0..n_left {
            write!(f, "{}", self.segments[self.segments.len() - i - 1])?;
        }
        if let Some(n_right) = n_right {
            write!(f, "..")?;
            for i in self.segments.len() - n_right..self.segments.len() {
                write!(f, "{}", self.segments[self.segments.len() - i - 1])?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for ReferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Description of a reference.
pub struct Reference {
    root: Root,
    path: ReferencePath,
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            root: Root::Schema(0),
            path: ReferencePath::default(),
        }
    }
}

impl Describe for Reference {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        let (path_limit, root_limit) = limit.split(self.path.len());
        match &self.root {
            Root::Unresolved => write!(f, "?")?,
            Root::Expression(e) => {
                write!(f, "(")?;
                e.describe(f, root_limit)?;
                write!(f, ")")?;
            }
            Root::Schema(0) => write!(f, "<>")?,
            Root::Schema(n) => write!(f, "<{n}>")?,
        }
        self.path.describe(f, path_limit)
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Parse a struct field index into its data type.
fn parse_struct_field_index(
    x: &i32,
    _y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<Arc<data_type::DataType>> {
    let index = *x;
    if index < 0 {
        return Err(cause!(
            IllegalValue,
            "struct indices cannot be less than zero"
        ));
    }
    let index: usize = index.try_into().unwrap();
    if root.is_struct() {
        let size = root.parameters().len();
        root.type_parameter(index)
            .ok_or_else(|| cause!(IllegalValue, "struct index out of range (size = {size})"))
    } else {
        Ok(Arc::default())
    }
}

/// Parse a reference root.
fn parse_root_type(
    x: &substrait::expression::field_reference::RootType,
    y: &mut context::Context,
) -> diagnostic::Result<Root> {
    match x {
        substrait::expression::field_reference::RootType::Expression(x) => {
            expressions::parse_expression(x.as_ref(), y).map(Root::from)
        }
        substrait::expression::field_reference::RootType::RootReference(_) => {
            describe!(y, Misc, "Reference to field of current query");
            y.set_data_type(y.schema(0)?);
            Ok(Root::Schema(0))
        }
        substrait::expression::field_reference::RootType::OuterReference(x) => {
            describe!(
                y,
                Misc,
                "Reference to field of {} outer query",
                string_util::describe_nth(x.steps_out)
            );
            proto_primitive_field!(x, y, steps_out, |x, y| {
                if *x < 1 {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "must be at least 1 (use RootReference instead)"
                    );
                }
                Ok(())
            });
            let steps_out = x.steps_out as usize;
            y.set_data_type(y.schema(steps_out)?);
            Ok(Root::Schema(steps_out))
        }
    }
}

/// Parse a reference path.
fn parse_reference_type(
    x: &substrait::expression::field_reference::ReferenceType,
    y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<ReferencePath> {
    match x {
        substrait::expression::field_reference::ReferenceType::DirectReference(x) => {
            scalar::parse_reference_segment(x, y, root)
        }
        substrait::expression::field_reference::ReferenceType::MaskedReference(x) => {
            mask::parse_mask_expression(x, y, root, false)?;
            Ok(ReferencePath::new().prefix(String::from(".mask(..)")))
        }
    }
}

/// Parse a field reference. Returns a description of the nested reference.
pub fn parse_field_reference(
    x: &substrait::expression::FieldReference,
    y: &mut context::Context,
) -> diagnostic::Result<Reference> {
    // Parse the root of the reference.
    let (root_node, root) = proto_required_field!(x, y, root_type, parse_root_type);
    let root = root.unwrap_or_default();

    // Parse the reference type.
    let (path_node, path) = proto_required_field!(
        x,
        y,
        reference_type,
        parse_reference_type,
        &root_node.data_type()
    );
    let path = path.unwrap_or_default();

    // Set the data type.
    y.set_data_type(path_node.data_type());

    // Describe node.
    let reference = Reference { root, path };
    describe!(y, Expression, "Selects {}", &reference);
    summary!(y, "Full reference path: {:#}", &reference);
    if let Root::Schema(depth) = &reference.root {
        let depth = *depth;
        y.push_summary(comment::Comment::new().nl());
        if depth == 0 {
            summary!(
                y,
                "Here, <> is used to refer to the row currently being processed."
            );
        } else {
            summary!(y, "Here, <{depth}> is used to refer to the row being processed by the {} outer query.", string_util::describe_nth(depth as u32));
        }
    }
    Ok(reference)
}
