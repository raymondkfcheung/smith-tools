#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! ```

use serde::Deserialize;
use std::{collections::HashSet, env, fs, process::Command};

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
    println!("Parsing YAML file: {}", yaml_path);
    let yaml_content = fs::read_to_string(&yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", yaml_path));

    // Deserialise into structs
    println!("YAML content:\n{}", yaml_content);
    let pr_doc: PrDoc = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

    // Track failed tests
    let mut failed_tests = vec![];
    let mut failed_clippy_checks = vec![];

    // Loop over each crate and run `cargo test`
    let ignored_crates: HashSet<_> = vec![
        "pallet-xcm",
        "parachains-common",
        "staging-xcm-builder",
        "xcm-runtime-apis",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    for krate in pr_doc.crates {
        println!("Running tests for: {}", krate.name);
        let mut args = vec!["test", "-p", &krate.name];
        if !ignored_crates.contains(&krate.name) {
            args.push("--all-features");
        }

        // Run `cargo test`
        let status = Command::new("cargo")
            .args(args)
            .current_dir(&working_dir)
            .status()
            .expect("Failed to execute cargo test");

        if status.success() {
            println!("✅ Tests passed for {}", krate.name);

            // Run `cargo clippy`
            let clippy_status = Command::new("cargo")
                .args(["clippy", "-p", &krate.name])
                .current_dir(&working_dir)
                .status()
                .expect("Failed to execute cargo clippy");
            if clippy_status.success() {
                println!("✅ Clippy passed for {}", krate.name);
            } else {
                eprintln!("❌ Clippy failed for {}", krate.name);
                failed_clippy_checks.push(krate.name);
            }
        } else {
            eprintln!("❌ Tests failed for {}", krate.name);
            failed_tests.push(krate.name);
        }
    }

    // Print summary if any tests failed
    if failed_tests.is_empty() && failed_clippy_checks.is_empty() {
        println!("\n🎉 All tests passed successfully!");
    } else {
        if !failed_tests.is_empty() {
            let mut failed_mods = String::new();
            eprintln!("\n🚨 Some tests failed:");
            for failed in failed_tests {
                eprintln!("- {}", failed);
                failed_mods.push_str(&format!("-p {} ", failed));
            }
            eprintln!("\nTo run the tests for the failed crates, use:");
            eprintln!("cargo test {failed_mods}");
        }

        if !failed_clippy_checks.is_empty() {
            let mut failed_mods = String::new();
            eprintln!("\n🚨 Some clippy checks failed:");
            for failed in failed_clippy_checks {
                eprintln!("- {}", failed);
                failed_mods.push_str(&format!("-p {} ", failed));
            }
            eprintln!("\nTo run the clippy checks for the failed crates, use:");
            eprintln!("cargo clippy {failed_mods}");
        }

        std::process::exit(1);
    }
}
