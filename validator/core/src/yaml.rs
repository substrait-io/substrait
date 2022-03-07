use crate::context;
use crate::diagnostic::DiagResult;
use crate::diagnostic::Result;
use crate::path;
use crate::primitives;
use crate::tree;

use yaml_rust::yaml::Yaml;

/// Type for the type used for arbitrary YAML values.
pub type Value = serde_json::value::Value;

/// Typedef for the type used for YAML arrays.
pub type Array = Vec<Value>;

/// Typedef for the type used for YAML maps.
pub type Map = serde_json::map::Map<String, Value>;

/// Converts a YAML structure into a serde JSON structure, which is needed in
/// order to run schema validation with jsonschema.
fn yaml_to_json(y: Yaml, path: &path::Path) -> DiagResult<Value> {
    match y {
        Yaml::Real(ref s) => Ok(Value::Number(
            serde_json::value::Number::from_f64(y.as_f64().ok_or_else(|| {
                diag!(
                    path.to_path_buf(),
                    Error,
                    YamlParseFailed,
                    "failed to parse {} as float",
                    s
                )
            })?)
            .ok_or_else(|| {
                diag!(
                    path.to_path_buf(),
                    Error,
                    YamlParseFailed,
                    "{} float is not supported",
                    s
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
                            diag!(
                                path.to_path_buf(),
                                Error,
                                YamlParseFailed,
                                "non-string map keys are not supported"
                            )
                        })?
                        .to_string();
                    let path = path.with_field(&key);
                    let value = yaml_to_json(value, &path)?;
                    Ok((key, value))
                })
                .collect::<DiagResult<serde_json::value::Map<String, Value>>>()?,
        )),
        Yaml::Alias(_) => Err(diag!(
            path.to_path_buf(),
            Error,
            YamlParseFailed,
            "YAML aliases are not supported"
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
                    "../../../text/simple_extensions_schema.yaml"
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
fn resolve_yaml_uri(uri: &str, config: &context::Config) -> Result<context::BinaryData> {
    // Apply yaml_uri_overrides configuration.
    let remapped_uri = config
        .yaml_uri_overrides
        .iter()
        .find_map(|(pattern, mapping)| {
            if pattern.matches(uri) {
                Some(mapping.as_ref().map(|x| &x[..]))
            } else {
                None
            }
        });
    let is_remapped = remapped_uri.is_some();
    let remapped_uri = remapped_uri.unwrap_or(Some(uri));

    let remapped_uri = if let Some(remapped_uri) = remapped_uri {
        remapped_uri
    } else {
        return Err(cause!(
            YamlResolutionDisabled,
            "YAML resolution for {} was disabled",
            uri
        ));
    };

    // If a custom download function is specified, use it to resolve.
    if let Some(ref resolver) = config.yaml_uri_resolver {
        return resolver(remapped_uri)
            .map_err(|x| cause!(YamlResolutionFailed, x.as_ref().to_string()));
    }

    // Parse as a URL.
    let url = match url::Url::parse(remapped_uri) {
        Ok(url) => url,
        Err(e) => {
            return Err(if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "configured URI remapping ({}) did not parse as URL: {}",
                    remapped_uri,
                    e
                )
            } else {
                cause!(
                    YamlResolutionFailed,
                    "failed to parse {} as URL: {}",
                    remapped_uri,
                    e
                )
            });
        }
    };

    // Reject anything that isn't file://-based.
    if url.scheme() != "file" {
        return Err(if is_remapped {
            cause!(
                YamlResolutionFailed,
                "configured URI remapping ({}) does not use file:// scheme",
                remapped_uri
            )
        } else {
            cause!(YamlResolutionFailed, "URI does not use file:// scheme")
        });
    }

    // Convert to path.
    let path = match url.to_file_path() {
        Ok(path) => path,
        Err(_) => {
            return Err(if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "configured URI remapping ({}) could not be converted to file path",
                    remapped_uri
                )
            } else {
                cause!(
                    YamlResolutionFailed,
                    "URI could not be converted to file path"
                )
            });
        }
    };

    // Read the file.
    std::fs::read(path)
        .map_err(|e| {
            if is_remapped {
                cause!(
                    YamlResolutionFailed,
                    "failed to file remapping for URI ({}): {}",
                    remapped_uri,
                    e
                )
            } else {
                cause!(YamlResolutionFailed, e)
            }
        })
        .map(|d| -> context::BinaryData { Box::new(d) })
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
    let string_data = match std::str::from_utf8(binary_data.as_ref().as_ref()) {
        Err(e) => {
            diagnostic!(y, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as YAML.
    let yaml_data = match yaml_rust::YamlLoader::load_from_str(string_data) {
        Err(e) => {
            diagnostic!(y, Error, YamlParseFailed, e);
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
            diagnostic!(y, Error, YamlSchemaValidationFailed, e);
        }
        return None;
    }

    Some(json_data)
}

/// Converts a YAML value to a tree node, similar to
/// proto::meta::ProtoDatum::proto_data_to_node().
pub fn yaml_to_node(yaml: &serde_json::value::Value) -> tree::Node {
    match yaml {
        serde_json::Value::Null => tree::NodeType::YamlPrimitive(primitives::PrimitiveData::Null),
        serde_json::Value::Bool(b) => {
            tree::NodeType::YamlPrimitive(primitives::PrimitiveData::Bool(*b))
        }
        serde_json::Value::Number(n) => tree::NodeType::YamlPrimitive(
            n.as_u64()
                .map(primitives::PrimitiveData::Unsigned)
                .or_else(|| n.as_i64().map(primitives::PrimitiveData::Signed))
                .or_else(|| n.as_f64().map(primitives::PrimitiveData::Float))
                .unwrap(),
        ),
        serde_json::Value::String(s) => {
            tree::NodeType::YamlPrimitive(primitives::PrimitiveData::String(s.clone()))
        }
        serde_json::Value::Array(_) => tree::NodeType::YamlArray,
        serde_json::Value::Object(_) => tree::NodeType::YamlMap,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url() {
        let (result, node) = with_context!(load_simple_extension_yaml, (""));
        assert!(result.is_none());
        assert_eq!(crate::get_diagnostic(&node).map(|x| x.to_string()), Some("Warning at temp: failed to resolve YAML: failed to parse  as URL: relative URL without a base (2002)".to_string()));
    }

    #[test]
    fn test_valid_file() {
        let (result, _) = with_context!(
            load_simple_extension_yaml,
            ("file:///this/file/hopefully/does/not/exist")
        );
        assert!(result.is_none());
    }
}
