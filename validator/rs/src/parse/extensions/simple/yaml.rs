// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML extension
//! files.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::path;
use crate::parse::context;
use crate::parse::extensions::simple::function_decls;
use crate::parse::extensions::simple::type_decls;
use crate::parse::extensions::simple::type_variation_decls;
use crate::parse::traversal;
use crate::string_util;
use std::sync::Arc;

/// Toplevel parse function for a simple extension YAML file.
fn parse_root(x: &yaml::Value, y: &mut context::Context) -> Result<()> {
    yaml_repeated_field!(x, y, "types", type_decls::parse_type)?;
    yaml_repeated_field!(
        x,
        y,
        "type_variations",
        type_variation_decls::parse_type_variation
    )?;
    yaml_repeated_field!(
        x,
        y,
        "scalar_functions",
        function_decls::parse_scalar_function
    )?;
    yaml_repeated_field!(
        x,
        y,
        "aggregate_functions",
        function_decls::parse_aggregate_function
    )?;
    Ok(())
}

/// Parse a YAML extension URI string.
pub fn parse_uri<S: AsRef<str>>(
    x: &S,
    y: &mut context::Context,
) -> Result<Arc<extension::YamlInfo>> {
    // Check URI syntax.
    let x = x.as_ref();
    if let Err(e) = string_util::check_uri(x) {
        diagnostic!(y, Error, e);
    }

    // The schema for YAML extension files.
    static SCHEMA: once_cell::sync::Lazy<jsonschema::JSONSchema> =
        once_cell::sync::Lazy::new(|| {
            jsonschema::JSONSchema::compile(
                &yaml::yaml_to_json(
                    yaml_rust::YamlLoader::load_from_str(include_str!(
                        "../../../resources/text/simple_extensions_schema.yaml"
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
