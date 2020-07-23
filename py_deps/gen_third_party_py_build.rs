use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pkg_deps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./requirements.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Err(_) = line {
                println!("Read line error");
                break;
            } else if let Ok(l) = line {
                let trimmed = l.trim();
                if trimmed.starts_with("#") { continue; }
                if trimmed.is_empty() { continue; }

                let via_pos = trimmed.find("# via ");
                if via_pos.is_none() { continue; }
                let dep_req = &trimmed[..via_pos.unwrap()].trim();
                let pkg_str = &trimmed[via_pos.unwrap() + 6..].trim();

                let req_pos = dep_req.find("==");
                if req_pos.is_none() { continue; }
                let dep = &dep_req[..req_pos.unwrap()];

                let pkgs: Vec<&str> = pkg_str.split(", ").collect();
                for p in pkgs {
                    if p.starts_with("-r ") { continue; }
                    let dep_set = pkg_deps.entry(p.to_string()).or_insert_with(|| vec![].into_iter().collect());
                    dep_set.insert(dep.to_string());
                }
                pkg_deps.entry(dep.to_string()).or_insert_with(|| vec![].into_iter().collect());
            }
        }


        println!(r###"# Third-party python package dependencies.
# This file is auto-generated.

package(default_visibility = ["//visibility:public"])

load("@ds_py_deps//:requirements.bzl", "requirement")
"###);
        for (pkg, deps) in pkg_deps.iter() {
            println!(r###"
py_library(
    name = "{}","###, pkg);
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
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
