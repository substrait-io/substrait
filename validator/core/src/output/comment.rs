// SPDX-License-Identifier: Apache-2.0

//! Module for comments.
//!
//! [`Comment`]s can be added to nodes between the child edges to attach
//! additional miscellaneous information that doesn't fit in any of the more
//! structured types, intended purely to be formatted for and interpreted by
//! humans.

use crate::output::path;

/// Representation of a comment message intended only for human consumption.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Comment {
    /// Spans that make up the comment. These are simply concatenated, but
    /// spans may contain optional formatting information.
    pub spans: Vec<Span>,
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in self.spans.iter() {
            span.fmt(f)?;
        }
        Ok(())
    }
}

impl From<String> for Comment {
    fn from(text: String) -> Self {
        Self {
            spans: vec![text.into()],
        }
    }
}

impl Comment {
    /// Creates an empty comment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a piece of plain text to the comment.
    pub fn with_plain(mut self, text: String) -> Self {
        self.spans.push(text.into());
        self
    }

    /// Adds a piece of text to the comment that links to the given path.
    pub fn with_link_to_path(mut self, text: String, path: path::PathBuf) -> Self {
        self.spans.push(Span {
            text,
            link: Some(Link::Path(path)),
        });
        self
    }

    /// Adds a piece of text to the comment that links to the given URL.
    pub fn with_link_to_url(mut self, text: String, url: String) -> Self {
        self.spans.push(Span {
            text,
            link: Some(Link::Url(url)),
        });
        self
    }
}

/// A span of text within a comment.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// The span of text.
    pub text: String,

    /// Whether this span of text should link to something.
    pub link: Option<Link>,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<String> for Span {
    fn from(text: String) -> Self {
        Span { text, link: None }
    }
}

/// A link to something.
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    /// Link to another node in the tree, via an absolute node path.
    Path(path::PathBuf),

    /// Link to some external URL.
    Url(String),
}
