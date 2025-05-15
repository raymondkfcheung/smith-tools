#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! walkdir = "2"
//! ```

use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let dir = env::args().nth(1).unwrap_or_else(|| {
        let home = env::var("HOME").expect("Failed to get HOME directory");
        let workspace = env::var("WORKSPACE").unwrap_or_else(|_| format!("{}/projects", home));
        format!("{}/polkadot-sdk", workspace)
    });
    search_for_map_err(&dir);
}

fn search_for_map_err(dir: &str) {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let lines: Vec<&str> = content.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let file_path = path.display().to_string();
                if file_path.contains("xcm")
                    && !file_path.contains("benchmarking")
                    && !file_path.contains("tests")
                    && line.contains("map_err")
                    && line.contains("Error")
                {
                    if i + 1 < lines.len() && !lines[i + 1].contains("tracing") {
                        println!("❗ Missing tracing in: {}:{}", path.display(), i + 1);
                    }
                }
            }
        }
    }
}
