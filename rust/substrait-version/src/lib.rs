
include!(concat!(env!("OUT_DIR"), "/version_constants.rs"));

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    /// The git hash if set.
    pub git_hash: Option<String>,
    /// The producer string if set.
    pub producer: Option<String>,
}

/// Returns the version of Substrait used to build this crate.
///
/// Note that this does not set [Version::producer]. See
/// [version_with_producer].
pub fn version() -> Version {
    Version {
        major: SUBSTRAIT_MAJOR_VERSION,
        minor: SUBSTRAIT_MINOR_VERSION,
        patch: SUBSTRAIT_PATCH_VERSION,
        git_hash: if SUBSTRAIT_GIT_DEPTH != 0 {
            Some(String::from(SUBSTRAIT_GIT_SHA))
        } else {
            None
        },
        producer: None
    }
}

/// Returns the version of Substrait used to build this crate with
/// [Version::producer] set to the passed `producer`.
pub fn version_with_producer(producer: impl Into<String>) -> Version {
    Version {
        producer: Some(producer.into()),
        ..version()
    }
}
