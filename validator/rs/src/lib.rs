use substrait_validator_core::context;
pub use substrait_validator_core::*;

/// Attempts to resolve and fetch the data for the given URI using libcurl,
/// allowing the validator to handle remote YAML extension URLs with most
/// protocols.
pub fn resolve_with_curl(uri: &str) -> Result<Vec<u8>, curl::Error> {
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
    fn add_curl_yaml_uri_resolver(&mut self);
}

impl WithCurlResolver for context::Config {
    /// Registers the resolve_with_curl() function for resolving YAML URIs.
    /// If libcurl fails, any yaml_uri_resolver registered previously will
    /// be used as a fallback.
    fn add_curl_yaml_uri_resolver(&mut self) {
        self.add_yaml_uri_resolver(resolve_with_curl)
    }
}
