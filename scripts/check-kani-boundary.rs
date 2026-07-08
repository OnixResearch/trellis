#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic checker for Trellis Kani complementary-evidence boundaries.
//!
//! The pure core validates Markdown reference records in docs/kani-boundary-fixtures
//! and keeps Kani evidence scoped to external runtime evidence rather than Verus proof claims.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const GUIDE_PATH: &str = "docs/kani-boundary.md";
const README_PATH: &str = "README.md";
const FIXTURE_DIR: &str = "docs/kani-boundary-fixtures";
const REFERENCE_PREFIX: &str = "## Reference: ";
const CLASSIFICATION_FIELD: &str = "Classification";
const SOURCE_SCOPE_FIELD: &str = "Source scope";
const RECEIPT_IDENTITY_FIELD: &str = "Receipt identity";
const PROPERTY_CLASS_FIELD: &str = "Property class";
const TRELLIS_PROOF_ARTIFACT_FIELD: &str = "Trellis proof artifact";
const CLAIM_FIELD: &str = "Claim";
const NON_CLAIMS_FIELD: &str = "Non-claims";
const EXPECTED_DIAGNOSTIC_FIELD: &str = "Expected diagnostic";
const REQUIRED_CLASSIFICATION: &str = "complementary-external-evidence";
const EXPECTED_NONE: &str = "none";
const BLAKE3_PREFIX: &str = "blake3:";
const SELF_TEST_ARG: &str = "--self-test";
const HELP_ARG: &str = "--help";
const HELP_SHORT_ARG: &str = "-h";
const ARGS_WITH_ROOT_COUNT: usize = 1;

const ALLOWED_PROPERTY_CLASSES: &[&str] = &[
    "runtime-wrapper",
    "panic-freedom",
    "boundary-regression",
    "consumer-reference",
];

const REQUIRED_NON_CLAIMS: &[&str] = &[
    "not a verus proof artifact",
    "not whole-module correctness",
    "not trusted-boundary closure",
    "not ghost-spec theorem evidence",
];

const GUIDE_REQUIRED_TERMS: &[&str] = &[
    "complementary external evidence",
    "Verus remains the authority",
    "proof fn",
    "ensures",
    "trusted-boundary closure",
    "Octet",
    "Valence",
    "non-claims",
    "Source scope",
    "Receipt identity",
    "Property class",
];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct KaniReference {
    name: String,
    classification: String,
    source_scope: String,
    receipt_identity: String,
    property_class: String,
    trellis_proof_artifact: String,
    claim: String,
    non_claims: String,
    expected_diagnostic: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FixtureCheck {
    references_checked: usize,
    errors: Vec<String>,
}

fn parse_references(markdown: &str) -> Vec<KaniReference> {
    let mut references = Vec::new();
    let mut current: Option<KaniReference> = None;

    for line in markdown.lines() {
        if let Some(name) = line.strip_prefix(REFERENCE_PREFIX) {
            if let Some(reference) = current.take() {
                references.push(reference);
            }
            current = Some(KaniReference {
                name: name.trim().to_string(),
                ..KaniReference::default()
            });
            continue;
        }

        let Some(reference) = current.as_mut() else {
            continue;
        };
        let Some((field, value)) = parse_field_line(line) else {
            continue;
        };
        assign_field(reference, field, value);
    }

    if let Some(reference) = current {
        references.push(reference);
    }

    references
}

fn parse_field_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("- ")?;
    let (field, value) = rest.split_once(':')?;
    Some((field.trim(), value.trim()))
}

fn assign_field(reference: &mut KaniReference, field: &str, value: &str) {
    match field {
        CLASSIFICATION_FIELD => reference.classification = value.to_string(),
        SOURCE_SCOPE_FIELD => reference.source_scope = value.to_string(),
        RECEIPT_IDENTITY_FIELD => reference.receipt_identity = value.to_string(),
        PROPERTY_CLASS_FIELD => reference.property_class = value.to_string(),
        TRELLIS_PROOF_ARTIFACT_FIELD => reference.trellis_proof_artifact = value.to_string(),
        CLAIM_FIELD => reference.claim = value.to_string(),
        NON_CLAIMS_FIELD => reference.non_claims = value.to_string(),
        EXPECTED_DIAGNOSTIC_FIELD => reference.expected_diagnostic = value.to_string(),
        _ => {}
    }
}

fn validate_reference(reference: &KaniReference) -> Vec<String> {
    let mut errors = Vec::new();
    require_non_empty(&mut errors, &reference.name, "reference name");
    require_non_empty(&mut errors, &reference.classification, CLASSIFICATION_FIELD);
    require_non_empty(&mut errors, &reference.source_scope, SOURCE_SCOPE_FIELD);
    require_non_empty(&mut errors, &reference.receipt_identity, RECEIPT_IDENTITY_FIELD);
    require_non_empty(&mut errors, &reference.property_class, PROPERTY_CLASS_FIELD);
    require_non_empty(&mut errors, &reference.trellis_proof_artifact, TRELLIS_PROOF_ARTIFACT_FIELD);
    require_non_empty(&mut errors, &reference.claim, CLAIM_FIELD);
    require_non_empty(&mut errors, &reference.non_claims, NON_CLAIMS_FIELD);
    require_non_empty(&mut errors, &reference.expected_diagnostic, EXPECTED_DIAGNOSTIC_FIELD);

    if !reference.classification.is_empty() && reference.classification != REQUIRED_CLASSIFICATION {
        errors.push("classification must be complementary-external-evidence".to_string());
    }
    if !reference.receipt_identity.is_empty() && !reference.receipt_identity.starts_with(BLAKE3_PREFIX) {
        errors.push("receipt identity must use a blake3: prefix".to_string());
    }
    if !reference.property_class.is_empty() && !ALLOWED_PROPERTY_CLASSES.contains(&reference.property_class.as_str()) {
        errors.push(format!("property class must be one of {}", ALLOWED_PROPERTY_CLASSES.join(", ")));
    }

    let proof_lower = reference.trellis_proof_artifact.to_ascii_lowercase();
    if proof_lower.contains("kani receipt replaces") || proof_lower.contains("replaces the verus proof") {
        errors.push("Kani receipt must not replace Verus proof artifact".to_string());
    }

    let claim_lower = reference.claim.to_ascii_lowercase();
    if claim_lower.contains("whole module")
        || claim_lower.contains("whole-module")
        || claim_lower.contains("entire module")
    {
        errors.push("claim must not promote scoped Kani evidence to whole-module correctness".to_string());
    }
    if claim_lower.contains("trusted boundary") && (claim_lower.contains("close") || claim_lower.contains("closes")) {
        errors.push("claim must not close trusted boundary with Kani only".to_string());
    }
    if claim_lower.contains("verus proof")
        || claim_lower.contains("proof fn")
        || claim_lower.contains("ensures obligation")
    {
        errors.push("claim must not promote Kani receipt to Verus proof evidence".to_string());
    }

    let non_claims_lower = reference.non_claims.to_ascii_lowercase();
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims_lower.contains(required) {
            errors.push(format!("missing required non-claim `{required}`"));
        }
    }

    errors
}

fn require_non_empty(errors: &mut Vec<String>, value: &str, field_name: &str) {
    if value.trim().is_empty() {
        errors.push(format!("{field_name} must not be empty"));
    }
}

fn check_fixture(markdown: &str, fixture_name: &str) -> FixtureCheck {
    let references = parse_references(markdown);
    let mut errors = Vec::new();
    if references.is_empty() {
        errors.push(format!("{fixture_name} must contain at least one `{REFERENCE_PREFIX}` section"));
    }

    for reference in &references {
        let reference_errors = validate_reference(reference);
        let expected = reference.expected_diagnostic.trim();
        if expected.eq_ignore_ascii_case(EXPECTED_NONE) {
            if !reference_errors.is_empty() {
                errors.push(format!(
                    "{} expected no diagnostics but got: {}",
                    reference.name,
                    reference_errors.join("; ")
                ));
            }
        } else if expected.is_empty() {
            errors.push(format!("{} must declare an expected diagnostic", reference.name));
        } else if !reference_errors.iter().any(|error| error.contains(expected)) {
            errors.push(format!(
                "{} expected diagnostic `{expected}` but got: {}",
                reference.name,
                reference_errors.join("; ")
            ));
        }
    }

    FixtureCheck {
        references_checked: references.len(),
        errors,
    }
}

fn check_docs(readme: &str, guide: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if !readme.contains(GUIDE_PATH) {
        errors.push(format!("{README_PATH} must link {GUIDE_PATH}"));
    }
    if !readme.contains("check-kani-boundary.rs") {
        errors.push(format!("{README_PATH} must list scripts/check-kani-boundary.rs"));
    }
    for term in GUIDE_REQUIRED_TERMS {
        if !guide.contains(term) {
            errors.push(format!("{GUIDE_PATH} must mention `{term}`"));
        }
    }
    errors
}

fn check_repo(root: &Path) -> Result<FixtureCheck, String> {
    let readme = read_to_string(root, README_PATH)?;
    let guide = read_to_string(root, GUIDE_PATH)?;
    let mut errors = check_docs(&readme, &guide);
    let mut references_checked = 0usize;

    for fixture_path in fixture_paths(root)? {
        let fixture_name = fixture_path.display().to_string();
        let markdown = fs::read_to_string(&fixture_path).map_err(|error| format!("reading {fixture_name}: {error}"))?;
        let check = check_fixture(&markdown, &fixture_name);
        references_checked += check.references_checked;
        errors.extend(check.errors.into_iter().map(|error| format!("{fixture_name}: {error}")));
    }

    if references_checked == 0 {
        errors.push(format!("{FIXTURE_DIR} must contain at least one checked reference"));
    }

    Ok(FixtureCheck {
        references_checked,
        errors,
    })
}

fn read_to_string(root: &Path, relative_path: &str) -> Result<String, String> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn fixture_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let dir = root.join(FIXTURE_DIR);
    let mut paths = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|error| format!("reading {}: {error}", dir.display()))? {
        let path = entry.map_err(|error| format!("reading {} entry: {error}", dir.display()))?.path();
        if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn run_self_test() -> Result<(), String> {
    let positive = "## Reference: positive\n\n- Classification: complementary-external-evidence\n- Source scope: src/example.rs::wrapper\n- Receipt identity: blake3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n- Property class: runtime-wrapper\n- Trellis proof artifact: Verus proof artifact remains authoritative.\n- Claim: Kani receipt supports complementary external evidence for panic-freedom.\n- Non-claims: not a Verus proof artifact; not whole-module correctness; not trusted-boundary closure; not ghost-spec theorem evidence\n- Expected diagnostic: none\n";
    let positive_check = check_fixture(positive, "positive-self-test");
    if !positive_check.errors.is_empty() || positive_check.references_checked != ARGS_WITH_ROOT_COUNT {
        return Err(format!("positive self-test failed: {positive_check:?}"));
    }

    let negative = "## Reference: negative\n\n- Classification: verus-proof-artifact\n- Source scope: src/example.rs::wrapper\n- Receipt identity: blake3:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n- Property class: runtime-wrapper\n- Trellis proof artifact: Kani receipt replaces the Verus proof artifact.\n- Claim: Kani proves the Verus proof fn and closes the ensures obligation.\n- Non-claims: not whole-module correctness\n- Expected diagnostic: classification must be complementary-external-evidence\n";
    let negative_check = check_fixture(negative, "negative-self-test");
    if !negative_check.errors.is_empty() || negative_check.references_checked != ARGS_WITH_ROOT_COUNT {
        return Err(format!("negative self-test failed: {negative_check:?}"));
    }

    let missed_negative =
        negative.replace("classification must be complementary-external-evidence", "diagnostic that will not appear");
    let missed_check = check_fixture(&missed_negative, "missed-negative-self-test");
    if missed_check.errors.is_empty() {
        return Err("negative self-test must fail when expected diagnostic is absent".to_string());
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Option<PathBuf>, String> {
    match args {
        [] => Ok(Some(PathBuf::from("."))),
        [flag] if flag == SELF_TEST_ARG => Ok(None),
        [flag] if flag == HELP_ARG || flag == HELP_SHORT_ARG => Err(usage()),
        [root] => Ok(Some(PathBuf::from(root))),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: check-kani-boundary.rs [--self-test|ROOT]".to_string()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match parse_args(&args) {
        Ok(None) => run_self_test().map(|_| FixtureCheck {
            references_checked: ARGS_WITH_ROOT_COUNT,
            errors: Vec::new(),
        }),
        Ok(Some(root)) => check_repo(&root),
        Err(error) => Err(error),
    };

    match result {
        Ok(check) if check.errors.is_empty() => {
            println!("Kani boundary check passed: {} references checked", check.references_checked);
            ExitCode::SUCCESS
        }
        Ok(check) => {
            for error in check.errors {
                eprintln!("error: {error}");
            }
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
