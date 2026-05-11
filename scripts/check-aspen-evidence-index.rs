#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_INDEX: &str = "docs/aspen-verified-logic-index.tsv";
const INDEX_HEADER: &str = "# verified-logic aspen-evidence-index-v1";
const FIELD_COUNT: usize = 12;

#[derive(Debug)]
struct Args {
    root: PathBuf,
    index: PathBuf,
    write_report: Option<PathBuf>,
    fixture_malformed_row: bool,
    fixture_missing_source_path: bool,
    fixture_missing_snapshot_manifest: bool,
}

#[derive(Clone, Debug)]
struct Row {
    id: String,
    source_repo: String,
    source_commit: String,
    source_paths: Vec<String>,
    source_blobs: Vec<String>,
    property_summary: String,
    verus_command: String,
    verus_result: String,
    focused_test_command: String,
    focused_test_result: String,
    remaining_trusted_boundaries: String,
    local_snapshot_manifest: String,
}

fn parse_args() -> Result<Args, String> {
    let mut root = PathBuf::from(".");
    let mut index = PathBuf::from(DEFAULT_INDEX);
    let mut write_report = None;
    let mut fixture_malformed_row = false;
    let mut fixture_missing_source_path = false;
    let mut fixture_missing_snapshot_manifest = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = PathBuf::from(args.next().ok_or("--root requires a path")?),
            "--index" => index = PathBuf::from(args.next().ok_or("--index requires a path")?),
            "--write-report" => {
                write_report = Some(PathBuf::from(args.next().ok_or("--write-report requires a path")?));
            }
            "--fixture-malformed-row" => fixture_malformed_row = true,
            "--fixture-missing-source-path" => fixture_missing_source_path = true,
            "--fixture-missing-snapshot-manifest" => fixture_missing_snapshot_manifest = true,
            "--help" | "-h" => {
                println!("usage: check-aspen-evidence-index.rs [--root PATH] [--index PATH] [--write-report PATH] [--fixture-malformed-row] [--fixture-missing-source-path] [--fixture-missing-snapshot-manifest]");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Args {
        root,
        index,
        write_report,
        fixture_malformed_row,
        fixture_missing_source_path,
        fixture_missing_snapshot_manifest,
    })
}

fn resolve_under(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn is_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_hexdigit())
}

fn parse_csv(field: &str) -> Vec<String> {
    field
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn parse_index(root: &Path, index: &Path, args: &Args) -> Result<Vec<Row>, String> {
    let path = resolve_under(root, index);
    let text = fs::read_to_string(&path).map_err(|err| format!("failed to read index {}: {err}", path.display()))?;
    let header = text.lines().next().ok_or("index is empty")?;
    if header.trim_end() != INDEX_HEADER {
        return Err(format!("index malformed: first line must be `{INDEX_HEADER}`"));
    }

    let mut rows = Vec::new();
    for (idx, line) in text.lines().enumerate().skip(1) {
        let trimmed = line.trim_end();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        if fields.len() != FIELD_COUNT || fields.iter().any(|field| field.trim().is_empty()) {
            return Err(format!(
                "index malformed at line {}: expected {FIELD_COUNT} tab-separated non-empty fields",
                idx + 1
            ));
        }
        let mut row = Row {
            id: fields[0].to_string(),
            source_repo: fields[1].to_string(),
            source_commit: fields[2].to_string(),
            source_paths: parse_csv(fields[3]),
            source_blobs: parse_csv(fields[4]),
            property_summary: fields[5].to_string(),
            verus_command: fields[6].to_string(),
            verus_result: fields[7].to_string(),
            focused_test_command: fields[8].to_string(),
            focused_test_result: fields[9].to_string(),
            remaining_trusted_boundaries: fields[10].to_string(),
            local_snapshot_manifest: fields[11].to_string(),
        };
        if args.fixture_malformed_row && rows.is_empty() {
            row.verus_result.clear();
        }
        if args.fixture_missing_source_path && rows.is_empty() && !row.source_paths.is_empty() {
            row.source_paths[0].push_str(".missing");
        }
        if args.fixture_missing_snapshot_manifest && rows.is_empty() {
            row.local_snapshot_manifest = "docs/aspen-proof-snapshots/missing.tsv".to_string();
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("index malformed: at least one evidence row is required".to_string());
    }
    Ok(rows)
}

fn git_stdout(repo: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|err| format!("failed to run git -C {} {}: {err}", repo.display(), args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git -C {} {} failed: {}",
            repo.display(),
            args.join(" "),
            stderr.trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn validate_path_field(name: &str, value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if path.is_absolute() || value.contains("..") || value.contains('\\') {
        return Err(format!("{name} `{value}` must be a relative repository path without `..`"));
    }
    Ok(())
}

fn validate_row(root: &Path, row: &Row) -> Result<(), String> {
    if row.id.chars().any(|ch| !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')) {
        return Err(format!("row {}: id must use lowercase letters, digits, and hyphens", row.id));
    }
    if row.source_repo.is_empty() || Path::new(&row.source_repo).is_absolute() || row.source_repo.contains("..") && row.source_repo != "../aspen" {
        return Err(format!("row {}: unsupported source_repo `{}`", row.id, row.source_repo));
    }
    if !is_hex(&row.source_commit, 40) {
        return Err(format!("row {}: source_commit must be a full 40-character hex commit", row.id));
    }
    if row.source_paths.is_empty() || row.source_paths.len() != row.source_blobs.len() {
        return Err(format!("row {}: source_paths and source_blobs must be non-empty and length-matched", row.id));
    }
    for path in &row.source_paths {
        validate_path_field("source path", path)?;
    }
    for blob in &row.source_blobs {
        if !is_hex(blob, 40) {
            return Err(format!("row {}: source blob `{blob}` must be a 40-character hex object id", row.id));
        }
    }
    for (label, value) in [
        ("property_summary", &row.property_summary),
        ("verus_command", &row.verus_command),
        ("verus_result", &row.verus_result),
        ("focused_test_command", &row.focused_test_command),
        ("focused_test_result", &row.focused_test_result),
        ("remaining_trusted_boundaries", &row.remaining_trusted_boundaries),
    ] {
        if value.trim().is_empty() {
            return Err(format!("row {}: {label} is required", row.id));
        }
    }
    if !row.verus_result.starts_with("pass:") || !row.focused_test_result.starts_with("pass:") {
        return Err(format!("row {}: verification results must begin with `pass:`", row.id));
    }
    if row.remaining_trusted_boundaries == "none" {
        return Err(format!("row {}: boundary notes must be explicit, not `none`", row.id));
    }
    if row.local_snapshot_manifest != "-" {
        validate_path_field("local snapshot manifest", &row.local_snapshot_manifest)?;
        let manifest = root.join(&row.local_snapshot_manifest);
        if !manifest.is_file() {
            return Err(format!("row {}: local snapshot manifest {} is missing", row.id, row.local_snapshot_manifest));
        }
    }

    let repo = resolve_under(root, Path::new(&row.source_repo));
    if !repo.join(".git").exists() {
        return Err(format!("row {}: source repo {} is not a git checkout", row.id, repo.display()));
    }
    git_stdout(&repo, &["cat-file", "-e", &format!("{}^{{commit}}", row.source_commit)])?;
    for (path, expected_blob) in row.source_paths.iter().zip(row.source_blobs.iter()) {
        let actual_blob = git_stdout(&repo, &["rev-parse", &format!("{}:{path}", row.source_commit)])?;
        if &actual_blob != expected_blob {
            return Err(format!(
                "row {}: source blob drift for {}: expected {}, got {}",
                row.id, path, expected_blob, actual_blob
            ));
        }
    }
    Ok(())
}

fn write_report(path: &Path, rows: &[Row]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("failed to create report directory {}: {err}", parent.display()))?;
    }
    let mut file = fs::File::create(path).map_err(|err| format!("failed to create report {}: {err}", path.display()))?;
    writeln!(file, "# Aspen Verified Logic Evidence Index Report").map_err(|err| err.to_string())?;
    writeln!(file).map_err(|err| err.to_string())?;
    writeln!(file, "- status: pass").map_err(|err| err.to_string())?;
    writeln!(file, "- rows: {}", rows.len()).map_err(|err| err.to_string())?;
    writeln!(file).map_err(|err| err.to_string())?;
    for row in rows {
        writeln!(file, "## {}", row.id).map_err(|err| err.to_string())?;
        writeln!(file, "- source commit: `{}`", row.source_commit).map_err(|err| err.to_string())?;
        writeln!(file, "- source paths: `{}`", row.source_paths.join(", ")).map_err(|err| err.to_string())?;
        writeln!(file, "- local snapshot manifest: `{}`", row.local_snapshot_manifest).map_err(|err| err.to_string())?;
        writeln!(file, "- boundary notes: {}", row.remaining_trusted_boundaries).map_err(|err| err.to_string())?;
        writeln!(file).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let root = resolve_under(Path::new("."), &args.root);
    let rows = parse_index(&root, &args.index, &args)?;
    for row in &rows {
        validate_row(&root, row)?;
    }
    if let Some(report) = &args.write_report {
        write_report(&resolve_under(&root, report), &rows)?;
    }
    println!("aspen evidence index valid: {} rows", rows.len());
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("aspen evidence index drift detected");
        eprintln!("{err}");
        std::process::exit(1);
    }
}
