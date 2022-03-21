// SPDX-License-Identifier: Apache-2.0

//! This module provides a human-readable export format based on HTML.

use crate::output::comment;
use crate::output::diagnostic;
use crate::output::parse_result;
use crate::output::path;
use crate::output::tree;

const HEADER: &str = r#"
<!DOCTYPE html>
<html>
<head>
<link href="//netdna.bootstrapcdn.com/font-awesome/3.2.1/css/font-awesome.css" rel="stylesheet">
<style>
body {
    font-family: sans-serif;
}

details,
div.card {
    border: 1px solid;
    border-color: rgba(0, 0, 0, .3);
    color: rgba(0, 0, 0, .8);
    border-radius: 4px;
    margin-top: .2em;
}

details:hover,
div.card:hover {
    border-color: #000;
    color: rgba(0, 0, 0, .9);
}

details:target,
div.card:target {
    box-shadow: 0 0 .3em .2em rgba(0, 0, 0, 0.3);
    border-color: #000;
    color: rgba(0, 0, 0, .9);
}

details {
    padding: .2em .5em 0;
}

summary {
    margin: -.2em -.5em 0;
    padding: .2em .5em;
}

details[open] {
    padding: .2em .5em;
}

details[open] > summary {
    border-bottom: 1px solid rgba(0, 0, 0, .3);
    margin-bottom: .2em;
}

div.card {
    padding: .2em .5em;
}

details.ok {
    background-color: #dfd;
}

details.warn_child {
    background-color: #fed;
}

details.warn_here {
    background-color: #fdb;
}

details.error_child {
    background-color: #fdd;
}

details.error_here {
    background-color: #fbb;
}

details.unknown {
    background-color: #ddd;
}

details.data_type {
    background-color: #def;
}

details.data_type > summary::before {
    font-family: "Font Awesome 5 Free";
    color: #048;
    content: "\f0db";
    padding-right: .2em;
}

div.data_type {
    background-color: #bdf;
}

div.data_type::before {
    font-family: "Font Awesome 5 Free";
    color: #048;
    content: "\f0db";
    padding-right: .2em;
}

div.comment {
    background-color: #bfd;
}

div.comment::before {
    font-family: "Font Awesome 5 Free";
    color: #084;
    content: "\f249";
    padding-right: .2em;
}

details > p {
    margin: 0 0 0.2em;
    font-style: italic;
}

details.relation_tree {
    background-color: #bdf;
}

details.relation_tree > summary::before {
    font-family: "Font Awesome 5 Free";
    color: #048;
    content: "\f0e8";
    padding-right: .2em;
}

div.diag_info {
    background-color: #9f9;
    color: #333;
}

div.diag_info::before,
summary.valid::before {
    font-family: "Font Awesome 5 Free";
    color: #080;
    content: "\f058";
}

span.valid {
    color: #080;
    font-weight: bold;
}

div.diag_warn {
    background-color: #fc9;
    color: #333;
    font-weight: bold;
}

div.diag_warn::before,
summary.maybe_valid::before {
    font-family: "Font Awesome 5 Free";
    color: #840;
    content: "\f059";
}

div.diag_error {
    background-color: #f99;
    color: #000;
    font-weight: bold;
}

div.diag_error::before,
summary.invalid::before {
    font-family: "Font Awesome 5 Free";
    color: #800;
    content: "\f00d";
}

span.invalid {
    color: #c00;
    font-weight: bold;
}

a.anchor {
    opacity: 0.4;
    text-decoration: none;
    float: right;
}

a.anchor:hover {
    opacity: 1.0;
}

a.anchor::before {
    font-family: "Font Awesome 5 Free";
    color: #000;
    content: "\f0c1";
}

details:target,
div.card:target {
  animation: highlight 1000ms ease-out;
}

@keyframes highlight {
  0% { box-shadow: 0 0 2em 1em rgba(0, 0, 0, 0.3); }
  50% { box-shadow: 0 0 2em 1em rgba(0, 0, 0, 0.3); }
  100% { }
}

span.field {
    font-weight: bold;
    color: #333;
}

span.value {
    font-weight: bold;
    color: #000;
}

span.brief {
    font-style: italic;
    color: #000;
}

span.type {
    font-style: italic;
    font-size: 80%;
    color: #555;
}

span.cause {
    font-weight: normal;
}

div.note {
    font-style: italic;
    color: #555;
}

.tree,
.tree ul,
.tree li {
    list-style: none;
    margin: 0;
    padding: 0;
    position: relative;
}

.tree {
    margin: 0 auto 1em;
    text-align: center;
}

.tree,
.tree ul {
    display: table;
}

.tree ul {
    width: 100%;
}

.tree li {
    display: table-cell;
    padding: 1.5em 0 0;
    vertical-align: top;
}

/* _________ */
.tree li:before {
    outline: solid 1px #666;
    content: "";
    left: 0;
    position: absolute;
    right: 0;
    top: 0;
}

.tree li:first-child:before {
    left: 50%;
}

.tree li:last-child:before {
    right: 50%;
}

.tree span {
    border: solid 0.1em #666;
    border-radius: 0.2em;
    display: inline-block;
    margin: 0 0.2em 0.5em;
    padding: 0.2em 0.5em;
    position: relative;
}

/* | */
.tree ul:before {
    outline: solid 1px #555;
    content: "";
    height: 0.5em;
    left: 50%;
    position: absolute;
}

.tree span:before {
    margin-left: -1px;
    padding-left: 0.2em;
    font-size: 100%;
    content: "";
    height: 1.5em;
    left: 50%;
    position: absolute;
}

.tree span.data_source:before {
    border-left: solid 2px #555;
}

.tree span.subquery:before {
    border-left: dotted 2px #555;
}

.tree ul:before {
    top: -0.5em;
}

.tree span:before {
    top: -1.55em;
}

/* The root node doesn't connect upwards */
.tree > li {
    margin-top: 0;
}

.tree > li:before,
.tree > li:after,
.tree > li > span:before {
    outline: none !important;
    border: none !important;
}

</style>
</head>
<body>
"#;

const FOOTER: &str = r#"
<script>
function open_cards(element) {
    if (element.tagName.toLowerCase() === 'details') {
        element.open = true;
    }
    if (element.parentElement !== null) {
        open_cards(element.parentElement);
    }
}
function select() {
    var hash = location.hash.substring(1);
    if (hash) {
        var details = document.getElementById(hash);
        if (details) {
            open_cards(details);
        }
    }
}
window.addEventListener('hashchange', select);
select();
</script>

</body>
</html>
"#;

/// All the error levels for nodes that we have different formatting for in
/// the context of HTML output.
#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Level {
    /// Subtree is valid.
    Ok,

    /// There are descendent nodes with warnings.
    ChildWarning,

    /// The current node has warnings.
    Warning,

    /// There are descendent nodes with errors.
    ChildError,

    /// The current node has errors.
    Error,
}

impl From<diagnostic::Level> for Level {
    fn from(level: diagnostic::Level) -> Self {
        match level {
            diagnostic::Level::Info => Level::Ok,
            diagnostic::Level::Warning => Level::Warning,
            diagnostic::Level::Error => Level::Error,
        }
    }
}

impl Level {
    pub fn class(&self) -> &'static str {
        match self {
            Level::Ok => "ok",
            Level::ChildWarning => "warn_child",
            Level::Warning => "warn_here",
            Level::ChildError => "error_child",
            Level::Error => "error_here",
        }
    }
}

/// Escapes HTML text or parameter values using character entities.
fn html_escape<S: AsRef<str>>(text: S) -> String {
    let text = text.as_ref();
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => result += "&amp;",
            '<' => result += "&lt;",
            '>' => result += "&gt;",
            '"' => result += "&quot;",
            '\'' => result += "&apos;",
            c => result.push(c),
        }
    }
    result
}

/// Encodes part of an URL using percent escape sequences.
fn url_encode<S: AsRef<str>>(text: S) -> String {
    use std::fmt::Write;
    let text = text.as_ref();
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_alphanumeric() || "-._~!$&'()*+,;=:@".contains(c) {
            result.push(c);
        } else {
            let mut buf = [0; 4];
            for b in c.encode_utf8(&mut buf).as_bytes() {
                write!(result, "%{:02x}", *b).unwrap();
            }
        }
    }
    result
}

/// Encodes a node path using () instead of [] and {}. Such paths should be
/// still be unambiguous, and should be more readable than their
/// percent-encoded variants (only round parentheses are unreserved in URLs).
fn path_encode<S: AsRef<str>>(text: S) -> String {
    text.as_ref()
        .chars()
        .map(|c| match c {
            '[' => '(',
            ']' => ')',
            '<' => '(',
            '>' => ')',
            c => c,
        })
        .collect()
}

/// Formats a path to a node or diagnostic.
fn format_path(path: &path::PathBuf, index: Option<usize>) -> String {
    if let Some(index) = index {
        format!("{path}:{index}")
    } else {
        path.to_string()
    }
}

/// Formats the parameters of an <a> tag to a node or diagnostic.
fn format_reference_parameters(path: &path::PathBuf, index: Option<usize>) -> String {
    let path = format_path(path, index);
    format!(
        "href=\"#{}\" title=\"{}\"",
        html_escape(url_encode(path_encode(&path))),
        html_escape(&path)
    )
}

/// Formats a link to a node (index = None)
/// or diagnostic (index = Some(index of NodeData entry)).
fn format_reference<S: std::fmt::Display>(
    text: S,
    path: &path::PathBuf,
    index: Option<usize>,
) -> String {
    format!("<a {}>{text}</a>", format_reference_parameters(path, index))
}

/// Formats an anchor/permalink tag for a node (index = None)
/// or diagnostic (index = Some(index of NodeData entry)).
fn format_anchor(path: &path::PathBuf, index: Option<usize>) -> String {
    format!(
        "<a {} class=\"anchor\"></a>",
        format_reference_parameters(path, index)
    )
}

/// Formats the id parameter for a div/details tag for a node (index = None)
/// or diagnostic (index = Some(index of NodeData entry)).
fn format_id(path: &path::PathBuf, index: Option<usize>) -> String {
    format!(
        "id=\"{}\"",
        html_escape(url_encode(path_encode(format_path(path, index))))
    )
}

/// Creates a span with the given class name. The text is HTML-escaped.
fn format_span<S: std::fmt::Display>(class: &'static str, text: S) -> String {
    format!(
        "<span class=\"{class}\">{}</span>",
        html_escape(text.to_string())
    )
}

/// Formats a diagnostic message box. path should be the node that the
/// diagnostic is defined in, and index should be its index within Node::data.
/// with_id specifies whether the HTML id parameter should be included.
fn format_diagnostic(
    diag: &diagnostic::Diagnostic,
    path: &path::PathBuf,
    index: usize,
    with_id: bool,
    with_path: bool,
) -> String {
    let cause = format_span(
        "cause",
        if with_path {
            diag.to_string()
        } else {
            format!("{:#}", diag)
        },
    );
    let cause = if &diag.path == path {
        cause
    } else {
        format_reference(cause, &diag.path, None)
    };
    let id = if with_id {
        let mut id = format_id(path, Some(index));
        id.push(' ');
        id
    } else {
        String::new()
    };
    let anchor = format_anchor(path, Some(index));

    let class = match diag.adjusted_level {
        diagnostic::Level::Info => "diag_info",
        diagnostic::Level::Warning => "diag_warn",
        diagnostic::Level::Error => "diag_error",
    };

    format!("<div {id}class=\"card {class}\">\n{cause}\n{anchor}\n</div>")
}

/// Format a flattened list of diagnostic cards.
fn format_diagnostics(path: &path::Path, node: &tree::Node) -> (Vec<String>, diagnostic::Level) {
    let mut html = vec![];
    let mut level = diagnostic::Level::Info;
    for (index, data) in node.data.iter().enumerate() {
        match data {
            tree::NodeData::Child(child) => {
                let (sub_html, sub_level) =
                    format_diagnostics(&path.with(child.path_element.clone()), &child.node);
                html.extend(sub_html);
                level = std::cmp::max(level, sub_level);
            }
            tree::NodeData::Diagnostic(diag) => {
                html.push(format_diagnostic(
                    diag,
                    &path.to_path_buf(),
                    index,
                    false,
                    true,
                ));
                level = std::cmp::max(level, diag.adjusted_level);
            }
            _ => {}
        }
    }
    (html, level)
}

/// Formats a comment span.
fn format_comment_span(span: &comment::Span) -> String {
    match &span.link {
        None => html_escape(&span.text),
        Some(comment::Link::Path(path)) => format_reference(html_escape(&span.text), path, None),
        Some(comment::Link::Url(url)) => format!(
            "<a href=\"{}\">{}</a>",
            html_escape(url),
            html_escape(&span.text)
        ),
    }
}

/// Formats a comment using HTML markup.
fn format_comment(comment: &comment::Comment) -> String {
    let mut result = String::new();
    let mut p_open = false;
    for element in comment.elements().iter() {
        match element {
            comment::Element::Span(span) => {
                if !p_open {
                    result += "<p>";
                    p_open = true;
                }
                result += &format_comment_span(span);
            }
            comment::Element::NewLine => {
                if p_open {
                    result += "</p>";
                    p_open = false;
                }
            }
            comment::Element::ListOpen => {
                if p_open {
                    result += "</p>";
                    p_open = false;
                }
                result += "<ul><li>";
            }
            comment::Element::ListNext => {
                if p_open {
                    result += "</p>";
                    p_open = false;
                }
                result += "</li><li>";
            }
            comment::Element::ListClose => {
                if p_open {
                    result += "</p>";
                    p_open = false;
                }
                result += "</li></ul>";
            }
        }
    }
    if p_open {
        result += "</p>";
    }
    result
}

/// Formats a brief comment using HTML markup.
fn format_brief(brief: &comment::Brief) -> String {
    let mut result = String::new();
    for span in brief.spans().iter() {
        result += &format_comment_span(span);
    }
    result
}

// Format the relation trees.
fn format_relation_tree(
    path: &path::Path,
    node: &tree::Node,
    index: &mut usize,
    is_root: bool,
    in_expression: bool,
) -> Vec<String> {
    let mut html = vec![];

    let text = node
        .brief
        .as_ref()
        .map(format_brief)
        .unwrap_or_else(|| String::from("unknown"));
    let is_relation = matches!(node.class, tree::Class::Relation);
    let is_expression = matches!(node.class, tree::Class::Expression);

    if is_relation {
        if is_root {
            html.push("<details class=\"relation_tree\">".to_string());
            html.push(format!(
                "<summary>Query/relation graph #{}</summary>",
                *index
            ));
            html.push("<ul class=\"tree\"><li><span class=\"root\">Sink</span><ul>".to_string());
        };
        html.push(format!(
            "<li><span class=\"{}\">{text} ({})</span>",
            if in_expression {
                "subquery"
            } else {
                "data_source"
            },
            format_reference("link", &path.to_path_buf(), None)
        ));
    }

    let mut has_children = false;
    for data in node.data.iter() {
        if let tree::NodeData::Child(child) = data {
            let sub_html = format_relation_tree(
                &path.with(child.path_element.clone()),
                &child.node,
                index,
                is_root && !is_relation,
                (in_expression && !is_relation) || is_expression,
            );
            if !sub_html.is_empty() {
                if is_relation && !has_children {
                    html.push("<ul>".to_string());
                }
                has_children = true;
                html.extend(sub_html);
            }
        }
    }

    if is_relation {
        if has_children {
            html.push("</ul>".to_string());
        }
        html.push("</li>".to_string());
        if is_root {
            html.push("</ul></li></ul>".to_string());
            html.push("</details>".to_string());
            *index += 1;
        }
    }

    html
}

// Format the node tree.
fn format_node_tree(
    path: &path::Path,
    unknown_subtree: bool,
    node: &tree::Node,
) -> (Vec<String>, Level) {
    // Get the HTML ID for this card.
    let pathbuf = path.to_path_buf();
    let id = format_id(&pathbuf, None);

    // Format the card header.
    let brief = if let Some(brief) = &node.brief {
        format_span("brief", format_brief(brief))
    } else {
        String::from("")
    };
    let value = match &node.node_type {
        tree::NodeType::ProtoMessage(proto_type) => {
            format!("{brief} {}", format_span("type", proto_type))
        }
        tree::NodeType::ProtoPrimitive(proto_type, data) => {
            format!(
                "= {}{brief} {}",
                format_span("value", data),
                format_span("type", proto_type)
            )
        }
        tree::NodeType::ProtoMissingOneOf => "?".to_string(),
        tree::NodeType::NodeReference(num, target) => format_reference(
            format!(
                "= {}{brief} {}",
                format_span("value", num),
                format_span("type", "uint32, reference")
            ),
            &target.path,
            None,
        ),
        tree::NodeType::YamlReference(yaml) => {
            format!(
                "= {}{brief} {}",
                format_span("value", &yaml.uri),
                format_span("type", "string, resolved to YAML")
            )
        }
        tree::NodeType::YamlMap => format!("{brief} {}", format_span("type", "YAML map")),
        tree::NodeType::YamlArray => format!("{brief} {}", format_span("type", "YAML array")),
        tree::NodeType::YamlPrimitive(data) => format!("= {}{brief}", format_span("value", data)),
    };
    let header = format!(
        "{} {value} {}",
        format_span("field", path.end_to_string()),
        format_anchor(&pathbuf, None)
    );

    // If the node doesn't have any additional data associated with it, output
    // a normal <div> rather than a <details> card.
    if node.data.is_empty() && node.summary.is_none() {
        let class = if unknown_subtree { "unknown" } else { "ok" };
        return (
            vec![format!("<div {id} class=\"card {class}\">{header}</div>")],
            Level::Ok,
        );
    }

    // Gather child nodes here. The first entry of the html Vec is reserved for
    // the open tags, which we don't have all the information for just yet.
    let mut html = vec![String::new()];
    let mut level = Level::Ok;

    // Add the summary.
    if let Some(ref summary) = node.summary {
        html.push(format_comment(summary));
    }

    // Iterate over node data here, recursively entering children.
    for (index, data) in node.data.iter().enumerate() {
        match data {
            tree::NodeData::Child(child) => {
                let (sub_html, sub_level) = format_node_tree(
                    &path.with(child.path_element.clone()),
                    !child.recognized,
                    &child.node,
                );
                html.extend(sub_html);
                level = std::cmp::max(level, sub_level);
            }
            tree::NodeData::Diagnostic(diag) => {
                html.push(format_diagnostic(
                    diag,
                    &pathbuf,
                    index,
                    true,
                    diag.path != pathbuf,
                ));
                level = std::cmp::max(level, diag.adjusted_level.into());
            }
            tree::NodeData::DataType(data_type) => {
                // TODO: print an actual tree structure for nested types
                html.push("<div class=\"card data_type\">\n".to_string());
                let prefix = if matches!(node.class, tree::Class::Relation) {
                    "Schema"
                } else {
                    "Data type"
                };
                html.push(format!(
                    "{}: {}",
                    prefix,
                    html_escape(data_type.to_string())
                ));
                html.push("\n</div>".to_string());
            }
            tree::NodeData::Comment(comment) => {
                html.push("<div class=\"card comment\">\n".to_string());
                html.push(format_comment(comment));
                html.push("\n</div>".to_string());
            }
        }
    }

    // Add the surrounding <details> tags now that we have the error level
    // information we needed.
    let class = if unknown_subtree {
        "unknown"
    } else {
        level.class()
    };
    html[0] = format!("<details {id} class=\"{class}\">\n<summary>\n{header}\n</summary>");
    html.push("</details>".to_string());

    // Determine the minimum error level for the parent.
    let level = match level {
        Level::Error => Level::ChildError,
        Level::Warning => Level::ChildWarning,
        x => x,
    };

    (html, level)
}

/// Export the tree in HTML format, with as many details as possible, and as
/// human-readable as possible. Purely intended for debugging.
pub fn export<T: std::io::Write>(
    out: &mut T,
    root_name: &'static str,
    result: &parse_result::ParseResult,
) -> std::io::Result<()> {
    let path = path::Path::Root(root_name);
    write!(out, "{HEADER}")?;

    // Emit the node graph.
    writeln!(out, "<details class=\"relation_tree\" open=\"true\">")?;
    writeln!(out, "<summary>Relation graphs</summary>")?;
    writeln!(
        out,
        "<div class=\"note\">Note: data flows upwards in these graphs.</div>"
    )?;
    let mut index = 0;
    for s in format_relation_tree(&path, &result.root, &mut index, true, false) {
        writeln!(out, "{s}")?;
    }
    writeln!(out, "</details>")?;

    // Emit diagnostics summary.
    let (diag_html, level) = format_diagnostics(&path, &result.root);
    let validity_class = match level {
        diagnostic::Level::Info => "valid",
        diagnostic::Level::Warning => "maybe_valid",
        diagnostic::Level::Error => "invalid",
    };
    let validity_summary = match level {
        diagnostic::Level::Info => "This plan is <span class=\"valid\">VALID</span>",
        diagnostic::Level::Warning => "The validator was unable to determine validity",
        diagnostic::Level::Error => "This plan is <span class=\"invalid\">INVALID</span>",
    };
    writeln!(
        out,
        "<details class=\"{}\" open=\"true\">",
        Level::from(level).class()
    )?;
    writeln!(
        out,
        "<summary class=\"{validity_class}\">{validity_summary}</summary>"
    )?;
    if diag_html.is_empty() {
        writeln!(
            out,
            "<div class=\"note\">No diagnostics were reported.</div>"
        )?;
    } else {
        for s in diag_html {
            writeln!(out, "{s}")?;
        }
    }
    writeln!(out, "</details>")?;

    // Emit protobuf-level raw node tree.
    for s in format_node_tree(&path, false, &result.root).0 {
        writeln!(out, "{s}")?;
    }

    write!(out, "{FOOTER}")
}
