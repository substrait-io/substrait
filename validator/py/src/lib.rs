// SPDX-License-Identifier: Apache-2.0

// This happens in PyO3 generated code, and there doesn't seem to be a more
// narrow scope that this can be disabled in (clippy seems a bit confused about
// the code causing the warning, in general).
#![allow(clippy::needless_option_as_deref)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Represents a validator/parser configuration.
#[pyclass]
struct Config {
    config: substrait_validator::Config,
}

#[pymethods]
impl Config {
    #[new]
    pub fn new() -> Self {
        Config {
            config: substrait_validator::Config::new(),
        }
    }

    /// Instructs the validator to ignore protobuf fields that it doesn't know
    /// about yet (i.e., that have been added to the Substrait protobuf
    /// descriptions, but haven't yet been implemented in the validator) if the
    /// fields are set to their default value. If this option isn't set, or if
    /// an unknown field is not set to its default value, a warning is emitted.
    pub fn ignore_unknown_fields_set_to_default(&mut self) {
        self.config.ignore_unknown_fields_set_to_default = true;
    }

    /// Explicitly allows a protobuf message type to be used in advanced
    /// extensions, despite the fact that the validator can't validate it. If
    /// an advanced extension is encountered that isn't explicitly allowed, a
    /// warning is emitted. The pattern may include * and ? wildcards for
    /// glob-like matching (see
    /// https://docs.rs/glob/latest/glob/struct.Pattern.html for the complete
    /// syntax).
    pub fn allow_any_url(&mut self, pattern: &str) -> PyResult<()> {
        let pattern = match substrait_validator::Pattern::new(pattern) {
            Ok(p) => p,
            Err(e) => {
                return Err(PyValueError::new_err(format!(
                    "invalid pattern {pattern:?}: {e}"
                )));
            }
        };
        self.config.allow_any_url(pattern);
        Ok(())
    }

    /// Sets a minimum and/or maximum error level for the given class of
    /// diagnostic messages. Any previous settings for this class are
    /// overridden.
    pub fn override_diagnostic_level(
        &mut self,
        class: u32,
        minimum: &str,
        maximum: &str,
    ) -> PyResult<()> {
        fn str_to_level(level: &str) -> PyResult<substrait_validator::Level> {
            match level {
                "info" => Ok(substrait_validator::Level::Info),
                "warning" => Ok(substrait_validator::Level::Warning),
                "error" => Ok(substrait_validator::Level::Error),
                level => Err(PyValueError::new_err(format!(
                    "invalid level {level:?}; must be \"info\", \"warning\", or \"error\""
                ))),
            }
        }
        let class = match substrait_validator::Classification::from_code(class) {
            Some(c) => c,
            None => {
                return Err(PyValueError::new_err(format!(
                    "unknown diagnostic class {class}"
                )))
            }
        };
        let minimum = str_to_level(minimum)?;
        let maximum = str_to_level(maximum)?;
        self.config
            .override_diagnostic_level(class, minimum, maximum);
        Ok(())
    }

    /// Overrides the resolution behavior for YAML URIs matching the given
    /// pattern. The pattern may include * and ? wildcards for glob-like
    /// matching (see https://docs.rs/glob/latest/glob/struct.Pattern.html
    /// for the complete syntax). If resolve_as is None, the YAML file will not
    /// be resolved; otherwise it should be a string representing the URI it
    /// should be resolved as.
    pub fn override_yaml_uri(&mut self, pattern: &str, resolve_as: Option<&str>) -> PyResult<()> {
        let pattern = match substrait_validator::Pattern::new(pattern) {
            Ok(p) => p,
            Err(e) => {
                return Err(PyValueError::new_err(format!(
                    "invalid pattern {pattern:?}: {e}"
                )));
            }
        };
        self.config.override_yaml_uri(pattern, resolve_as);
        Ok(())
    }

    /// Registers a YAML URI resolution function with this configuration. If
    /// the given function fails, any previously registered function will be
    /// used as a fallback. The callback function must take a single string
    /// argument and return a bytes object, or throw an exception on failure.
    pub fn add_yaml_uri_resolver(&mut self, callback: PyObject) {
        self.config
            .add_yaml_uri_resolver(move |uri| -> Result<Vec<u8>, PyErr> {
                pyo3::Python::with_gil(|py| {
                    Ok(callback
                        .call1(py, (uri,))?
                        .as_ref(py)
                        .downcast::<pyo3::types::PyBytes>()?
                        .as_bytes()
                        .to_owned())
                })
            })
    }
}

/// Represents a Substrait plan parse tree, as parsed by the validator.
///
/// To construct a parse tree (and in doing so, validate the Substrait plan),
/// simply pass a bytes object containing the substrait.plan message to the
/// constructor. Note that this "never fails:" any failures to parse the
/// bytes object will be embedded as diagnostics in the ParseResult object.
/// This allows multiple error messages to be contained within the object. Use
/// check(), check_valid(), or check_not_invalid() to check validity.
#[pyclass]
struct ParseResult {
    root: substrait_validator::ParseResult,
}

#[pymethods]
impl ParseResult {
    #[new]
    pub fn new(data: &[u8], config: Option<&Config>) -> Self {
        Self {
            root: if let Some(config) = config {
                substrait_validator::parse(data, &config.config)
            } else {
                substrait_validator::parse(data, &substrait_validator::Config::default())
            },
        }
    }

    /// Checks the validity of the plan passed to this ParseResult during
    /// construction. Returns -1 for invalid plans, 0 for possibly valid
    /// plans (i.e. the validator was unable to prove validity either way),
    /// or 1 for valid plans.
    pub fn check(&self) -> i32 {
        match self.root.check() {
            substrait_validator::Validity::Valid => 1,
            substrait_validator::Validity::MaybeValid => 0,
            substrait_validator::Validity::Invalid => -1,
        }
    }

    /// Throws a ValueError exception containing the first error or warning
    /// encountered in the plan if the plan was not proven to be valid by the
    /// validator.
    pub fn check_valid(&self) -> PyResult<()> {
        if let Some(diag) = self.root.get_diagnostic() {
            if diag.adjusted_level >= substrait_validator::Level::Warning {
                return Err(PyValueError::new_err(diag.to_string()));
            }
        }
        Ok(())
    }

    /// Throws a ValueError exception containing the first error encountered
    /// in the plan if the plan was proven to be invalid by the validator.
    pub fn check_not_invalid(&self) -> PyResult<()> {
        if let Some(diag) = self.root.get_diagnostic() {
            if diag.adjusted_level >= substrait_validator::Level::Error {
                return Err(PyValueError::new_err(diag.to_string()));
            }
        }
        Ok(())
    }

    /// Exports all diagnostic messages contained in this parse result as a
    /// multiline string.
    pub fn export_diagnostics(&self) -> PyResult<String> {
        let mut result: Vec<u8> = vec![];
        self.root.export(
            &mut result,
            substrait_validator::export::Format::Diagnostics,
        )?;
        let result = String::from_utf8(result)?;
        Ok(result)
    }

    /// Exports the parse tree as a HTML multiline string, intended for
    /// debugging.
    pub fn export_html(&self) -> PyResult<String> {
        let mut result: Vec<u8> = vec![];
        self.root
            .export(&mut result, substrait_validator::export::Format::Html)?;
        let result = String::from_utf8(result)?;
        Ok(result)
    }

    /// Exports the entire parse tree as a substrait.validator.Node protobuf
    /// message, using binary serialization.
    pub fn export_proto(&self, py: Python) -> PyResult<PyObject> {
        let mut result = vec![];
        self.root
            .export(&mut result, substrait_validator::export::Format::Proto)?;
        let result = PyBytes::new(py, &result).into();
        Ok(result)
    }
}

/// Rust-native module for the validator.
#[pymodule]
fn substrait_validator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Config>()?;
    m.add_class::<ParseResult>()?;
    Ok(())
}
