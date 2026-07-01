#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic module-guide reference checker.
//!
//! The pure core checks that Markdown links to `src/<module>.rs` point at files
//! that exist and are exported from `src/lib.rs`. The shell owns file reads.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const GUIDE_PATH: &str = "docs/module-guide.md";
const LIB_PATH: &str = "src/lib.rs";
const SRC_PREFIX: &str = "src/";
const DOC_SRC_PREFIX: &str = "../src/";
const RUST_EXTENSION: &str = ".rs";
const IDENT_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const REQUIRED_GOAL_LABELS: &[&str] = &[
    "bounded arithmetic",
    "quorum admission",
    "authorization",
    "memory layout",
    "compiler analysis",
];

#[derive(Debug, Eq, PartialEq)]
struct GuideCheck {
    links: BTreeSet<String>,
    errors: Vec<String>,
}

fn is_ident(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|ch| IDENT_CHARS.contains(ch))
}

fn extract_declared_modules(lib_rs: &str) -> BTreeSet<String> {
    let mut modules = BTreeSet::new();
    for line in lib_rs.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("pub mod ") {
            continue;
        }
        let rest = &trimmed["pub mod ".len()..];
        let Some(name) = rest.strip_suffix(';') else {
            continue;
        };
        if is_ident(name) {
            modules.insert(name.to_string());
        }
    }
    modules
}

fn normalize_source_href(href: &str) -> Option<String> {
    let without_fragment = href.split_once('#').map_or(href, |(path, _fragment)| path);
    let source_path = without_fragment
        .strip_prefix(DOC_SRC_PREFIX)
        .or_else(|| without_fragment.strip_prefix(SRC_PREFIX))?;
    if !source_path.ends_with(RUST_EXTENSION) || source_path.contains('/') {
        return None;
    }
    Some(format!("{SRC_PREFIX}{source_path}"))
}

fn extract_source_links(markdown: &str) -> BTreeSet<String> {
    let mut links = BTreeSet::new();
    let mut rest = markdown;
    while let Some(open_index) = rest.find("](") {
        rest = &rest[open_index + "](".len()..];
        let Some(close_index) = rest.find(')') else {
            break;
        };
        let href = &rest[..close_index];
        if let Some(source_path) = normalize_source_href(href) {
            links.insert(source_path);
        }
        rest = &rest[close_index + ")".len()..];
    }
    links
}

fn module_name_from_source_path(source_path: &str) -> Option<String> {
    let relative = source_path.strip_prefix(SRC_PREFIX)?;
    let module = relative.strip_suffix(RUST_EXTENSION)?;
    if is_ident(module) {
        Some(module.to_string())
    } else {
        None
    }
}

fn missing_goal_labels(markdown: &str) -> Vec<String> {
    let lower = markdown.to_ascii_lowercase();
    REQUIRED_GOAL_LABELS
        .iter()
        .filter(|label| !lower.contains(**label))
        .map(|label| (*label).to_string())
        .collect()
}

fn check_guide(markdown: &str, lib_rs: &str, existing_source_paths: &BTreeSet<String>) -> GuideCheck {
    let declared = extract_declared_modules(lib_rs);
    let links = extract_source_links(markdown);
    let mut errors = Vec::new();

    if links.is_empty() {
        errors.push(format!("{GUIDE_PATH} must link at least one public module source file"));
    }

    for label in missing_goal_labels(markdown) {
        errors.push(format!("{GUIDE_PATH} is missing goal label `{label}`"));
    }

    for source_path in &links {
        if !existing_source_paths.contains(source_path) {
            errors.push(format!("module guide link `{source_path}` does not exist"));
            continue;
        }
        let Some(module_name) = module_name_from_source_path(source_path) else {
            errors.push(format!("module guide link `{source_path}` is not a flat Rust module path"));
            continue;
        };
        if !declared.contains(&module_name) {
            errors.push(format!(
                "module guide link `{source_path}` references `{module_name}`, which is not declared in {LIB_PATH}"
            ));
        }
    }

    GuideCheck { links, errors }
}

fn repo_root_from_args() -> Result<Option<PathBuf>, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(Some(PathBuf::from("."))),
        [flag] if flag == "--self-test" => Ok(None),
        [root] => Ok(Some(PathBuf::from(root))),
        _ => Err("usage: check-module-guide.rs [--self-test|ROOT]".to_string()),
    }
}

fn collect_existing_source_paths(root: &Path, links: &BTreeSet<String>) -> BTreeSet<String> {
    links.iter().filter(|source_path| root.join(source_path).is_file()).cloned().collect()
}

fn run(root: &Path) -> Result<GuideCheck, String> {
    let guide =
        fs::read_to_string(root.join(GUIDE_PATH)).map_err(|err| format!("failed to read {GUIDE_PATH}: {err}"))?;
    let lib_rs = fs::read_to_string(root.join(LIB_PATH)).map_err(|err| format!("failed to read {LIB_PATH}: {err}"))?;
    let links = extract_source_links(&guide);
    let existing = collect_existing_source_paths(root, &links);
    Ok(check_guide(&guide, &lib_rs, &existing))
}

fn self_test_markdown(module: &str) -> String {
    format!(
        "# Guide\n\nbounded arithmetic quorum admission authorization memory layout compiler analysis\n\nUse [`{module}`](../src/{module}.rs).\n"
    )
}

fn self_test() -> Result<(), String> {
    let lib_rs = "pub mod arithmetic;\npub mod quorum;\n";
    let mut existing = BTreeSet::new();
    existing.insert("src/arithmetic.rs".to_string());
    let positive = check_guide(&self_test_markdown("arithmetic"), lib_rs, &existing);
    if !positive.errors.is_empty() {
        return Err(format!("positive fixture failed: {}", positive.errors.join("; ")));
    }

    let stale = check_guide(&self_test_markdown("missing"), lib_rs, &existing);
    if !stale.errors.iter().any(|err| err.contains("does not exist")) {
        return Err(format!("stale-link fixture missed expected error: {}", stale.errors.join("; ")));
    }

    let missing_goal = check_guide("# Guide\n\nUse [`arithmetic`](../src/arithmetic.rs).\n", lib_rs, &existing);
    if !missing_goal.errors.iter().any(|err| err.contains("goal label")) {
        return Err(format!("missing-goal fixture missed expected error: {}", missing_goal.errors.join("; ")));
    }

    Ok(())
}

fn main() -> ExitCode {
    match repo_root_from_args() {
        Ok(None) => match self_test() {
            Ok(()) => {
                println!("✓ module guide checker self-test passed");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("✗ module guide checker self-test failed: {err}");
                ExitCode::FAILURE
            }
        },
        Ok(Some(root)) => match run(&root) {
            Ok(check) if check.errors.is_empty() => {
                println!("✓ module guide links {} public module source files", check.links.len());
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
