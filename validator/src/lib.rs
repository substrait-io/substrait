pub mod data_type;
pub mod diagnostic;

#[macro_use]
pub mod doc_tree;
pub mod comment;
pub mod context;
pub mod export;
pub mod extension;
pub mod path;
pub mod proto;
mod validate;

/// Validity of a plan.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Validity {
    /// The plan is valid.
    Valid,

    /// The plan may or may not be valid; the validator was not able to prove
    /// or disprove validity.
    MaybeValid,

    /// The plan is invalid.
    Invalid,
}

impl From<diagnostic::Level> for Validity {
    fn from(level: diagnostic::Level) -> Self {
        match level {
            diagnostic::Level::Info => Validity::Valid,
            diagnostic::Level::Warning => Validity::MaybeValid,
            diagnostic::Level::Error => Validity::Invalid,
        }
    }
}

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf>(buffer: B) -> doc_tree::Node {
    doc_tree::Node::parse_proto::<proto::substrait::Plan, _, _>(
        buffer,
        "plan",
        validate::parse_plan,
        &mut context::State::default(),
        &context::Config::default(),
    )
}

/// Returns whether the plan represented by the given parse tree is valid.
pub fn check(root: &doc_tree::Node) -> Validity {
    root.iter_diagnostics()
        .map(|x| x.level)
        .fold(diagnostic::Level::Info, std::cmp::max)
        .into()
}

/// Exports a parse tree to a file or other output device using the specified
/// data format.
pub fn export<T: std::io::Write>(
    out: &mut T,
    format: export::Format,
    root: &doc_tree::Node,
) -> std::io::Result<()> {
    export::export(out, format, "plan", root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // TPC-H 01 as returned by
        // https://github.com/jvanstraten/duckdb-substrait-demo/tree/28b30b58a6caa22cc5e074ae5d3c251def836ac7
        // This needs to not be bytes. Testing strategy is TBD.
        let buffer = prost::bytes::Bytes::from(vec![
            18, 17, 26, 15, 26, 13, 108, 101, 115, 115, 116, 104, 97, 110, 101, 113, 117, 97, 108,
            18, 17, 26, 15, 16, 1, 26, 11, 105, 115, 95, 110, 111, 116, 95, 110, 117, 108, 108, 18,
            9, 26, 7, 16, 2, 26, 3, 97, 110, 100, 18, 7, 26, 5, 16, 3, 26, 1, 42, 18, 7, 26, 5, 16,
            4, 26, 1, 45, 18, 9, 26, 7, 16, 5, 26, 3, 115, 117, 109, 18, 7, 26, 5, 16, 6, 26, 1,
            43, 18, 9, 26, 7, 16, 7, 26, 3, 97, 118, 103, 18, 16, 26, 14, 16, 8, 26, 10, 99, 111,
            117, 110, 116, 95, 115, 116, 97, 114, 26, 152, 4, 10, 149, 4, 42, 146, 4, 18, 245, 3,
            58, 242, 3, 18, 141, 3, 34, 138, 3, 18, 215, 1, 58, 212, 1, 18, 102, 10, 100, 10, 2,
            10, 0, 26, 50, 26, 48, 8, 2, 18, 28, 26, 26, 18, 8, 18, 6, 10, 4, 18, 2, 8, 10, 18, 14,
            10, 12, 98, 10, 49, 57, 57, 56, 45, 48, 57, 45, 48, 50, 18, 14, 26, 12, 8, 1, 18, 8,
            18, 6, 10, 4, 18, 2, 8, 10, 34, 30, 10, 28, 10, 2, 8, 10, 10, 2, 8, 8, 10, 2, 8, 9, 10,
            2, 8, 4, 10, 2, 8, 5, 10, 2, 8, 6, 10, 2, 8, 7, 58, 10, 10, 8, 108, 105, 110, 101, 105,
            116, 101, 109, 26, 8, 18, 6, 10, 4, 18, 2, 8, 1, 26, 8, 18, 6, 10, 4, 18, 2, 8, 2, 26,
            8, 18, 6, 10, 4, 18, 2, 8, 3, 26, 8, 18, 6, 10, 4, 18, 2, 8, 4, 26, 46, 26, 44, 8, 3,
            18, 8, 18, 6, 10, 4, 18, 2, 8, 4, 18, 30, 26, 28, 8, 4, 18, 14, 10, 12, 194, 1, 9, 10,
            3, 49, 48, 48, 16, 16, 24, 2, 18, 8, 18, 6, 10, 4, 18, 2, 8, 5, 26, 8, 18, 6, 10, 4,
            18, 2, 8, 6, 26, 8, 18, 6, 10, 4, 18, 2, 8, 5, 26, 18, 10, 6, 18, 4, 10, 2, 18, 0, 10,
            8, 18, 6, 10, 4, 18, 2, 8, 1, 34, 14, 10, 12, 8, 5, 18, 8, 18, 6, 10, 4, 18, 2, 8, 2,
            34, 14, 10, 12, 8, 5, 18, 8, 18, 6, 10, 4, 18, 2, 8, 3, 34, 14, 10, 12, 8, 5, 18, 8,
            18, 6, 10, 4, 18, 2, 8, 4, 34, 52, 10, 50, 8, 5, 18, 46, 26, 44, 8, 3, 18, 8, 18, 6,
            10, 4, 18, 2, 8, 4, 18, 30, 26, 28, 8, 6, 18, 14, 10, 12, 194, 1, 9, 10, 3, 49, 48, 48,
            16, 16, 24, 2, 18, 8, 18, 6, 10, 4, 18, 2, 8, 5, 34, 14, 10, 12, 8, 7, 18, 8, 18, 6,
            10, 4, 18, 2, 8, 2, 34, 14, 10, 12, 8, 7, 18, 8, 18, 6, 10, 4, 18, 2, 8, 3, 34, 14, 10,
            12, 8, 7, 18, 8, 18, 6, 10, 4, 18, 2, 8, 6, 34, 4, 10, 2, 8, 8, 26, 6, 18, 4, 10, 2,
            18, 0, 26, 8, 18, 6, 10, 4, 18, 2, 8, 1, 26, 8, 18, 6, 10, 4, 18, 2, 8, 2, 26, 8, 18,
            6, 10, 4, 18, 2, 8, 3, 26, 8, 18, 6, 10, 4, 18, 2, 8, 4, 26, 8, 18, 6, 10, 4, 18, 2, 8,
            5, 26, 8, 18, 6, 10, 4, 18, 2, 8, 6, 26, 8, 18, 6, 10, 4, 18, 2, 8, 7, 26, 8, 18, 6,
            10, 4, 18, 2, 8, 8, 26, 8, 18, 6, 10, 4, 18, 2, 8, 9, 26, 10, 10, 6, 18, 4, 10, 2, 18,
            0, 16, 1, 26, 12, 10, 8, 18, 6, 10, 4, 18, 2, 8, 1, 16, 1,
        ]);
        let root = parse(buffer);
        assert_eq!(check(&root), Validity::Invalid);
        export(&mut std::io::stdout(), export::Format::Diagnostics, &root).unwrap();

        //let mut out = std::fs::File::create("test.html").unwrap();
        //export::html::export(&mut out, "plan", &data).unwrap();

        //assert_eq!(diags, vec!["Warning (plan): found values for field(s) not yet understood by the validator: extensions, relations".to_string()])
    }
}
