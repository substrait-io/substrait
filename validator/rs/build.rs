// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

/// Copies the file at src_tree/path to dest_tree/path if it's newer.
/// Automatically creates parent directories in dest as needed.
fn synchronize(src_tree: &Path, dest_tree: &Path, path: &Path) -> Result<()> {
    // Construct paths.
    let src = src_tree.join(path);
    let dest = dest_tree.join(path);

    // Inform cargo that we should re-run if src changes.
    println!("cargo:rerun-if-changed={}", src.display());

    // Ensure that the source exists.
    if !src.exists() {
        return Err(Error::new(ErrorKind::Other, "source file not found"));
    }

    // Check if destination already exists.
    if dest.exists() {
        // Check if it's newer than or equally old as the source; in that case
        // we don't have to copy it again.
        if dest.metadata()?.modified()? >= src.metadata()?.modified()? {
            return Ok(());
        }
    } else {
        // Check if the destination directory exists, and if not, create it.
        if let Some(parent) = dest.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        }
    }

    // Copy the file.
    std::fs::copy(&src, &dest)?;

    Ok(())
}

/// Returns all protobuf files in the given directory.
fn find_proto_files(proto_path: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("proto")) && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect()
}

fn main() -> Result<()> {
    // Determine the directory of Cargo.toml for this crate.
    let manifest_dir =
        PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let resource_dir = manifest_dir.join("src/resources");

    // Determine whether we're building from the git repository or from a
    // crate file. If the former, we first synchronize our src/resources
    // directory with the rest of the repository.
    if manifest_dir.join("in-git-repo").exists() {
        let git_dir = manifest_dir.join("../..");

        // Synchronize the YAML extension file schema.
        synchronize(
            &git_dir,
            &resource_dir,
            &PathBuf::from("text/simple_extensions_schema.yaml"),
        )?;

        // Synchronize the protobuf files.
        for proto_file in find_proto_files(&git_dir.join("proto")) {
            synchronize(
                &git_dir,
                &resource_dir,
                proto_file
                    .strip_prefix(&git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }
    }

    // Find all protobuf files in our resource directory. We just synchronized
    // these files if we're building from git.
    let proto_path = PathBuf::from(&resource_dir).join("proto");
    let proto_files: Vec<_> = find_proto_files(&proto_path);

    // Compile the protobuf files using prost.
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");
    config.compile_protos(&proto_files, &[&proto_path.display().to_string()])?;

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    Ok(())
}
