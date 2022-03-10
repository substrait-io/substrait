// SPDX-License-Identifier: Apache-2.0

//! This module provides a basic export format that's just a listing of the
//! diagnostic messages contained in the tree.

use crate::output::parse_result;

/// Export the diagnostic messages of the tree as a multiline string.
pub fn export<T: std::io::Write>(
    out: &mut T,
    _root_name: &'static str,
    result: &parse_result::ParseResult,
) -> std::io::Result<()> {
    for diag in result.root.iter_diagnostics() {
        writeln!(out, "{diag}")?;
    }
    Ok(())
}
