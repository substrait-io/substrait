// SPDX-License-Identifier: Apache-2.0

//! Test runner for the [substrait_validator] crate.

use rayon::prelude::*;
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
struct TestDescription {
    /// Test case name.
    pub name: String,

    /// List of diagnostic level overrides to apply.
    pub diag_overrides: Vec<DiagOverride>,

    /// The binary serialization of the plan.
    pub plan: Vec<u8>,

    /// The instructions for checking the validation result.
    pub instructions: Vec<Instruction>,
}

/// The result of a test case including messages.
#[derive(Default)]
struct TestResult {
    /// Log messages generated while running the test.
    pub messages: Vec<String>,

    /// Whether there were failures in this test case.
    pub failed: bool,

    /// Whether the test case was skipped.
    pub skipped: bool,
}

impl TestResult {
    pub fn log<S: std::fmt::Display>(&mut self, msg: S) {
        self.messages.push(msg.to_string());
    }

    pub fn error<S: std::fmt::Display>(&mut self, msg: S) {
        self.failed = true;
        self.log(format!("Error: {msg}"));
    }

    pub fn handle_result<T, E, F, S>(&mut self, e: Result<T, E>, msg: F) -> Option<T>
    where
        F: FnOnce() -> S,
        S: std::fmt::Display,
        E: std::error::Error,
    {
        match e {
            Ok(x) => Some(x),
            Err(e) => {
                let msg = msg();
                self.error(format!("{msg}: {e}"));
                None
            }
        }
    }

    pub fn handle_option<T, S, F>(&mut self, option: Option<T>, msg: F) -> Option<T>
    where
        F: FnOnce() -> S,
        S: std::fmt::Display,
    {
        if option.is_none() {
            let msg = msg();
            self.error(format!("{msg}"));
        }
        option
    }
}

/// Configuration structure for the test runner.
struct Configuration {
    /// Skip test cases for which the name does not match this pattern.
    pub filter: glob::Pattern,

    /// Whether HTML output files should be written.
    pub enable_html: bool,
}

/// All information related to a test case, including its result.
struct TestCase {
    /// Path to the test case input file.
    pub path: std::path::PathBuf,

    /// The test description file, if parsing succeeded.
    pub description: Option<TestDescription>,

    /// The result of the test.
    pub result: TestResult,
}

impl TestCase {
    /// Traverse the given path within the given node tree, and then apply f on the
    /// selected node.
    fn traverse<'a, I, F>(
        result: &mut TestResult,
        node: &mut sv::output::tree::Node,
        mut path: I,
        f: F,
    ) where
        I: Iterator<Item = &'a sv::output::path::PathElement>,
        F: FnOnce(&mut TestResult, &mut sv::output::tree::Node),
    {
        match path.next() {
            Some(path_element) => {
                for data in node.data.iter_mut() {
                    if let sv::output::tree::NodeData::Child(c) = data {
                        if &c.path_element == path_element {
                            let mut node = c.node.as_ref().clone();
                            Self::traverse(result, &mut node, path, f);
                            c.node = std::sync::Arc::new(node);
                            return;
                        }
                    }
                }
                result.error(format!("missing child node {path_element}"));
            }
            None => f(result, node),
        }
    }

    /// Searches for the child node of node at the given path element and
    /// returns its index. If the child does not exist, None is returned, and
    /// an error is pushed.
    fn find_child_index(
        result: &mut TestResult,
        node: &mut sv::output::tree::Node,
        desc: &PathElement,
    ) -> Option<usize> {
        let path_element = sv::output::path::PathElement::from(desc.clone());
        result.handle_option(
            node.data.iter().enumerate().find_map(|(index, data)| {
                if let sv::output::tree::NodeData::Child(c) = data {
                    if c.path_element == path_element {
                        return Some(index);
                    }
                }
                None
            }),
            || format!("child {path_element} does not exist"),
        )
    }

    /// Runs the given level test instruction.
    fn run_level_test(
        result: &mut TestResult,
        root: &mut sv::output::tree::Node,
        desc: &LevelTest,
    ) {
        let path = convert_path(&desc.path);
        result.log(format!("Checking level at {path}..."));
        Self::traverse(result, root, path.elements.iter(), |result, node| {
            let actual_level = node
                .get_diagnostic()
                .map(|d| d.adjusted_level)
                .unwrap_or(sv::Level::Info);
            if !desc
                .allowed_levels
                .iter()
                .any(|l| sv::Level::from(*l) == actual_level)
            {
                result.error(format!("unexpected error level {actual_level:?}"));
            }
        });
    }

    /// Runs the given diagnostic test instruction.
    fn run_diag_test(
        result: &mut TestResult,
        root: &mut sv::output::tree::Node,
        desc: &DiagnosticTest,
    ) {
        let path = convert_path(&desc.path);
        result.log(format!("Checking diagnostic at {path}..."));
        Self::traverse(result, root, path.elements.iter(), |result, node| {
            // Find node data start index based on after (if specified).
            let start_index = desc
                .after
                .as_ref()
                .and_then(|path_element| Self::find_child_index(result, node, path_element))
                .unwrap_or(0);

            // Find node data end index based on before (if specified).
            let end_index = desc
                .before
                .as_ref()
                .and_then(|path_element| Self::find_child_index(result, node, path_element))
                .unwrap_or(node.data.len());

            // Look for diagnostics within that range.
            let diag_index = result.handle_option(
                node.data[start_index..end_index]
                    .iter()
                    .enumerate()
                    .find_map(|(index, data)| {
                        if let sv::output::tree::NodeData::Diagnostic(diag) = data {
                            if desc.matches(diag) {
                                return Some(index);
                            }
                        }
                        None
                    }),
                || "no diagnostic found that matches expectations",
            );

            // Remove the diagnostic we found from the tree.
            if let Some(diag_index) = diag_index {
                node.data.remove(diag_index);
            }
        });
    }

    /// Runs the given data type test instruction.
    fn run_data_type_test(
        result: &mut TestResult,
        root: &mut sv::output::tree::Node,
        desc: &DataTypeTest,
    ) {
        let path = convert_path(&desc.path);
        result.log(format!("Checking data type at {path}..."));
        Self::traverse(result, root, path.elements.iter(), |result, node| {
            let actual = format!("{:#}", node.data_type());
            if actual != desc.data_type {
                result.error(format!("data type mismatch; found {actual}"));
            }
        })
    }

    /// Runs the given test case, updating result.
    fn run(
        result: &mut TestResult,
        path: &std::path::Path,
        desc: &TestDescription,
        cfg: &Configuration,
    ) {
        // Create validator configuration.
        let mut validator_config = sv::Config::new();
        for diag_override in desc.diag_overrides.iter() {
            validator_config.override_diagnostic_level(
                result
                    .handle_option(sv::Classification::from_code(diag_override.code), || {
                        format!("invalid error code {}", diag_override.code)
                    })
                    .unwrap_or_default(),
                diag_override.min.into(),
                diag_override.max.into(),
            );
        }
        let path_os_str = path.as_os_str().to_owned();
        validator_config.add_uri_resolver(move |uri| {
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
        let parse_result = sv::parse(&desc.plan[..], &validator_config);

        // Export result to HTML for debugging.
        if cfg.enable_html {
            let mut html_path = path.as_os_str().to_owned();
            html_path.push(".html");
            result.handle_result(
                std::fs::File::create(html_path)
                    .and_then(|mut f| parse_result.export(&mut f, sv::export::Format::Html)),
                || "Error while attempting to write HTML output",
            );
        }

        // Execute test instructions.
        let mut root = parse_result.root;
        for insn in desc.instructions.iter() {
            match insn {
                Instruction::Level(level) => Self::run_level_test(result, &mut root, level),
                Instruction::Diag(diag) => Self::run_diag_test(result, &mut root, diag),
                Instruction::DataType(data_type) => {
                    Self::run_data_type_test(result, &mut root, data_type)
                }
            }
        }
    }

    /// Loads a plan from the given file and runs it, returning the result.
    pub fn load_and_run<P: Into<std::path::PathBuf>>(
        path: P,
        cfg: &Configuration,
    ) -> Box<TestCase> {
        // Construct the path.
        let path = path.into();

        // Construct the result object.
        let mut result = TestResult::default();

        // Read input file.
        let input = result.handle_result(std::fs::read_to_string(&path), || {
            "failed to read test file"
        });

        // Parse input file.
        let description = input.and_then(|input| {
            result.handle_result(serde_json::from_str::<TestDescription>(&input), || {
                "failed to parse test file"
            })
        });

        // Match test case filter.
        let skip = description
            .as_ref()
            .map(|d| !cfg.filter.matches(&d.name))
            .unwrap_or_default();

        // Run the test case.
        if skip {
            result.skipped = true;
        } else if let Some(desc) = &description {
            Self::run(&mut result, &path, desc, cfg);
        }

        // Log the result.
        result.log(format!(
            "Test case {} ({}): {}",
            description.as_ref().map(|d| &d.name[..]).unwrap_or("?"),
            path.display(),
            if result.skipped {
                "skipped"
            } else if result.failed {
                "FAILED"
            } else {
                "passed"
            }
        ));

        Box::new(TestCase {
            path,
            description,
            result,
        })
    }
}

fn print_usage_and_fail() -> ! {
    let me = std::env::args()
        .next()
        .unwrap_or_else(|| String::from("test_runner"));
    println!("Usage: {me} <test-directory> <enable-html> <name-pattern>");
    println!("Runs all *.test files in the test directory for which the name matches the pattern.");
    println!("NOTE: you should be running this with runner.py.");
    std::process::exit(2);
}

pub fn main() {
    // "Parse" command line arguments.
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        print_usage_and_fail();
    }
    let cfg = Configuration {
        filter: glob::Pattern::new(&args[3]).expect("invalid filter pattern"),
        enable_html: match &args[2][..] {
            "1" => true,
            "0" => false,
            _ => print_usage_and_fail(),
        },
    };

    // Find all test cases and run them.
    println!("Running test suite...");
    let paths = walkdir::WalkDir::new(&args[1])
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(std::ffi::OsStr::new("test"))
                && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect::<Vec<_>>();
    let test_cases = paths
        .par_iter()
        .map(|p| TestCase::load_and_run(p, &cfg))
        .collect::<Vec<_>>();

    // Print logs for failing tests.
    for test_case in test_cases.iter().filter(|x| x.result.failed) {
        println!();
        if let Some(desc) = &test_case.description {
            println!("Test {} ({}) FAILED:", desc.name, test_case.path.display());
        } else {
            println!("Test {} FAILED:", test_case.path.display());
        }
        for msg in test_case.result.messages.iter() {
            println!("  {msg}");
        }
    }

    // Print summary.
    let n_total = test_cases.len();
    let n_run = test_cases.iter().filter(|x| !x.result.skipped).count();
    let n_failed = test_cases.iter().filter(|x| x.result.failed).count();
    if n_total == 0 {
        println!("FAIL: no test cases were found. Did you run me using runner.py?");
        std::process::exit(1);
    } else if n_run == 0 {
        println!("FAIL: none of the {n_total} test case(s) matched the specified filter.");
        std::process::exit(1);
    } else if n_failed == 0 {
        if n_run != n_total {
            println!("PASS: all {n_run}/{n_total} matching test case(s) passed.");
        } else {
            println!("PASS: all {n_run} test case(s) passed.");
        }
        std::process::exit(0);
    } else {
        println!();
        if n_run != n_total {
            println!("FAIL: {n_failed} out of {n_run}/{n_total} matching test case(s) failed.");
        } else {
            println!("FAIL: {n_failed} out of {n_run} test case(s) failed.");
        }
        std::process::exit(1);
    }
}
