pub mod diagnostics;
pub mod html;
pub mod proto;

use crate::doc_tree;

/// Supported output formats for exporting.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    /// Emit a newline-separated, flattened list of diagnostics.
    Diagnostics,

    /// Emit a HTML page with detailed information about the parsed plan.
    Html,

    /// Emit all parse information as a substrait.validator.Node protobuf
    /// message, using binary serialization.
    Proto,
}

/// Exports the given doctree with the given format to the given output.
pub fn export<T: std::io::Write>(
    out: &mut T,
    format: Format,
    root_name: &'static str,
    root: &doc_tree::Node,
) -> std::io::Result<()> {
    match format {
        Format::Diagnostics => diagnostics::export(out, root_name, root),
        Format::Html => html::export(out, root_name, root),
        Format::Proto => proto::export(out, root_name, root),
    }
}
