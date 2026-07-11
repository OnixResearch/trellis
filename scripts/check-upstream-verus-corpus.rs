#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Validate the typed upstream Verus regression-corpus export.

use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Component, Path};
use std::process::ExitCode;

const DEFAULT_CATALOG_PATH: &str = "upstream-verus-corpus/generated/catalog.json";
const EXPECTED_SCHEMA: &str = "trellis-upstream-verus-corpus/v1";
const EXPECTED_CLAIM_SCOPE: &str = "source-intake-and-regression-selection-only";
const EXPECTED_HASH_PROTOCOL: &str = "nix-sha256-unpacked";
const REPOSITORY_PREFIX: &str = "https://github.com/verus-lang/";
const SHA256_SRI_PREFIX: &str = "sha256-";
const GIT_REVISION_HEX_LENGTH: usize = 40;
const EXPECTED_IDS: &[&str] = &[
    "paper-sosp24-artifact",
    "verified-ironkv",
    "verified-memory-allocator",
    "verified-node-replication",
    "verus-belt",
];
const ALLOWED_LICENSE_STATUSES: &[&str] = &["mit-reviewed", "mixed-reviewed-reference-only"];
const ALLOWED_ROLES: &[&str] = &[
    "semantic-foundation",
    "reproducibility-benchmark",
    "allocator-proof-stress",
    "distributed-host-boundary-stress",
    "local-numa-proof-stress",
];
const ALLOWED_EXECUTION_POLICIES: &[&str] = &[
    "reference-only",
    "opt-in-pinned",
    "blocked-current-verifier",
];

#[derive(Clone, Debug, Deserialize)]
struct Catalog {
    schema_version: String,
    claim_scope: String,
    entries: Vec<CorpusEntry>,
    global_non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct CorpusEntry {
    id: String,
    repository_url: String,
    revision: String,
    source_hash: String,
    source_hash_protocol: String,
    license_status: String,
    license_sentinel: String,
    role: String,
    execution_policy: String,
    verifier_context: String,
    sentinels: Vec<String>,
    limitations: Vec<String>,
    non_claims: Vec<String>,
}

fn is_nonempty(values: &[String]) -> bool {
    !values.is_empty() && values.iter().all(|value| !value.trim().is_empty())
}

fn is_git_revision(value: &str) -> bool {
    value.len() == GIT_REVISION_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_normalized_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && path.components().all(|component| {
            matches!(component, Component::Normal(_))
        })
}

fn validate_entry(entry: &CorpusEntry, errors: &mut Vec<String>) {
    if entry.id.trim().is_empty() {
        errors.push("entry id must be non-empty".to_string());
    }
    if !entry.repository_url.starts_with(REPOSITORY_PREFIX) {
        errors.push(format!("{}: repository URL is outside verus-lang", entry.id));
    }
    if !is_git_revision(&entry.revision) {
        errors.push(format!("{}: revision must be forty lowercase-or-uppercase hexadecimal characters", entry.id));
    }
    if !entry.source_hash.starts_with(SHA256_SRI_PREFIX) {
        errors.push(format!("{}: source hash must be a SHA-256 SRI value", entry.id));
    }
    if entry.source_hash_protocol != EXPECTED_HASH_PROTOCOL {
        errors.push(format!("{}: unsupported source hash protocol", entry.id));
    }
    if !ALLOWED_LICENSE_STATUSES.contains(&entry.license_status.as_str()) {
        errors.push(format!("{}: unsupported license status", entry.id));
    }
    if !ALLOWED_ROLES.contains(&entry.role.as_str()) {
        errors.push(format!("{}: unsupported corpus role", entry.id));
    }
    if !ALLOWED_EXECUTION_POLICIES.contains(&entry.execution_policy.as_str()) {
        errors.push(format!("{}: unsupported execution policy", entry.id));
    }
    if entry.verifier_context.trim().is_empty() {
        errors.push(format!("{}: verifier context must be non-empty", entry.id));
    }
    if !is_nonempty(&entry.sentinels) {
        errors.push(format!("{}: sentinels must be non-empty", entry.id));
    }
    for sentinel in &entry.sentinels {
        if !is_normalized_relative_path(sentinel) {
            errors.push(format!("{}: sentinel path is not normalized: {sentinel}", entry.id));
        }
    }
    if !is_normalized_relative_path(&entry.license_sentinel)
        || !entry.sentinels.contains(&entry.license_sentinel)
    {
        errors.push(format!("{}: license sentinel must be a declared normalized sentinel", entry.id));
    }
    if !is_nonempty(&entry.limitations) {
        errors.push(format!("{}: limitations must be non-empty", entry.id));
    }
    if !is_nonempty(&entry.non_claims) {
        errors.push(format!("{}: non-claims must be non-empty", entry.id));
    }
}

fn validate_catalog(catalog: &Catalog) -> Vec<String> {
    let mut errors = Vec::new();
    if catalog.schema_version != EXPECTED_SCHEMA {
        errors.push("unsupported catalog schema".to_string());
    }
    if catalog.claim_scope != EXPECTED_CLAIM_SCOPE {
        errors.push("catalog claim scope would transfer proof authority".to_string());
    }
    if !is_nonempty(&catalog.global_non_claims) {
        errors.push("global non-claims must be non-empty".to_string());
    }

    let ids: Vec<&str> = catalog.entries.iter().map(|entry| entry.id.as_str()).collect();
    let expected_ids: Vec<&str> = EXPECTED_IDS.to_vec();
    if ids != expected_ids {
        errors.push("catalog ids must be complete, unique, and sorted".to_string());
    }
    let unique_ids: BTreeSet<&str> = ids.iter().copied().collect();
    if unique_ids.len() != ids.len() {
        errors.push("catalog ids must not contain duplicates".to_string());
    }

    for entry in &catalog.entries {
        validate_entry(entry, &mut errors);
    }
    errors
}

fn fixture_entry(id: &str) -> CorpusEntry {
    CorpusEntry {
        id: id.to_string(),
        repository_url: format!("{REPOSITORY_PREFIX}{id}"),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        source_hash: "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string(),
        source_hash_protocol: EXPECTED_HASH_PROTOCOL.to_string(),
        license_status: "mit-reviewed".to_string(),
        license_sentinel: "LICENSE".to_string(),
        role: "semantic-foundation".to_string(),
        execution_policy: "reference-only".to_string(),
        verifier_context: "fixture verifier context".to_string(),
        sentinels: vec!["LICENSE".to_string(), "README.md".to_string()],
        limitations: vec!["fixture limitation".to_string()],
        non_claims: vec!["fixture non-claim".to_string()],
    }
}

fn fixture_catalog() -> Catalog {
    Catalog {
        schema_version: EXPECTED_SCHEMA.to_string(),
        claim_scope: EXPECTED_CLAIM_SCOPE.to_string(),
        entries: EXPECTED_IDS.iter().map(|id| fixture_entry(id)).collect(),
        global_non_claims: vec!["fixture global non-claim".to_string()],
    }
}

fn self_test() -> Result<(), String> {
    let valid = fixture_catalog();
    if !validate_catalog(&valid).is_empty() {
        return Err("positive catalog fixture failed".to_string());
    }

    let mut mutable_revision = valid.clone();
    mutable_revision.entries[0].revision = "main".to_string();
    if !validate_catalog(&mutable_revision)
        .iter()
        .any(|error| error.contains("revision"))
    {
        return Err("mutable-revision negative fixture did not fail".to_string());
    }

    let mut proof_transfer = valid.clone();
    proof_transfer.claim_scope = "upstream-proofs-certify-trellis".to_string();
    if !validate_catalog(&proof_transfer)
        .iter()
        .any(|error| error.contains("proof authority"))
    {
        return Err("proof-transfer negative fixture did not fail".to_string());
    }

    let mut escaping_path = valid.clone();
    escaping_path.entries[0].sentinels.push("../LICENSE".to_string());
    if !validate_catalog(&escaping_path)
        .iter()
        .any(|error| error.contains("not normalized"))
    {
        return Err("escaping-sentinel negative fixture did not fail".to_string());
    }

    println!("upstream Verus corpus checker self-test passed");
    Ok(())
}

fn check_file(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    let catalog: Catalog = serde_json::from_str(&text)
        .map_err(|error| format!("parsing {}: {error}", path.display()))?;
    let errors = validate_catalog(&catalog);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    println!("validated {} pinned upstream Verus corpus entries", catalog.entries.len());
    Ok(())
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("--self-test") => {
            if args.next().is_some() {
                return Err("--self-test does not accept additional arguments".to_string());
            }
            self_test()
        }
        Some(path) => {
            if args.next().is_some() {
                return Err("expected at most one catalog path".to_string());
            }
            check_file(Path::new(path))
        }
        None => check_file(Path::new(DEFAULT_CATALOG_PATH)),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("upstream Verus corpus validation failed:\n{error}");
            ExitCode::FAILURE
        }
    }
}
