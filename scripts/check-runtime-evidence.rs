#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic runtime-cost evidence checker.
//!
//! The pure core validates that each evidence record carries command context,
//! profile/target fields, a result, and an observation/non-guarantee statement.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const EVIDENCE_PATH: &str = "docs/runtime-cost.md";
const RECORD_PREFIX: &str = "## Evidence record:";
const REQUIRED_FIELDS: &[&str] = &[
    "Command:",
    "Profile:",
    "Target:",
    "Result:",
    "Measurement status:",
    "Refresh:",
];
const OBSERVATION_WORD: &str = "observation";
const GUARANTEE_WORD: &str = "guarantee";

#[derive(Debug, Eq, PartialEq)]
struct RuntimeEvidenceCheck {
    record_count: usize,
    errors: Vec<String>,
}

fn record_sections(markdown: &str) -> Vec<(String, String)> {
    let mut records = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_body: Vec<String> = Vec::new();

    for line in markdown.lines() {
        if line.starts_with(RECORD_PREFIX) {
            if let Some(heading) = current_heading.take() {
                records.push((heading, current_body.join("\n")));
                current_body.clear();
            }
            current_heading = Some(line.trim().to_string());
            continue;
        }
        if current_heading.is_some() && line.starts_with("## ") {
            let heading = current_heading.take().expect("heading exists when record is open");
            records.push((heading, current_body.join("\n")));
            current_body.clear();
            continue;
        }
        if current_heading.is_some() {
            current_body.push(line.to_string());
        }
    }

    if let Some(heading) = current_heading {
        records.push((heading, current_body.join("\n")));
    }

    records
}

fn check_runtime_evidence(markdown: &str) -> RuntimeEvidenceCheck {
    let records = record_sections(markdown);
    let mut errors = Vec::new();

    if records.is_empty() {
        errors.push(format!("{EVIDENCE_PATH} must contain at least one `{RECORD_PREFIX}` section"));
    }

    for (heading, body) in &records {
        for field in REQUIRED_FIELDS {
            if !body.contains(field) {
                errors.push(format!("{heading} is missing required field `{field}`"));
            }
        }
        let status_lower = body.to_ascii_lowercase();
        if body.contains("Measurement status:")
            && !(status_lower.contains(OBSERVATION_WORD) && status_lower.contains(GUARANTEE_WORD))
        {
            errors.push(format!("{heading} measurement status must identify observations and non-guarantees"));
        }
    }

    RuntimeEvidenceCheck {
        record_count: records.len(),
        errors,
    }
}

fn repo_root_from_args() -> Result<Option<PathBuf>, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(Some(PathBuf::from("."))),
        [flag] if flag == "--self-test" => Ok(None),
        [root] => Ok(Some(PathBuf::from(root))),
        _ => Err("usage: check-runtime-evidence.rs [--self-test|ROOT]".to_string()),
    }
}

fn run(root: &Path) -> Result<RuntimeEvidenceCheck, String> {
    let evidence =
        fs::read_to_string(root.join(EVIDENCE_PATH)).map_err(|err| format!("failed to read {EVIDENCE_PATH}: {err}"))?;
    Ok(check_runtime_evidence(&evidence))
}

fn complete_fixture() -> &'static str {
    "# Runtime Cost\n\n## Evidence record: example\n\nCommand: `cargo build --example core_basics`\n\nProfile: dev\n\nTarget: example `core_basics`\n\nResult: build succeeded.\n\nMeasurement status: observation, not a guarantee.\n\nRefresh: rerun the command.\n"
}

fn self_test() -> Result<(), String> {
    let positive = check_runtime_evidence(complete_fixture());
    if !positive.errors.is_empty() {
        return Err(format!("positive fixture failed: {}", positive.errors.join("; ")));
    }

    let missing_field = complete_fixture().replace("Profile:", "Build profile:");
    let missing_field_check = check_runtime_evidence(&missing_field);
    if !missing_field_check.errors.iter().any(|err| err.contains("Profile:")) {
        return Err(format!("missing-field fixture missed expected error: {}", missing_field_check.errors.join("; ")));
    }

    let hard_guarantee = complete_fixture().replace("observation, not a guarantee", "hard performance promise");
    let hard_guarantee_check = check_runtime_evidence(&hard_guarantee);
    if !hard_guarantee_check.errors.iter().any(|err| err.contains("non-guarantees")) {
        return Err(format!("non-guarantee fixture missed expected error: {}", hard_guarantee_check.errors.join("; ")));
    }

    Ok(())
}

fn main() -> ExitCode {
    match repo_root_from_args() {
        Ok(None) => match self_test() {
            Ok(()) => {
                println!("✓ runtime evidence checker self-test passed");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("✗ runtime evidence checker self-test failed: {err}");
                ExitCode::FAILURE
            }
        },
        Ok(Some(root)) => match run(&root) {
            Ok(check) if check.errors.is_empty() => {
                println!("✓ runtime evidence contains {} complete records", check.record_count);
                ExitCode::SUCCESS
            }
            Ok(check) => {
                for error in check.errors {
                    eprintln!("✗ {error}");
                }
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("✗ {err}");
                ExitCode::FAILURE
            }
        },
        Err(usage) => {
            eprintln!("{usage}");
            ExitCode::FAILURE
        }
    }
}
