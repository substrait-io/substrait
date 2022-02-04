use prost::Message;

pub mod diagnostic;
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
        _path: diagnostic::Path,
    ) -> Result<String> {
        Ok("ok".into())
    }

    fn validate_plan(
        &mut self,
        plan: &proto::substrait::Plan,
        path: diagnostic::Path,
    ) -> Result<String> {
        for (index, relation) in plan.relations.iter().enumerate() {
            let sub_path = path.select_repeated("relations", index);
            self.validate_plan_rel(relation, sub_path).unwrap(); // TODO
        }
        Ok("ok".into())
    }

    fn validate<B: prost::bytes::Buf>(&mut self, buf: B) -> Result<String> {
        let plan = proto::substrait::Plan::decode(buf)?;
        self.validate_plan(&plan, diagnostic::Path::Root("plan"))
    }
}

pub fn validate<B: prost::bytes::Buf>(buf: B) -> (diagnostic::Diagnostics, Option<String>) {
    let mut validator = Validator::default();
    let description = validator
        .validate(buf)
        .map_err(|e| {
            validator.diagnostics.push(diagnostic::Diagnostic {
                cause: e,
                level: diagnostic::Level::Error,
                path: diagnostic::Path::Root("unknown").to_path_buf(),
            })
        })
        .ok();
    (validator.diagnostics, description)
}

pub fn test() {
    println!("Hello, world!");
}
