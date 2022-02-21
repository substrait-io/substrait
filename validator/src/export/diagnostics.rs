use crate::doc_tree;

pub fn export<T: std::io::Write>(
    out: &mut T,
    _root_name: &'static str,
    root: &doc_tree::Node,
) -> std::io::Result<()> {
    for diag in root.iter_diagnostics() {
        writeln!(out, "{}", diag)?;
    }
    Ok(())
}
