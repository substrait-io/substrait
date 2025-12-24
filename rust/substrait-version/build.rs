use std::error::Error;
use std::{env, fs};
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = Path::new(&env::var("OUT_DIR")?).to_path_buf();
    let version_in_file = out_dir.join("version_constants.rs");

    // Get the version of the submodule by directly calling `git describe`.
    let git_describe = String::from_utf8(
        Command::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--long")
            .arg("--dirty=-dirty")
            .arg("--abbrev=40")
            .output()?
            .stdout,
    )?;

    // Extract the parts.
    let mut split = git_describe.split('-');
    let git_version = split.next().unwrap_or_default();
    let git_depth = split.next().unwrap_or_default();
    let git_hash = split.next().unwrap_or_default().trim_end();
    let git_dirty = git_describe.ends_with("dirty");
    let version = semver::Version::parse(git_version.trim_start_matches('v'))?;

    let &semver::Version {
        major,
        minor,
        patch,
        ..
    } = &version;

    fs::write(
        version_in_file,
        format!(
            r#"// SPDX-License-Identifier: Apache-2.0

// Note that this file is auto-generated and auto-synced using `build.rs`. It is
// included in `version.rs`.

/// The major version of Substrait used to build this crate
pub const SUBSTRAIT_MAJOR_VERSION: u32 = {major};

/// The minor version of Substrait used to build this crate
pub const SUBSTRAIT_MINOR_VERSION: u32 = {minor};

/// The patch version of Substrait used to build this crate
pub const SUBSTRAIT_PATCH_VERSION: u32 = {patch};

/// The Git SHA (lower hex) of Substrait used to build this crate
pub const SUBSTRAIT_GIT_SHA: &str = "{git_hash}";

/// The `git describe` output of the Substrait submodule used to build this
/// crate
pub const SUBSTRAIT_GIT_DESCRIBE: &str = "{git_describe}";

/// The amount of commits between the latest tag and the version of the
/// Substrait submodule used to build this crate
pub const SUBSTRAIT_GIT_DEPTH: u32 = {git_depth};

/// The dirty state of the Substrait submodule used to build this crate
pub const SUBSTRAIT_GIT_DIRTY: bool = {git_dirty};
"#
        ),
    )?;

    Ok(())
}