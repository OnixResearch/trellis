use serde::Serialize;

use super::model::CrossVerifierManifest;
use super::model::ProofReceipt;

const BLAKE3_PREFIX: &str = "b3:";
const RUN_IDENTITY_DOMAIN: &str = "trellis-aeneas-run/v1";
const FIELD_SEPARATOR: u8 = 0;

pub(crate) fn prefixed(bytes: &[u8]) -> String {
    format!("{BLAKE3_PREFIX}{}", blake3::hash(bytes).to_hex())
}

pub(crate) fn pretty_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| format!("serialize JSON: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn run_identity(llbc: &str, translation: &str, generated: &str) -> String {
    let mut framed = Vec::new();
    for field in [RUN_IDENTITY_DOMAIN, llbc, translation, generated] {
        framed.extend_from_slice(field.as_bytes());
        framed.push(FIELD_SEPARATOR);
    }
    prefixed(&framed)
}

pub(crate) fn receipt_identity(receipt: &ProofReceipt) -> Result<String, String> {
    let mut projection = receipt.clone();
    projection.receipt_identity.clear();
    pretty_bytes(&projection).map(|bytes| prefixed(&bytes))
}

pub(crate) fn manifest_identity(manifest: &CrossVerifierManifest) -> Result<String, String> {
    let mut projection = manifest.clone();
    projection.manifest_identity.clear();
    pretty_bytes(&projection).map(|bytes| prefixed(&bytes))
}
