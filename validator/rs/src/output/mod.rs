// SPDX-License-Identifier: Apache-2.0

//! Output representation module.
//!
//! This module provides the data structures for representing the output of the
//! validator.

#[macro_use]
pub mod diagnostic;

pub mod comment;
pub mod data_type;
pub mod extension;
pub mod parse_result;
pub mod path;
pub mod primitive_data;
pub mod tree;
