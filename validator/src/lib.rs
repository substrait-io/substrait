use prost::Message;

pub mod data_type;
pub mod diagnostic;
pub mod doc_tree;
pub mod extension;
pub mod path;
pub mod proto;

/// Default result type.
pub type Result<T> = diagnostic::Result<T>;

#[derive(Default)]
struct Validator {
    diagnostics: diagnostic::Diagnostics,
}

impl Validator {
    fn validate_plan_rel(
        &mut self,
        _rel: &proto::substrait::PlanRel,
        _path: path::Path,
    ) -> Result<doc_tree::Node> {
        Ok(doc_tree::NodeType::ProtoMessage("substrait.PlanRel".to_string()).into())
    }

    fn validate_plan(
        &mut self,
        plan: &proto::substrait::Plan,
        path: path::Path,
    ) -> Result<doc_tree::Node> {
        for (index, relation) in plan.relations.iter().enumerate() {
            let sub_path = path.select_repeated("relations", index);
            self.validate_plan_rel(relation, sub_path).unwrap(); // TODO
        }
        Ok(doc_tree::NodeType::ProtoMessage("substrait.Plan".to_string()).into())
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

pub fn test() {
    println!("Hello, world!");
}
