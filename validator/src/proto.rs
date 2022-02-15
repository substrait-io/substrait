pub mod meta {
    use crate::context;
    use crate::doc_tree;

    #[derive(Clone, Debug, PartialEq)]
    pub enum ProtoPrimitiveData {
        /// Used for booleans.
        Bool(bool),

        /// Used for unsigned integers.
        Unsigned(u64),

        /// Used for signed integers.
        Signed(i64),

        /// Used for floating-point values.
        Float(f64),

        /// Used for UTF-8 strings.
        String(String),

        /// Used for bytestrings.
        Bytes(Vec<u8>),

        /// Used for enumerations.
        Enum(&'static str),

        /// Used for Any messages.
        Any(prost_types::Any),
    }

    /// Trait for all Rust types that represent protobuf types.
    pub trait ProtoDatum {
        /// Creates an empty doctree node for a protobuf datum of this type.
        ///
        /// For primitive types, this fills the value with protobuf's default.
        fn proto_type_to_node() -> doc_tree::Node;

        /// Creates an empty doctree node for a protobuf datum with this value.
        fn proto_data_to_node(&self) -> doc_tree::Node;

        /// Returns the name of the selected variant of a oneof field, if this
        /// is a rust enum used to represent a oneof field.
        fn proto_data_variant(&self) -> Option<&'static str>;

        /// Complete the subtrees of this datum in output that have not already
        /// been parsed using UnknownField nodes. Returns if any such nodes
        /// were added.
        fn proto_parse_unknown(
            &self,
            breadcrumb: &mut context::Breadcrumb,
            output: &mut doc_tree::Node,
        ) -> bool;
    }

    /// Trait for all Rust types that represent protobuf messages. These are
    /// always structs for which all fields implement ProtoDatum.
    pub trait ProtoMessage: ProtoDatum {
        /// Returns the protobuf type name for messages of this type.
        fn proto_message_type() -> &'static str;
    }

    /// Trait for all Rust types that represent protobuf's oneof abstraction.
    /// In the world of protobuf, these aren't really a thing of their own, but
    /// in Rust, they are defined as enums, each variant containing a one-tuple
    /// of some type implementing ProtoDatum.
    pub trait ProtoOneOf: ProtoDatum {
        /// Returns the name of the selected variant of a oneof field.
        fn proto_one_of_variant(&self) -> &'static str;
    }

    /// Trait for Rust types that map to the protobuf primitive types.
    pub trait ProtoPrimitive: ProtoDatum {
        /// Returns the protobuf type name for primitives of this type.
        fn proto_primitive_type() -> &'static str;

        /// Returns the protobuf-specified default value for this primitive
        /// data type.
        fn proto_primitive_default() -> ProtoPrimitiveData;

        /// Returns the actual value for this primitive data type as a
        /// ProtoPrimitiveData variant.
        fn proto_primitive_data(&self) -> ProtoPrimitiveData;

        /// Returns whether this is the default value of the primitive.
        fn proto_primitive_is_default(&self) -> bool;
    }

    /// Trait for all Rust types that represent protobuf enums. These are
    /// always represented as a Rust enum with no contained values for any of
    /// the variants.
    pub trait ProtoEnum: ProtoPrimitive {
        /// Returns the protobuf type name for enums of this type.
        fn proto_enum_type() -> &'static str;

        /// Returns the name of the default variant of an enum.
        fn proto_enum_default_variant() -> &'static str;

        /// Returns the name of the selected variant of an enum.
        fn proto_enum_variant(&self) -> &'static str;
    }

    /// Blanket implementation to make all protobuf enums behave like
    /// primitives as well.
    impl<T: ProtoEnum> ProtoPrimitive for T {
        fn proto_primitive_type() -> &'static str {
            T::proto_enum_type()
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Enum(T::proto_enum_default_variant())
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Enum(self.proto_enum_variant())
        }

        fn proto_primitive_is_default(&self) -> bool {
            self.proto_enum_variant() == T::proto_enum_default_variant()
        }
    }

    /// Blanket implementation to make all protobuf primitives behave like
    /// generic protobuf datums.
    ///
    /// Note: if Rust would allow it, we could define blanket implementations
    /// for ProtoMessage and ProtoOneOf as well, since they're always the same.
    /// Unfortunately, we can only define a single blanket implementation, so
    /// we opt for the one that isn't already generated via derive macros.
    impl<T: ProtoPrimitive> ProtoDatum for T {
        fn proto_type_to_node() -> doc_tree::Node {
            doc_tree::NodeType::ProtoPrimitive(
                T::proto_primitive_type(),
                T::proto_primitive_default(),
            )
            .into()
        }

        fn proto_data_to_node(&self) -> doc_tree::Node {
            doc_tree::NodeType::ProtoPrimitive(
                T::proto_primitive_type(),
                self.proto_primitive_data(),
            )
            .into()
        }

        fn proto_data_variant(&self) -> Option<&'static str> {
            None
        }

        fn proto_parse_unknown(
            &self,
            _breadcrumb: &mut context::Breadcrumb,
            _output: &mut doc_tree::Node,
        ) -> bool {
            false
        }
    }

    impl ProtoPrimitive for bool {
        fn proto_primitive_type() -> &'static str {
            "bool"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Bool(false)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Bool(*self)
        }

        fn proto_primitive_is_default(&self) -> bool {
            !*self
        }
    }

    impl ProtoPrimitive for u32 {
        fn proto_primitive_type() -> &'static str {
            "uint32"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Unsigned(0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Unsigned((*self).into())
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0
        }
    }

    impl ProtoPrimitive for u64 {
        fn proto_primitive_type() -> &'static str {
            "uint64"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Unsigned(0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Unsigned(*self)
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0
        }
    }

    impl ProtoPrimitive for i32 {
        fn proto_primitive_type() -> &'static str {
            "int32"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Signed(0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Signed((*self).into())
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0
        }
    }

    impl ProtoPrimitive for i64 {
        fn proto_primitive_type() -> &'static str {
            "int64"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Signed(0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Signed(*self)
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0
        }
    }

    impl ProtoPrimitive for f32 {
        fn proto_primitive_type() -> &'static str {
            "float"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Float(0.0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Float((*self).into())
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0.0
        }
    }

    impl ProtoPrimitive for f64 {
        fn proto_primitive_type() -> &'static str {
            "double"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Float(0.0)
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Float(*self)
        }

        fn proto_primitive_is_default(&self) -> bool {
            *self != 0.0
        }
    }

    impl ProtoPrimitive for String {
        fn proto_primitive_type() -> &'static str {
            "string"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::String(String::new())
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::String(self.clone())
        }

        fn proto_primitive_is_default(&self) -> bool {
            self.is_empty()
        }
    }

    impl ProtoPrimitive for Vec<u8> {
        fn proto_primitive_type() -> &'static str {
            "bytes"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Bytes(vec![])
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Bytes(self.clone())
        }

        fn proto_primitive_is_default(&self) -> bool {
            self.is_empty()
        }
    }

    impl ProtoPrimitive for prost_types::Any {
        fn proto_primitive_type() -> &'static str {
            "any"
        }

        fn proto_primitive_default() -> ProtoPrimitiveData {
            ProtoPrimitiveData::Any(prost_types::Any::default())
        }

        fn proto_primitive_data(&self) -> ProtoPrimitiveData {
            ProtoPrimitiveData::Any(self.clone())
        }

        fn proto_primitive_is_default(&self) -> bool {
            self.type_url.is_empty()
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub mod substrait {
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
}

#[cfg(test)]
mod tests {
    use super::meta::*;
    use super::*;
    use crate::doc_tree;

    #[test]
    fn message() {
        assert_eq!(substrait::Plan::proto_message_type(), "substrait.Plan");
        assert_eq!(
            substrait::Plan::proto_type_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoMessage("substrait.Plan"),
                data_type: None,
                data: vec![],
            }
        );

        let msg = substrait::Plan::default();
        assert_eq!(
            msg.proto_data_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoMessage("substrait.Plan"),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(msg.proto_data_variant(), None);
    }

    #[test]
    fn oneof() {
        assert_eq!(
            substrait::plan_rel::RelType::proto_type_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoMissingOneOf,
                data_type: None,
                data: vec![],
            }
        );

        let oneof = substrait::plan_rel::RelType::Rel(substrait::Rel::default());
        assert_eq!(oneof.proto_one_of_variant(), "rel");
        assert_eq!(
            oneof.proto_data_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoMessage("substrait.Rel"),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(oneof.proto_data_variant(), Some("rel"));
    }

    #[test]
    fn enumeration() {
        assert_eq!(
            substrait::AggregationPhase::proto_enum_type(),
            "substrait.AggregationPhase"
        );
        assert_eq!(
            substrait::AggregationPhase::proto_enum_default_variant(),
            "AGGREGATION_PHASE_UNSPECIFIED"
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.proto_enum_variant(),
            "AGGREGATION_PHASE_UNSPECIFIED"
        );

        assert_eq!(
            substrait::AggregationPhase::proto_primitive_type(),
            "substrait.AggregationPhase"
        );
        assert_eq!(
            substrait::AggregationPhase::proto_primitive_default(),
            ProtoPrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.proto_primitive_data(),
            ProtoPrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
        );

        assert_eq!(
            substrait::AggregationPhase::proto_type_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoPrimitive(
                    "substrait.AggregationPhase",
                    ProtoPrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.proto_data_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoPrimitive(
                    "substrait.AggregationPhase",
                    ProtoPrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.proto_data_variant(),
            None
        );
    }

    #[test]
    fn primitive() {
        assert_eq!(u32::proto_primitive_type(), "uint32");
        assert_eq!(
            u32::proto_primitive_default(),
            ProtoPrimitiveData::Unsigned(0)
        );
        assert_eq!(
            42u32.proto_primitive_data(),
            ProtoPrimitiveData::Unsigned(42)
        );

        assert_eq!(
            u32::proto_type_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoPrimitive(
                    "uint32",
                    ProtoPrimitiveData::Unsigned(0)
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            42u32.proto_data_to_node(),
            doc_tree::Node {
                node_type: doc_tree::NodeType::ProtoPrimitive(
                    "uint32",
                    ProtoPrimitiveData::Unsigned(42)
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(42u32.proto_data_variant(), None);
    }
}
