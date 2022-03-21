// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for advanced extensions, i.e.
//! those based around YAML files.

use crate::input::proto::substrait;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::parse::context;
use crate::string_util;
use std::sync::Arc;

mod yaml;

/// "Parse" an anchor. This just reports an error if the anchor is 0.
fn parse_anchor(x: &u32, _y: &mut context::Context) -> Result<u32> {
    if *x == 0 {
        Err(cause!(
            IllegalValue,
            "anchor 0 is reserved to disambiguate unspecified optional references"
        ))
    } else {
        Ok(*x)
    }
}

/// Parse a mapping from a URI anchor to a YAML extension.
fn parse_simple_extension_yaml_uri_mapping(
    x: &substrait::extensions::SimpleExtensionUri,
    y: &mut context::Context,
) -> Result<()> {
    // Parse the fields.
    let anchor = proto_primitive_field!(x, y, extension_uri_anchor, parse_anchor).1;
    let yaml_data = proto_primitive_field!(x, y, uri, yaml::parse_uri)
        .1
        .unwrap();

    // If the specified anchor is valid, insert a mapping for it.
    if let Some(anchor) = anchor {
        if let Err((prev_data, prev_path)) = y.define_extension_uri(anchor, yaml_data) {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "anchor {anchor} is already in use for URI {}",
                prev_data.uri
            );
            link!(y, prev_path, "Previous definition was here.");
        }
    }

    Ok(())
}

/// Parse an URI reference and resolve it.
fn parse_uri_reference(x: &u32, y: &mut context::Context) -> Result<Arc<extension::YamlInfo>> {
    match y.extension_uris().resolve(x).cloned() {
        Some((yaml_data, path)) => {
            link!(y, path, "URI anchor is defined here");
            Ok(yaml_data)
        }
        None => Err(cause!(LinkMissingAnchor, "URI anchor {x} does not exist")),
    }
}

/// Parse a type variation reference and resolve it.
pub fn parse_type_variation_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::TypeVariation>>> {
    match y.tvars().resolve(x).cloned() {
        Some((variation, path)) => {
            link!(y, path, "Type variation anchor is defined here");
            Ok(variation)
        }
        None => Err(cause!(
            LinkMissingAnchor,
            "type variation anchor {x} does not exist"
        )),
    }
}

/// Parse a type reference and resolve it.
pub fn parse_type_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::DataType>>> {
    match y.types().resolve(x).cloned() {
        Some((data_type, path)) => {
            link!(y, path, "Type anchor is defined here");
            Ok(data_type)
        }
        None => Err(cause!(LinkMissingAnchor, "type anchor {x} does not exist")),
    }
}

/*
/// Parse a function reference and resolve it.
pub fn parse_function_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::Function>>> {
    match y.fns().resolve(x).cloned() {
        Some((function, path)) => {
            link!(y, path, "Function anchor is defined here");
            Ok(function)
        }
        None => Err(cause!(
            LinkMissingAnchor,
            "Function anchor {x} does not exist"
        )),
    }
}
*/

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping_data(
    x: &substrait::extensions::simple_extension_declaration::MappingType,
    y: &mut context::Context,
) -> Result<()> {
    match x {
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionType(x) => {

            // Parse the fields.
            let yaml_info = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, type_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name).1.unwrap();

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the data type in it.
            let data_type = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data.as_ref().and_then(|data| {
                    let data_type = data.types.get(&name.to_lowercase()).cloned();
                    if data_type.is_none() {
                        diagnostic!(y, Error, LinkMissingTypeName, "failed to resolve data type {name:?} in {yaml_info}");
                    }
                    data_type
                })
            });

            // Construct a reference for this data type.
            let reference = Arc::new(extension::Reference {
                common: extension::Common {
                    name,
                    yaml_info,
                    anchor_path: Some(y.path_buf())
                },
                definition: data_type
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_type(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for data type {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(x) => {

            // Parse the fields.
            let yaml_info = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, type_variation_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name).1.unwrap();

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the type variation in it.
            let type_variation = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data.as_ref().and_then(|data| {
                    let type_variation = data.type_variations.get(&name.to_lowercase()).cloned();
                    if type_variation.is_none() {
                        diagnostic!(y, Error, LinkMissingTypeVariationName, "failed to resolve type variation {name:?} in {yaml_info}");
                    }
                    type_variation
                })
            });

            // Construct a reference for this type variation.
            let reference = Arc::new(extension::Reference {
                common: extension::Common {
                    name,
                    yaml_info,
                    anchor_path: Some(y.path_buf())
                },
                definition: type_variation
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_tvar(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for type variation {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionFunction(x) => {

            // Parse the fields.
            let yaml_info = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, function_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name).1.unwrap();

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the data type in it.
            let function = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data.as_ref().and_then(|data| {
                    let function = data.functions.get(&name.to_lowercase()).cloned();
                    if function.is_none() {
                        diagnostic!(y, Error, LinkMissingFunctionName, "failed to resolve function {name:?} in {yaml_info}");
                    }
                    function
                })
            });

            // Construct a reference for this data type.
            let reference = Arc::new(extension::Reference {
                common: extension::Common {
                    name,
                    yaml_info,
                    anchor_path: Some(y.path_buf())
                },
                definition: function
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_fn(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for function {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
    };
    Ok(())
}

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping(
    x: &substrait::extensions::SimpleExtensionDeclaration,
    y: &mut context::Context,
) -> Result<()> {
    proto_required_field!(x, y, mapping_type, parse_extension_mapping_data);
    Ok(())
}

/// Parses the simple extension information in a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) {
    proto_repeated_field!(
        x,
        y,
        extension_uris,
        parse_simple_extension_yaml_uri_mapping
    );
    proto_repeated_field!(x, y, extensions, parse_extension_mapping);
}

/// Generate Info diagnostics for any extension definitions that weren't used.
pub fn check_unused_definitions(y: &mut context::Context) {
    // List unused function declarations.
    for (anchor, info, path) in y.fns().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantFunctionDeclaration,
            "anchor {anchor} for function {} is not present in the plan",
            string_util::as_ident_or_string(&info.common.name)
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type declarations.
    for (anchor, info, path) in y.types().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantTypeDeclaration,
            "anchor {anchor} for type {} is not present in the plan",
            string_util::as_ident_or_string(&info.common.name)
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type variation declarations.
    for (anchor, info, path) in y.tvars().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantTypeVariationDeclaration,
            "anchor {anchor} for type variation {} is not present in the plan",
            string_util::as_ident_or_string(&info.common.name)
        );
        link!(y, path, "Declaration was here.");
    }
}
