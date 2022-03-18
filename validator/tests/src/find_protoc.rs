// SPDX-License-Identifier: Apache-2.0

//! Prost has some magic for finding the path to protoc, so let's use that in
//! the Python code as well...

fn main() {
    println!("{}", prost_build::protoc().display());
}
