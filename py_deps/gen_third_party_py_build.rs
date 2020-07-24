use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let pkg_deps = match parse_pkg_deps("./requirements.txt") {
        Err(msg) => {
            println!("{}", msg);
            return;
        }
        Ok(pkg_deps) => pkg_deps,
    };

    println!(
        r###"# Third-party python package dependencies.
# This file is auto-generated.

package(default_visibility = ["//visibility:public"])

load("@pip_deps//:requirements.bzl", "requirement")
"###
    );
    for (pkg, deps) in pkg_deps.iter() {
        println!(
            r###"
py_library(
name = "{}","###,
            pkg
        );
        if deps.is_empty() {
            println!(r###"    deps = [requirement("{}")],"###, pkg);
        } else {
            println!(r###"    deps = ["###);
            for dep in deps {
                println!(r###"        ":{}","###, dep);
            }
            println!(r###"        requirement("{}"),"###, pkg);
            println!(r###"    ],"###);
        }
        println!(")\n");
    }
}

fn parse_pkg_deps(filename: &str) -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let mut pkg_deps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for _line in lines {
            let line = match _line {
                Err(_) => {
                    return Err("Error reading lines.".to_string());
                }
                Ok(l) => l,
            };
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }

            // Assuming we have the input:
            // "six==1.0.0       # via protobuf, tensorflow
            let via_pos = match trimmed.find("# via ") {
                None => {
                    continue;
                }
                Some(t) => t,
            };
            // dep_req: "six==1.0.0"
            // pkg_str: "protobuf, tensorflow"
            let dep_req = &trimmed[..via_pos].trim();
            let pkg_str = &trimmed[via_pos + 6..].trim();

            // dep: "six"
            let req_pos = match dep_req.find("==") {
                None => {
                    continue;
                }
                Some(t) => t,
            };
            let dep = &dep_req[..req_pos];

            // pkgs = ["protobuf", "tensorflow"]
            let pkgs: Vec<&str> = pkg_str.split(", ").collect();
            for p in pkgs {
                if p.starts_with("-r ") {
                    continue;
                }
                let dep_set = pkg_deps
                    .entry(p.to_string())
                    .or_insert_with(|| vec![].into_iter().collect());
                dep_set.insert(dep.to_string());
            }
            pkg_deps
                .entry(dep.to_string())
                .or_insert_with(|| vec![].into_iter().collect());
        }
        Ok(pkg_deps)
    } else {
        Err("Error reading file".to_string())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
