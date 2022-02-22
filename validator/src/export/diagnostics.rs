use crate::tree;

/// Export the diagnostic messages of the tree as a multiline string.
pub fn export<T: std::io::Write>(
    out: &mut T,
    _root_name: &'static str,
    root: &tree::Node,
) -> std::io::Result<()> {
    for diag in root.iter_diagnostics() {
        writeln!(out, "{}", diag)?;
    }
    Ok(())
}
