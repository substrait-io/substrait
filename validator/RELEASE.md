Release process
===============

Note: this is only intended for maintainers. See `README.md` for general
usage information.

Incrementing version numbers
----------------------------

There are version numbers all over the place, though some of them aren't that
important:

 - `derive/Cargo.toml` and its reference as dependency in `rs/Cargo.toml`:
   these two version numbers must be kept in sync, but only need to be
   incremented when anything changes in `substrait-validator-derive`.
 - `rs/Cargo.toml` and its references as dependencies in `py/Cargo.toml`,
   `c/Cargo.toml`, and `tests/Cargo.toml`, as well as in `rs/README.md` for
   the Cargo dependency copypasta: these must be kept in sync and incremented
   when the `substrait-validator` sources, the protobuf files, OR the YAML
   schema files are updated.
 - `py/Cargo.toml` and `py/pyproject.toml`: must be kept in sync, and must be
   incremented whenever the `substrait-validator` crate is updated OR the
   Python bindings are modified.
 - `c/Cargo.toml`: not very important as it should always be built from source
   by corrosion, but good to synchronize with the version of the main crate.
 - `tests/Cargo.toml`: can be ignored.

Relation of `substrait-validator` crate version to the Substrait specification
version is TBD.

Pushing to crates.io
--------------------

Note in advance: the crates in the `py`, `c`, and `tests` directories should
NOT be pushed to `crates.io`:

 - the Python bindings crate is either embedded as sources in Python source
   distributions or is shipped pre-built from the git repo in binary wheels;
 - the C bindings should be built by CMake/Corrosion after it obtains the
   complete git repo or a tarball thereof; and
 - the `tests` crate is just a test runner that serves no purpose outside of
   this repository.

Only the crates in the `derive` and `rs` directories, respectively
`substrait-validator` and `substrait-validator-derive` should be released.

The release steps are as follows.

 - Update version numbers (see section above).
 - If `substrait-validator-derive` changed, release it per normal procedures.
 - Remove the `rs/src/resources` directory, if one exists.
 - Run `cargo build` locally for `substrait-validator` to recreate above
   directory using the protobuf and schema files from outside the validator
   folder.
 - Run `cargo package`. Verify that it ONLY complains about files in
   `src/resources` not being committed yet. This is unavoidable without
   checking in the protobuf files in multiple places.
 - Release `substrait-validator` per normal procedures, but using
   `--allow-dirty` to suppress the above.

Pushing to PyPI
---------------

The release steps are as follows, though they should probably be performed by
CI to use the appropriate environment.

 - Update version numbers (see section above).
 - Run `python3 prepare_build.py clean`.
 - Run `python3 prepare_build.py populate`. This makes a local copy of the
   protobuf files for inclusion in an sdist.
 - Run `maturin sdist` to build the source distribution.
 - Run `maturin build` in the appropriate environments to build binary
   distributions.
