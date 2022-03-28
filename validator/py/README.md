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

## Command-line usage

The module exposes a command-line program named `substrait-validator` for
running the validator manually. You can also use the tool to convert between
various serialization formats of the `substrait.Plan` message. Run
`substrait-validator --help` for more information.

## Library usage

The library essentially provides a bunch of type conversion functions at
module scope to convert between the various representations of a Substrait
plan, including the result of the validator. The most important functions are
arguably `check_plan_valid(plan, config=None)` and
`check_plan_not_invalid(plan, config=None)`, which run validation on the given
plan and throw a Python exception corresponding to the first diagnostic
returned by the validator of the highest severity encountered if the plan is
not strictly or loosely valid respectively. That is, `check_plan_valid` will
throw an exception if the plan could not be proven to be valid, while
`check_plan_not_invalid` will only throw if it could be proven to be invalid.

The `plan` argument can be a number of things:

 - `bytes`: treated as a binary serialization of `substrait.Plan`.
 - `str`: treated as a protobuf JSON serialization of `substrait.Plan`.
 - `dict`: treated as the above using Python's data model (JSON objects map
   to `dict`s, JSON arrays map to `list`s).
 - `substrait_validator.substrait.Plan`: a previously deserialized plan.
 - `substrait_validator.ResultHandle`: a previously validated plan.

`config` can be `None`/unspecified, or can be set to a
`substrait_validator.Config` object to configure the validator with.

For more information, use Python's `help()` function.
