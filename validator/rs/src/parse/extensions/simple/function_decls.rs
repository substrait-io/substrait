// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML function
//! declarations.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::parse::context;

/// Parse a scalar function declaration.
pub fn parse_scalar_function(_x: &yaml::Value, _y: &mut context::Context) -> Result<()> {
    // TODO
    Ok(())
}

/// Parse an aggregate function declaration.
pub fn parse_aggregate_function(_x: &yaml::Value, _y: &mut context::Context) -> Result<()> {
    // TODO
    Ok(())
}
