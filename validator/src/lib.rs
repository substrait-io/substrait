use std::collections::HashSet;

pub mod data_type;
pub mod diagnostic;

#[macro_use]
pub mod doc_tree;
pub mod extension;
pub mod path;
pub mod proto;

/// Default result type.
pub type Result<T> = diagnostic::Result<T>;

pub struct Context<'a> {
    pub parent: Option<&'a Context<'a>>,

    pub path: path::Path<'a>,

    pub data_type: Option<data_type::DataType>,

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
    set_type!(output, context, data_type);

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
/*
#[derive(Default)]
struct Validator {
    /// Vector of all diagnostics we've gathered thus far.
    diagnostics: diagnostic::Diagnostics,
}

impl Validator {
    fn validate_plan_rel(
        &mut self,
        _rel: &proto::substrait::PlanRel,
        _path: path::Path,
    ) -> Result<doc_tree::Node> {
        Ok(doc_tree::Node::message("substrait.PlanRel"))
    }

    fn validate_plan(
        &mut self,
        plan: &proto::substrait::Plan,
        path: path::Path,
    ) -> Result<doc_tree::Node> {
        for (index, relation) in plan.relations.iter().enumerate() {
            let sub_path = path.with_repeated("relations", index);
            self.validate_plan_rel(relation, sub_path).unwrap(); // TODO
        }
        Ok(doc_tree::Node::message("substrait.Plan"))
    }

    fn validate<B: prost::bytes::Buf>(&mut self, buf: B) -> Result<doc_tree::Node> {
        let plan = proto::substrait::Plan::decode(buf)?;
        self.validate_plan(&plan, path::Path::Root("plan"))
    }
}

pub fn validate<B: prost::bytes::Buf>(buf: B) -> (diagnostic::Diagnostics, Option<doc_tree::Node>) {
    let mut validator = Validator::default();
    let description = validator
        .validate(buf)
        .map_err(|e| {
            validator.diagnostics.push(diagnostic::Diagnostic {
                cause: e,
                level: diagnostic::Level::Error,
                path: path::Path::Root("unknown").to_path_buf(),
            })
        })
        .ok();
    (validator.diagnostics, description)
}
*/
pub fn test() {
    use proto::meta::ProtoMessage;
    println!(
        "Hello, world! {}",
        proto::substrait::Plan::proto_message_type()
    );
}
