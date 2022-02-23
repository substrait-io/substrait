use std::{ffi::OsStr, io::Result};

fn main() -> Result<()> {
    let proto_path = "../../proto";

    let proto_files: Vec<_> = walkdir::WalkDir::new(proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("proto")) && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect();

    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");

    config.compile_protos(&proto_files, &[proto_path])
}
