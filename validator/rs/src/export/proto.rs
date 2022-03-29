// SPDX-License-Identifier: Apache-2.0

//! This module provides an export format based on protobuf, to represent the
//! output tree as accurately as possible.
//!
//! This is primarily intended to be used to cross programming language
//! boundaries for the validator output, whenever the simplified formats are
//! not comprehensive enough. The Python bindings specifically make extensive
//! use of this.

use crate::input::proto::substrait::validator;
use crate::output::comment;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::parse_result;
use crate::output::path;
use crate::output::primitive_data;
use crate::output::tree;
use prost::Message;

impl From<&parse_result::ParseResult> for validator::ParseResult {
    fn from(result: &parse_result::ParseResult) -> Self {
        Self {
            root: Some((&result.root).into()),
        }
    }
}

impl From<&tree::Node> for validator::Node {
    fn from(node: &tree::Node) -> Self {
        Self {
            node_type: Some((&node.node_type).into()),
            class: (&node.class).into(),
            brief: node.brief.as_ref().map(|x| x.into()),
            summary: node.summary.as_ref().map(|x| x.into()),
            data_type: node.data_type.as_ref().map(|x| x.as_ref().into()),
            data: node.data.iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&tree::Class> for i32 {
    fn from(class: &tree::Class) -> Self {
        match class {
            tree::Class::Misc => validator::node::Class::Unspecified,
            tree::Class::Type => validator::node::Class::Type,
            tree::Class::Expression => validator::node::Class::Expression,
            tree::Class::Relation => validator::node::Class::Relation,
        }
        .into()
    }
}

impl From<&tree::NodeData> for validator::node::Data {
    fn from(node: &tree::NodeData) -> Self {
        Self {
            kind: Some(match node {
                tree::NodeData::Child(child) => validator::node::data::Kind::Child(child.into()),
                tree::NodeData::Diagnostic(diagnostic) => {
                    validator::node::data::Kind::Diagnostic(diagnostic.into())
                }
                tree::NodeData::DataType(data_type) => {
                    validator::node::data::Kind::DataType(data_type.as_ref().into())
                }
                tree::NodeData::Comment(comment) => {
                    validator::node::data::Kind::Comment(comment.into())
                }
            }),
        }
    }
}

impl From<&tree::Child> for validator::node::Child {
    fn from(node: &tree::Child) -> Self {
        Self {
            path: Some((&node.path_element).into()),
            node: Some(node.node.as_ref().into()),
            recognized: node.recognized,
        }
    }
}

impl From<&diagnostic::Diagnostic> for validator::Diagnostic {
    fn from(node: &diagnostic::Diagnostic) -> Self {
        Self {
            original_level: (&node.original_level).into(),
            adjusted_level: (&node.adjusted_level).into(),
            cause: node.cause.classification.into(),
            msg: node.cause.to_string(),
            path: Some((&node.path).into()),
        }
    }
}

impl From<&diagnostic::Level> for i32 {
    fn from(node: &diagnostic::Level) -> Self {
        match node {
            diagnostic::Level::Error => validator::diagnostic::Level::Error,
            diagnostic::Level::Warning => validator::diagnostic::Level::Warning,
            diagnostic::Level::Info => validator::diagnostic::Level::Info,
        }
        .into()
    }
}

impl From<&comment::Comment> for validator::Comment {
    fn from(node: &comment::Comment) -> Self {
        Self {
            elements: node.elements().iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&comment::Brief> for validator::Comment {
    fn from(node: &comment::Brief) -> Self {
        Self {
            elements: node
                .spans()
                .iter()
                .map(|x| validator::comment::Element {
                    kind: Some(validator::comment::element::Kind::Span(x.into())),
                })
                .collect(),
        }
    }
}

impl From<&comment::Element> for validator::comment::Element {
    fn from(node: &comment::Element) -> Self {
        validator::comment::Element {
            kind: Some(match node {
                comment::Element::Span(span) => {
                    validator::comment::element::Kind::Span(span.into())
                }
                comment::Element::NewLine => {
                    validator::comment::element::Kind::NewLine(validator::Empty {})
                }
                comment::Element::ListOpen => {
                    validator::comment::element::Kind::ListOpen(validator::Empty {})
                }
                comment::Element::ListNext => {
                    validator::comment::element::Kind::ListNext(validator::Empty {})
                }
                comment::Element::ListClose => {
                    validator::comment::element::Kind::ListClose(validator::Empty {})
                }
            }),
        }
    }
}

impl From<&comment::Span> for validator::comment::Span {
    fn from(node: &comment::Span) -> Self {
        Self {
            text: node.text.to_string(),
            link: node.link.as_ref().map(|x| x.into()),
        }
    }
}

impl From<&comment::Link> for validator::comment::span::Link {
    fn from(node: &comment::Link) -> Self {
        match node {
            comment::Link::Path(path) => validator::comment::span::Link::Path(path.into()),
            comment::Link::Url(url) => validator::comment::span::Link::Url(url.into()),
        }
    }
}

impl From<&tree::NodeType> for validator::node::NodeType {
    fn from(node: &tree::NodeType) -> Self {
        match node {
            tree::NodeType::ProtoMessage(proto_type) => {
                validator::node::NodeType::ProtoMessage(validator::node::ProtoMessage {
                    path: proto_type.to_string(),
                })
            }
            tree::NodeType::ProtoPrimitive(proto_type, data) => {
                validator::node::NodeType::ProtoPrimitive(validator::node::ProtoPrimitive {
                    path: proto_type.to_string(),
                    data: Some(data.into()),
                })
            }
            tree::NodeType::ProtoMissingOneOf => {
                validator::node::NodeType::ProtoMissingOneof(validator::Empty::default())
            }
            tree::NodeType::NodeReference(anchor, node) => {
                validator::node::NodeType::NodeReference(validator::node::NodeReference {
                    value: *anchor,
                    path: Some((&node.path).into()),
                })
            }
            tree::NodeType::YamlReference(info) => {
                validator::node::NodeType::YamlReference(validator::node::YamlReference {
                    uri: info.uri.name().unwrap_or_default().to_string(),
                })
            }
            tree::NodeType::YamlMap => {
                validator::node::NodeType::YamlMap(validator::Empty::default())
            }
            tree::NodeType::YamlArray => {
                validator::node::NodeType::YamlArray(validator::Empty::default())
            }
            tree::NodeType::YamlPrimitive(data) => {
                validator::node::NodeType::YamlPrimitive(data.into())
            }
        }
    }
}

impl From<&primitive_data::PrimitiveData> for validator::node::PrimitiveData {
    fn from(node: &primitive_data::PrimitiveData) -> Self {
        Self {
            data: match node {
                primitive_data::PrimitiveData::Null => None,
                primitive_data::PrimitiveData::Bool(x) => {
                    Some(validator::node::primitive_data::Data::Boolean(*x))
                }
                primitive_data::PrimitiveData::Unsigned(x) => {
                    Some(validator::node::primitive_data::Data::Unsigned(*x))
                }
                primitive_data::PrimitiveData::Signed(x) => {
                    Some(validator::node::primitive_data::Data::Signed(*x))
                }
                primitive_data::PrimitiveData::Float(x) => {
                    Some(validator::node::primitive_data::Data::Real(*x))
                }
                primitive_data::PrimitiveData::String(x) => Some(
                    validator::node::primitive_data::Data::Unicode(x.to_string()),
                ),
                primitive_data::PrimitiveData::Bytes(x) => {
                    Some(validator::node::primitive_data::Data::Binary(x.clone()))
                }
                primitive_data::PrimitiveData::Enum(x) => Some(
                    validator::node::primitive_data::Data::Variant(x.to_string()),
                ),
                primitive_data::PrimitiveData::Any(x) => {
                    Some(validator::node::primitive_data::Data::Any(x.clone()))
                }
            },
        }
    }
}

impl From<&path::PathBuf> for validator::Path {
    fn from(node: &path::PathBuf) -> Self {
        Self {
            root: node.root.to_string(),
            elements: node.elements.iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&path::PathElement> for validator::path::Element {
    fn from(node: &path::PathElement) -> Self {
        Self {
            kind: Some(match node {
                path::PathElement::Field(field) => {
                    validator::path::element::Kind::Field(validator::path::Field {
                        field: field.to_string(),
                    })
                }
                path::PathElement::Repeated(field, index) => {
                    validator::path::element::Kind::RepeatedField(validator::path::RepeatedField {
                        field: field.to_string(),
                        index: (*index).try_into().unwrap(),
                    })
                }
                path::PathElement::Variant(field, variant) => {
                    validator::path::element::Kind::OneofField(validator::path::OneOfField {
                        field: field.to_string(),
                        variant: variant.to_string(),
                    })
                }
                path::PathElement::Index(index) => {
                    validator::path::element::Kind::ArrayElement(validator::path::ArrayElement {
                        index: (*index).try_into().unwrap(),
                    })
                }
            }),
        }
    }
}

impl From<&data_type::DataType> for validator::DataType {
    fn from(node: &data_type::DataType) -> Self {
        Self {
            class: Some(node.class().into()),
            nullable: node.nullable(),
            variation: node.variation().as_ref().map(|x| x.as_ref().into()),
            parameters: node.parameters().iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&data_type::Class> for validator::data_type::Class {
    fn from(node: &data_type::Class) -> Self {
        validator::data_type::Class {
            kind: Some(match node {
                data_type::Class::Simple(simple) => {
                    validator::data_type::class::Kind::Simple(simple.into())
                }
                data_type::Class::Compound(compound) => {
                    validator::data_type::class::Kind::Compound(compound.into())
                }
                data_type::Class::UserDefined(user_defined) => {
                    validator::data_type::class::Kind::UserDefinedType(user_defined.as_ref().into())
                }
                data_type::Class::Unresolved => {
                    validator::data_type::class::Kind::UnresolvedType(validator::Empty {})
                }
            }),
        }
    }
}

impl From<&data_type::Simple> for i32 {
    fn from(node: &data_type::Simple) -> Self {
        match node {
            data_type::Simple::Boolean => validator::data_type::Simple::Boolean,
            data_type::Simple::I8 => validator::data_type::Simple::I8,
            data_type::Simple::I16 => validator::data_type::Simple::I16,
            data_type::Simple::I32 => validator::data_type::Simple::I32,
            data_type::Simple::I64 => validator::data_type::Simple::I64,
            data_type::Simple::Fp32 => validator::data_type::Simple::Fp32,
            data_type::Simple::Fp64 => validator::data_type::Simple::Fp64,
            data_type::Simple::String => validator::data_type::Simple::String,
            data_type::Simple::Binary => validator::data_type::Simple::Binary,
            data_type::Simple::Timestamp => validator::data_type::Simple::Timestamp,
            data_type::Simple::TimestampTz => validator::data_type::Simple::TimestampTz,
            data_type::Simple::Date => validator::data_type::Simple::Date,
            data_type::Simple::Time => validator::data_type::Simple::Time,
            data_type::Simple::IntervalYear => validator::data_type::Simple::IntervalYear,
            data_type::Simple::IntervalDay => validator::data_type::Simple::IntervalDay,
            data_type::Simple::Uuid => validator::data_type::Simple::Uuid,
        }
        .into()
    }
}

impl From<&data_type::Compound> for i32 {
    fn from(node: &data_type::Compound) -> Self {
        match node {
            data_type::Compound::FixedChar => validator::data_type::Compound::FixedChar,
            data_type::Compound::VarChar => validator::data_type::Compound::VarChar,
            data_type::Compound::FixedBinary => validator::data_type::Compound::FixedBinary,
            data_type::Compound::Decimal => validator::data_type::Compound::Decimal,
            data_type::Compound::Struct => validator::data_type::Compound::Struct,
            data_type::Compound::NamedStruct => validator::data_type::Compound::NamedStruct,
            data_type::Compound::List => validator::data_type::Compound::List,
            data_type::Compound::Map => validator::data_type::Compound::Map,
        }
        .into()
    }
}

impl From<&extension::Reference<extension::DataType>> for validator::data_type::UserDefinedType {
    fn from(node: &extension::Reference<extension::DataType>) -> Self {
        Self {
            uri: node.uri.name().unwrap_or_default().to_string(),
            name: node.name.name().unwrap_or_default().to_string(),
            definition: node.definition.as_ref().map(|x| x.as_ref().into()),
        }
    }
}

impl From<&extension::DataType> for validator::data_type::user_defined_type::Definition {
    fn from(node: &extension::DataType) -> Self {
        Self {
            structure: node
                .structure
                .iter()
                .map(
                    |(name, simple)| validator::data_type::user_defined_type::Element {
                        name: name.to_string(),
                        kind: simple.into(),
                    },
                )
                .collect(),
        }
    }
}

impl From<&extension::Reference<extension::TypeVariation>> for validator::data_type::Variation {
    fn from(node: &extension::Reference<extension::TypeVariation>) -> Self {
        if let Some(ref definition) = node.definition {
            validator::data_type::Variation::UserDefinedVariation(
                validator::data_type::UserDefinedVariation {
                    uri: node.uri.name().unwrap_or_default().to_string(),
                    name: node.name.name().unwrap_or_default().to_string(),
                    definition: Some(Box::new(definition.as_ref().into())),
                },
            )
        } else {
            validator::data_type::Variation::UnresolvedVariation(validator::Empty {})
        }
    }
}

impl From<&extension::TypeVariation> for validator::data_type::user_defined_variation::Definition {
    fn from(node: &extension::TypeVariation) -> Self {
        Self {
            base_type: None,
            function_behavior: (&node.function_behavior).into(),
        }
    }
}

impl From<&extension::FunctionBehavior> for i32 {
    fn from(node: &extension::FunctionBehavior) -> Self {
        match node {
            extension::FunctionBehavior::Inherits => {
                validator::data_type::user_defined_variation::FunctionBehavior::Inherits
            }
            extension::FunctionBehavior::Separate => {
                validator::data_type::user_defined_variation::FunctionBehavior::Separate
            }
        }
        .into()
    }
}

impl From<&data_type::Parameter> for validator::data_type::Parameter {
    fn from(node: &data_type::Parameter) -> Self {
        Self {
            kind: Some(match node {
                data_type::Parameter::Type(data_type) => {
                    validator::data_type::parameter::Kind::DataType(data_type.as_ref().into())
                }
                data_type::Parameter::NamedType(name, data_type) => {
                    validator::data_type::parameter::Kind::NamedType(validator::data_type::Named {
                        name: name.to_string(),
                        data_type: Some(data_type.as_ref().into()),
                    })
                }
                data_type::Parameter::Unsigned(unsigned) => {
                    validator::data_type::parameter::Kind::Unsigned(*unsigned)
                }
            }),
        }
    }
}

/// Export the complete parse tree in protobuf substrait.validator.Node format.
pub fn export<T: std::io::Write>(
    out: &mut T,
    _root_name: &'static str,
    result: &parse_result::ParseResult,
) -> std::io::Result<()> {
    let root = validator::ParseResult::from(result);
    let buf = root.encode_to_vec();
    if out.write(&buf)? < buf.len() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to write all bytes",
        ))
    } else {
        Ok(())
    }
}
