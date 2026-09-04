use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::model::AUTHORITY_SCHEMA;
use crate::model::AuthorityBaseline;
use crate::model::AuthorityEntry;
use crate::validation::safe_relative;

const SOURCE_ROOT: &str = "src";
const BIN_COMPONENT: &str = "bin";
const CARGO_MANIFEST: &str = "Cargo.toml";
const FLAKE_MANIFEST: &str = "flake.nix";
const EXPECTED_INPUT: &str = "url = \"github:OnixResearch/nickel-export/257fafc1c746f1faf156207043a4c826bfb16d49\";";
const LOCAL_INPUT_FORMS: &[&str] = &["nickel-export.url = \"path:", "nickel-export = { url = \"path:"];
const MAX_SOURCE_FILES: usize = 2_048;
const MAX_SOURCE_BYTES: u64 = 16_777_216;
const AUTHORITY_TOKENS: &[&str] = &["Command::new", "std::env", "std::fs", "std::process"];
const STANDALONE_TOKENS: &[&str] = &["nickel-export", "nickel_export"];
const TRACE_REFERENCE_PREFIXES: &[&str] = &[
    "r[impl trellis.nickel_export_cutover.",
    "r[verify trellis.nickel_export_cutover.",
];

// r[impl trellis.nickel_export_cutover.source]
pub fn check_release_pin(root: &Path) -> Result<(), String> {
    let path = root.join(FLAKE_MANIFEST);
    let text = fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    if !text.contains(EXPECTED_INPUT) {
        return Err("standalone exporter flake input is missing or drifted".to_owned());
    }
    if LOCAL_INPUT_FORMS.iter().any(|form| text.contains(form)) {
        return Err("release input uses a local standalone exporter override".to_owned());
    }
    Ok(())
}

pub fn write(root: &Path, destination: &str) -> Result<(), String> {
    require_safe_destination(destination)?;
    let baseline = collect(root)?;
    let mut bytes = serde_json::to_vec_pretty(&baseline).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    let path = root.join(destination);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    fs::write(&path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
}

pub fn check(root: &Path, source: &str) -> Result<(), String> {
    require_safe_destination(source)?;
    let path = root.join(source);
    let bytes = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let expected: AuthorityBaseline =
        serde_json::from_slice(&bytes).map_err(|error| format!("parse {}: {error}", path.display()))?;
    let actual = collect(root)?;
    compare(&expected, &actual)
}

pub fn compare(expected: &AuthorityBaseline, actual: &AuthorityBaseline) -> Result<(), String> {
    if expected.schema != AUTHORITY_SCHEMA {
        return Err("authority baseline schema drift".to_owned());
    }
    if expected == actual {
        Ok(())
    } else {
        Err("verified-library authority baseline drift".to_owned())
    }
}

// r[impl trellis.nickel_export_cutover.authority]
pub fn collect(root: &Path) -> Result<AuthorityBaseline, String> {
    let source_root = root.join(SOURCE_ROOT);
    let mut files = Vec::new();
    collect_rust_files(&source_root, &mut files)?;
    files.sort();
    if files.len() > MAX_SOURCE_FILES {
        return Err(format!("source file count {} exceeds bound {MAX_SOURCE_FILES}", files.len()));
    }

    let mut entries = Vec::new();
    for path in files {
        let relative = path.strip_prefix(root).map_err(|error| format!("relative source path: {error}"))?;
        if relative.components().nth(1).is_some_and(|component| component.as_os_str() == BIN_COMPONENT) {
            continue;
        }
        let metadata = fs::metadata(&path).map_err(|error| format!("stat {}: {error}", path.display()))?;
        if metadata.len() > MAX_SOURCE_BYTES {
            return Err(format!("source file exceeds byte bound: {}", path.display()));
        }
        let text = fs::read_to_string(&path).map_err(|error| format!("read source {}: {error}", path.display()))?;
        for token in STANDALONE_TOKENS {
            let unauthorized = text.lines().any(|line| {
                line.contains(token)
                    && !(line.trim_start().starts_with("//")
                        && TRACE_REFERENCE_PREFIXES.iter().any(|prefix| line.contains(prefix)))
            });
            if unauthorized {
                return Err(format!(
                    "verified-library source acquired standalone export authority: {}",
                    relative.display()
                ));
            }
        }
        for token in AUTHORITY_TOKENS {
            let count = text.matches(token).count();
            if count > 0 {
                entries.push(AuthorityEntry {
                    path: relative.to_string_lossy().into_owned(),
                    token: (*token).to_owned(),
                    count,
                });
            }
        }
    }
    entries.sort();
    check_manifest(root)?;
    Ok(AuthorityBaseline {
        schema: AUTHORITY_SCHEMA.to_owned(),
        entries,
    })
}

fn collect_rust_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("read directory {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("read directory entry: {error}"))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| format!("inspect {}: {error}", path.display()))?;
        if file_type.is_symlink() {
            return Err(format!("source scan rejects symlink: {}", path.display()));
        }
        if file_type.is_dir() {
            collect_rust_files(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn check_manifest(root: &Path) -> Result<(), String> {
    let path = root.join(CARGO_MANIFEST);
    let text = fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    if STANDALONE_TOKENS.iter().any(|token| text.contains(token)) {
        Err("reusable verified-logic package acquired a nickel-export dependency".to_owned())
    } else {
        Ok(())
    }
}

fn require_safe_destination(path: &str) -> Result<(), String> {
    if safe_relative(path) {
        Ok(())
    } else {
        Err("unsafe authority baseline path".to_owned())
    }
}
