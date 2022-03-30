// SPDX-License-Identifier: Apache-2.0

//! Test runner for the [substrait_validator] crate.

use std::collections::HashSet;
use substrait_validator as sv;

#[derive(serde::Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum ErrorLevel {
    #[serde(rename(deserialize = "e"))]
    Error,
    #[serde(rename(deserialize = "w"))]
    Warning,
    #[serde(rename(deserialize = "i"))]
    Info,
}

impl From<ErrorLevel> for sv::Level {
    fn from(l: ErrorLevel) -> Self {
        match l {
            ErrorLevel::Error => sv::Level::Error,
            ErrorLevel::Warning => sv::Level::Warning,
            ErrorLevel::Info => sv::Level::Info,
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
enum PathElement {
    Field { field: String },
    Oneof { field: String, variant: String },
    Repeated { field: String, index: usize },
    Index { index: usize },
}

impl From<PathElement> for sv::output::path::PathElement {
    fn from(e: PathElement) -> Self {
        match e {
            PathElement::Field { field } => sv::output::path::PathElement::Field(field),
            PathElement::Oneof { field, variant } => {
                sv::output::path::PathElement::Variant(field, variant)
            }
            PathElement::Repeated { field, index } => {
                sv::output::path::PathElement::Repeated(field, index)
            }
            PathElement::Index { index } => sv::output::path::PathElement::Index(index),
        }
    }
}

fn convert_path(path: &[PathElement]) -> sv::output::path::PathBuf {
    sv::output::path::PathBuf {
        root: "plan",
        elements: path.iter().map(|x| x.clone().into()).collect(),
    }
}

#[derive(serde::Deserialize, Debug)]
struct LevelTest {
    pub path: Vec<PathElement>,
    pub allowed_levels: HashSet<ErrorLevel>,
}

#[derive(serde::Deserialize, Debug)]
struct DiagnosticTest {
    pub path: Vec<PathElement>,
    pub code: Option<u32>,
    pub level: Option<ErrorLevel>,
    pub original_level: Option<ErrorLevel>,
    pub msg: Option<String>,
    pub before: Option<PathElement>,
    pub after: Option<PathElement>,
}

#[derive(serde::Deserialize, Debug)]
struct DataTypeTest {
    pub path: Vec<PathElement>,
    pub data_type: String,
}

impl DiagnosticTest {
    pub fn matches(&self, diag: &sv::Diagnostic) -> bool {
        // Check code.
        if let Some(code) = &self.code {
            if diag.cause.classification.code() != *code {
                return false;
            }
        }

        // Check adjusted level.
        if let Some(level) = &self.level {
            let level = sv::Level::from(*level);
            if diag.adjusted_level != level {
                return false;
            }
        }

        // Check original level.
        if let Some(level) = &self.original_level {
            let level = sv::Level::from(*level);
            if diag.original_level != level {
                return false;
            }
        }

        // Check message.
        if let Some(msg) = &self.msg {
            let msg = glob::Pattern::new(msg).unwrap();
            if !msg.matches(&diag.cause.to_string()) {
                return false;
            }
        }

        true
    }
}

/// A validation result checking instruction.
#[derive(serde::Deserialize, Debug)]
enum Instruction {
    Level(LevelTest),
    Diag(DiagnosticTest),
    DataType(DataTypeTest),
}

/// A diagnostic level override command.
#[derive(serde::Deserialize, Debug)]
struct DiagOverride {
    code: u32,
    min: ErrorLevel,
    max: ErrorLevel,
}

/// Test case description structure.
#[derive(serde::Deserialize, Debug)]
struct TestCase {
    /// Test case name.
    pub name: String,

    /// List of diagnostic level overrides to apply.
    pub diag_overrides: Vec<DiagOverride>,

    /// The binary serialization of the plan.
    pub plan: Vec<u8>,

    /// The instructions for checking the validation result.
    pub instructions: Vec<Instruction>,
}

/// Traverse the given path within the given node tree, and then apply f on the
/// selected node.
fn traverse<'a, I, F>(node: &mut sv::output::tree::Node, mut path: I, f: F) -> Result<(), String>
where
    I: Iterator<Item = &'a sv::output::path::PathElement>,
    F: FnOnce(&mut sv::output::tree::Node) -> Result<(), String>,
{
    match path.next() {
        Some(path_element) => {
            for data in node.data.iter_mut() {
                if let sv::output::tree::NodeData::Child(c) = data {
                    if &c.path_element == path_element {
                        let mut node = c.node.as_ref().clone();
                        let result = traverse(&mut node, path, f);
                        c.node = std::sync::Arc::new(node);
                        return result;
                    }
                }
            }
            Err(format!("missing child node {path_element}"))
        }
        None => f(node),
    }
}

impl TestCase {
    /// Runs this test case. path must be set to the test filename in order to
    /// resolve test-local YAML files properly and in order to write the HTML
    /// representation of the output for debugging.
    pub fn run(&self, path: &std::path::Path, enable_html: bool) -> Result<(), String> {
        // Create validator configuration.
        let mut cfg = sv::Config::new();
        for diag_override in self.diag_overrides.iter() {
            cfg.override_diagnostic_level(
                sv::Classification::from_code(diag_override.code)
                    .ok_or_else(|| format!("invalid error code {}", diag_override.code))?,
                diag_override.min.into(),
                diag_override.max.into(),
            );
        }
        let path_os_str = path.as_os_str().to_owned();
        cfg.add_uri_resolver(move |uri| {
            if let Some(name) = uri.strip_prefix("test:") {
                let mut yaml_path = path_os_str.clone();
                yaml_path.push(".");
                yaml_path.push(name);
                let yaml_path = std::path::PathBuf::from(yaml_path);
                std::fs::read(yaml_path)
            } else if let Some(uri) = uri.strip_prefix('/') {
                std::fs::read(std::path::PathBuf::from("../../extensions").join(uri))
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "non-test URI",
                ))
            }
        });

        // Parse the plan.
        let result = sv::parse(&self.plan[..], &cfg);

        // Export result to HTML for debugging.
        if enable_html {
            let mut html_path = path.as_os_str().to_owned();
            html_path.push(".html");
            if let Err(e) = std::fs::File::create(html_path)
                .and_then(|mut f| result.export(&mut f, sv::export::Format::Html))
            {
                println!("Error while attempting to write HTML output: {e}");
            };
        }

        // Execute test instructions.
        let mut root = result.root;
        for insn in self.instructions.iter() {
            match insn {
                Instruction::Level(level_test) => {
                    let path = convert_path(&level_test.path);
                    println!("Checking level at {path}...");
                    traverse(&mut root, path.elements.iter(), |node| {
                        let actual_level = node
                            .get_diagnostic()
                            .map(|d| d.adjusted_level)
                            .unwrap_or(sv::Level::Info);
                        if level_test
                            .allowed_levels
                            .iter()
                            .any(|l| sv::Level::from(*l) == actual_level)
                        {
                            Ok(())
                        } else {
                            Err(format!("unexpected error level {actual_level:?}"))
                        }
                    })
                    .map_err(|e| format!("check failed at {path}: {e}"))?
                }
                Instruction::Diag(diag_test) => {
                    let path = convert_path(&diag_test.path);
                    println!("Checking diagnostic at {path}...");
                    traverse(&mut root, path.elements.iter(), |node| {
                        // Find node data start index based on after (if specified).
                        let start_index = diag_test
                            .after
                            .as_ref()
                            .map(|path_element| {
                                let path_element =
                                    sv::output::path::PathElement::from(path_element.clone());
                                node.data
                                    .iter()
                                    .enumerate()
                                    .find_map(|(index, data)| {
                                        if let sv::output::tree::NodeData::Child(c) = data {
                                            if c.path_element == path_element {
                                                return Some(index + 1);
                                            }
                                        }
                                        None
                                    })
                                    .ok_or_else(|| {
                                        format!("child {path_element} does not exist (after)")
                                    })
                            })
                            .transpose()?
                            .unwrap_or(0);

                        // Find node data end index based on before (if specified).
                        let end_index = diag_test
                            .before
                            .as_ref()
                            .map(|path_element| {
                                let path_element =
                                    sv::output::path::PathElement::from(path_element.clone());
                                node.data
                                    .iter()
                                    .enumerate()
                                    .find_map(|(index, data)| {
                                        if let sv::output::tree::NodeData::Child(c) = data {
                                            if c.path_element == path_element {
                                                return Some(index);
                                            }
                                        }
                                        None
                                    })
                                    .ok_or_else(|| {
                                        format!("child {path_element} does not exist (before)")
                                    })
                            })
                            .transpose()?
                            .unwrap_or(node.data.len());

                        // Look for diagnostics within that range.
                        let diag_index = node.data[start_index..end_index]
                            .iter()
                            .enumerate()
                            .find_map(|(index, data)| {
                                if let sv::output::tree::NodeData::Diagnostic(diag) = data {
                                    if diag_test.matches(diag) {
                                        return Some(index);
                                    }
                                }
                                None
                            })
                            .ok_or_else(|| {
                                String::from("no diagnostic found that matches expectations")
                            })?;

                        // Remove the diagnostic we found from the tree.
                        node.data.remove(diag_index);

                        Ok(())
                    })
                    .map_err(|e| format!("check failed at {path}: {e}"))?
                }
                Instruction::DataType(data_type) => {
                    let path = convert_path(&data_type.path);
                    println!("Checking data type at {path}...");
                    traverse(&mut root, path.elements.iter(), |node| {
                        let actual = format!("{:#}", node.data_type());
                        if actual != data_type.data_type {
                            Err(format!("data type mismatch; found {actual}"))
                        } else {
                            Ok(())
                        }
                    })
                    .map_err(|e| format!("check failed at {path}: {e}"))?
                }
            }
        }

        Ok(())
    }
}

fn print_usage_and_fail() -> ! {
    let me = std::env::args()
        .next()
        .unwrap_or_else(|| String::from("test_runner"));
    println!("Usage: {me} <test-directory> <enable-html>");
    println!("Runs all *.test files in the test directory.");
    std::process::exit(2);
}

pub fn main() {
    // "Parse" command line arguments.
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        print_usage_and_fail();
    }

    // Determine whether we should be emitting HTML files (disabling this makes
    // the runner a lot faster).
    let enable_html = match &args[2][..] {
        "1" => true,
        "0" => false,
        _ => print_usage_and_fail(),
    };

    // Find all test cases and run them.
    println!("Running test suite...");
    let mut n_cases = 0usize;
    let mut failures = vec![];
    for fname in walkdir::WalkDir::new(&args[1])
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(std::ffi::OsStr::new("test"))
                && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
    {
        println!();
        let fname_str = fname.display().to_string();
        n_cases += 1;
        match std::fs::read_to_string(&fname) {
            Err(e) => println!("Failed to read test file {fname_str}): {e}"),
            Ok(d) => match serde_json::from_str::<TestCase>(&d) {
                Err(e) => {
                    println!("Failed to parse test file {fname_str}): {e}");
                    failures.push(fname_str);
                }
                Ok(tc) => {
                    let name = &tc.name;
                    println!("Running test {name} ({fname_str})...");
                    if let Err(s) = tc.run(&fname, enable_html) {
                        println!("Test {name} ({fname_str}) failed: {s}");
                        failures.push(name.clone());
                    } else {
                        println!("Test {name} ({fname_str}) passed");
                    }
                }
            },
        }
    }
    if n_cases == 0 {
        println!("No test cases were found! (did you run compile.py?)");
        std::process::exit(1);
    }

    // Print summary.
    println!();
    if failures.is_empty() {
        println!("All {n_cases} test case(s) passed!");
        std::process::exit(0);
    } else {
        let n_failures = failures.len();
        println!("{n_failures} out of {n_cases} test case(s) failed:");
        for failure in failures {
            println!(" - {failure}");
        }
        std::process::exit(1);
    }
}
