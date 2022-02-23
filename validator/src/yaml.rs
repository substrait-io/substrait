use crate::context;
use crate::diagnostic;
use crate::diagnostic::DiagResult;
use crate::diagnostic::Result;
use crate::path;

use serde_json::value::Value;
use yaml_rust::yaml::Yaml;

/// Converts a YAML structure into a serde JSON structure, which is needed in
/// order to run schema validation with jsonschema.
fn yaml_to_json(y: Yaml, path: &path::Path) -> DiagResult<Value> {
    match y {
        Yaml::Real(ref s) => Ok(Value::Number(
            serde_json::value::Number::from_f64(y.as_f64().ok_or_else(|| {
                error!(
                    path.to_path_buf(),
                    YamlParseFailed, "failed to parse {} as float", s
                )
            })?)
            .ok_or_else(|| {
                error!(
                    path.to_path_buf(),
                    YamlParseFailed, "{} float is not supported", s
                )
            })?,
        )),
        Yaml::Integer(i) => Ok(Value::Number(i.into())),
        Yaml::String(s) => Ok(Value::String(s)),
        Yaml::Boolean(b) => Ok(Value::Bool(b)),
        Yaml::Array(a) => Ok(Value::Array(
            a.into_iter()
                .enumerate()
                .map(|(index, value)| yaml_to_json(value, &path.with_index(index)))
                .collect::<DiagResult<Vec<Value>>>()?,
        )),
        Yaml::Hash(m) => Ok(Value::Object(
            m.into_iter()
                .map(|(key, value)| {
                    let key = key
                        .as_str()
                        .ok_or_else(|| {
                            error!(
                                path.to_path_buf(),
                                YamlParseFailed, "non-string map keys are not supported"
                            )
                        })?
                        .to_string();
                    let path = path.with_field(&key);
                    let value = yaml_to_json(value, &path)?;
                    Ok((key, value))
                })
                .collect::<DiagResult<serde_json::value::Map<String, Value>>>()?,
        )),
        Yaml::Alias(_) => Err(error!(
            path.to_path_buf(),
            YamlParseFailed, "YAML aliases are not supported"
        )),
        Yaml::Null => Ok(Value::Null),
        Yaml::BadValue => panic!("encountered Yaml::BadValue"),
    }
}

/// Returns the schema for YAML extension files.
static SIMPLE_EXTENSIONS_SCHEMA: once_cell::sync::Lazy<jsonschema::JSONSchema> =
    once_cell::sync::Lazy::new(|| {
        jsonschema::JSONSchema::compile(
            &yaml_to_json(
                yaml_rust::YamlLoader::load_from_str(include_str!(
                    "../../text/simple_extensions_schema.yaml"
                ))
                .unwrap()
                .pop()
                .unwrap(),
                &path::Path::default(),
            )
            .unwrap(),
        )
        .unwrap()
    });

/// Attempts to resolve a YAML URI.
fn resolve_yaml_uri(uri: &str, config: &context::Config) -> Result<Vec<u8>> {
    // Figure out whether we should even try to resolve this URI. If not, send
    // a warning and return None.
    let pattern_matched = config
        .yaml_resolution_exceptions
        .iter()
        .any(|x| x.matches(uri));
    if config.disable_yaml_resolution ^ pattern_matched {
        return Err(diagnostic::Cause::YamlResolutionDisabled(
            if pattern_matched {
                format!(
                    "resolution is enabled, but URI {:?} matched an exception pattern",
                    uri
                )
            } else {
                format!(
                    "resolution is disabled, and URI {:?} did not match an exception pattern",
                    uri
                )
            },
        ));
    }

    // Resolves the given URI with libcurl.
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

/// Attempts to resolve, parse, and validate a simple extension YAML file.
pub fn load_simple_extension_yaml(uri: &str, y: &mut context::Context) -> Option<Value> {
    // Try to resolve the YAML file. Note that failure to resolve is a warning,
    // not an error; it means the plan isn't valid in the current environment,
    // but it might still be valid in another one, in particular for consumers
    // that don't need to be able to resolve the YAML files to use the plan.
    let binary_data = match resolve_yaml_uri(uri, y.config) {
        Err(e) => {
            diagnostic!(y, Warning, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as UTF-8.
    let string_data = match std::str::from_utf8(&binary_data) {
        Err(e) => {
            diagnostic!(y, Error, YamlParseFailed, "{}", e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as YAML.
    let yaml_data = match yaml_rust::YamlLoader::load_from_str(string_data) {
        Err(e) => {
            diagnostic!(y, Error, YamlParseFailed, "{}", e);
            return None;
        }
        Ok(x) => {
            if x.len() > 1 {
                diagnostic!(
                    y,
                    Warning,
                    YamlParseFailed,
                    "YAML file contains multiple documents; ignoring all but the first"
                );
            }
            match x.into_iter().next() {
                None => {
                    diagnostic!(
                        y,
                        Error,
                        YamlParseFailed,
                        "YAML file contains zero documents"
                    );
                    return None;
                }
                Some(x) => x,
            }
        }
    };

    // Convert to JSON DOM.
    let json_data = match yaml_to_json(yaml_data, &y.breadcrumb.path) {
        Err(e) => {
            diagnostic!(y, e);
            return None;
        }
        Ok(x) => x,
    };

    // Validate with schema.
    if let Err(es) = SIMPLE_EXTENSIONS_SCHEMA.validate(&json_data) {
        for e in es {
            diagnostic!(y, Error, YamlSchemaValidationFailed, "{}", e);
        }
        return None;
    }

    Some(json_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url() {
        let (result, node) = with_context!(load_simple_extension_yaml, (""));
        assert!(result.is_none());
        assert_eq!(crate::get_diagnostic(&node).map(|x| x.to_string()), Some("Warning (temp): failed to resolve YAML: [3] URL using bad/illegal format or missing URL".to_string()));
    }

    #[test]
    fn test_remote_url() {
        let (result, node) = with_context!(load_simple_extension_yaml, ("https://raw.githubusercontent.com/substrait-io/substrait/9d9805be19c1d606dc2811395f203857db782872/extensions/extension_types.yaml"));
        assert!(result.is_some());
        assert_eq!(crate::get_diagnostic(&node), None);
    }
}
