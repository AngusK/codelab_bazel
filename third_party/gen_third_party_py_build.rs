use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} <path to requirements_lock.txt>", args[0]);
        return;
    }

    let requirement_file: &String = &args[1];

    let pkg_deps = match parse_pkg_deps(requirement_file) {
        Err(msg) => {
            println!("{}", msg);
            return;
        }
        Ok(pkg_deps) => pkg_deps,
    };

    println!(
        r###"load("@pip_deps//:requirements.bzl", "requirement")

# Third-party python package dependencies.
# This file is auto-generated.

package(default_visibility = ["//visibility:public"])"###);
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
        println!(")");
    }
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
