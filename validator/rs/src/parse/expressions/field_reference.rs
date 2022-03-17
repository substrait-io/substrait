// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validation field references.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;

/// Parse a mask expression; that is, a field selection that can output a
/// nested structure.
pub fn parse_mask_expression(
    _x: &substrait::expression::MaskExpression,
    _y: &mut context::Context,
) -> diagnostic::Result<()> {
    // TODO

    Ok(())
}
