// SPDX-License-Identifier: Apache-2.0

//! Crate for validating [Substrait](https://substrait.io/).
//!
//! The usage pattern is roughly as follows.
//!
//!  1) Build a [`Config`] structure to configure the validator. You can also
//!     just use [`std::default::Default`] if you don't need to configure
//!     anything, but you might want to at least call
//!     [`Config::add_curl_yaml_uri_resolver()`] (if you're using the `curl`
//!     feature).
//!  2) Parse the incoming `substrait.Plan` message using [`parse()`]. This
//!     creates a [ParseResult], containing a [tree](output::tree) structure
//!     corresponding to the query plan that also contains diagnostics and
//!     other annotations added by the validator.
//!  3) You can traverse the tree yourself using [ParseResult::root], or you
//!     can use one of the methods associated with [ParseResult] to obtain the
//!     validation results you need.
//!
//! Note that only the binary protobuf serialization format is supported at the
//! input; the JSON format is *not* supported. This is a limitation of `prost`,
//! the crate that was used for protobuf deserialization. If you're looking for
//! a library (or CLI) that supports more human-friendly input, check out the
//! Python bindings.

#[macro_use]
pub mod output;

#[macro_use]
mod parse;

pub mod export;
pub mod input;

// Aliases for common types used on the crate interface.
pub use input::config::glob::Pattern;
pub use input::config::Config;
pub use output::diagnostic::Classification;
pub use output::diagnostic::Diagnostic;
pub use output::diagnostic::Level;
pub use output::parse_result::ParseResult;
pub use output::parse_result::Validity;

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf>(buffer: B, config: &Config) -> ParseResult {
    parse::parse(buffer, config)
}
