// SPDX-License-Identifier: Apache-2.0

//! Rust "bindings" for validating [Substrait](https://substrait.io/) plans.
//!
//! Note that the functionality of the validator is also written in Rust, but
//! is largely contained within the [`substrait_validator_core`] crate. See its
//! docs for why this is the case.
//!
//! The usage pattern is roughly as follows.
//!
//!  1) Build a [`Config`] structure to configure the validator. You can also
//!     just use [`std::default::Default`] if you don't need to configure
//!     anything, but you might want to at least call
//!     [`WithCurlResolver::add_curl_yaml_uri_resolver()`].
//!  2) Parse the incoming `substrait.Plan` message using [`parse()`]. This
//!     creates a [tree](substrait_validator_core::output::tree) structure
//!     corresponding to the query plan that also contains diagnostics and
//!     other annotations added by the validator.
//!  3) If you don't want to traverse the tree yourself, use
//!     [`check()`], [`get_diagnostic()`], [`iter_diagnostics()`], or
//!     [`export()`] to obtain the validation results you need.
//!
//! Note that only the binary protobuf serialization format is supported at the
//! input; the JSON format is *not* supported. This is a limitation of `prost`,
//! the crate that was used for protobuf deserialization. If you're looking for
//! a library (or CLI) that supports more human-friendly input, check out the
//! Python bindings.

pub use substrait_validator_core::*;

/// Attempts to resolve and fetch the data for the given URI using libcurl,
/// allowing the validator to handle remote YAML extension URLs with most
/// protocols.
fn resolve_with_curl(uri: &str) -> Result<Vec<u8>, curl::Error> {
    let mut binary_data: Vec<u8> = vec![];
    let mut curl_handle = curl::easy::Easy::new();
    curl_handle.url(uri)?;
    {
        let mut transfer = curl_handle.transfer();
        transfer.write_function(|buf| {
            binary_data.extend_from_slice(buf);
            Ok(buf.len())
        })?;
        transfer.perform()?;
    }
    Ok(binary_data)
}

pub trait WithCurlResolver {
    /// Registers the resolve_with_curl() function for resolving YAML URIs.
    /// If libcurl fails, any yaml_uri_resolver registered previously will
    /// be used as a fallback.
    fn add_curl_yaml_uri_resolver(&mut self);
}

impl WithCurlResolver for Config {
    fn add_curl_yaml_uri_resolver(&mut self) {
        self.add_yaml_uri_resolver(resolve_with_curl)
    }
}
