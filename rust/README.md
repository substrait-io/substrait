# substrait-rs

Experimental Rust bindings for [substrait](https://substrait.io/).

## Build & Test

``` bash
cargo test
```

## Generate Documentation

``` bash
cargo doc --no-deps
```

## Publishing the crate

We need to specify `--allow-dirty` when publishing because we copy the `.proto` files into the local directory but
do not commit these copies to git.

``` bash
cargo publish --allow-dirty
```