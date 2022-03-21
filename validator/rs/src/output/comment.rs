// SPDX-License-Identifier: Apache-2.0

//! Module for comments.
//!
//! [`Comment`]s can be added to nodes between the child edges to attach
//! additional miscellaneous information that doesn't fit in any of the more
//! structured types, intended purely to be formatted for and interpreted by
//! humans.

use crate::output::path;

/// Representation of a comment message intended only for human consumption.
/// Includes basic formatting information.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Comment {
    /// Formatting elements and spans that make up the comment.
    elements: Vec<Element>,
}

impl Comment {
    /// Creates an empty comment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a piece of plain text to the comment.
    pub fn plain<S: ToString>(mut self, text: S) -> Self {
        self.push(Element::Span(text.to_string().into()));
        self
    }

    /// Adds a piece of text to the comment that links to the given path.
    pub fn link<S: ToString>(mut self, text: S, path: path::PathBuf) -> Self {
        self.push(Element::Span(Span {
            text: text.to_string(),
            link: Some(Link::Path(path)),
        }));
        self
    }

    /// Adds a piece of text to the comment that links to the given URL.
    pub fn url<S: ToString, U: ToString>(mut self, text: S, url: U) -> Self {
        self.push(Element::Span(Span {
            text: text.to_string(),
            link: Some(Link::Url(url.to_string())),
        }));
        self
    }

    /// Adds a newline/paragraph break.
    pub fn nl(mut self) -> Self {
        self.push(Element::NewLine);
        self
    }

    /// Opens a list.
    pub fn lo(mut self) -> Self {
        self.push(Element::ListOpen);
        self
    }

    /// Advances to the next list item.
    pub fn li(mut self) -> Self {
        self.push(Element::ListNext);
        self
    }

    /// Closes the current list.
    pub fn lc(mut self) -> Self {
        self.push(Element::ListClose);
        self
    }

    /// Pushes an element into this comment.
    pub fn push(&mut self, element: Element) {
        // Some pairs of element types should never follow each other, because
        // one implies the other.
        match self.elements.pop() {
            None => self.elements.push(element),
            Some(Element::Span(s1)) => {
                if let Element::Span(s2) = element {
                    let (s1, maybe_s2) = merge_spans(s1, s2);
                    self.elements.push(Element::Span(s1));
                    if let Some(s2) = maybe_s2 {
                        self.elements.push(Element::Span(s2));
                    }
                } else {
                    self.elements.push(Element::Span(s1));
                    self.elements.push(element);
                }
            }
            Some(Element::NewLine) => {
                if matches!(element, Element::Span(_)) {
                    self.elements.push(Element::NewLine);
                }
                self.elements.push(element);
            }
            Some(Element::ListOpen) => {
                self.elements.push(Element::ListOpen);
                if !matches!(element, Element::ListNext) {
                    self.elements.push(element);
                }
            }
            Some(Element::ListNext) => {
                self.elements.push(Element::ListNext);
                if !matches!(element, Element::ListNext) {
                    self.elements.push(element);
                }
            }
            Some(Element::ListClose) => {
                self.elements.push(Element::ListClose);
                if !matches!(element, Element::NewLine) {
                    self.elements.push(element);
                }
            }
        }
    }

    /// Pushes a whole other comment's worth of elements into this comment.
    pub fn extend(&mut self, other: Comment) {
        let mut it = other.elements.into_iter();

        // The first element of other may need to be merged with its new
        // predecessor.
        if let Some(element) = it.next() {
            self.push(element);
        }

        // The rest of the elements would already have been merged, so we can
        // just copy them over.
        self.elements.extend(it);
    }

    /// Returns the slice of elements that comprise the comment.
    ///
    /// This list is "minimal:"
    ///  - there are no consecutive newlines, list item tags, or spans with
    ///    equal formatting (they are merged together);
    ///  - there are no empty lists, and there is never a list item immediately
    ///    following a list open tag (as this is redundant).
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        for element in self.elements.iter() {
            match element {
                Element::Span(span) => span.fmt(f),
                Element::NewLine => write!(f, "\n\n{: >1$}", "", indent),
                Element::ListOpen => {
                    indent += 3;
                    write!(f, "\n\n{: >1$}", "- ", indent)
                }
                Element::ListNext => {
                    write!(f, "\n\n{: >1$}", "- ", indent)
                }
                Element::ListClose => {
                    indent -= 3;
                    write!(f, "\n\n{: >1$}", "", indent)
                }
            }?;
        }
        Ok(())
    }
}

impl From<String> for Comment {
    fn from(text: String) -> Self {
        Self {
            elements: vec![Element::Span(text.into())],
        }
    }
}

/// A comment element.
#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    /// A span of text. Should not include newlines.
    Span(Span),

    /// A newline/paragraph break.
    NewLine,

    /// Starts a new list. Subsequent spans form the text for the first item.
    ListOpen,

    /// Advances to the next list item.
    ListNext,

    /// Closes a list.
    ListClose,
}

/// Like Comment, but single-line.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Brief {
    /// Spans that make up the comment. These are simply concatenated, but
    /// spans may contain optional formatting information.
    spans: Vec<Span>,
}

impl Brief {
    /// Creates an empty comment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a piece of plain text to the comment.
    pub fn plain<S: ToString>(mut self, text: S) -> Self {
        self.push(text.to_string().into());
        self
    }

    /// Adds a piece of text to the comment that links to the given path.
    pub fn link<S: ToString>(mut self, text: S, path: path::PathBuf) -> Self {
        self.push(Span {
            text: text.to_string(),
            link: Some(Link::Path(path)),
        });
        self
    }

    /// Adds a piece of text to the comment that links to the given URL.
    pub fn url<S: ToString, U: ToString>(mut self, text: S, url: U) -> Self {
        self.push(Span {
            text: text.to_string(),
            link: Some(Link::Url(url.to_string())),
        });
        self
    }

    /// Pushes a span into this brief.
    pub fn push(&mut self, span: Span) {
        if let Some(s1) = self.spans.pop() {
            let s2 = span;
            let (s1, maybe_s2) = merge_spans(s1, s2);
            self.spans.push(s1);
            if let Some(s2) = maybe_s2 {
                self.spans.push(s2);
            }
        } else {
            self.spans.push(span);
        }
    }

    /// Pushes a whole other brief's worth of elements into this brief.
    pub fn extend(&mut self, other: Brief) {
        let mut it = other.spans.into_iter();

        // The first span of other may need to be merged with its new
        // predecessor.
        if let Some(element) = it.next() {
            self.push(element);
        }

        // The rest of the spans would already have been merged, so we can
        // just copy them over.
        self.spans.extend(it);
    }

    /// Returns the slice of spans that comprise the brief.
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
}

impl std::fmt::Display for Brief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in self.spans.iter() {
            span.fmt(f)?;
        }
        Ok(())
    }
}

impl From<String> for Brief {
    fn from(text: String) -> Self {
        Self {
            spans: vec![text.into()],
        }
    }
}

impl From<Brief> for Comment {
    fn from(brief: Brief) -> Self {
        Self {
            elements: brief.spans.into_iter().map(Element::Span).collect(),
        }
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

/// Merges two spans together, if possible. A space is inserted between the
/// spans if there isn't one already.
fn merge_spans(mut a: Span, b: Span) -> (Span, Option<Span>) {
    if b.text.is_empty() {
        return (a, None);
    }
    if !a.text.ends_with(' ') && !b.text.starts_with(' ') {
        a.text.push(' ');
    }
    if a.link == b.link {
        a.text += &b.text;
        return (a, None);
    }
    (a, Some(b))
}

/// A link to something.
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    /// Link to another node in the tree, via an absolute node path.
    Path(path::PathBuf),

    /// Link to some external URL.
    Url(String),
}
