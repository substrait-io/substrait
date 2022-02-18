mod extensions;

use crate::context;
use crate::proto;
use crate::Result;

/// Toplevel parse function for a plan.
pub fn parse_plan(x: &proto::substrait::Plan, y: &mut context::Context) -> Result<()> {
    extensions::parse_extensions_before_relations(x, y);
    // TODO
    extensions::parse_extensions_after_relations(x, y);
    Ok(())
}
