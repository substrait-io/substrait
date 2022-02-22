use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_item_prefix("substrait_validator_")
        .rename_item("Handle", "handle")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/substrait_validator.h");
}
