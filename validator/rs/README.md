"Rust bindings" for validator
=============================

This directory contains the Rust library that you would link to from Rust-only
projects. It's a very thin shell around `substrait-validator-core`, but adds
the `curl` dependency to resolve many more URI schemes for YAML files than
supported by the core library.
