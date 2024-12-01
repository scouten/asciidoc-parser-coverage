// Quick and dirty tool to generate spec coverage for asciidoc-parser. Not
// intended at this time to generalize to any other use case.

// If asciidoc-parser goes well, I may build a more robust version of this at a
// later time. For now, please excuse the hard-coded settings and Q&D JSON
// generation.

use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut has_error = false;

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
            if let Ok(e) = e {
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
            } else {
                eprintln!("DIRECTORY READ ERROR: {e:?}");
                has_error = true;
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

    if has_error {
        std::process::exit(1);
    }
}
