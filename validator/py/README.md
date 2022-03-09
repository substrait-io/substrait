# Python bindings for validator

This directory contains a Rust/PyO3 project to generate Python bindings for the
validator library.

## Installation

No wheels are published yet at this time, so you have to build manually.
Running something along the lines of `pip install .` should work. You should
only need to have a [rust](https://www.rust-lang.org/tools/install) compiler
installed.

## Building wheels and source distributions

You can build wheels and source distributions using
[maturin](https://github.com/PyO3/maturin), specifically using the `build` and
`sdist` commands. However, before you can do this, you must run
`./prepare_build.py populate`. This makes local copies of some files in the
repository that live outside of this subdirectory, such as the protobuf
description files. When you use `pip` or some other tool based on
`pyproject.toml`, this will be done automatically via build system hooks, but
unfortunately maturin doesn't itself provide hooks with which this can be
automated.

## Running tests

You can test the module using `pytest` after you install it.

## Usage

TODO
