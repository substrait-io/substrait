use crate::context;
use crate::diagnostic;
use crate::diagnostic::Result;
use crate::extension;
use crate::proto;
use crate::tree;
use std::collections::HashMap;
use std::sync::Arc;

/// "Parse" an anchor. This just reports an error if the anchor is 0.
fn parse_anchor(x: &u32, _y: &mut context::Context) -> Result<u32> {
    if *x == 0 {
        Err(diagnostic::Cause::IllegalValue(
            "anchor 0 is reserved to disambiguate unspecified optional references".to_string(),
        ))
    } else {
        Ok(*x)
    }
}

/// Parse a YAML extension URI string.
fn parse_uri<S: AsRef<str>>(x: &S, y: &mut context::Context) -> Result<Arc<extension::YamlInfo>> {
    let x = x.as_ref();

    // Construct the YAML data object.
    let yaml_data = Arc::new(extension::YamlInfo {
        uri: x.to_string(),
        anchor_path: y.breadcrumb.parent.map(|x| x.path.to_path_buf()),
        data: None,
    });

    // The data field in the above struct should be set to the parse result of
    // the YAML file if it is resolved and parses. But that's not implemented
    // yet, so report a warning.
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "extension YAML resolution and parsing is not yet implemented"
    );

    // The node type will have been set as if this is a normal string
    // primitive. We want extra information though, namely the contents of the
    // YAML file. So we change the node type.
    y.output.node_type = tree::NodeType::YamlData(yaml_data.clone());

    Ok(yaml_data)
}

/// Parse a mapping from a URI anchor to a YAML extension.
fn parse_extension_uri_mapping(
    x: &proto::substrait::extensions::SimpleExtensionUri,
    y: &mut context::Context,
) -> Result<()> {
    // Parse the fields.
    let anchor = proto_primitive_field!(x, y, extension_uri_anchor, parse_anchor).1;
    let yaml_data = proto_primitive_field!(x, y, uri, parse_uri).1.unwrap();

    // If the specified anchor is valid, insert a mapping for it.
    if let Some(anchor) = anchor {
        if let Some(prev_data) = y.state.uris.insert(anchor, yaml_data) {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "anchor {} is already in use for URI {}",
                anchor,
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
        None => Err(diagnostic::Cause::MissingAnchor(format!(
            "URI anchor {} does not exist",
            uri_reference
        ))),
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
            let data_type = yaml_info.as_ref().map(|yaml_info| {
                yaml_info.data.as_ref().map(|data| {
                    let data_type = data.types.get(&name.to_lowercase()).cloned();
                    if data_type.is_none() {
                        diagnostic!(y, Error, NameResolutionFailed, "failed to resolve data type {:?} in {}", name, yaml_info);
                    }
                    data_type
                }).flatten()
            }).flatten();

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
                        "anchor {} is already in use for data type {}",
                        anchor,
                        prev_data
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
            let type_variation = yaml_info.as_ref().map(|yaml_info| {
                yaml_info.data.as_ref().map(|data| {
                    let type_variation = data.type_variations.get(&name.to_lowercase()).cloned();
                    if type_variation.is_none() {
                        diagnostic!(y, Error, NameResolutionFailed, "failed to resolve type variation {:?} in {}", name, yaml_info);
                    }
                    type_variation
                }).flatten()
            }).flatten();

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
                        "anchor {} is already in use for type variation {}",
                        anchor,
                        prev_data
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
            let function = yaml_info.as_ref().map(|yaml_info| {
                yaml_info.data.as_ref().map(|data| {
                    let function = data.functions.get(&name.to_lowercase()).cloned();
                    if function.is_none() {
                        diagnostic!(y, Error, NameResolutionFailed, "failed to resolve function {:?} in {}", name, yaml_info);
                    }
                    function
                }).flatten()
            }).flatten();

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
                        "anchor {} is already in use for function {}",
                        anchor,
                        prev_data
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

/// Resolves a protobuf "any" message. Returns whether it has been whitelisted
/// for validation.
fn resolve_any(x: &prost_types::Any, y: &mut context::Context) -> bool {
    y.state
        .pending_proto_url_dependencies
        .entry(x.type_url.clone())
        .or_insert_with(|| y.breadcrumb.path.to_path_buf());
    y.config.whitelisted_any_urls.contains(&x.type_url)
}

/// Parse a protobuf "any" message that consumers may ignore.
fn parse_hint_any(x: &prost_types::Any, y: &mut context::Context) -> Result<()> {
    if resolve_any(x, y) {
        diagnostic!(y, Info, ProtoAny, "whitelisted hint of type {}", x.type_url);
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
            "whitelisted enhancement of type {}",
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
            RedundantProtoAnyDeclaration,
            "message type {} redeclared",
            x
        );
        link!(y, path, "previous declaration was here");
    } else {
        diagnostic!(
            y,
            Info,
            RedundantProtoAnyDeclaration,
            "message type {} is never used",
            x
        );
    }
    Ok(())
}

/// Parses the extension information in a plan that needs to be parsed *before*
/// the relations are parsed.
pub fn parse_extensions_before_relations(x: &proto::substrait::Plan, y: &mut context::Context) {
    proto_repeated_field!(x, y, extension_uris, parse_extension_uri_mapping);
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
        diagnostic!(y, Error, MissingProtoAnyDeclaration, "{}", url);
        link!(y, path, "message type is first used here");
    }
}
