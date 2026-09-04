use super::identity;
use super::model::ArtifactIdentity;
use super::model::CrossVerifierManifest;
use super::model::FinalOutcome;
use super::model::ProofReceipt;
use super::model::PropertyLink;
use super::model::StageObservation;
use super::model::StageReceipt;
use super::model::StageStatus;
use super::validation::Inputs;

const RECEIPT_SCHEMA: &str = "trellis-aeneas-proof-receipt/v1";
const MANIFEST_SCHEMA: &str = "trellis-cross-verifier-manifest/v1";
const EXPECTED_STATUS: &str = "passed";
const ARTIFACT_VERIFICATION_VALID: &str = "valid";
const EXPECTED_SOURCE_PATH: &str = "src/arithmetic.rs";
const RECEIPT_PATH: &str = "aeneas-pilot/generated/aeneas-proof-receipt.json";

pub(crate) fn build_receipt(inputs: &Inputs) -> Result<ProofReceipt, String> {
    let source = inputs
        .profile
        .consumer
        .source
        .paths
        .first()
        .ok_or_else(|| "profile source binding is missing".to_owned())?;
    let selected_item = inputs
        .profile
        .consumer
        .source
        .selected_items
        .first()
        .ok_or_else(|| "profile selected item is missing".to_owned())?
        .clone();
    let run_identity = artifact_run_identity(&inputs.profile.artifacts)?;
    let verus_bytes = super::validation::material(inputs, &inputs.profile.verus.receipt_path)?;
    let mut charon_argv = inputs.profile.extraction.charon_argv.clone();
    charon_argv.push("--".to_owned());
    charon_argv.extend(inputs.profile.extraction.cargo_argv.clone());
    let stages = vec![
        StageReceipt {
            stage: "charon".to_owned(),
            status: StageStatus::Passed,
            argv: charon_argv,
            artifact_roles: vec!["llbc".to_owned()],
        },
        StageReceipt {
            stage: "aeneas".to_owned(),
            status: StageStatus::Passed,
            argv: inputs.profile.extraction.aeneas_argv.clone(),
            artifact_roles: vec!["translation-manifest".to_owned(), "generated-lean".to_owned()],
        },
        StageReceipt {
            stage: "lean".to_owned(),
            status: StageStatus::Passed,
            argv: inputs.profile.extraction.lean_argv.clone(),
            artifact_roles: vec![
                "symmetry-theorem".to_owned(),
                "self-zero-theorem".to_owned(),
                "axiom-audit".to_owned(),
            ],
        },
    ];
    let mut receipt = ProofReceipt {
        schema_version: RECEIPT_SCHEMA.to_owned(),
        outcome: FinalOutcome::Supported,
        profile_id: inputs.profile.pilot_id.clone(),
        profile_identity: identity::prefixed(&inputs.profile_bytes),
        source_path: source.path.clone(),
        source_identity: source.identity.clone(),
        selected_item,
        run_identity,
        stages,
        resource_policy_identity: inputs.profile.resource_policy.identity.clone(),
        theorem_mappings: inputs.profile.theorem_mappings.clone(),
        external_models: inputs.profile.consumer.external_models.clone(),
        trust_inputs: inputs.profile.consumer.trust_inputs.clone(),
        selected_axioms: inputs.profile.trust_closure.selected_axioms.clone(),
        artifacts: inputs.profile.artifacts.clone(),
        artifact_verification: ARTIFACT_VERIFICATION_VALID.to_owned(),
        verus_receipt_path: inputs.profile.verus.receipt_path.clone(),
        verus_receipt_identity: identity::prefixed(verus_bytes),
        octet_policy_artifact_identity: inputs.profile.octet_source.policy_artifact_identity.clone(),
        non_claims: inputs.profile.required_non_claims.clone(),
        receipt_identity: String::new(),
    };
    receipt.receipt_identity = identity::receipt_identity(&receipt)?;
    Ok(receipt)
}

pub(crate) fn build_manifest(inputs: &Inputs, receipt: &ProofReceipt) -> Result<CrossVerifierManifest, String> {
    let properties = inputs
        .profile
        .theorem_mappings
        .iter()
        .map(|mapping| PropertyLink {
            theorem_id: mapping.theorem_id.clone(),
            requirement_id: mapping.requirement_id.clone(),
            verus_proof_symbol: mapping.verus_proof_symbol.clone(),
            lean_theorem_symbol: mapping.lean_theorem_symbol.clone(),
            theorem_source_identity: mapping.approved_statement_blake3.clone(),
            integer_model: mapping.integer_model.clone(),
            overflow_model: mapping.overflow_model.clone(),
            axioms: mapping.axioms.clone(),
        })
        .collect();
    let mut manifest = CrossVerifierManifest {
        schema_version: MANIFEST_SCHEMA.to_owned(),
        status: EXPECTED_STATUS.to_owned(),
        source_path: receipt.source_path.clone(),
        source_identity: receipt.source_identity.clone(),
        run_identity: receipt.run_identity.clone(),
        receipt_path: RECEIPT_PATH.to_owned(),
        receipt_identity: receipt.receipt_identity.clone(),
        properties,
        authority_merged: false,
        artifact_verification: ARTIFACT_VERIFICATION_VALID.to_owned(),
        non_claims: inputs.profile.required_non_claims.clone(),
        manifest_identity: String::new(),
    };
    manifest.manifest_identity = identity::manifest_identity(&manifest)?;
    Ok(manifest)
}

pub(crate) fn classify(observations: &[StageObservation]) -> FinalOutcome {
    if observations.is_empty() {
        return FinalOutcome::Failed;
    }
    for observation in observations {
        match observation.status {
            StageStatus::Passed => {}
            StageStatus::Unsupported
                if nonempty(observation.construct.as_deref())
                    && b3(observation.diagnostic_identity.as_deref().unwrap_or_default())
                    && nonempty(observation.next_action.as_deref()) =>
            {
                return FinalOutcome::Blocked;
            }
            StageStatus::Unsupported
            | StageStatus::ProcessFailed
            | StageStatus::PolicyFailed
            | StageStatus::ArtifactFailed
            | StageStatus::TimedOut
            | StageStatus::BudgetExceeded => return FinalOutcome::Failed,
        }
    }
    FinalOutcome::Supported
}

pub(crate) fn validate_receipt(inputs: &Inputs, receipt: &ProofReceipt, diagnostics: &mut Vec<String>) {
    if receipt.schema_version != RECEIPT_SCHEMA || receipt.outcome != FinalOutcome::Supported {
        diagnostics.push("proof receipt schema or outcome drifted".to_owned());
    }
    let source_identity = inputs.profile.consumer.source.paths.first().map(|source| source.identity.as_str());
    if receipt.source_path != EXPECTED_SOURCE_PATH || Some(receipt.source_identity.as_str()) != source_identity {
        diagnostics.push("receipt source identity does not match the selected runtime source".to_owned());
    }
    if receipt.artifact_verification != ARTIFACT_VERIFICATION_VALID {
        diagnostics.push("artifact verification is not valid".to_owned());
    }
    if receipt.stages.iter().any(|stage| stage.status != StageStatus::Passed) {
        diagnostics.push("stage outcome is not a passing supported result".to_owned());
    }
    for required in &inputs.profile.required_non_claims {
        if !receipt.non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(format!("required non-claim `{required}` is missing from receipt"));
        }
    }
    match identity::receipt_identity(receipt) {
        Ok(expected) if expected == receipt.receipt_identity => {}
        Ok(_) => diagnostics.push("proof receipt identity is stale".to_owned()),
        Err(error) => diagnostics.push(error),
    }
    match build_receipt(inputs) {
        Ok(expected) if &expected == receipt => {}
        Ok(_) => diagnostics.push("proof receipt does not match current profile and artifacts".to_owned()),
        Err(error) => diagnostics.push(error),
    }
}

pub(crate) fn validate_manifest(
    inputs: &Inputs,
    receipt: &ProofReceipt,
    manifest: &CrossVerifierManifest,
    diagnostics: &mut Vec<String>,
) {
    if manifest.schema_version != MANIFEST_SCHEMA
        || manifest.status != EXPECTED_STATUS
        || manifest.authority_merged
        || manifest.artifact_verification != ARTIFACT_VERIFICATION_VALID
    {
        diagnostics
            .push("cross-verifier manifest status, artifact verification, or authority boundary drifted".to_owned());
    }
    if manifest.source_identity != receipt.source_identity
        || manifest.receipt_identity != receipt.receipt_identity
        || manifest.run_identity != receipt.run_identity
    {
        diagnostics.push("cross-verifier manifest receipt linkage drifted".to_owned());
    }
    for required in &inputs.profile.required_non_claims {
        if !manifest.non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(format!("required non-claim `{required}` is missing from manifest"));
        }
    }
    match identity::manifest_identity(manifest) {
        Ok(expected) if expected == manifest.manifest_identity => {}
        Ok(_) => diagnostics.push("cross-verifier manifest identity is stale".to_owned()),
        Err(error) => diagnostics.push(error),
    }
    match build_manifest(inputs, receipt) {
        Ok(expected) if &expected == manifest => {}
        Ok(_) => diagnostics.push("cross-verifier manifest does not match current receipt".to_owned()),
        Err(error) => diagnostics.push(error),
    }
}

fn artifact_run_identity(artifacts: &[ArtifactIdentity]) -> Result<String, String> {
    let llbc = artifact_for_role(artifacts, "llbc").ok_or_else(|| "LLBC artifact is missing".to_owned())?;
    let translation = artifact_for_role(artifacts, "translation-manifest")
        .ok_or_else(|| "translation artifact is missing".to_owned())?;
    let generated = artifact_for_role(artifacts, "generated-lean")
        .ok_or_else(|| "generated Lean artifact is missing".to_owned())?;
    Ok(identity::run_identity(&llbc.blake3, &translation.blake3, &generated.blake3))
}

fn artifact_for_role<'a>(artifacts: &'a [ArtifactIdentity], role: &str) -> Option<&'a ArtifactIdentity> {
    artifacts.iter().find(|artifact| artifact.role == role)
}

fn b3(value: &str) -> bool {
    value.strip_prefix("b3:").is_some_and(|digest| {
        const HEX_LENGTH: usize = 64;
        digest.len() == HEX_LENGTH && digest.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn nonempty(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.trim().is_empty())
}
