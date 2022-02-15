pub mod data_type;
pub mod diagnostic;

#[macro_use]
pub mod doc_tree;
pub mod context;
pub mod extension;
pub mod path;
pub mod proto;

/// Default result type.
pub type Result<T> = diagnostic::Result<T>;

pub fn validate_embedded_function(
    x: &mut context::Context<proto::substrait::expression::EmbeddedFunction>,
) -> Result<()> {
    // Immediate death/cannot continue: just return Err() (or use ? operator
    // to do so.

    // Recoverable diagnostics and information:
    diagnostic!(x, Error, UnknownType, "hello");
    diagnostic!(x, Warning, UnknownType, "can also {} here", "format");
    diagnostic!(
        x,
        Info,
        diagnostic::Cause::UnknownType("or make the Cause directly".to_string())
    );
    comment!(x, "hello");

    // Setting type information (can be called multiple times):
    let data_type = data_type::DataType {
        class: data_type::Class::Simple(data_type::Simple::Boolean),
        nullable: false,
        variation: None,
        parameters: vec![],
    };
    data_type!(x, data_type);

    // Parsing an optional field:
    let _maybe_node = proto_field!(
        x,
        output_type,              /* field name */
        |_x| todo!(),             /* optional parser */
        |_x, _field_node| Ok(())  /* optional validator */
    );

    // Parsing a required field:
    let _node = proto_required_field!(
        x,
        output_type,                /* field name */
        |_x| todo!(),               /* optional parser */
        |_x, _field_output| Ok(())  /* optional validator */
    );

    // Parsing a oneof field (can also use proto_field!() if optional):
    let _node = proto_required_field!(
        x,
        kind,                       /* field name */
        |_x| todo!(),               /* optional parser */
        |_x, _field_output| Ok(())  /* optional validator */
    );

    // Parsing a repeated field:
    let _vec_node = proto_repeated_field!(
        x,
        arguments,                        /* repeated field name */
        |_x| todo!(),                     /* optional parser */
        |_x, _field_node, _index| Ok(())  /* optional validator */
    );

    // Note: for primitive fields (i.e. fields with a primitive type, like an
    // integer), the parser

    Ok(())
}

pub fn validate_list(x: &mut context::Context<proto::substrait::r#type::List>) -> Result<()> {
    let _maybe_node = proto_boxed_field!(
        x,
        r#type,                   /* field name */
        |_x| todo!(),             /* optional parser */
        |_x, _field_node| Ok(())  /* optional validator */
    );

    Ok(())
}

pub fn validate<B: prost::bytes::Buf>(buffer: B) -> doc_tree::Node {
    doc_tree::Node::parse_proto::<proto::substrait::Plan, _, _>(
        buffer,
        "plan",
        |_| Ok(()),
        &mut context::State::default(),
        &context::Config::default(),
    )
}

pub fn test() {
    use proto::meta::ProtoMessage;
    println!(
        "Hello, world! {}",
        proto::substrait::Plan::proto_message_type()
    );
}
