// include the generated protobuf source as a submodule
#[allow(clippy::all)]
pub mod protobuf {
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
}

#[cfg(test)]
mod tests {
    use crate::protobuf::expression::literal::LiteralType;
    use crate::protobuf::expression::Literal;

    #[test]
    fn literal() {
        let _ = Literal {
            nullable: true,
            literal_type: Some(LiteralType::I32(123)),
        };
    }
}
