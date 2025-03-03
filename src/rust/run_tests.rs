#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! ```

use serde::Deserialize;
use std::{env, fs, process::Command};

#[derive(Debug, Deserialize)]
struct PrDoc {
    crates: Vec<Crate>,
}

#[derive(Debug, Deserialize)]
struct Crate {
    name: String,
}

fn main() {
    // Get the first argument (PR number)
    let args: Vec<String> = env::args().collect();
    let pr_number = args.get(1).unwrap_or_else(|| {
        eprintln!("Usage: rust-script run_tests.rs <pr_number>");
        std::process::exit(1);
    });

    // Get the working directory
    let home = env::var("HOME").expect("Failed to get HOME directory");
    let workspace = env::var("WORKSPACE").unwrap_or_else(|_| format!("{}/projects", home));
    let working_dir = format!("{}/polkadot-sdk", workspace);
    let yaml_path = format!("{}/prdoc/pr_{}.prdoc", working_dir, pr_number);

    // Read the YAML file
    let yaml_content = fs::read_to_string(&yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", yaml_path));

    // Deserialise into structs
    let pr_doc: PrDoc = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

    // Track failed tests
    let mut failed_tests = vec![];

    // Loop over each crate and run `cargo test`
    for krate in pr_doc.crates {
        println!("Running tests for: {}", krate.name);
        let mut args = vec!["test".to_string(), "-p".to_string(), krate.name.clone()];

        // Enable all features only for `sp-tracing`
        if krate.name == "sp-tracing" {
            args.push("--all-features".to_string());
        }

        let status = Command::new("cargo")
            .args(&args)
            .current_dir(&working_dir)
            .status()
            .expect("Failed to execute cargo test");

        if !status.success() {
            eprintln!("❌ Tests failed for {}", krate.name);
            failed_tests.push(krate.name);
        } else {
            println!("✅ Tests passed for {}", krate.name);
        }
    }

    // Print summary if any tests failed
    if !failed_tests.is_empty() {
        eprintln!("\n🚨 Some tests failed:");
        for failed in failed_tests {
            eprintln!("- {}", failed);
        }
        std::process::exit(1);
    } else {
        println!("\n🎉 All tests passed successfully!");
    }
}
