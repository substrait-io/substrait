// SPDX-License-Identifier: Apache-2.0

//! This module provides the configuration structure for the validator.
//!
//! This structure, [`Config`], is to be constructed by the application using
//! the validator to configure it. Alternatively, the default configuration can
//! be constructed by using the [`std::default::Default`] trait.

use crate::output::diagnostic;
pub use glob;
use std::collections::HashMap;

/// Trait object representing some immutable binary data.
pub type BinaryData = Box<dyn AsRef<[u8]>>;

/// Trait object representing some error data.
pub type ErrorData = Box<dyn std::error::Error>;

/// Callback function type for resolving/downloading URIs.
pub type UriResolver = Box<dyn Fn(&str) -> std::result::Result<BinaryData, ErrorData> + Send>;

/// Attempts to resolve and fetch the data for the given URI using libcurl,
/// allowing the validator to handle remote YAML extension URLs with most
/// protocols.
#[cfg(feature = "curl")]
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

/// Configuration structure.
#[derive(Default)]
pub struct Config {
    /// When set, do not generate warnings for unknown protobuf fields that are
    /// set to their protobuf-defined default value.
    pub ignore_unknown_fields: bool,

    /// Protobuf message URLs that are explicitly allowed for use in "any"
    /// messages, i.e. that the caller warrants the existence of in the
    /// consumer that the plan is validated for.
    pub allowed_proto_any_urls: Vec<glob::Pattern>,

    /// Allows the level of diagnostic messages to be overridden based on their
    /// classification/code. The logic for this is as follows:
    ///
    ///  - if an entry exists for the classication of the incoming diagnostic,
    ///    override its error level to at most the second argument, and then to
    ///    at least the first argument. Otherwise,
    ///  - if an entry exists for the group of said classification, use its
    ///    level limits instead. Otherwise,
    ///  - if an entry exists for Unclassified (code 0), use its level limits
    ///    instead. Otherwise, do not adjust the level.
    ///
    /// Note that setting an entry to  (Info, Error) leaves the diagnostic
    /// level unchanged.
    pub diagnostic_level_overrides:
        HashMap<diagnostic::Classification, (diagnostic::Level, diagnostic::Level)>,

    /// Allows URIs from the plan to be remapped (Some(mapping)) or ignored
    /// (None). All resolution can effectively be disabled by just adding a
    /// rule that maps * to None. Furthermore, in the absence of a custom
    /// yaml_uri_resolver function, this can be used to remap URIs to
    /// pre-downloaded files.
    pub uri_overrides: Vec<(glob::Pattern, Option<String>)>,

    /// Optional callback function for resolving URIs. If specified, all
    /// URIs (after processing yaml_uri_overrides) are resolved using this
    /// function. The function takes the URI as its argument, and should either
    /// return the download contents as a Vec<u8> or return a String-based
    /// error. If no downloader is specified, only file:// URLs with an
    /// absolute path are supported.
    pub uri_resolver: Option<UriResolver>,
}

impl Config {
    /// Creates a default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Instructs the validator to ignore protobuf fields that it doesn't know
    /// about yet (i.e., that have been added to the Substrait protobuf
    /// descriptions, but haven't yet been implemented in the validator) if the
    /// fields are set to their default value. If this option isn't set, or if
    /// an unknown field is not set to its default value, a warning is emitted.
    pub fn ignore_unknown_fields(&mut self) {
        self.ignore_unknown_fields = true;
    }

    /// Explicitly allows a protobuf message type to be used in advanced
    /// extensions, despite the fact that the validator can't validate it. If
    /// an advanced extension is encountered that isn't explicitly allowed, a
    /// warning is emitted.
    pub fn allow_proto_any_url(&mut self, pattern: glob::Pattern) {
        self.allowed_proto_any_urls.push(pattern);
    }

    /// Sets a minimum and/or maximum error level for the given class of
    /// diagnostic messages. Any previous settings for this class are
    /// overridden.
    pub fn override_diagnostic_level(
        &mut self,
        class: diagnostic::Classification,
        minimum: diagnostic::Level,
        maximum: diagnostic::Level,
    ) {
        self.diagnostic_level_overrides
            .insert(class, (minimum, maximum));
    }

    /// Overrides the resolution behavior for (YAML) URIs matching the given
    /// pattern. If resolve_as is None, the URI file will not be resolved;
    /// if it is Some(s), it will be resolved as if the URI in the plan had
    /// been s.
    pub fn override_uri<S: Into<String>>(&mut self, pattern: glob::Pattern, resolve_as: Option<S>) {
        self.uri_overrides
            .push((pattern, resolve_as.map(|s| s.into())));
    }

    /// Registers a URI resolution function with this configuration. If
    /// the given function fails, any previously registered function will be
    /// used as a fallback.
    pub fn add_uri_resolver<F, D, E>(&mut self, resolver: F)
    where
        F: Fn(&str) -> Result<D, E> + Send + 'static,
        D: AsRef<[u8]> + 'static,
        E: std::error::Error + 'static,
    {
        let previous = self.uri_resolver.take();
        self.uri_resolver = Some(Box::new(move |uri| match resolver(uri) {
            Ok(d) => Ok(Box::new(d)),
            Err(e) => match &previous {
                Some(f) => f.as_ref()(uri),
                None => Err(Box::new(e)),
            },
        }));
    }

    /// Registers a URI resolver based on libcurl. If libcurl fails, any
    /// `uri_resolver` registered previously will be used as a fallback.
    #[cfg(feature = "curl")]
    pub fn add_curl_uri_resolver(&mut self) {
        self.add_uri_resolver(resolve_with_curl)
    }
}
