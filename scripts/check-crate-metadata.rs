#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic Cargo package metadata checker.
//!
//! This validates the discoverability fields that Cargo and crates.io readers
//! use without changing the crate identity. The pure core consumes manifest text
//! plus local path status; the shell owns filesystem and git-ignore checks.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

const CARGO_TOML_PATH: &str = "Cargo.toml";
const EXPECTED_PACKAGE_NAME: &str = "verified-logic";
const EXPECTED_README_PATH: &str = "README.md";
const URL_SCHEME: &str = "https://";
const MIN_KEYWORDS: usize = 1;
const MAX_KEYWORDS: usize = 5;
const MIN_CATEGORIES: usize = 1;
const MAX_CATEGORIES: usize = 5;
const MAX_KEYWORD_CHARS: usize = 20;
const FIELD_NAME: &str = "name";
const FIELD_REPOSITORY: &str = "repository";
const FIELD_HOMEPAGE: &str = "homepage";
const FIELD_DOCUMENTATION: &str = "documentation";
const FIELD_README: &str = "readme";
const FIELD_KEYWORDS: &str = "keywords";
const FIELD_CATEGORIES: &str = "categories";
const REQUIRED_STRING_FIELDS: &[&str] = &[
    FIELD_NAME,
    FIELD_REPOSITORY,
    FIELD_HOMEPAGE,
    FIELD_DOCUMENTATION,
    FIELD_README,
];
const URL_FIELDS: &[&str] = &[FIELD_REPOSITORY, FIELD_HOMEPAGE, FIELD_DOCUMENTATION];
const ALLOWED_CATEGORIES: &[&str] = &["algorithms", "data-structures", "development-tools"];
const KEYWORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";

#[derive(Clone, Debug, Eq, PartialEq)]
struct PathStatus {
    exists: bool,
    ignored: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PackageMetadata {
    strings: BTreeMap<String, String>,
    arrays: BTreeMap<String, Vec<String>>,
}

impl PackageMetadata {
    fn string_field(&self, field: &str) -> Option<&str> {
        self.strings.get(field).map(String::as_str)
    }

    fn array_field(&self, field: &str) -> Option<&[String]> {
        self.arrays.get(field).map(Vec::as_slice)
    }
}

fn package_section(manifest: &str) -> Result<String, String> {
    let mut in_package = false;
    let mut lines = Vec::new();
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            continue;
        }
        if in_package && trimmed.starts_with('[') {
            break;
        }
        if in_package {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        Err(format!("{CARGO_TOML_PATH} is missing a [package] section"))
    } else {
        Ok(lines.join("\n"))
    }
}

fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        Some(index) => &line[..index],
        None => line,
    }
}

fn parse_string_literal(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let body = trimmed.strip_prefix('"')?.strip_suffix('"')?;
    if body.contains('"') {
        return None;
    }
    Some(body.to_string())
}

fn parse_array_literal(value: &str) -> Option<Vec<String>> {
    let trimmed = value.trim();
    let body = trimmed.strip_prefix('[')?.strip_suffix(']')?.trim();
    if body.is_empty() {
        return Some(Vec::new());
    }
    let mut values = Vec::new();
    for raw in body.split(',') {
        values.push(parse_string_literal(raw.trim())?);
    }
    Some(values)
}

fn parse_package_metadata(manifest: &str) -> Result<PackageMetadata, String> {
    let section = package_section(manifest)?;
    let mut strings = BTreeMap::new();
    let mut arrays = BTreeMap::new();
    for line in section.lines() {
        let trimmed = strip_comment(line).trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((raw_key, raw_value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = raw_key.trim();
        let value = raw_value.trim();
        if let Some(parsed) = parse_string_literal(value) {
            strings.insert(key.to_string(), parsed);
            continue;
        }
        if let Some(parsed) = parse_array_literal(value) {
            arrays.insert(key.to_string(), parsed);
        }
    }
    Ok(PackageMetadata { strings, arrays })
}

fn valid_keyword(keyword: &str) -> bool {
    !keyword.is_empty()
        && keyword.chars().count() <= MAX_KEYWORD_CHARS
        && keyword.chars().all(|ch| KEYWORD_CHARS.contains(ch))
}

fn validate_array_count(field: &str, values: &[String], min: usize, max: usize, errors: &mut Vec<String>) {
    if values.len() < min {
        errors.push(format!("{field} must contain at least {min} item"));
    }
    if values.len() > max {
        errors.push(format!("{field} must contain at most {max} items"));
    }
}

fn validate_package(metadata: &PackageMetadata, readme_status: PathStatus) -> Vec<String> {
    let mut errors = Vec::new();
    for field in REQUIRED_STRING_FIELDS {
        match metadata.string_field(field) {
            Some(value) if !value.trim().is_empty() => {}
            Some(_) => errors.push(format!("{field} must not be empty")),
            None => errors.push(format!("missing package metadata field `{field}`")),
        }
    }

    if metadata.string_field(FIELD_NAME) != Some(EXPECTED_PACKAGE_NAME) {
        errors.push(format!("package name must remain `{EXPECTED_PACKAGE_NAME}` for crate identity stability"));
    }

    for field in URL_FIELDS {
        if let Some(value) = metadata.string_field(field) {
            if !value.starts_with(URL_SCHEME) {
                errors.push(format!("{field} must be an {URL_SCHEME} URL"));
            }
        }
    }

    match metadata.string_field(FIELD_README) {
        Some(EXPECTED_README_PATH) => {
            if !readme_status.exists {
                errors.push(format!("readme path `{EXPECTED_README_PATH}` does not exist"));
            }
            if readme_status.ignored {
                errors.push(format!("readme path `{EXPECTED_README_PATH}` is ignored by source filtering"));
            }
        }
        Some(value) => errors.push(format!("readme must be `{EXPECTED_README_PATH}`, got `{value}`")),
        None => {}
    }

    match metadata.array_field(FIELD_KEYWORDS) {
        Some(values) => {
            validate_array_count(FIELD_KEYWORDS, values, MIN_KEYWORDS, MAX_KEYWORDS, &mut errors);
            for keyword in values {
                if !valid_keyword(keyword) {
                    errors.push(format!("invalid keyword `{keyword}`"));
                }
            }
        }
        None => errors.push(format!("missing package metadata field `{FIELD_KEYWORDS}`")),
    }

    match metadata.array_field(FIELD_CATEGORIES) {
        Some(values) => {
            validate_array_count(FIELD_CATEGORIES, values, MIN_CATEGORIES, MAX_CATEGORIES, &mut errors);
            for category in values {
                if !ALLOWED_CATEGORIES.contains(&category.as_str()) {
                    errors.push(format!("unsupported package category `{category}`"));
                }
            }
        }
        None => errors.push(format!("missing package metadata field `{FIELD_CATEGORIES}`")),
    }

    errors
}

fn check_manifest(manifest: &str, readme_status: PathStatus) -> Result<(), Vec<String>> {
    let metadata = parse_package_metadata(manifest).map_err(|err| vec![err])?;
    let errors = validate_package(&metadata, readme_status);
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn repo_root_from_args() -> Result<PathBuf, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(PathBuf::from(".")),
        [root] if root != "--self-test" => Ok(PathBuf::from(root)),
        [flag] if flag == "--self-test" => Err("self-test".to_string()),
        _ => Err("usage: check-crate-metadata.rs [--self-test|ROOT]".to_string()),
    }
}

fn git_ignored(root: &Path, relative_path: &str) -> Result<bool, String> {
    let output = Command::new("git")
        .args(["check-ignore", "--quiet", relative_path])
        .current_dir(root)
        .output()
        .map_err(|err| format!("failed to run git check-ignore: {err}"))?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        Some(code) => Err(format!("git check-ignore exited with status {code}")),
        None => Err("git check-ignore terminated by signal".to_string()),
    }
}

fn load_path_status(root: &Path, relative_path: &str) -> Result<PathStatus, String> {
    Ok(PathStatus {
        exists: root.join(relative_path).is_file(),
        ignored: git_ignored(root, relative_path)?,
    })
}

fn run(root: &Path) -> Result<(), Vec<String>> {
    let manifest = fs::read_to_string(root.join(CARGO_TOML_PATH))
        .map_err(|err| vec![format!("failed to read {CARGO_TOML_PATH}: {err}")])?;
    let readme_status = load_path_status(root, EXPECTED_README_PATH).map_err(|err| vec![err])?;
    check_manifest(&manifest, readme_status)
}

fn positive_manifest() -> &'static str {
    r#"
[package]
name = "verified-logic"
version = "0.2.0"
edition = "2024"
repository = "https://github.com/OnixResearch/trellis"
homepage = "https://github.com/OnixResearch/trellis"
documentation = "https://docs.rs/verified-logic"
readme = "README.md"
keywords = ["verification", "verus", "logic"]
categories = ["algorithms", "data-structures"]

[dependencies]
"#
}

fn assert_error_contains(name: &str, manifest: &str, readme_status: PathStatus, expected: &str) -> Result<(), String> {
    match check_manifest(manifest, readme_status) {
        Ok(()) => Err(format!("{name}: expected failure containing `{expected}`")),
        Err(errors) if errors.iter().any(|err| err.contains(expected)) => Ok(()),
        Err(errors) => Err(format!("{name}: expected failure containing `{expected}`, got {}", errors.join("; "))),
    }
}

fn self_test() -> Result<(), String> {
    let present = PathStatus {
        exists: true,
        ignored: false,
    };
    check_manifest(positive_manifest(), present.clone()).map_err(|errors| errors.join("; "))?;

    let missing_readme = PathStatus {
        exists: false,
        ignored: false,
    };
    assert_error_contains("missing readme", positive_manifest(), missing_readme, "does not exist")?;

    let bad_url = positive_manifest().replace(
        "repository = \"https://github.com/OnixResearch/trellis\"",
        "repository = \"git://example.invalid/trellis\"",
    );
    assert_error_contains("bad url", &bad_url, present.clone(), "https://")?;

    let bad_category = positive_manifest()
        .replace("categories = [\"algorithms\", \"data-structures\"]", "categories = [\"made-up-category\"]");
    assert_error_contains("bad category", &bad_category, present, "unsupported package category")?;

    Ok(())
}

fn main() -> ExitCode {
    match repo_root_from_args() {
        Err(flag) if flag == "self-test" => match self_test() {
            Ok(()) => {
                println!("✓ crate metadata checker self-test passed");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("✗ crate metadata checker self-test failed: {err}");
                ExitCode::FAILURE
            }
        },
        Err(usage) => {
            eprintln!("{usage}");
            ExitCode::FAILURE
        }
        Ok(root) => match run(&root) {
            Ok(()) => {
                println!("✓ Cargo package metadata is discoverable and locally valid");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                for error in errors {
                    eprintln!("✗ {error}");
                }
                ExitCode::FAILURE
            }
        },
    }
}
