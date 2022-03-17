// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML extension
//! files.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::path;
use crate::parse::context;
use crate::parse::traversal;
use std::sync::Arc;

/// Toplevel parse function for a simple extension YAML file.
fn parse_root(_x: &yaml::Value, _y: &mut context::Context) -> Result<()> {
    // TODO
    Ok(())
}

/// Parse a YAML extension URI string.
pub fn parse_uri<S: AsRef<str>>(
    x: &S,
    y: &mut context::Context,
) -> Result<Arc<extension::YamlInfo>> {
    // The schema for YAML extension files.
    static SCHEMA: once_cell::sync::Lazy<jsonschema::JSONSchema> =
        once_cell::sync::Lazy::new(|| {
            jsonschema::JSONSchema::compile(
                &yaml::yaml_to_json(
                    yaml_rust::YamlLoader::load_from_str(include_str!(
                        "../../../../../../text/simple_extensions_schema.yaml"
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

    Ok(traversal::parse_yaml(x, y, Some(&SCHEMA), parse_root))
}
