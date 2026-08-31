#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Enforce: every non-test public Rust exec function in `src/**/*.rs` has an `r[impl ...]` Tracey
//! annotation. Exemptions: add `// @verified-logic:exempt <reason>` above the `pub fn`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const LOOKBACK: usize = 10;
const MAX_SOURCE_FILES: usize = 20_000;
const MAX_SOURCE_DIRECTORIES: usize = 2_000;
const SELF_TEST_SOURCE_FILE_COUNT: usize = 2;

#[derive(Default)]
struct Report {
    failed: bool,
    checked: usize,
    exempt: usize,
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            PathBuf::from(stdout.trim())
        }
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn is_cfg_test(line: &str) -> bool {
    let compact: String = line.chars().filter(|ch| !ch.is_whitespace()).collect();
    compact.starts_with("#[cfg(test)]")
}

fn starts_spec_fn(trimmed: &str) -> bool {
    trimmed.starts_with("spec fn ")
        || trimmed.starts_with("open spec fn ")
        || trimmed.starts_with("pub spec fn ")
        || trimmed.starts_with("pub open spec fn ")
        || trimmed.starts_with("closed spec fn ")
        || trimmed.starts_with("pub closed spec fn ")
}

fn starts_proof_fn(trimmed: &str) -> bool {
    trimmed.starts_with("proof fn ") || trimmed.starts_with("pub proof fn ")
}

fn starts_private_fn(trimmed: &str) -> bool {
    trimmed.starts_with("fn ") && !trimmed.starts_with("pub ")
}

fn starts_pub_fn(trimmed: &str) -> bool {
    trimmed.starts_with("pub fn ")
}

fn has_prior_marker(lines: &[String], n: usize, needle: &str) -> bool {
    let start = n.saturating_sub(LOOKBACK);
    let mut found = false;
    for prev in &lines[start..n] {
        if prev.contains(needle) {
            found = true;
            break;
        }
        let trimmed = prev.trim();
        if trimmed.is_empty() || trimmed.starts_with('}') {
            found = false;
        }
    }
    found
}

fn check_file(path: &Path, display_path: &str, report: &mut Report) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let lines: Vec<String> = text.lines().map(ToOwned::to_owned).collect();
    let mut in_test_mod = false;

    for (n, line) in lines.iter().enumerate() {
        if is_cfg_test(line) {
            in_test_mod = true;
        }
        if in_test_mod {
            continue;
        }

        let trimmed = line.trim_start();
        if starts_spec_fn(trimmed) || starts_proof_fn(trimmed) || starts_private_fn(trimmed) {
            continue;
        }

        if starts_pub_fn(trimmed) {
            report.checked += 1;
            let line_num = n + 1;
            let is_exempt = has_prior_marker(&lines, n, "@verified-logic:exempt");
            if is_exempt {
                report.exempt += 1;
                continue;
            }
            let has_impl = has_prior_marker(&lines, n, "r[impl ");
            if !has_impl {
                println!("MISSING r[impl]: {display_path}:{line_num}: {}", trimmed);
                report.failed = true;
            }
        }
    }

    Ok(())
}

fn collect_src_rs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let src = root.join("src");
    let mut pending = vec![src.clone()];
    let mut files = Vec::new();
    let mut directory_count = 0usize;

    while let Some(directory) = pending.pop() {
        directory_count += 1;
        if directory_count > MAX_SOURCE_DIRECTORIES {
            return Err(format!(
                "source directory count exceeds {MAX_SOURCE_DIRECTORIES} under {}",
                src.display()
            ));
        }

        for entry in fs::read_dir(&directory)
            .map_err(|err| format!("failed to read {}: {err}", directory.display()))?
        {
            let entry = entry.map_err(|err| format!("failed to read source entry: {err}"))?;
            let file_type = entry
                .file_type()
                .map_err(|err| format!("failed to inspect {}: {err}", entry.path().display()))?;
            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if !file_type.is_file() || path.file_name().and_then(|name| name.to_str()) == Some("lib.rs") {
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                if files.len() >= MAX_SOURCE_FILES {
                    return Err(format!("source file count exceeds {MAX_SOURCE_FILES} under {}", src.display()));
                }
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

fn run_self_test() -> Result<(), String> {
    let fixture = std::env::temp_dir().join(format!("trellis-check-coverage-{}", std::process::id()));
    if fixture.exists() {
        fs::remove_dir_all(&fixture).map_err(|err| format!("failed to reset fixture: {err}"))?;
    }
    let nested = fixture.join("src/nested");
    fs::create_dir_all(&nested).map_err(|err| format!("failed to create fixture: {err}"))?;
    fs::write(fixture.join("src/lib.rs"), "pub mod nested;\n")
        .map_err(|err| format!("failed to write fixture lib: {err}"))?;
    fs::write(fixture.join("src/root.rs"), "pub fn root() {}\n")
        .map_err(|err| format!("failed to write root fixture: {err}"))?;
    fs::write(nested.join("mod.rs"), "pub fn nested() {}\n")
        .map_err(|err| format!("failed to write nested fixture: {err}"))?;

    let files = collect_src_rs(&fixture)?;
    if files.len() != SELF_TEST_SOURCE_FILE_COUNT
        || !files.iter().any(|path| path.ends_with("src/root.rs"))
        || !files.iter().any(|path| path.ends_with("src/nested/mod.rs"))
    {
        return Err("recursive source collection missed a positive fixture".to_string());
    }
    if collect_src_rs(&fixture.join("missing")).is_ok() {
        return Err("missing source roots must fail".to_string());
    }

    fs::remove_dir_all(&fixture).map_err(|err| format!("failed to remove fixture: {err}"))?;
    println!("✓ recursive source collection self-test passed");
    Ok(())
}

fn checker_root() -> Result<PathBuf, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(repo_root()),
        [flag, path] if flag == "--root" => Ok(PathBuf::from(path)),
        _ => Err("usage: check-coverage.rs [--root <repo-or-fixture-root>] | --self-test".to_string()),
    }
}

fn run_check(root: &Path) -> Result<(), String> {
    std::env::set_current_dir(root).map_err(|err| format!("failed to cd {}: {err}", root.display()))?;
    let mut report = Report::default();

    for path in collect_src_rs(root)? {
        let display = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        check_file(&path, &display, &mut report)?;
    }

    println!();
    println!("Checked {} pub fn declarations, {} exempted", report.checked, report.exempt);
    if report.failed {
        println!();
        println!("✗ pub fns found without r[impl] annotations");
        println!("  Fix: add '// r[impl requirement.id]' above the function");
        println!("  Or:  add '// @verified-logic:exempt <reason>' to exempt");
        std::process::exit(1);
    }
    println!("✓ all pub fns have r[impl] annotations");
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.as_slice() == ["--self-test"] {
        run_self_test()
    } else {
        run_check(&checker_root()?)
    }
}
