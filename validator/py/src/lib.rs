use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Represents a Substrait plan parse tree, as parsed by the validator.
///
/// To construct a parse tree (and in doing so, validate the Substrait plan),
/// simply pass a bytes object containing the substrait.plan message to the
/// constructor. Note that this "never fails:" any failures to parse the
/// bytes object will be embedded as diagnostics in the ParseTree object. This
/// allows multiple error messages to be contained within the object. Use
/// check(), check_valid(), or check_not_invalid() to check validity.
#[pyclass]
struct ParseTree {
    root: substrait_validator::tree::Node,
}

#[pymethods]
impl ParseTree {
    #[new]
    pub fn new(data: &[u8]) -> Self {
        Self {
            root: substrait_validator::parse(data),
        }
    }

    /// Checks the validity of the plan passed to this ParseTree during
    /// construction. Returns -1 for invalid plans, 0 for possibly valid
    /// plans (i.e. the validator was unable to prove validity either way),
    /// or 1 for valid plans.
    pub fn check(&self) -> i32 {
        match substrait_validator::check(&self.root) {
            substrait_validator::Validity::Valid => 1,
            substrait_validator::Validity::MaybeValid => 0,
            substrait_validator::Validity::Invalid => -1,
        }
    }

    /// Throws a ValueError exception containing the first error or warning
    /// encountered in the plan if the plan was not proven to be valid by the
    /// validator.
    pub fn check_valid(&self) -> PyResult<()> {
        if let Some(diag) = substrait_validator::get_diagnostic(&self.root) {
            if diag.level >= substrait_validator::diagnostic::Level::Warning {
                return Err(PyValueError::new_err(diag.to_string()));
            }
        }
        Ok(())
    }

    /// Throws a ValueError exception containing the first error encountered
    /// in the plan if the plan was proven to be invalid by the validator.
    pub fn check_not_invalid(&self) -> PyResult<()> {
        if let Some(diag) = substrait_validator::get_diagnostic(&self.root) {
            if diag.level >= substrait_validator::diagnostic::Level::Error {
                return Err(PyValueError::new_err(diag.to_string()));
            }
        }
        Ok(())
    }

    /// Exports all diagnostic messages contained in this parse result as a
    /// multiline string.
    pub fn export_diagnostics(&self) -> PyResult<String> {
        let mut result: Vec<u8> = vec![];
        substrait_validator::export(
            &mut result,
            substrait_validator::export::Format::Diagnostics,
            &self.root,
        )?;
        let result = String::from_utf8(result)?;
        Ok(result)
    }

    /// Exports the parse tree as a HTML multiline string, intended for
    /// debugging.
    pub fn export_html(&self) -> PyResult<String> {
        let mut result: Vec<u8> = vec![];
        substrait_validator::export(
            &mut result,
            substrait_validator::export::Format::Html,
            &self.root,
        )?;
        let result = String::from_utf8(result)?;
        Ok(result)
    }

    /// Exports the entire parse tree as a substrait.validator.Node protobuf
    /// message, using binary serialization.
    pub fn export_proto(&self, py: Python) -> PyResult<PyObject> {
        let mut result = vec![];
        substrait_validator::export(
            &mut result,
            substrait_validator::export::Format::Proto,
            &self.root,
        )?;
        let result = PyBytes::new(py, &result).into();
        Ok(result)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn substrait_validator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseTree>()?;
    Ok(())
}
