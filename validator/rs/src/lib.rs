pub use substrait_validator_core::*;

/*// Resolves the given URI with libcurl.
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
}*/
