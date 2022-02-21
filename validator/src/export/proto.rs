use crate::comment;
use crate::data_type;
use crate::diagnostic;
use crate::doc_tree;
use crate::extension;
use crate::path;
use crate::proto::meta;
use crate::proto::substrait::validator;
use prost::Message;

impl From<&doc_tree::Node> for validator::Node {
    fn from(node: &doc_tree::Node) -> Self {
        Self {
            node_type: Some((&node.node_type).into()),
            data_type: node.data_type.as_ref().map(|x| x.into()),
            relation: None, // TODO, doesn't exist in doc_tree yet
            data: node.data.iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&doc_tree::NodeData> for validator::node::Data {
    fn from(node: &doc_tree::NodeData) -> Self {
        Self {
            kind: Some(match node {
                doc_tree::NodeData::Child(child) => {
                    validator::node::data::Kind::Child(child.into())
                }
                doc_tree::NodeData::Diagnostic(diagnostic) => {
                    validator::node::data::Kind::Diagnostic(diagnostic.into())
                }
                doc_tree::NodeData::DataType(data_type) => {
                    validator::node::data::Kind::DataType(data_type.into())
                }
                doc_tree::NodeData::Comment(comment) => {
                    validator::node::data::Kind::Comment(comment.into())
                }
            }),
        }
    }
}

impl From<&doc_tree::Child> for validator::node::Child {
    fn from(node: &doc_tree::Child) -> Self {
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
            level: (&node.level).into(),
            cause: validator::diagnostic::Cause::Unspecified.into(),
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
            spans: node.spans.iter().map(|x| x.into()).collect(),
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

impl From<&doc_tree::NodeType> for validator::node::NodeType {
    fn from(node: &doc_tree::NodeType) -> Self {
        match node {
            doc_tree::NodeType::ProtoMessage(proto_type) => {
                validator::node::NodeType::ProtoMessage(validator::node::ProtoMessage {
                    path: proto_type.to_string(),
                })
            }
            doc_tree::NodeType::ProtoPrimitive(proto_type, data) => {
                validator::node::NodeType::ProtoPrimitive(validator::node::ProtoPrimitive {
                    path: proto_type.to_string(),
                    data: Some(data.into()),
                })
            }
            doc_tree::NodeType::ProtoMissingOneOf => panic!("found ProtoMissingOneOf in tree!"),
            doc_tree::NodeType::Reference(anchor, node) => {
                validator::node::NodeType::NodeReference(validator::node::NodeReference {
                    value: *anchor,
                    path: Some((&node.path).into()),
                })
            }
            doc_tree::NodeType::YamlData(info) => {
                validator::node::NodeType::YamlReference(validator::node::YamlReference {
                    uri: info.uri.clone(),
                })
            }
            doc_tree::NodeType::YamlMap => {
                validator::node::NodeType::YamlMap(validator::Empty::default())
            }
            doc_tree::NodeType::YamlArray => {
                validator::node::NodeType::YamlArray(validator::Empty::default())
            }
            doc_tree::NodeType::YamlPrimitive(data) => {
                validator::node::NodeType::YamlPrimitive(data.into())
            }
        }
    }
}

impl From<&meta::ProtoPrimitiveData> for validator::node::PrimitiveData {
    fn from(node: &meta::ProtoPrimitiveData) -> Self {
        Self {
            data: Some(match node {
                meta::ProtoPrimitiveData::Bool(x) => {
                    validator::node::primitive_data::Data::Boolean(*x)
                }
                meta::ProtoPrimitiveData::Unsigned(x) => {
                    validator::node::primitive_data::Data::Unsigned(*x)
                }
                meta::ProtoPrimitiveData::Signed(x) => {
                    validator::node::primitive_data::Data::Signed(*x)
                }
                meta::ProtoPrimitiveData::Float(x) => {
                    validator::node::primitive_data::Data::Real(*x)
                }
                meta::ProtoPrimitiveData::String(x) => {
                    validator::node::primitive_data::Data::Unicode(x.to_string())
                }
                meta::ProtoPrimitiveData::Bytes(x) => {
                    validator::node::primitive_data::Data::Binary(x.clone())
                }
                meta::ProtoPrimitiveData::Enum(x) => {
                    validator::node::primitive_data::Data::Variant(x.to_string())
                }
                meta::ProtoPrimitiveData::Any(x) => {
                    validator::node::primitive_data::Data::Any(x.clone())
                }
            }),
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
            kind: Some((&node.class).into()),
            nullable: node.nullable,
            variation: node.variation.as_ref().map(|x| x.as_ref().into()),
            parameters: node.parameters.iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<&data_type::Class> for validator::data_type::Kind {
    fn from(node: &data_type::Class) -> Self {
        match node {
            data_type::Class::Simple(simple) => validator::data_type::Kind::Simple(simple.into()),
            data_type::Class::Compound(compound) => {
                validator::data_type::Kind::Compound(compound.into())
            }
            data_type::Class::UserDefined(user_defined) => {
                validator::data_type::Kind::UserDefinedType(user_defined.as_ref().into())
            }
            data_type::Class::Unresolved(description) => {
                validator::data_type::Kind::UnresolvedType(validator::data_type::Unresolved {
                    description: description.to_string(),
                })
            }
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
            uri: node
                .common
                .yaml_info
                .as_ref()
                .map(|x| x.uri.clone())
                .unwrap_or_default(),
            name: node.common.name.to_string(),
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
                    uri: node
                        .common
                        .yaml_info
                        .as_ref()
                        .map(|x| x.uri.clone())
                        .unwrap_or_default(),
                    name: node.common.name.to_string(),
                    definition: Some(definition.as_ref().into()),
                },
            )
        } else {
            validator::data_type::Variation::UnresolvedVariation(validator::data_type::Unresolved {
                description: node.common.name.to_string(),
            })
        }
    }
}

impl From<&extension::TypeVariation> for validator::data_type::user_defined_variation::Definition {
    fn from(node: &extension::TypeVariation) -> Self {
        Self {
            function_behavior: (&node.behavior).into(),
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
                    validator::data_type::parameter::Kind::DataType(data_type.into())
                }
                data_type::Parameter::NamedType(name, data_type) => {
                    validator::data_type::parameter::Kind::NamedType(validator::data_type::Named {
                        name: name.to_string(),
                        data_type: Some(data_type.into()),
                    })
                }
                data_type::Parameter::Unsigned(unsigned) => {
                    validator::data_type::parameter::Kind::Unsigned(*unsigned)
                }
            }),
        }
    }
}

pub fn export<T: std::io::Write>(
    out: &mut T,
    _root_name: &'static str,
    root: &doc_tree::Node,
) -> std::io::Result<()> {
    let root = validator::Node::from(root);
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
