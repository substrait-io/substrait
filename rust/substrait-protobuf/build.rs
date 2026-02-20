use prost_build::Config;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[cfg(feature = "serde")]
use std::path::Path;

const PROTO_ROOT: &str = "../../proto";

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let protos = WalkDir::new(PROTO_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() || entry.file_type().is_symlink())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .filter(|&extension| extension == "proto")
                .is_some()
        })
        .map(DirEntry::into_path)
        .collect::<Vec<_>>();

    let descriptor_path = out_dir.join("proto_descriptor.bin");

    #[cfg(feature = "serde")]
    serde(&protos, descriptor_path.clone())?;

    #[cfg(not(feature = "serde"))]
    Config::new()
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(&protos, &[PROTO_ROOT])?;

    Ok(())
}

#[cfg(feature = "serde")]
/// Serialize and deserialize implementations for proto types using `pbjson`
fn serde(protos: &[impl AsRef<Path>], descriptor_path: PathBuf) -> Result<(), Box<dyn Error>> {
    use pbjson_build::Builder;
    use std::fs;

    let mut cfg = Config::new();
    cfg.file_descriptor_set_path(&descriptor_path);
    cfg.compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos(protos, &[PROTO_ROOT])?;

    Builder::new()
        .register_descriptors(&fs::read(descriptor_path)?)?
        .ignore_unknown_fields()
        .build(&[".substrait"])?;

    Ok(())
}
