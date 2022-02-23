pub mod data_type;

#[macro_use]
pub mod diagnostic;

#[macro_use]
pub mod tree;
pub mod comment;
pub mod context;
pub mod export;
pub mod extension;
pub mod path;
pub mod primitives;
pub mod proto;
mod validate;
pub mod yaml;

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
pub fn parse<B: prost::bytes::Buf>(buffer: B) -> tree::Node {
    tree::Node::parse_proto::<proto::substrait::Plan, _, _>(
        buffer,
        "plan",
        validate::parse_plan,
        &mut context::State::default(),
        &context::Config::default(),
    )
}

/// Returns whether the plan represented by the given parse tree is valid.
pub fn check(root: &tree::Node) -> Validity {
    root.iter_diagnostics()
        .map(|x| x.level)
        .fold(diagnostic::Level::Info, std::cmp::max)
        .into()
}

/// Returns the first diagnostic of the highest severity level in the tree.
pub fn get_diagnostic(root: &tree::Node) -> Option<&diagnostic::Diagnostic> {
    let mut result: Option<&diagnostic::Diagnostic> = None;
    for diag in root.iter_diagnostics() {
        // We can return immediately for error diagnostics, since this is the
        // highest level.
        if diag.level == diagnostic::Level::Error {
            return Some(diag);
        }

        // For other levels, update only if the incoming diagnostic is of a
        // higher level/severity than the current one.
        if let Some(cur) = result.as_mut() {
            if diag.level > (*cur).level {
                *cur = diag;
            }
        } else {
            result = Some(diag);
        }
    }
    result
}

/// Exports a parse tree to a file or other output device using the specified
/// data format.
pub fn export<T: std::io::Write>(
    out: &mut T,
    format: export::Format,
    root: &tree::Node,
) -> std::io::Result<()> {
    export::export(out, format, "plan", root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
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

        //let mut out = std::fs::File::create("test1.html").unwrap();
        //export(&mut out, export::Format::Html, &root).unwrap();
    }

    #[test]
    fn test3() {
        // TPC-H 03 as returned by
        // https://github.com/jvanstraten/duckdb-substrait-demo/tree/28b30b58a6caa22cc5e074ae5d3c251def836ac7
        // This needs to not be bytes. Testing strategy is TBD.
        let buffer = prost::bytes::Bytes::from(vec![
            18, 15, 26, 13, 26, 11, 103, 114, 101, 97, 116, 101, 114, 116, 104, 97, 110, 18, 17,
            26, 15, 16, 1, 26, 11, 105, 115, 95, 110, 111, 116, 95, 110, 117, 108, 108, 18, 9, 26,
            7, 16, 2, 26, 3, 97, 110, 100, 18, 14, 26, 12, 16, 3, 26, 8, 108, 101, 115, 115, 116,
            104, 97, 110, 18, 11, 26, 9, 16, 4, 26, 5, 101, 113, 117, 97, 108, 18, 9, 26, 7, 16, 5,
            26, 3, 115, 117, 109, 18, 7, 26, 5, 16, 6, 26, 1, 42, 18, 7, 26, 5, 16, 7, 26, 1, 45,
            26, 160, 5, 10, 157, 5, 26, 154, 5, 18, 149, 5, 42, 146, 5, 18, 243, 4, 58, 240, 4, 18,
            199, 4, 34, 196, 4, 18, 237, 3, 58, 234, 3, 18, 143, 3, 50, 140, 3, 18, 155, 2, 58,
            152, 2, 18, 209, 1, 50, 206, 1, 18, 88, 10, 86, 10, 2, 10, 0, 26, 50, 26, 48, 8, 2, 18,
            28, 26, 26, 18, 8, 18, 6, 10, 4, 18, 2, 8, 10, 18, 14, 10, 12, 98, 10, 49, 57, 57, 53,
            45, 48, 51, 45, 49, 53, 18, 14, 26, 12, 8, 1, 18, 8, 18, 6, 10, 4, 18, 2, 8, 10, 34,
            16, 10, 14, 10, 0, 10, 2, 8, 10, 10, 2, 8, 5, 10, 2, 8, 6, 58, 10, 10, 8, 108, 105,
            110, 101, 105, 116, 101, 109, 26, 88, 10, 86, 10, 2, 10, 0, 26, 52, 26, 50, 8, 2, 18,
            30, 26, 28, 8, 3, 18, 8, 18, 6, 10, 4, 18, 2, 8, 4, 18, 14, 10, 12, 98, 10, 49, 57, 57,
            53, 45, 48, 51, 45, 49, 53, 18, 14, 26, 12, 8, 1, 18, 8, 18, 6, 10, 4, 18, 2, 8, 4, 34,
            16, 10, 14, 10, 2, 8, 1, 10, 0, 10, 2, 8, 4, 10, 2, 8, 7, 58, 8, 10, 6, 111, 114, 100,
            101, 114, 115, 34, 22, 26, 20, 8, 4, 18, 6, 18, 4, 10, 2, 18, 0, 18, 8, 18, 6, 10, 4,
            18, 2, 8, 5, 48, 1, 26, 6, 18, 4, 10, 2, 18, 0, 26, 8, 18, 6, 10, 4, 18, 2, 8, 1, 26,
            8, 18, 6, 10, 4, 18, 2, 8, 2, 26, 8, 18, 6, 10, 4, 18, 2, 8, 3, 26, 8, 18, 6, 10, 4,
            18, 2, 8, 4, 26, 8, 18, 6, 10, 4, 18, 2, 8, 6, 26, 8, 18, 6, 10, 4, 18, 2, 8, 7, 26,
            80, 10, 78, 10, 2, 10, 0, 26, 50, 26, 48, 8, 2, 18, 28, 26, 26, 8, 4, 18, 8, 18, 6, 10,
            4, 18, 2, 8, 6, 18, 12, 10, 10, 98, 8, 66, 85, 73, 76, 68, 73, 78, 71, 18, 14, 26, 12,
            8, 1, 18, 8, 18, 6, 10, 4, 18, 2, 8, 6, 34, 8, 10, 6, 10, 2, 8, 6, 10, 0, 58, 10, 10,
            8, 99, 117, 115, 116, 111, 109, 101, 114, 34, 24, 26, 22, 8, 4, 18, 8, 18, 6, 10, 4,
            18, 2, 8, 4, 18, 8, 18, 6, 10, 4, 18, 2, 8, 8, 48, 1, 26, 6, 18, 4, 10, 2, 18, 0, 26,
            8, 18, 6, 10, 4, 18, 2, 8, 1, 26, 8, 18, 6, 10, 4, 18, 2, 8, 2, 26, 8, 18, 6, 10, 4,
            18, 2, 8, 3, 26, 8, 18, 6, 10, 4, 18, 2, 8, 4, 26, 8, 18, 6, 10, 4, 18, 2, 8, 5, 26, 8,
            18, 6, 10, 4, 18, 2, 8, 6, 26, 8, 18, 6, 10, 4, 18, 2, 8, 7, 26, 8, 18, 6, 10, 4, 18,
            2, 8, 8, 26, 28, 10, 6, 18, 4, 10, 2, 18, 0, 10, 8, 18, 6, 10, 4, 18, 2, 8, 5, 10, 8,
            18, 6, 10, 4, 18, 2, 8, 6, 34, 52, 10, 50, 8, 5, 18, 46, 26, 44, 8, 6, 18, 8, 18, 6,
            10, 4, 18, 2, 8, 2, 18, 30, 26, 28, 8, 7, 18, 14, 10, 12, 194, 1, 9, 10, 3, 49, 48, 48,
            16, 16, 24, 2, 18, 8, 18, 6, 10, 4, 18, 2, 8, 3, 26, 6, 18, 4, 10, 2, 18, 0, 26, 8, 18,
            6, 10, 4, 18, 2, 8, 3, 26, 8, 18, 6, 10, 4, 18, 2, 8, 1, 26, 8, 18, 6, 10, 4, 18, 2, 8,
            2, 26, 12, 10, 8, 18, 6, 10, 4, 18, 2, 8, 1, 16, 3, 26, 12, 10, 8, 18, 6, 10, 4, 18, 2,
            8, 2, 16, 1, 32, 10,
        ]);
        let root = parse(buffer);
        assert_eq!(check(&root), Validity::Invalid);
        export(&mut std::io::stdout(), export::Format::Diagnostics, &root).unwrap();

        //let mut out = std::fs::File::create("test3.html").unwrap();
        //export(&mut out, export::Format::Html, &root).unwrap();
    }
}