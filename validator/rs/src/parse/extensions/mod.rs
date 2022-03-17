// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions relating to extensions.

use crate::input::proto::substrait;
use crate::parse::context;

pub mod advanced;
pub mod simple;

/// Parses the extension information in a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) {
    advanced::parse_plan(x, y);
    simple::parse_plan(x, y);
}

/// Generate Info diagnostics for any extension definitions that weren't used.
pub fn check_unused_definitions(y: &mut context::Context) {
    advanced::check_unused_definitions(y);
    simple::check_unused_definitions(y);
}
