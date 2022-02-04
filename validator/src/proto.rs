#[allow(clippy::large_enum_variant)]
pub mod substrait {
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
}
