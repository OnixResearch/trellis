use serde_json::Value;

use super::model::FinalOutcome;
use super::model::StageObservation;
use super::model::StageStatus;
use super::validation::Inputs;

const DRIFT_IDENTITY: &str = "b3:0000000000000000000000000000000000000000000000000000000000000000";
const REQUIRED_AUTHORITY_NON_CLAIM: &str = "does not merge Verus and Aeneas proof authority";

pub(crate) fn run(inputs: &Inputs) -> Result<(), String> {
    let valid_diagnostics = super::validation::validate_complete(inputs);
    if !valid_diagnostics.is_empty() {
        return Err(format!("positive checker fixture failed: {}", valid_diagnostics.join("; ")));
    }
    classifier_cases()?;
    for case in &inputs.negative.cases {
        let mut mutated = inputs.clone();
        apply_mutation(&case.id, &mut mutated)?;
        let diagnostics = super::validation::validate_complete(&mutated);
        if !diagnostics.iter().any(|diagnostic| diagnostic.contains(&case.expected_diagnostic)) {
            return Err(format!(
                "negative case `{}` missed `{}`: {}",
                case.id,
                case.expected_diagnostic,
                diagnostics.join("; ")
            ));
        }
    }
    Ok(())
}

fn classifier_cases() -> Result<(), String> {
    let passed = StageObservation {
        stage: "lean".to_owned(),
        status: StageStatus::Passed,
        construct: None,
        diagnostic_identity: None,
        next_action: None,
    };
    require_outcome(std::slice::from_ref(&passed), FinalOutcome::Supported, "passing stage")?;
    let blocked = StageObservation {
        stage: "charon".to_owned(),
        status: StageStatus::Unsupported,
        construct: Some("reviewed-unsupported-item".to_owned()),
        diagnostic_identity: Some(DRIFT_IDENTITY.to_owned()),
        next_action: Some("run the next exact-item canary".to_owned()),
    };
    require_outcome(&[blocked], FinalOutcome::Blocked, "complete blocker")?;
    let incomplete = StageObservation {
        stage: "charon".to_owned(),
        status: StageStatus::Unsupported,
        construct: None,
        diagnostic_identity: None,
        next_action: None,
    };
    require_outcome(&[incomplete], FinalOutcome::Failed, "incomplete blocker")?;
    for status in [
        StageStatus::ProcessFailed,
        StageStatus::PolicyFailed,
        StageStatus::ArtifactFailed,
        StageStatus::TimedOut,
        StageStatus::BudgetExceeded,
    ] {
        let failed = StageObservation {
            stage: "provider".to_owned(),
            status,
            construct: None,
            diagnostic_identity: None,
            next_action: None,
        };
        require_outcome(&[failed], FinalOutcome::Failed, "non-language failure")?;
    }
    Ok(())
}

fn require_outcome(observations: &[StageObservation], expected: FinalOutcome, label: &str) -> Result<(), String> {
    let actual = super::validation::classify(observations);
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label} classified as {actual:?}, expected {expected:?}"))
    }
}

fn apply_mutation(id: &str, inputs: &mut Inputs) -> Result<(), String> {
    match id {
        "provider-profile-drift" => {
            inputs.profile.octet_source.provider_profile_identity = DRIFT_IDENTITY.to_owned();
        }
        "extraction-command-drift" => {
            inputs.profile.extraction.charon_argv.push("--skip-borrowck".to_owned());
        }
        "source-identity-drift" => {
            inputs.profile.consumer.source.paths[0].identity = DRIFT_IDENTITY.to_owned();
        }
        "shadow-source-path" => {
            inputs.profile.consumer.source.paths[0].path = "fixtures/shadow.rs".to_owned();
            inputs.profile.consumer.source.selected_items[0].source_file = "fixtures/shadow.rs".to_owned();
        }
        "errorful-llbc" => set_json(&mut inputs.llbc, "/has_errors", Value::Bool(true))?,
        "skip-borrowck" => set_json(&mut inputs.llbc, "/translated/options/skip_borrowck", Value::Bool(true))?,
        "target-drift" => set_json(
            &mut inputs.llbc,
            "/translated/target_information/0/key",
            Value::String("aarch64-unknown-linux-gnu".to_owned()),
        )?,
        "stale-translation" => set_json(
            &mut inputs.translation,
            "/functions/0/lean_name",
            Value::String("verified_logic.arithmetic.other".to_owned()),
        )?,
        "cross-run-def-id" => {
            set_json(&mut inputs.translation, "/functions/0/def_id", Value::Number(serde_json::Number::from(1_u64)))?
        }
        "stale-generated-lean" => {
            let path = artifact_path(inputs, "generated-lean")?;
            inputs
                .materials
                .get_mut(&path)
                .ok_or_else(|| "generated Lean material missing".to_owned())?
                .push(b' ');
        }
        "theorem-hash-drift" => {
            inputs.profile.theorem_mappings[0].approved_statement_blake3 = DRIFT_IDENTITY.to_owned();
            inputs.profile.consumer.theorems[0].approved_statement_blake3 = DRIFT_IDENTITY.to_owned();
        }
        "theorem-symbol-drift" => {
            inputs.profile.theorem_mappings[0].lean_theorem_symbol = "wrong_theorem".to_owned();
        }
        "trust-closure-drift" => {
            inputs.profile.trust_closure.complete = false;
            inputs.profile.consumer.trust_closure_complete = false;
        }
        "budget-identity-drift" => {
            inputs.profile.resource_policy.identity = DRIFT_IDENTITY.to_owned();
        }
        "tool-failure-classification" => {
            receipt_mut(inputs)?.stages[0].status = StageStatus::ProcessFailed;
        }
        "receipt-swap" => {
            receipt_mut(inputs)?.source_identity = DRIFT_IDENTITY.to_owned();
        }
        "missing-artifact-verification" => {
            receipt_mut(inputs)?.artifact_verification = "missing".to_owned();
        }
        "overclaim-text" => {
            receipt_mut(inputs)?.non_claims.retain(|claim| claim != REQUIRED_AUTHORITY_NON_CLAIM);
        }
        unknown => return Err(format!("unknown negative case `{unknown}`")),
    }
    Ok(())
}

fn set_json(root: &mut Value, pointer: &str, value: Value) -> Result<(), String> {
    let selected = root.pointer_mut(pointer).ok_or_else(|| format!("negative pointer `{pointer}` is missing"))?;
    *selected = value;
    Ok(())
}

fn artifact_path(inputs: &Inputs, role: &str) -> Result<String, String> {
    inputs
        .profile
        .artifacts
        .iter()
        .find(|artifact| artifact.role == role)
        .map(|artifact| artifact.path.clone())
        .ok_or_else(|| format!("artifact role `{role}` is missing"))
}

fn receipt_mut(inputs: &mut Inputs) -> Result<&mut super::model::ProofReceipt, String> {
    inputs.receipt.as_mut().ok_or_else(|| "proof receipt is missing".to_owned())
}
