// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for advanced extensions, i.e.
//! those based around YAML files.

use crate::input::proto::substrait;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::parse::context;
use std::sync::Arc;

mod function_decls;
mod type_decls;
mod type_variation_decls;
mod yaml;

/// Parse a user-defined name. Note that names are matched case-insensitively
/// because we return the name as lowercase.
#[allow(clippy::ptr_arg)]
pub fn parse_name(x: &String, _y: &mut context::Context) -> Result<String> {
    // FIXME: nothing seems to say anything about the validity of names for
    // things, but this seems rather important to define.
    if x.is_empty() {
        Err(cause!(IllegalValue, "names cannot be empty"))
    } else {
        Ok(x.to_lowercase())
    }
}

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
                prev_data.uri()
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
            describe!(y, Misc, "{}", yaml_data.uri());
            link!(y, path, "URI anchor is defined here");
            Ok(yaml_data)
        }
        None => {
            describe!(y, Misc, "Unresolved URI");
            Err(cause!(LinkMissingAnchor, "URI anchor {x} does not exist"))
        }
    }
}

/// Adds a description to a resolved function/type/variation reference node.
fn describe_reference<T>(y: &mut context::Context, reference: &Arc<extension::Reference<T>>) {
    describe!(y, Misc, "{}", reference);
}

/// Parse a type variation reference and resolve it.
pub fn parse_type_variation_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::TypeVariation>>> {
    match y.tvars().resolve(x).cloned() {
        Some((variation, path)) => {
            describe_reference(y, &variation);
            link!(y, path, "Type variation anchor is defined here");
            Ok(variation)
        }
        None => {
            describe!(y, Misc, "Unresolved type variation");
            Err(cause!(
                LinkMissingAnchor,
                "Type variation anchor {x} does not exist"
            ))
        }
    }
}

/// Parse a type reference and resolve it.
pub fn parse_type_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::DataType>>> {
    match y.types().resolve(x).cloned() {
        Some((data_type, path)) => {
            describe_reference(y, &data_type);
            link!(y, path, "Type anchor is defined here");
            Ok(data_type)
        }
        None => {
            describe!(y, Misc, "Unresolved type");
            Err(cause!(LinkMissingAnchor, "Type anchor {x} does not exist"))
        }
    }
}

/// Parse a function reference and resolve it.
pub fn parse_function_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::Reference<extension::Function>>> {
    match y.fns().resolve(x).cloned() {
        Some((function, path)) => {
            describe_reference(y, &function);
            link!(y, path, "Function anchor is defined here");
            Ok(function)
        }
        None => {
            describe!(y, Misc, "Unresolved function");
            Err(cause!(
                LinkMissingAnchor,
                "Function anchor {x} does not exist"
            ))
        }
    }
}

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
            let name = proto_primitive_field!(x, y, name, parse_name).1;

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the data type in it.
            let data_type = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data().and_then(|data| {
                    name.as_ref().and_then(|name| {
                        let data_type = data.types.get(name).cloned();
                        if data_type.is_none() {
                            diagnostic!(y, Error, LinkMissingTypeName, "failed to resolve data type {name:?} in {yaml_info}");
                        }
                        data_type
                    })
                })
            });

            // Construct a reference for this data type.
            let reference = Arc::new(extension::Reference {
                name: extension::NamedReference::new(name, Some(y.path_buf())),
                uri: yaml_info.as_ref().map(|x| x.uri().clone()).unwrap_or_default(),
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
            let name = proto_primitive_field!(x, y, name, parse_name).1;

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the type variation in it.
            let type_variation = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data().and_then(|data| {
                    name.as_ref().and_then(|name| {
                        let type_variation = data.type_variations.get(name).cloned();
                        if type_variation.is_none() {
                            diagnostic!(y, Error, LinkMissingTypeVariationName, "failed to resolve type variation {name:?} in {yaml_info}");
                        }
                        type_variation
                    })
                })
            });

            // Construct a reference for this type variation.
            let reference = Arc::new(extension::Reference {
                name: extension::NamedReference::new(name, Some(y.path_buf())),
                uri: yaml_info.as_ref().map(|x| x.uri().clone()).unwrap_or_default(),
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
            let name = proto_primitive_field!(x, y, name).1;

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the data type in it.
            let function = yaml_info.as_ref().and_then(|yaml_info| {
                yaml_info.data().and_then(|data| {
                    name.as_ref().and_then(|name| {
                        let function = data.functions.get(name).cloned();
                        if function.is_none() {
                            diagnostic!(y, Error, LinkMissingFunctionName, "failed to resolve function {name:?} in {yaml_info}");
                        }
                        function
                    })
                })
            });

            // Construct a reference for this data type.
            let reference = Arc::new(extension::Reference {
                name: extension::NamedReference::new(name, Some(y.path_buf())),
                uri: yaml_info.as_ref().map(|x| x.uri().clone()).unwrap_or_default(),
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
            "anchor {anchor} for function {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type declarations.
    for (anchor, info, path) in y.types().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantTypeDeclaration,
            "anchor {anchor} for type {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type variation declarations.
    for (anchor, info, path) in y.tvars().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantTypeVariationDeclaration,
            "anchor {anchor} for type variation {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }
}
