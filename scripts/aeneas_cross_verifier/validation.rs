use std::collections::BTreeMap;

use serde_json::Value;

use super::model::CrossVerifierManifest;
use super::model::FinalOutcome;
use super::model::NegativeCatalog;
use super::model::Profile;
use super::model::ProofReceipt;
use super::model::ResourcePolicy;
use super::model::StageObservation;

#[derive(Clone, Debug)]
pub(crate) struct Inputs {
    pub(crate) profile: Profile,
    pub(crate) profile_bytes: Vec<u8>,
    pub(crate) materials: BTreeMap<String, Vec<u8>>,
    pub(crate) llbc: Value,
    pub(crate) translation: Value,
    pub(crate) resource: ResourcePolicy,
    pub(crate) negative: NegativeCatalog,
    pub(crate) receipt: Option<ProofReceipt>,
    pub(crate) manifest: Option<CrossVerifierManifest>,
}

pub(crate) fn validate_base(inputs: &Inputs) -> Vec<String> {
    let mut diagnostics = Vec::new();
    super::profile_validation::validate(inputs, &mut diagnostics);
    super::artifact_validation::validate(inputs, &mut diagnostics);
    finish(diagnostics)
}

pub(crate) fn validate_complete(inputs: &Inputs) -> Vec<String> {
    let mut diagnostics = validate_base(inputs);
    let Some(receipt) = inputs.receipt.as_ref() else {
        diagnostics.push("proof receipt is missing".to_owned());
        return finish(diagnostics);
    };
    super::receipt_validation::validate_receipt(inputs, receipt, &mut diagnostics);
    let Some(manifest) = inputs.manifest.as_ref() else {
        diagnostics.push("cross-verifier manifest is missing".to_owned());
        return finish(diagnostics);
    };
    super::receipt_validation::validate_manifest(inputs, receipt, manifest, &mut diagnostics);
    finish(diagnostics)
}

pub(crate) fn build_receipt(inputs: &Inputs) -> Result<ProofReceipt, String> {
    super::receipt_validation::build_receipt(inputs)
}

pub(crate) fn build_manifest(inputs: &Inputs, receipt: &ProofReceipt) -> Result<CrossVerifierManifest, String> {
    super::receipt_validation::build_manifest(inputs, receipt)
}

pub(crate) fn classify(observations: &[StageObservation]) -> FinalOutcome {
    super::receipt_validation::classify(observations)
}

pub(crate) fn material<'a>(inputs: &'a Inputs, path: &str) -> Result<&'a [u8], String> {
    inputs.materials.get(path).map(Vec::as_slice).ok_or_else(|| format!("material `{path}` is missing"))
}

fn finish(mut diagnostics: Vec<String>) -> Vec<String> {
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}
