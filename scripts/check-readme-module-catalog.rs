#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic README module catalog drift checker.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const README_PATH: &str = "README.md";
const LIB_PATH: &str = "src/lib.rs";
const MODULES_HEADING: &str = "## Modules";
const EXCLUSIONS_HEADING: &str = "### README module catalog exclusions";
const IDENT_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[derive(Debug)]
struct CatalogCheck {
    declared: BTreeSet<String>,
    cataloged: BTreeSet<String>,
    excluded: BTreeSet<String>,
}

impl CatalogCheck {
    fn missing(&self) -> Vec<String> {
        self.declared
            .difference(&self.cataloged)
            .filter(|module| !self.excluded.contains(*module))
            .cloned()
            .collect()
    }

    fn stale(&self) -> Vec<String> {
        self.cataloged
            .difference(&self.declared)
            .filter(|module| !self.excluded.contains(*module))
            .cloned()
            .collect()
    }

    fn invalid_exclusions(&self) -> Vec<String> {
        self.excluded
            .iter()
            .filter(|module| !self.declared.contains(*module))
            .cloned()
            .collect()
    }
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

fn modules_section(readme: &str) -> Result<String, String> {
    let mut in_modules = false;
    let mut section = Vec::new();
    for line in readme.lines() {
        if line.trim() == MODULES_HEADING {
            in_modules = true;
            section.push(line);
            continue;
        }
        if in_modules && line.starts_with("## ") {
            break;
        }
        if in_modules {
            section.push(line);
        }
    }
    if section.is_empty() {
        Err(format!("README is missing `{MODULES_HEADING}` section"))
    } else {
        Ok(section.join("\n"))
    }
}

fn extract_backtick_idents(text: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut rest = text;
    while let Some(start) = rest.find('`') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('`') else { break };
        let token = &rest[..end];
        if is_ident(token) {
            names.insert(token.to_string());
        }
        rest = &rest[end + 1..];
    }
    names
}

fn extract_exclusions(section: &str) -> Result<BTreeSet<String>, String> {
    let mut exclusions = BTreeSet::new();
    let mut in_exclusions = false;
    for (index, line) in section.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed == EXCLUSIONS_HEADING {
            in_exclusions = true;
            continue;
        }
        if in_exclusions && trimmed.starts_with("### ") {
            break;
        }
        if !in_exclusions || trimmed.is_empty() {
            continue;
        }
        if !trimmed.starts_with("- `") {
            return Err(format!(
                "invalid exclusion format at README line within modules section {}: expected `- `module` — rationale`",
                index + 1
            ));
        }
        let Some(name_end) = trimmed[3..].find('`') else {
            return Err(format!("unterminated exclusion module at modules-section line {}", index + 1));
        };
        let name = &trimmed[3..3 + name_end];
        let rationale = trimmed[3 + name_end + 1..].trim();
        if !is_ident(name) {
            return Err(format!("invalid exclusion module `{name}` at modules-section line {}", index + 1));
        }
        if rationale.is_empty() || rationale == "—" || rationale == "-" {
            return Err(format!("missing exclusion rationale for `{name}` at modules-section line {}", index + 1));
        }
        exclusions.insert(name.to_string());
    }
    Ok(exclusions)
}

fn is_ident(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|ch| IDENT_CHARS.contains(ch))
}

fn load_check(root: &Path) -> Result<CatalogCheck, String> {
    let lib_rs = fs::read_to_string(root.join(LIB_PATH))
        .map_err(|err| format!("failed to read {LIB_PATH}: {err}"))?;
    let readme = fs::read_to_string(root.join(README_PATH))
        .map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    let section = modules_section(&readme)?;
    let excluded = extract_exclusions(&section)?;
    Ok(CatalogCheck {
        declared: extract_declared_modules(&lib_rs),
        cataloged: extract_backtick_idents(&section),
        excluded,
    })
}

fn run_check(root: &Path) -> Result<(), String> {
    let check = load_check(root)?;
    let missing = check.missing();
    let stale = check.stale();
    let invalid_exclusions = check.invalid_exclusions();

    if missing.is_empty() && stale.is_empty() && invalid_exclusions.is_empty() {
        println!(
            "✓ README module catalog covers {} public modules ({} catalog entries, {} exclusions)",
            check.declared.len(),
            check.cataloged.len(),
            check.excluded.len()
        );
        return Ok(());
    }

    if !missing.is_empty() {
        eprintln!("missing README module catalog entries ({}):", missing.len());
        for module in &missing {
            eprintln!("  - `{module}`");
        }
    }
    if !stale.is_empty() {
        eprintln!("stale README module catalog entries ({}):", stale.len());
        for module in &stale {
            eprintln!("  - `{module}`");
        }
    }
    if !invalid_exclusions.is_empty() {
        eprintln!("invalid README module catalog exclusions ({}):", invalid_exclusions.len());
        for module in &invalid_exclusions {
            eprintln!("  - `{module}`");
        }
    }
    Err("README module catalog drift detected".to_string())
}

fn write_fixture(root: &Path, lib: &str, readme: &str) -> Result<(), String> {
    fs::create_dir_all(root.join("src")).map_err(|err| err.to_string())?;
    fs::write(root.join(LIB_PATH), lib).map_err(|err| err.to_string())?;
    fs::write(root.join(README_PATH), readme).map_err(|err| err.to_string())?;
    Ok(())
}

fn temp_root() -> Result<PathBuf, String> {
    let mut root = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_nanos();
    root.push(format!("readme-module-catalog-fixtures-{nanos}"));
    fs::create_dir_all(&root).map_err(|err| err.to_string())?;
    Ok(root)
}

fn run_self_test() -> Result<(), String> {
    let root = temp_root()?;
    let valid = root.join("valid");
    write_fixture(
        &valid,
        "pub mod alpha;\npub mod beta;\npub mod omitted;\n",
        "# Fixture\n\n## Modules\n\n| Module | Purpose |\n|---|---|\n| `alpha` | documented |\n| `beta` | documented |\n\n### README module catalog exclusions\n\n- `omitted` — intentionally omitted from the human catalog in this fixture.\n\n## Usage\n",
    )?;
    run_check(&valid).map_err(|err| format!("valid fixture failed: {err}"))?;

    let missing = root.join("missing");
    write_fixture(
        &missing,
        "pub mod alpha;\npub mod beta;\n",
        "# Fixture\n\n## Modules\n\n| Module | Purpose |\n|---|---|\n| `alpha` | documented |\n\n## Usage\n",
    )?;
    if run_check(&missing).is_ok() {
        return Err("missing-module fixture unexpectedly passed".to_string());
    }

    let stale = root.join("stale");
    write_fixture(
        &stale,
        "pub mod alpha;\n",
        "# Fixture\n\n## Modules\n\n| Module | Purpose |\n|---|---|\n| `alpha` | documented |\n| `removed` | stale |\n\n## Usage\n",
    )?;
    if run_check(&stale).is_ok() {
        return Err("stale-module fixture unexpectedly passed".to_string());
    }

    let _ = fs::remove_dir_all(root);
    println!("✓ README module catalog self-test fixtures passed");
    Ok(())
}

fn repo_root() -> PathBuf {
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
    {
        if output.status.success() {
            return PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [] => run_check(&repo_root()),
        [flag] if flag == "--self-test" => run_self_test(),
        [root] => run_check(Path::new(root)),
        _ => Err("usage: check-readme-module-catalog.rs [--self-test|ROOT]".to_string()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("✗ {err}");
            ExitCode::from(1)
        }
    }
}
