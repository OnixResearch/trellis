#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BaselineFile {
    schema_version: u64,
    finding_schema_version: u64,
    metadata: BaselineMetadata,
    baseline_hash: String,
    findings: Vec<Finding>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BaselineMetadata {
    tool_name: String,
    tool_version: String,
    rustc_version: String,
    toolchain: String,
    profile_name: String,
    profile_hash: String,
    config_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Finding {
    stable_id: String,
    semantic_id: String,
    identity_status: String,
    lint: String,
    group: String,
    severity: String,
    file: String,
    line: u64,
    column: u64,
    message: String,
    crate_name: String,
}

const EXPECTED_ACCEPTED_BEFORE: usize = 3_867;
const EXPECTED_NEW: usize = 1;
const EXPECTED_STALE: usize = 1;
const EXPECTED_ACCEPTED_AFTER: usize = 3_867;
const CHANGE_SOURCE_PREFIX: &str = "src/paren_tree/";

const REPLACEMENTS: [(&str, &str); EXPECTED_NEW] = [
    ("b3:b11221ab391e117f", "b3:4a37b4f2612a74b5"),
];

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

fn string_set(value: &Value, field: &str) -> BTreeSet<String> {
    value[field]
        .as_array()
        .unwrap_or_else(|| panic!("{field} must be an array"))
        .iter()
        .map(|item| item.as_str().unwrap_or_else(|| panic!("{field} item must be a string")).to_owned())
        .collect()
}

fn finding_map(value: &Value) -> BTreeMap<String, Value> {
    value["findings"]
        .as_array()
        .expect("findings must be an array")
        .iter()
        .map(|finding| {
            let id = finding["stable_id"].as_str().expect("stable_id must be a string").to_owned();
            (id, finding.clone())
        })
        .collect()
}

fn text<'a>(finding: &'a Value, field: &str) -> &'a str {
    finding[field].as_str().unwrap_or_else(|| panic!("finding field {field} must be a string"))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4, "usage: migrate OLD GENERATED STATUS");
    let old_path = Path::new(&args[1]);
    let generated_path = Path::new(&args[2]);
    let status_path = Path::new(&args[3]);

    let old = read_json(old_path);
    let generated = read_json(generated_path);
    let status = read_json(status_path);
    let old_findings = finding_map(&old);
    let generated_findings = finding_map(&generated);
    let new_ids = string_set(&status["baseline"], "new_finding_ids");
    let stale_ids = string_set(&status["baseline"], "stale_finding_ids");
    let expected_new: BTreeSet<String> = REPLACEMENTS.iter().map(|(_, new)| (*new).to_owned()).collect();

    assert_eq!(old_findings.len(), EXPECTED_ACCEPTED_BEFORE);
    assert_eq!(new_ids.len(), EXPECTED_NEW);
    assert_eq!(new_ids, expected_new);
    assert_eq!(stale_ids.len(), EXPECTED_STALE);
    assert_eq!(generated_findings.len(), EXPECTED_ACCEPTED_AFTER);
    assert_eq!(old["schema_version"], generated["schema_version"]);
    assert_eq!(old["finding_schema_version"], generated["finding_schema_version"]);
    assert_eq!(old["metadata"], generated["metadata"]);
    assert_eq!(
        EXPECTED_ACCEPTED_BEFORE - EXPECTED_STALE + EXPECTED_NEW,
        EXPECTED_ACCEPTED_AFTER
    );

    let expected_ids: BTreeSet<String> = old_findings
        .keys()
        .filter(|id| !stale_ids.contains(*id))
        .cloned()
        .chain(new_ids.iter().cloned())
        .collect();
    let generated_ids: BTreeSet<String> = generated_findings.keys().cloned().collect();
    assert_eq!(generated_ids, expected_ids, "generated baseline changed more than stale/new IDs");

    for finding in generated_findings.values() {
        assert!(
            !text(finding, "file").starts_with(CHANGE_SOURCE_PREFIX),
            "new canonical-tree source must have zero accepted findings"
        );
    }

    for (old_id, new_id) in REPLACEMENTS {
        assert!(stale_ids.contains(old_id));
        assert!(new_ids.contains(new_id));
        let before = old_findings.get(old_id).expect("replacement source ID must exist");
        let after = generated_findings.get(new_id).expect("replacement target ID must exist");
        assert_eq!(text(before, "file"), text(after, "file"));
        assert_eq!(text(before, "lint"), text(after, "lint"));
        assert_eq!(before["column"], after["column"]);
        if text(before, "lint") != "excessive_file_length" {
            assert_eq!(text(before, "message"), text(after, "message"));
        }
    }

    let mut migrated_entries = old["findings"]
        .as_array()
        .expect("old findings must be an array")
        .iter()
        .filter(|finding| !stale_ids.contains(text(finding, "stable_id")))
        .cloned()
        .collect::<Vec<_>>();
    for new_id in &new_ids {
        migrated_entries.push(
            generated_findings
                .get(new_id)
                .unwrap_or_else(|| panic!("generated finding is missing {new_id}"))
                .clone(),
        );
    }

    let mut migrated = old.clone();
    migrated["baseline_hash"] = generated["baseline_hash"].clone();
    migrated["findings"] = Value::Array(migrated_entries);
    let migrated_findings = finding_map(&migrated);
    assert_eq!(migrated_findings.len(), EXPECTED_ACCEPTED_AFTER);
    for (id, finding) in &old_findings {
        if !stale_ids.contains(id) {
            assert_eq!(
                migrated_findings.get(id),
                Some(finding),
                "unchanged finding record drifted for {id}"
            );
        }
    }

    let ordered: BaselineFile =
        serde_json::from_value(migrated).expect("decode guarded baseline for ordered output");
    let mut bytes = serde_json::to_vec_pretty(&ordered).expect("serialize guarded baseline migration");
    bytes.push(b'\n');
    fs::write(old_path, bytes).expect("write guarded baseline migration");
    println!(
        "guarded Octet migration: accepted {EXPECTED_ACCEPTED_BEFORE} -> {EXPECTED_ACCEPTED_AFTER}; stale removed={EXPECTED_STALE}; replacements={EXPECTED_NEW}; new canonical-tree findings=0"
    );
}
