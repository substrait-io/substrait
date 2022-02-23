Validator
=========

This directory contains a validator for Substrait plans. It's written in Rust,
but bindings are available for Python and C. Other languages may use the C API
via their respective foreign function interface systems.

NOTE: in order to build everything in the workspace with Cargo, you need to
have Python `protobuf-gen` >= 0.0.4 installed.
