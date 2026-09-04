use std::collections::BTreeSet;

use super::identity;
use super::validation::Inputs;

const PROFILE_SCHEMA: &str = "trellis-aeneas-cross-verifier-profile/v1";
const RESOURCE_SCHEMA: &str = "trellis-aeneas-resource-policy/v1";
const NEGATIVE_SCHEMA: &str = "trellis-aeneas-negative-catalog/v1";
const EXPECTED_OUTCOME: &str = "supported";
const EXPECTED_OCTET_REVISION: &str = "890f424cd7b1b2015f39c0eb5747f1bd1e576028";
const EXPECTED_CONTRACT_IDENTITY: &str = "b3:63ea1e9e78916728a4a83719c29a6443a8952dbaadeeaae2a7420365afa7ff07";
const EXPECTED_POLICY_IDENTITY: &str = "b3:cc07811243f39a70c08d80b2d145fbc97596644d56b74284d5f870d93826f605";
const EXPECTED_PROVIDER_IDENTITY: &str = "b3:cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625";
const EXPECTED_COMPATIBILITY_IDENTITY: &str = "b3:403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5";
const EXPECTED_SOURCE_PATH: &str = "src/arithmetic.rs";
const EXPECTED_RUST_ITEM: &str = "verified_logic::arithmetic::abs_diff";
const EXPECTED_CHARON_ITEM: &str = "crate::arithmetic::abs_diff";
const EXPECTED_LEAN_ITEM: &str = "verified_logic.arithmetic.abs_diff";
const EXPECTED_BEGIN_LINE: u64 = 21;
const EXPECTED_END_LINE: u64 = 25;
const EXPECTED_POINTER_WIDTH_BYTES: u64 = 8;
const EXPECTED_SOURCE_COUNT: usize = 1;
const EXPECTED_THEOREM_COUNT: usize = 2;
const EXPECTED_NEGATIVE_COUNT: usize = 18;
const REQUIRED_NON_CLAIM_EQUIVALENCE: &str = "does not prove semantic equivalence";
const REQUIRED_NON_CLAIM_AUTHORITY: &str = "does not merge Verus and Aeneas proof authority";

pub(crate) fn validate(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    validate_profile(inputs, diagnostics);
    validate_materials(inputs, diagnostics);
    validate_resource(inputs, diagnostics);
    validate_negative_catalog(inputs, diagnostics);
}

fn validate_profile(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let profile = &inputs.profile;
    if profile.schema_version != PROFILE_SCHEMA || profile.outcome != EXPECTED_OUTCOME {
        diagnostics.push("profile schema or supported outcome drifted".to_owned());
    }
    let octet = &profile.octet_source;
    if octet.revision != EXPECTED_OCTET_REVISION
        || octet.consumer_contract_identity != EXPECTED_CONTRACT_IDENTITY
        || octet.policy_artifact_identity != EXPECTED_POLICY_IDENTITY
        || octet.provider_profile_identity != EXPECTED_PROVIDER_IDENTITY
        || octet.compatibility_matrix_identity != EXPECTED_COMPATIBILITY_IDENTITY
    {
        diagnostics.push("provider identity or pinned Octet contract drifted".to_owned());
    }
    let provider = &profile.consumer.provider;
    if provider.profile_identity != EXPECTED_PROVIDER_IDENTITY
        || provider.compatibility_matrix_identity != EXPECTED_COMPATIBILITY_IDENTITY
        || provider.network_allowed
    {
        diagnostics.push("provider identity or authority exceeds the reviewed profile".to_owned());
    }
    let source = &profile.consumer.source;
    if source.paths.len() != EXPECTED_SOURCE_COUNT
        || source.selected_items.len() != EXPECTED_SOURCE_COUNT
        || source.broad_selection_allowed
    {
        diagnostics.push("exact source path selection is broad or incomplete".to_owned());
    }
    if let (Some(path), Some(item)) = (source.paths.first(), source.selected_items.first()) {
        if path.path != EXPECTED_SOURCE_PATH
            || item.source_file != EXPECTED_SOURCE_PATH
            || item.qualified_name != EXPECTED_RUST_ITEM
            || item.generated_name != EXPECTED_LEAN_ITEM
            || item.begin_line != EXPECTED_BEGIN_LINE
            || item.end_line != EXPECTED_END_LINE
        {
            diagnostics.push("exact source path or selected item drifted".to_owned());
        }
        if path.identity != item.source_identity {
            diagnostics.push("source identity differs between file and selected item".to_owned());
        }
    }
    let expected_charon = [
        "charon",
        "cargo",
        "--preset",
        "aeneas",
        "--dest-file",
        "llbc.llbc",
        "--start-from",
        EXPECTED_CHARON_ITEM,
    ];
    if profile.extraction.charon_argv.iter().map(String::as_str).collect::<Vec<_>>() != expected_charon
        || profile.extraction.cargo_argv != ["--lib"]
        || profile.extraction.expected_has_errors
        || profile.extraction.expected_skip_borrowck
    {
        diagnostics.push("extraction command or borrow-check policy drifted".to_owned());
    }
    if profile.extraction.expected_target != "x86_64-unknown-linux-gnu"
        || profile.extraction.expected_pointer_width_bytes != EXPECTED_POINTER_WIDTH_BYTES
        || !profile.extraction.expected_little_endian
    {
        diagnostics.push("target metadata policy drifted".to_owned());
    }
    if profile.consumer.theorems.len() != EXPECTED_THEOREM_COUNT
        || profile.theorem_mappings.len() != EXPECTED_THEOREM_COUNT
    {
        diagnostics.push("theorem mapping set is incomplete".to_owned());
    }
    if !profile.consumer.trust_closure_complete || !profile.trust_closure.complete {
        diagnostics.push("trust closure is incomplete".to_owned());
    }
    if profile.consumer.external_models.is_empty() || profile.consumer.trust_inputs.is_empty() {
        diagnostics.push("trust closure omits external models or axioms".to_owned());
    }
    for required in [REQUIRED_NON_CLAIM_EQUIVALENCE, REQUIRED_NON_CLAIM_AUTHORITY] {
        if !profile.required_non_claims.iter().any(|claim| claim == required)
            || !profile.consumer.non_claims.iter().any(|claim| claim == required)
        {
            diagnostics.push(format!("required non-claim `{required}` is missing"));
        }
    }
}

fn validate_materials(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let mut roles = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for artifact in &inputs.profile.artifacts {
        if !roles.insert(&artifact.role) || !paths.insert(&artifact.path) {
            diagnostics.push("artifact roles and paths must be unique".to_owned());
        }
        let Ok(bytes) = super::validation::material(inputs, &artifact.path) else {
            diagnostics.push(format!("artifact material is missing for `{}`", artifact.path));
            continue;
        };
        let observed_bytes = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if artifact.bytes != observed_bytes || artifact.blake3 != identity::prefixed(bytes) {
            diagnostics.push(format!("artifact identity drifted for `{}`", artifact.path));
        }
    }
    let source = inputs.profile.consumer.source.paths.first();
    if let Some(source) = source {
        match super::validation::material(inputs, &source.path) {
            Ok(bytes) if source.identity == identity::prefixed(bytes) => {}
            _ => diagnostics.push("source identity does not match exact runtime source bytes".to_owned()),
        }
    }
}

fn validate_resource(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    if inputs.resource.schema_version != RESOURCE_SCHEMA
        || inputs.resource.network_allowed
        || inputs.resource.timeout_ms != inputs.profile.consumer.resource_policy.timeout_ms
        || inputs.resource.stdout_max_bytes != inputs.profile.consumer.resource_policy.stdout_max_bytes
        || inputs.resource.stderr_max_bytes != inputs.profile.consumer.resource_policy.stderr_max_bytes
        || inputs.resource.environment_allowlist != inputs.profile.consumer.resource_policy.environment_allowlist
    {
        diagnostics.push("resource policy values exceed or differ from provider authority".to_owned());
    }
    match super::validation::material(inputs, &inputs.profile.resource_policy.path) {
        Ok(bytes) if inputs.profile.resource_policy.identity == identity::prefixed(bytes) => {}
        _ => diagnostics.push("resource policy identity drifted".to_owned()),
    }
}

fn validate_negative_catalog(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let catalog = &inputs.negative;
    if catalog.schema_version != NEGATIVE_SCHEMA || catalog.cases.len() != EXPECTED_NEGATIVE_COUNT {
        diagnostics.push("negative fixture catalog is incomplete".to_owned());
    }
    let mut ids = BTreeSet::new();
    if catalog
        .cases
        .iter()
        .any(|case| case.id.trim().is_empty() || case.expected_diagnostic.trim().is_empty() || !ids.insert(&case.id))
    {
        diagnostics.push("negative fixture catalog has empty or duplicate cases".to_owned());
    }
}
