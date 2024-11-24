// I Language tools build script.
// Version: 1.0.0

// Copyright (c) 2023-present I Language Development.

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

/////////////
// IMPORTS //
/////////////

use std::{env, fmt::Write as _, fs, path::Path, process::exit};

use cargo_metadata::MetadataCommand;
use glob::glob;


//////////////////
// BUILD SCRIPT //
//////////////////

// Helper function for just returning an empty list of dependencies, so that the panic handler can still run.
fn empty_list() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("dependencies.rs");
    fs::write(
        &path,
        "pub const DEPENDENCIES: &[(&str, &[(&str, &str)])] = &[];",
    )
    .expect("Failed to write dependency information");
}

fn main() {
    let cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| {
            empty_list();
            exit(0);
        })
        .parent()
        .unwrap_or_else(|| {
            empty_list();
            exit(0);
        })
        .join("Cargo.toml");

    let metadata = MetadataCommand::new()
        .manifest_path(cargo_toml_path)
        .exec()
        .unwrap_or_else(|_| {
            empty_list();
            exit(0);
        });

    let root_package = metadata
        .packages
        .iter()
        .find(|package| metadata.workspace_members.contains(&package.id))
        .unwrap_or_else(|| {
            empty_list();
            exit(0);
        });

    let mut dependency_list = String::new();
    writeln!(
        dependency_list,
        "pub const DEPENDENCIES: &[(&str, &[(&str, &str)])] = &["
    );

    writeln!(dependency_list, "    (\"{}\", &[", root_package.name);
    for dependency in &root_package.dependencies {
        writeln!(
            dependency_list,
            "        (\"{}\", \"{}\"),",
            dependency.name, dependency.req
        );
    }
    writeln!(dependency_list, "    ]),");

    for package in metadata.packages.clone() {
        if package.id == root_package.id {
            continue;
        }

        if metadata.workspace_members.contains(&package.id) {
            dependency_list.push_str(&format!("    (\"{}\", &[\n", package.name));
            for dependency in &package.dependencies {
                writeln!(
                    dependency_list,
                    "        (\"{}\", \"{}\"),",
                    dependency.name, dependency.req
                );
            }
            writeln!(dependency_list, "    ]),");
        }
    }

    writeln!(dependency_list, "];");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("dependencies.rs");
    fs::write(&path, dependency_list).expect("Failed to write dependency information");

    println!("cargo:rerun-if-changed=../../Cargo.toml");
    for entry in glob("../../crates/*/Cargo.toml").unwrap_or_else(|_| exit(0)) {
        if let Ok(path) = entry {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
