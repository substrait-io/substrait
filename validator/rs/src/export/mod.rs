// SPDX-License-Identifier: Apache-2.0

//! Module dealing with serializing a [ParseResult](parse_result::ParseResult)
//! to a byte stream in various formats.

mod diagnostics;
mod html;
mod proto;

use crate::output::parse_result;

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
    result: &parse_result::ParseResult,
) -> std::io::Result<()> {
    match format {
        Format::Diagnostics => diagnostics::export(out, root_name, result),
        Format::Html => html::export(out, root_name, result),
        Format::Proto => proto::export(out, root_name, result),
    }
}
