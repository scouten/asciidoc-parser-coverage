// Quick and dirty tool to generate spec coverage for asciidoc-parser. Not
// intended at this time to generalize to any other use case.

// If asciidoc-parser goes well, I may build a more robust version of this at a
// later time. For now, please excuse the hard-coded settings and other shortcuts taken.

use std::collections::HashMap;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut spec_coverage: HashMap<String, Vec<(String, bool)>> = HashMap::new();

    let rs_files: Vec<DirEntry> = WalkDir::new("../parser/src/tests")
    .into_iter()
    .filter_entry(|e| {
        if let Some(file_name) = e.file_name().to_str() {
            !file_name.starts_with(".")
        } else {
            false
        }
    })
    .filter_map(|e| {
        let e = e.expect("Directory read error");

        if !e.file_type().is_file() {
            return None;
        }

        if let Some(file_name) = e.file_name().to_str() {
            if file_name.ends_with(".rs") {
                Some(e)
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect();

    for entry in rs_files {
        let path = entry.path();
        if let Some((spec_path, cov)) = parse_rs_file(path) {
            spec_coverage.insert(spec_path, cov);
        }
    }

    println!("{{\n    \"coverage\": {{");

    let adoc_files: Vec<DirEntry> = WalkDir::new("../docs/modules")
        .into_iter()
        .filter_entry(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                !file_name.starts_with(".")
            } else {
                false
            }
        })
        .filter_map(|e| {
            let e = e.expect("Directory read error");

            if !e.file_type().is_file() {
                    return None;
                }

                if let Some(file_name) = e.file_name().to_str() {
                    if file_name.ends_with(".adoc") {
                        Some(e)
                    } else {
                        None
                    }
                } else {
                    None
                }
        })
        .collect();

    let last_index = adoc_files.len() - 1;

    for (count, entry) in adoc_files.into_iter().enumerate() {
        let path = entry.path().to_str().unwrap().trim_start_matches("../");
        // (unwrap: Should have been filtered out above.)

        println!("        {path:?}: {{");
        println!("            \"1\": 0");
        if count < last_index {
            println!("        }},");
        } else {
            println!("        }}");
        }
    }

    println!("    }}\n}}");
}

fn parse_rs_file(_path: &Path) -> Option<(String, Vec<(String, bool)>)> {
    None
}
