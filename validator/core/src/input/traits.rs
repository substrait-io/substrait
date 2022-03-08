//! Module providing introspection traits for [`prost`]-generated protobuf
//! types.

use crate::output::primitive_data;
use crate::output::tree;
use crate::parse::context;

/// Trait for all Rust types that represent input tree node types.
pub trait InputNode {
    /// Creates an empty output node for a protobuf datum of this type.
    ///
    /// For primitive types, this fills the value with protobuf's default.
    fn type_to_node() -> tree::Node;

    /// Creates an empty output node for a protobuf datum with this value.
    fn data_to_node(&self) -> tree::Node;

    /// Returns the name of the selected variant of a oneof field, if this
    /// is a rust enum used to represent a oneof field.
    fn oneof_variant(&self) -> Option<&'static str>;

    /// Complete the subtrees of this datum in output that have not already
    /// been parsed using UnknownField nodes. Returns whether any such nodes
    /// were added.
    fn parse_unknown(&self, context: &mut context::Context<'_>) -> bool;
}

/// Trait for all Rust types that represent protobuf messages. These are
/// always structs for which all fields implement InputNode.
pub trait ProtoMessage: InputNode {
    /// Returns the protobuf type name for messages of this type.
    fn proto_message_type() -> &'static str;
}

/// Trait for all Rust types that represent protobuf's oneof abstraction.
/// In the world of protobuf, these aren't really a thing of their own, but
/// in Rust, they are defined as enums, each variant containing a one-tuple
/// of some type implementing InputNode.
pub trait ProtoOneOf: InputNode {
    /// Returns the name of the selected variant of a oneof field.
    fn proto_oneof_variant(&self) -> &'static str;
}

/// Trait for Rust types that map to the protobuf primitive types.
pub trait ProtoPrimitive: InputNode {
    /// Returns the protobuf type name for primitives of this type.
    fn proto_primitive_type() -> &'static str;

    /// Returns the protobuf-specified default value for this primitive
    /// data type.
    fn proto_primitive_default() -> primitive_data::PrimitiveData;

    /// Returns the actual value for this primitive data type as a
    /// ProtoPrimitiveData variant.
    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData;

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

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Enum(T::proto_enum_default_variant())
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Enum(self.proto_enum_variant())
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
impl<T: ProtoPrimitive> InputNode for T {
    fn type_to_node() -> tree::Node {
        tree::NodeType::ProtoPrimitive(T::proto_primitive_type(), T::proto_primitive_default())
            .into()
    }

    fn data_to_node(&self) -> tree::Node {
        tree::NodeType::ProtoPrimitive(T::proto_primitive_type(), self.proto_primitive_data())
            .into()
    }

    fn oneof_variant(&self) -> Option<&'static str> {
        None
    }

    fn parse_unknown(&self, _context: &mut context::Context<'_>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::proto::substrait;
    use crate::output::primitive_data;
    use crate::output::tree;

    #[test]
    fn message() {
        assert_eq!(substrait::Plan::proto_message_type(), "substrait.Plan");
        assert_eq!(
            substrait::Plan::type_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoMessage("substrait.Plan"),
                data_type: None,
                data: vec![],
            }
        );

        let msg = substrait::Plan::default();
        assert_eq!(
            msg.data_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoMessage("substrait.Plan"),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(msg.oneof_variant(), None);
    }

    #[test]
    fn oneof() {
        assert_eq!(
            substrait::plan_rel::RelType::type_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoMissingOneOf,
                data_type: None,
                data: vec![],
            }
        );

        let oneof = substrait::plan_rel::RelType::Rel(substrait::Rel::default());
        assert_eq!(oneof.proto_oneof_variant(), "rel");
        assert_eq!(
            oneof.data_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoMessage("substrait.Rel"),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(oneof.oneof_variant(), Some("rel"));
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
            primitive_data::PrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.proto_primitive_data(),
            primitive_data::PrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
        );

        assert_eq!(
            substrait::AggregationPhase::type_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoPrimitive(
                    "substrait.AggregationPhase",
                    primitive_data::PrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.data_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoPrimitive(
                    "substrait.AggregationPhase",
                    primitive_data::PrimitiveData::Enum("AGGREGATION_PHASE_UNSPECIFIED")
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            substrait::AggregationPhase::Unspecified.oneof_variant(),
            None
        );
    }

    #[test]
    fn primitive() {
        assert_eq!(u32::proto_primitive_type(), "uint32");
        assert_eq!(
            u32::proto_primitive_default(),
            primitive_data::PrimitiveData::Unsigned(0)
        );
        assert_eq!(
            42u32.proto_primitive_data(),
            primitive_data::PrimitiveData::Unsigned(42)
        );

        assert_eq!(
            u32::type_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoPrimitive(
                    "uint32",
                    primitive_data::PrimitiveData::Unsigned(0)
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(
            42u32.data_to_node(),
            tree::Node {
                node_type: tree::NodeType::ProtoPrimitive(
                    "uint32",
                    primitive_data::PrimitiveData::Unsigned(42)
                ),
                data_type: None,
                data: vec![],
            }
        );
        assert_eq!(42u32.oneof_variant(), None);
    }
}
