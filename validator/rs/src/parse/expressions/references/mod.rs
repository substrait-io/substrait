// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating references.

use crate::output::data_type;
use crate::output::diagnostic;
use crate::parse::context;
use std::sync::Arc;

pub mod mask;
pub mod scalar;

/// Parse a struct field index into its data type.
fn parse_struct_field_index(
    x: &i32,
    _y: &mut context::Context,
    root: &Arc<data_type::DataType>,
) -> diagnostic::Result<Arc<data_type::DataType>> {
    let index = *x;
    if index < 0 {
        return Err(cause!(
            IllegalValue,
            "struct indices cannot be less than zero"
        ));
    }
    let index: usize = index.try_into().unwrap();
    if root.is_struct() {
        let size = root.parameters().len();
        root.type_parameter(index)
            .ok_or_else(|| cause!(IllegalValue, "struct index out of range (size = {size})"))
    } else {
        Ok(Arc::default())
    }
}
