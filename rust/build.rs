// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let pwd = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let proto_path_root = PathBuf::from(&pwd).join("../proto");
    let output_root = PathBuf::from(&pwd).join("proto");

    let proto_defs = [
        "substrait/algebra.proto",
        "substrait/capabilities.proto",
        "substrait/extensions/extensions.proto",
        "substrait/function.proto",
        "substrait/parameterized_types.proto",
        "substrait/plan.proto",
        "substrait/type_expressions.proto",
        "substrait/type.proto",
    ];

    // copy proto files into crate directory during build and packaging
    // phase (but not publish phase)
    if proto_path_root.exists() {
        for proto in &proto_defs {
            let src = proto_path_root.join(proto);
            let dest = output_root.join(proto);
            if let Some(p) = dest.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            fs::copy(src, dest)?;
        }
    }

    let paths: Vec<String> = proto_defs
        .iter()
        .map(|s| output_root.join(s).display().to_string())
        .collect();

    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    // rebuild if any proto files changed
    for path in &paths {
        println!("cargo:rerun-if-changed={}", path);
    }

    let path = output_root.display().to_string();
    prost_build::compile_protos(&paths, &[&path])
}
