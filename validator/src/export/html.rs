use crate::comment;
use crate::diagnostic;
use crate::doc_tree;
use crate::path;
use crate::proto;
use std::cmp::max;

const HEADER: &str = r#"
<!DOCTYPE html>
<html>
<head>
<link href="//netdna.bootstrapcdn.com/font-awesome/3.2.1/css/font-awesome.css" rel="stylesheet">
<style>
body {
    font-family: sans-serif;
}

details, div.card {
    border: 1px solid;
    border-color: rgba(0, 0, 0, .3);
    color: rgba(0, 0, 0, .8);
    border-radius: 4px;
    margin-top: .2em;
}

details:hover, div.card:hover {
    border-color: #000;
    color: rgba(0, 0, 0, .9);
}

details:target, div.card:target {
    box-shadow: 0 0 .3em .2em rgba(0, 0, 0, 0.3);
    border-color: #000;
    color: rgba(0, 0, 0, .9);
}

details {
    padding: .2em .5em 0;
}

summary {
    font-weight: bold;
    margin: -.2em -.5em 0;
    padding: .2em .5em;
}

details[open] {
    padding: .2em .5em;
}

details[open] > summary {
    border-bottom: 1px solid;
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
}

div.comment {
    background-color: #9fc;
}

div.comment::before {
    font-family: "Font Awesome 5 Free";
    color: #084;
    content: "\f249";
}

div.diag_info {
    background-color: #9f9;
}

div.diag_info::before {
    font-family: "Font Awesome 5 Free";
    color: #080;
    content: "\f058";
}

div.diag_warn {
    background-color: #fc9;
}

div.diag_warn::before {
    font-family: "Font Awesome 5 Free";
    color: #840;
    content: "\f059";
}

div.diag_error {
    background-color: #f99;
}

div.diag_error::before {
    font-family: "Font Awesome 5 Free";
    color: #800;
    content: "\f00d";
}

a.anchor {
    opacity: 0.4;
    text-decoration: none;
}

a.anchor:hover {
    opacity: 1.0;
}

a.anchor::before {
    font-family: "Font Awesome 5 Free";
    color: #000;
    content: "\f0c1";
}

details:target, div.card:target {
  animation: highlight 1000ms ease-out;
}

@keyframes highlight {
  0% { box-shadow: 0 0 2em 1em rgba(0, 0, 0, 0.3); }
  50% { box-shadow: 0 0 2em 1em rgba(0, 0, 0, 0.3); }
  100% { }
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

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Level {
    Ok,
    ChildWarning,
    Warning,
    ChildError,
    Error,
}

fn str_escape<S: AsRef<str>>(text: S) -> String {
    // TODO
    text.as_ref().to_string()
}

fn href_escape<S: AsRef<str>>(text: S) -> String {
    // TODO
    text.as_ref().to_string()
}

fn html_escape<S: AsRef<str>>(text: S) -> String {
    // TODO
    text.as_ref().to_string()
}

fn format_reference(path: &path::PathBuf) -> String {
    let path = path.to_string();
    format!(
        "href=\"#{}\" title=\"{}\"",
        href_escape(&path),
        str_escape(&path)
    )
}

fn format_primitive_data(data: &proto::meta::ProtoPrimitiveData) -> String {
    // TODO
    html_escape(format!("{:?}", data))
}

fn format_node(
    path: &path::Path,
    unknown_subtree: bool,
    node: &doc_tree::Node,
) -> (Vec<String>, Level) {
    let value = match &node.node_type {
        doc_tree::NodeType::ProtoMessage(proto_type) => html_escape(proto_type),
        doc_tree::NodeType::ProtoPrimitive(proto_type, data) => {
            format!(
                "{} ({})",
                format_primitive_data(data),
                html_escape(proto_type)
            )
        }
        doc_tree::NodeType::ProtoMissingOneOf => "?".to_string(),
        doc_tree::NodeType::Reference(num, target) => {
            format!(
                "<a {}>{} (uint32, reference)</a>",
                format_reference(&target.path),
                num
            )
        }
        doc_tree::NodeType::YamlData(yaml) => {
            format!("{} (string, resolved to YAML)", html_escape(&yaml.uri))
        }
        doc_tree::NodeType::YamlMap => "YAML map".to_string(),
        doc_tree::NodeType::YamlArray => "YAML array".to_string(),
        doc_tree::NodeType::YamlPrimitive(data) => format_primitive_data(data),
    };

    let header = format!(
        "{}: {} <a {} class=\"anchor\"></a>",
        html_escape(path.end_to_string()),
        value,
        format_reference(&path.to_path_buf())
    );
    let id = str_escape(path.to_string());

    if node.data.is_empty() {
        let class = if unknown_subtree { "unknown" } else { "ok" };

        return (
            vec![format!(
                "<div id=\"{}\" class=\"card {}\">{}</div>",
                id, class, header
            )],
            Level::Ok,
        );
    }

    let mut html = vec![String::new()];
    let mut level = Level::Ok;

    for data in node.data.iter() {
        match data {
            doc_tree::NodeData::Child(child) => {
                let (sub_html, sub_level) = format_node(
                    &path.with(child.path_element.clone()),
                    !child.recognized,
                    &child.node,
                );
                html.extend(sub_html.into_iter());
                level = max(level, sub_level);
            }
            doc_tree::NodeData::Diagnostic(diag) => {
                let cause = html_escape(diag.cause.to_string());
                match diag.level {
                    diagnostic::Level::Error => {
                        level = max(level, Level::Error);
                        html.push(format!(
                            "<div class=\"card diag_error\">\nError: {}\n</div>",
                            cause
                        ));
                    }
                    diagnostic::Level::Warning => {
                        level = max(level, Level::Warning);
                        html.push(format!(
                            "<div class=\"card diag_warn\">\nWarning: {}\n</div>",
                            cause
                        ));
                    }
                    diagnostic::Level::Info => {
                        html.push(format!(
                            "<div class=\"card diag_info\">\nInfo: {}\n</div>",
                            cause
                        ));
                    }
                }
            }
            doc_tree::NodeData::DataType(_data_type) => {
                // todo
            }
            doc_tree::NodeData::Comment(comment) => {
                html.push("<div class=\"card comment\">\n".to_string());
                for span in comment.spans.iter() {
                    html.push(match &span.link {
                        None => html_escape(&span.text),
                        Some(comment::Link::Path(path)) => format!(
                            "<a {}>{}</a>",
                            format_reference(path),
                            html_escape(&span.text)
                        ),
                        Some(comment::Link::Url(url)) => format!(
                            "<a href=\"{}\">{}</a>",
                            str_escape(url),
                            html_escape(&span.text)
                        ),
                    })
                }
                html.push("\n</div>".to_string());
            }
        }
    }

    let class = if unknown_subtree {
        "unknown"
    } else {
        match level {
            Level::Ok => "ok",
            Level::ChildWarning => "warn_child",
            Level::Warning => "warn_here",
            Level::ChildError => "error_child",
            Level::Error => "error_here",
        }
    };

    html[0] = format!(
        "<details id=\"{}\" class=\"{}\">\n<summary>\n{}\n</summary>",
        id, class, header
    );
    html.push("</details>".to_string());

    let level = match level {
        Level::Error => Level::ChildError,
        Level::Warning => Level::ChildWarning,
        x => x,
    };

    (html, level)
}

pub fn export<T: std::io::Write>(
    out: &mut T,
    root_name: &'static str,
    root: &doc_tree::Node,
) -> std::io::Result<()> {
    write!(out, "{}", HEADER)?;
    for s in format_node(&path::Path::Root(root_name), false, root).0 {
        writeln!(out, "{}", s)?;
    }
    write!(out, "{}", FOOTER)
}
