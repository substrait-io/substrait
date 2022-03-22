// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating function calls.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse a subquery.
pub fn parse_subquery(
    _x: &substrait::expression::Subquery,
    _y: &mut context::Context,
) -> diagnostic::Result<()> {
    // TODO
    Ok(())
}
