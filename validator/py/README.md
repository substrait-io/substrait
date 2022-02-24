# Python bindings for validator

This directory contains a Rust/PyO3 project to generate Python bindings for the
validator library.

## Installation

No wheels are published yet at this time. To build manually, you will need:

 - [rust](https://www.rust-lang.org/tools/install)
 - [maturin](https://github.com/PyO3/maturin)

At which point you can run:

```console
user@host:/path/to/substrait/validator/py$ maturin build
📦 Built wheel for CPython ... to /path/to/substrait/target/wheels/substrait_validator-...whl
user@host:/path/to/substrait/validator/py$ pip install --force-reinstall /path/to/substrait/target/wheels/substrait_validator-...whl
```

Note: copy the wheel path from `maturin`'s output.

Note: at this time, `pip install .` does NOT work, nor do source distributions.
This is because the build process needs context from the parent directories.
There does not seem to be a way to avoid this without making a separate
repository for the Python module and using the rest of the Substrait repository
as a submodule.

If you're using a venv, you can also run `maturin develop`.

## Running tests

You can test the module using `pytest` after you install it.

## Usage

TODO
