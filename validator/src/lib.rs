pub mod data_type;
pub mod diagnostic;
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
}

pub fn validate_embedded_function(
    input: &proto::substrait::expression::EmbeddedFunction,
    context: &mut Context,
    output: &mut doc_tree::Node,
) -> Result<()> {
    // Immediate death/cannot continue: just return Err() (or use ? operator
    // to do so.

    // Recoverable diagnostics and information:
    output.push_error(context, diagnostic::Cause::UnknownType("".to_string()));
    output.push_warning(context, diagnostic::Cause::UnknownType("".to_string()));
    output.push_info(context, diagnostic::Cause::UnknownType("".to_string()));
    output.push_comment("".to_string());

    // Setting type information (can be called multiple times):
    let data_type = data_type::DataType {
        class: data_type::Class::Simple(data_type::Simple::Boolean),
        nullable: false,
        variation: None,
        parameters: vec![],
    };
    output.push_type(context, data_type);

    // Parsing an optional field:
    output.push_proto_field(
        context,
        &input.output_type,
        "output_type", // want to automate this with a macro, must always be the field name above
        |_input, _context, _output| todo!(),
        |_field, _context, _output| Ok(()),
    );

    // Parsing a required field:
    output.push_proto_required_field(
        context,
        &input.output_type,
        "output_type",
        |_input, _context, _output| todo!(),
        |_field, _context, _output| Ok(()),
    );

    // Parsing a repeated field:
    output.push_proto_repeated_field(
        context,
        &input.arguments,
        "arguments",
        |_input, _context, _output| todo!(),
        |_index, _field, _context, _output| Ok(()),
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
