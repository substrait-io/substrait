# C bindings for validator

This directory contains a Rust/cbindgen project to generate C bindings for
the validator library.

## Installation

No binaries are published yet at this time.

### Building manually

To build manually, you will need:

 - [rust](https://www.rust-lang.org/tools/install)

At which point you can run:

```console
user@host:/path/to/substrait/validator/c$ cargo build --release
```

This will generate a static and shared library at
`/path/to/substrait/validator/target/release/libsubstrait_validator_c.[a|so|lib|dll|dylib]`,
and header at `/path/to/substrait/validator/c/include`.

### Building using CMake

You can also build via CMake, and in doing so use the validator from within a
CMake-based project. You should be able to simply add this directory as a
subdirectory and link against the `substrait-validator-c` target. This will
refer to the static or shared library based on `BUILD_SHARED_LIBS`.

You can also run tests as follows:

```console
user@host:/path/to/substrait/validator/c$ mkdir build
user@host:/path/to/substrait/validator/c$ cd build
user@host:/path/to/substrait/validator/c/build$ cmake .. -DSUBSTRAIT_VALIDATOR_BUILD_TESTS=ON
...
user@host:/path/to/substrait/validator/c/build$ cmake --build .
...
user@host:/path/to/substrait/validator/c/build$ ctest .
Test project /path/to/substrait/validator/c/build
    Start 1: BasicTest.BasicTest
1/1 Test #1: BasicTest.BasicTest ..............   Passed    0.00 sec

100% tests passed, 0 tests failed out of 1

Total Test time (real) =   0.00 sec
```

## Usage

The generated header file includes docstrings that should be fairly
self-explanatory.
