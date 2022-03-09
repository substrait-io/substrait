// SPDX-License-Identifier: Apache-2.0

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config = cbindgen::Config {
        cpp_compat: true,
        language: cbindgen::Language::C,
        ..Default::default()
    };
    config.export.prefix = Some("substrait_validator_".to_string());
    config
        .export
        .rename
        .insert("ConfigHandle".to_string(), "config_handle".to_string());
    config
        .export
        .rename
        .insert("ResultHandle".to_string(), "result_handle".to_string());
    config
        .export
        .rename
        .insert("Resolver".to_string(), "resolver".to_string());
    config
        .export
        .rename
        .insert("Deleter".to_string(), "deleter".to_string());
    config.header = Some("// SPDX-License-Identifier: Apache-2.0".to_string());

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/substrait_validator.h");
}
