use prost::Message;
use std::collections::HashSet;

pub mod data_type;
pub mod diagnostic;

#[macro_use]
pub mod doc_tree;
pub mod context;
pub mod extension;
pub mod path;
pub mod proto;

use proto::meta::ProtoDatum;

/// Default result type.
pub type Result<T> = diagnostic::Result<T>;

/// Contextual information available while parsing.
pub struct Context<'a> {
    /// Context object for the parent node.
    pub parent: Option<&'a Context<'a>>,

    /// The path leading up to the node we're validating. Used for generating
    /// diagnostics.
    pub path: path::Path<'a>,

    // The stack of table schemas that FieldRefs currently index into.
    //pub schema: Option<data_type::DataType>,
    /// The set of field names that we've already parsed. This is used to
    /// automatically search through message subtrees that the validator
    /// doesn't yet implement: after all normal validation for a node is done,
    /// the tree-walking logic checks whether there are fields with non-default
    /// data associated with them of which the field name hasn't been added to
    /// this set yet. It's also used to prevent validating the same node twice.
    pub fields_parsed: HashSet<String>,
}

pub fn validate_embedded_function(
    input: &proto::substrait::expression::EmbeddedFunction,
    context: &mut Context,
    output: &mut doc_tree::Node,
) -> Result<()> {
    // Immediate death/cannot continue: just return Err() (or use ? operator
    // to do so.

    // Recoverable diagnostics and information:
    diagnostic!(output, context, Error, UnknownType, "hello");
    diagnostic!(
        output,
        context,
        Warning,
        UnknownType,
        "can also {} here",
        "format"
    );
    diagnostic!(
        output,
        context,
        Info,
        diagnostic::Cause::UnknownType("or make the Cause directly".to_string())
    );
    comment!(output, "hello");
    output.push_comment("".to_string());

    // Setting type information (can be called multiple times):
    let data_type = data_type::DataType {
        class: data_type::Class::Simple(data_type::Simple::Boolean),
        nullable: false,
        variation: None,
        parameters: vec![],
    };
    set_type!(output, data_type);

    // Parsing an optional field:
    let _maybe_node = proto_field!(
        output,
        context,
        input,
        output_type,                         /* field name */
        |_input, _context, _output| todo!(), /* optional parser */
        |_field, _context, _output| Ok(())   /* optional validator */
    );

    // Parsing a required field:
    let _node = proto_required_field!(
        output,
        context,
        input,
        output_type,                         /* field name */
        |_input, _context, _output| todo!(), /* optional parser */
        |_field, _context, _output| Ok(())   /* optional validator */
    );

    // Parsing a oneof field (can also use proto_field!() if optional):
    let _node = proto_required_field!(
        output,
        context,
        input,
        kind, /* field name */
        |_input: &proto::substrait::expression::embedded_function::Kind, _context, _output| todo!(), /* optional parser */
        |_field, _context, _output| Ok(()) /* optional validator */
    );

    // Parsing a repeated field:
    let _vec_node = proto_repeated_field!(
        output,
        context,
        input,
        arguments,                                  /* repeated field name */
        |_input, _context, _output| todo!(),        /* optional parser */
        |_index, _field, _context, _output| Ok(())  /* optional validator */
    );

    // Note: for primitive fields (i.e. fields with a primitive type, like an
    // integer), the parser

    Ok(())
}

pub fn validate_list(
    input: &proto::substrait::r#type::List,
    context: &mut Context,
    output: &mut doc_tree::Node,
) -> Result<()> {
    let _maybe_node = proto_boxed_field!(
        output,
        context,
        input,
        r#type,                              /* field name */
        |_input, _context, _output| todo!(), /* optional parser */
        |_field, _context, _output| Ok(())   /* optional validator */
    );

    Ok(())
}

pub fn validate<B: prost::bytes::Buf>(buf: B) -> doc_tree::Node {
    let mut context = crate::Context {
        parent: None,
        path: path::Path::Root("plan"),
        fields_parsed: HashSet::new(),
    };

    match proto::substrait::Plan::decode(buf) {
        Err(err) => {
            let mut output = proto::substrait::Plan::proto_type_to_node();
            diagnostic!(output, &context, Error, err.into());
            output
        }
        Ok(plan) => {
            let mut output = plan.proto_data_to_node();
            output.handle_unknown_fields(&mut context, &plan, false);
            output
        }
    }
}

pub fn test() {
    use proto::meta::ProtoMessage;
    println!(
        "Hello, world! {}",
        proto::substrait::Plan::proto_message_type()
    );
}
