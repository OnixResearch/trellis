#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Enforce: every public exec function inside a `verus!` block has an `ensures` clause.
//! Exemptions: add `// @verified-logic:no-ensures <reason>` above the `pub fn`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
struct Report {
    failed: bool,
    checked: usize,
    exempt: usize,
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()),
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn compact(line: &str) -> String {
    line.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn is_cfg_test(line: &str) -> bool {
    compact(line).starts_with("#[cfg(test)]")
}

fn is_verus_start(line: &str) -> bool {
    let c = compact(line);
    c == "verus!{" || c.starts_with("verus!{")
}

fn is_verus_end(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed == "} // verus!" || trimmed == "} // verus"
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

fn has_no_ensures_exemption(lines: &[String], n: usize) -> bool {
    let start = n.saturating_sub(5);
    lines[start..n].iter().any(|line| line.contains("@verified-logic:no-ensures"))
}

fn code_before_comment(line: &str) -> &str {
    line.split_once("//").map_or(line, |(code, _)| code)
}

fn has_word_ensures(code: &str) -> bool {
    let bytes = code.as_bytes();
    let needle = b"ensures";
    if bytes.len() < needle.len() {
        return false;
    }
    for i in 0..=bytes.len() - needle.len() {
        if &bytes[i..i + needle.len()] == needle {
            let before = i == 0 || !is_ident_byte(bytes[i - 1]);
            let after_idx = i + needle.len();
            let after = after_idx == bytes.len() || !is_ident_byte(bytes[after_idx]);
            if before && after {
                return true;
            }
        }
    }
    false
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn signature_has_ensures(lines: &[String], n: usize) -> bool {
    for line in &lines[n..] {
        let code = code_before_comment(line);
        if has_word_ensures(code) {
            return true;
        }
        if code.trim_start().starts_with('{') {
            break;
        }
    }
    false
}

fn check_file(path: &Path, display_path: &str, report: &mut Report) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let lines: Vec<String> = text.lines().map(ToOwned::to_owned).collect();
    let mut in_verus = false;
    let mut in_test_mod = false;

    for (n, line) in lines.iter().enumerate() {
        if is_verus_start(line) {
            in_verus = true;
        }
        if is_cfg_test(line) {
            in_test_mod = true;
        }
        if in_test_mod {
            if is_verus_end(line) {
                in_verus = false;
            }
            continue;
        }
        if !in_verus {
            if is_verus_end(line) {
                in_verus = false;
            }
            continue;
        }

        let trimmed = line.trim_start();
        if starts_spec_fn(trimmed) || starts_proof_fn(trimmed) || starts_private_fn(trimmed) {
            if is_verus_end(line) {
                in_verus = false;
            }
            continue;
        }

        if starts_pub_fn(trimmed) {
            report.checked += 1;
            let line_num = n + 1;
            if has_no_ensures_exemption(&lines, n) {
                report.exempt += 1;
                if is_verus_end(line) {
                    in_verus = false;
                }
                continue;
            }
            if !signature_has_ensures(&lines, n) {
                println!("MISSING ensures: {display_path}:{line_num}: {trimmed}");
                report.failed = true;
            }
        }
        if is_verus_end(line) {
            in_verus = false;
        }
    }
    Ok(())
}

fn collect_src_rs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let src = root.join("src");
    let mut files = Vec::new();
    for entry in fs::read_dir(&src).map_err(|err| format!("failed to read {}: {err}", src.display()))? {
        let entry = entry.map_err(|err| format!("failed to read src entry: {err}"))?;
        let path = entry.path();
        if path.file_name().and_then(|name| name.to_str()) == Some("lib.rs") {
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn checker_root() -> Result<PathBuf, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(repo_root()),
        [flag, path] if flag == "--root" => Ok(PathBuf::from(path)),
        _ => Err("usage: check-ensures.rs [--root <repo-or-fixture-root>]".to_string()),
    }
}

fn main() -> Result<(), String> {
    let root = checker_root()?;
    std::env::set_current_dir(&root).map_err(|err| format!("failed to cd {}: {err}", root.display()))?;
    let mut report = Report::default();
    for path in collect_src_rs(&root)? {
        let display = path.strip_prefix(&root).unwrap_or(&path).display().to_string();
        check_file(&path, &display, &mut report)?;
    }

    println!();
    println!("Checked {} pub fn in verus! blocks, {} exempted", report.checked, report.exempt);
    if report.failed {
        println!();
        println!("✗ pub fns in verus! blocks found without ensures");
        println!("  Fix: add 'ensures result == *_spec(args)' to the function");
        println!("  Or:  add '// @verified-logic:no-ensures <reason>' to exempt");
        std::process::exit(1);
    }
    println!("✓ all verified pub fns have ensures clauses");
    Ok(())
}
