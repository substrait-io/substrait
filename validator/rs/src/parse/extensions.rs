// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions relating to extensions.

use crate::input::proto;
use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::path;
use crate::parse::context;
use crate::parse::traversal;
use std::collections::HashMap;
use std::sync::Arc;

/// Toplevel parse function for a simple extension YAML file.
fn parse_simple_extension_yaml(_x: &yaml::Value, _y: &mut context::Context) -> Result<()> {
    // TODO
    Ok(())
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

/// Parse a YAML extension URI string.
fn parse_simple_extension_yaml_uri<S: AsRef<str>>(
    x: &S,
    y: &mut context::Context,
) -> Result<Arc<extension::YamlInfo>> {
    // The schema for YAML extension files.
    static SCHEMA: once_cell::sync::Lazy<jsonschema::JSONSchema> =
        once_cell::sync::Lazy::new(|| {
            jsonschema::JSONSchema::compile(
                &yaml::yaml_to_json(
                    yaml_rust::YamlLoader::load_from_str(include_str!(
                        "../../../../text/simple_extensions_schema.yaml"
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

    Ok(traversal::parse_yaml(
        x,
        y,
        Some(&SCHEMA),
        parse_simple_extension_yaml,
    ))
}

/// Parse a mapping from a URI anchor to a YAML extension.
fn parse_simple_extension_yaml_uri_mapping(
    x: &proto::substrait::extensions::SimpleExtensionUri,
    y: &mut context::Context,
) -> Result<()> {
    // Parse the fields.
    let anchor = proto_primitive_field!(x, y, extension_uri_anchor, parse_anchor).1;
    let yaml_data = proto_primitive_field!(x, y, uri, parse_simple_extension_yaml_uri)
        .1
        .unwrap();

    // If the specified anchor is valid, insert a mapping for it.
    if let Some(anchor) = anchor {
        if let Some(prev_data) = y.state.uris.insert(anchor, yaml_data) {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "anchor {anchor} is already in use for URI {}",
                prev_data.uri
            );
            if let Some(ref path) = prev_data.anchor_path {
                link!(y, path.clone(), "previous definition was here");
            }
        }
    }

    Ok(())
}

/// Parse an URI reference and resolve it.
fn parse_uri_reference(
    uri_reference: &u32,
    y: &mut context::Context,
) -> Result<Arc<extension::YamlInfo>> {
    match y.state.uris.get(uri_reference).cloned() {
        Some(yaml_data) => {
            if let Some(ref path) = yaml_data.anchor_path {
                link!(y, path.clone(), "URI anchor is defined here");
            }
            Ok(yaml_data)
        }
        None => Err(cause!(
            LinkMissingAnchor,
            "URI anchor {uri_reference} does not exist"
        )),
    }
}

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping_data(
    x: &proto::substrait::extensions::simple_extension_declaration::MappingType,
    y: &mut context::Context,
) -> Result<()> {
    match x {
        proto::substrait::extensions::simple_extension_declaration::MappingType::ExtensionType(x) => {

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
                    anchor_path: Some(y.breadcrumb.path.to_path_buf())
                },
                definition: data_type
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Some(prev_data) = y.state.types.insert(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for data type {prev_data}"
                    );
                    if let Some(ref path) = prev_data.common.anchor_path {
                        link!(y, path.clone(), "previous definition was here");
                    }
                }
            }

        }
        proto::substrait::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(x) => {

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
                    anchor_path: Some(y.breadcrumb.path.to_path_buf())
                },
                definition: type_variation
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Some(prev_data) = y.state.type_variations.insert(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for type variation {prev_data}"
                    );
                    if let Some(ref path) = prev_data.common.anchor_path {
                        link!(y, path.clone(), "previous definition was here");
                    }
                }
            }

        }
        proto::substrait::extensions::simple_extension_declaration::MappingType::ExtensionFunction(x) => {

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
                    anchor_path: Some(y.breadcrumb.path.to_path_buf())
                },
                definition: function
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Some(prev_data) = y.state.functions.insert(anchor, reference) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for function {prev_data}"
                    );
                    if let Some(ref path) = prev_data.common.anchor_path {
                        link!(y, path.clone(), "previous definition was here");
                    }
                }
            }

        }
    };
    Ok(())
}

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping(
    x: &proto::substrait::extensions::SimpleExtensionDeclaration,
    y: &mut context::Context,
) -> Result<()> {
    proto_required_field!(x, y, mapping_type, parse_extension_mapping_data);
    Ok(())
}

/// Resolves a protobuf "any" message. Returns whether the user has explicitly
/// allowed this message type for use.
fn resolve_any(x: &prost_types::Any, y: &mut context::Context) -> bool {
    y.state
        .pending_proto_url_dependencies
        .entry(x.type_url.clone())
        .or_insert_with(|| y.breadcrumb.path.to_path_buf());
    y.config
        .allowed_any_urls
        .iter()
        .any(|p| p.matches(&x.type_url))
}

/// Parse a protobuf "any" message that consumers may ignore.
fn parse_hint_any(x: &prost_types::Any, y: &mut context::Context) -> Result<()> {
    if resolve_any(x, y) {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "explicitly allowed hint of type {}",
            x.type_url
        );
    } else {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "ignoring unknown hint of type {}",
            x.type_url
        );
    }
    Ok(())
}

/// Parse a protobuf "any" message that consumers are not allowed to ignore.
fn parse_functional_any(x: &prost_types::Any, y: &mut context::Context) -> Result<()> {
    if resolve_any(x, y) {
        diagnostic!(
            y,
            Info,
            ProtoAny,
            "explicitly allowed enhancement of type {}",
            x.type_url
        );
    } else {
        diagnostic!(
            y,
            Warning,
            ProtoAny,
            "unknown enhancement of type {}; plan is only valid \
            for consumers recognizing this enhancement",
            x.type_url
        );
    }
    Ok(())
}

/// Parse an advanced extension message (based on protobuf "any" messages).
fn parse_advanced_extension(
    x: &proto::substrait::extensions::AdvancedExtension,
    y: &mut context::Context,
) -> Result<()> {
    proto_field!(x, y, optimization, parse_hint_any);
    proto_field!(x, y, enhancement, parse_functional_any);
    Ok(())
}

/// Parse a protobuf "any" type declaration, after all "any" dependencies have
/// already been gathered.
#[allow(clippy::ptr_arg)]
fn parse_expected_type_url(x: &String, y: &mut context::Context) -> Result<()> {
    if let Some(path) = y.state.pending_proto_url_dependencies.remove(x) {
        link!(y, path, "message type is first used here");
    } else if let Some(path) = y
        .state
        .proto_url_declarations
        .insert(x.clone(), y.breadcrumb.path.to_path_buf())
    {
        diagnostic!(
            y,
            Info,
            ProtoRedundantAnyDeclaration,
            "message type {x} redeclared"
        );
        link!(y, path, "previous declaration was here");
    } else {
        diagnostic!(
            y,
            Info,
            ProtoRedundantAnyDeclaration,
            "message type {x} is never used"
        );
    }
    Ok(())
}

/// Parses the extension information in a plan that needs to be parsed *before*
/// the relations are parsed.
pub fn parse_extensions_before_relations(x: &proto::substrait::Plan, y: &mut context::Context) {
    proto_repeated_field!(
        x,
        y,
        extension_uris,
        parse_simple_extension_yaml_uri_mapping
    );
    proto_repeated_field!(x, y, extensions, parse_extension_mapping);
}

/// Parses the extension information in a plan that needs to be parsed *after*
/// the relations are parsed.
pub fn parse_extensions_after_relations(x: &proto::substrait::Plan, y: &mut context::Context) {
    proto_field!(x, y, advanced_extensions, parse_advanced_extension);
    proto_repeated_field!(x, y, expected_type_urls, parse_expected_type_url);

    // Throw errors if a proto "any" message type is used in the plan, but not
    // declared.
    let mut pending_dependencies = HashMap::new();
    std::mem::swap(
        &mut pending_dependencies,
        &mut y.state.pending_proto_url_dependencies,
    );
    for (url, path) in pending_dependencies.drain() {
        ediagnostic!(y, Error, ProtoMissingAnyDeclaration, url);
        link!(y, path, "message type is first used here");
    }
}
