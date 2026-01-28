use std::{env, fs, path::Path};
use schemars::schema::Schema;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("../../text/simple_extensions_schema.yaml")?;
    let schema = serde_yaml::from_str::<schemars::schema::RootSchema>(&content)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            // Preserve field order in YAML objects (see Substrait #915) so
            // struct field ordinals remain stable across parsers.
            .with_map_type("::indexmap::IndexMap")
            .with_struct_builder(true)
            .with_derive("PartialEq".to_string()),
    );
    type_space.add_ref_types(schema.definitions)?;
    type_space.add_type(&Schema::Object(schema.schema))?;
    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

    let mut out_file = Path::new(&env::var("OUT_DIR")?).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, contents)?;
    Ok(())
}