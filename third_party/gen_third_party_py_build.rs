use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!(
            "Usage: {} <path to requirements_lock.txt> <output BUILD file path>",
            args[0]
        );
        return;
    }

    let requirement_file: &String = &args[1];
    let output_build_path = Path::new(&args[2]);
    let output_path_display = output_build_path.display();
    let mut output_build_file = match File::create(&output_build_path) {
        Err(why) => panic!("Could not create {}: {}", output_path_display, why),
        Ok(file) => file,
    };

    let pkg_deps = match parse_pkg_deps(requirement_file) {
        Err(msg) => {
            println!("{}", msg);
            return;
        }
        Ok(pkg_deps) => pkg_deps,
    };

    let mut build_file_content: String = r###"load("@pip_deps//:requirements.bzl", "requirement")

# Third-party python package dependencies.
# This file is auto-generated.

package(default_visibility = ["//visibility:public"])

# Expose this so it can be updated by "bazel run //third_party:update_py_build" command
exports_files([
    "BUILD",
])
"###
    .to_owned();
    for (pkg, deps) in pkg_deps.iter() {
        build_file_content.push_str(&format!("\npy_library(\n    name = \"{}\",\n", pkg));
        if deps.is_empty() {
            build_file_content.push_str(&format!("    deps = [requirement(\"{}\")],\n", pkg));
        } else {
            build_file_content.push_str("    deps = [\n");
            for dep in deps {
                build_file_content.push_str(&format!("        \":{}\",\n", dep));
            }
            build_file_content.push_str(&format!("        requirement(\"{}\"),\n", pkg));
            build_file_content.push_str("    ],\n");
        }
        build_file_content.push_str(")\n");
    }

    match output_build_file.write_all(build_file_content.as_bytes()) {
        Err(why) => panic!("Error writing content to {}: {}", output_path_display, why),
        Ok(_) => println!("Successfully updated {}", &args[2]),
    };
}

fn parse_pkg_deps(filename: &str) -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let mut pkg_deps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut dep: String = "".to_string();
        for _line in lines {
            let line = match _line {
                Err(_) => {
                    return Err("Error reading lines.".to_string());
                }
                Ok(l) => l,
            };

            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            if !line.starts_with("    #") {
                // dep
                let eq_pos = match line.find("==") {
                    None => {
                        continue;
                    }
                    Some(t) => t,
                };
                // cut at the package option position if any
                let req_pos = match line.find("[") {
                    None => eq_pos,
                    Some(t) => t,
                };
                dep = line[..req_pos].to_string();
                pkg_deps
                    .entry(dep.to_string())
                    .or_insert_with(|| Default::default());
            } else {
                let p;
                // handle these lines
                //     # via selenium-wire
                //     #   requests
                if line.starts_with("    # via") {
                    p = line[9..].trim();
                } else {
                    p = line[8..].trim();
                }
                // escape these lines
                // # via -r requirements.in
                //     #   -r requirements.in
                if p.is_empty() || p.starts_with("-r") {
                    continue;
                }
                let dep_set = pkg_deps
                    .entry(p.to_string())
                    .or_insert_with(|| Default::default());
                dep_set.insert(dep.clone());
            }
        }
        Ok(pkg_deps)
    } else {
        Err(format!("Error reading file {}", filename))
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
