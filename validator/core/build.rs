// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let pwd = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let proto_path = PathBuf::from(&pwd).join("../../proto");

    let proto_files: Vec<_> = walkdir::WalkDir::new(&proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("proto")) && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect();

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");

    config.compile_protos(&proto_files, &[&proto_path.display().to_string()])
}
