use verified_logic::ucan_token::UcanCapability;
use verified_logic::ucan_token::UcanPayload;
use verified_logic::ucan_token::ValidationError;
use verified_logic::ucan_token::validate_envelope;
use verified_logic::web_of_trust_policy::threshold_accepts;
use verified_logic::web_of_trust_principal::PRINCIPAL_ID_LEN;
use verified_logic::web_of_trust_principal::PrincipalId;
use verified_logic::web_of_trust_principal::principal_id_from_digest_bytes;

const NOW: u64 = 100;
const NOT_BEFORE: u64 = 90;
const EXPIRES_AT: u64 = 120;
const TOLERANCE: u64 = 0;
const TRUST_THRESHOLD: usize = 1;
const TRUST_DEPTH: usize = 1;
const DIGEST_FILL_BYTE: u8 = 7;

fn capability(resource: &[u8], ability: &[u8]) -> UcanCapability {
    UcanCapability {
        resource: resource.to_vec(),
        ability: ability.to_vec(),
    }
}

fn payload(iss: &[u8], aud: &[u8], capabilities: Vec<UcanCapability>) -> UcanPayload {
    UcanPayload {
        iss: iss.to_vec(),
        aud: aud.to_vec(),
        att: capabilities,
        prf: Vec::new(),
        exp: EXPIRES_AT,
        nbf: NOT_BEFORE,
    }
}

fn principal() -> PrincipalId {
    let digest = vec![DIGEST_FILL_BYTE; PRINCIPAL_ID_LEN];
    principal_id_from_digest_bytes(digest).expect("fixed digest length is a valid principal id")
}

fn main() {
    let valid_payload = payload(b"did:key:issuer", b"did:key:audience", vec![capability(b"/repo", b"read")]);
    let accepted = validate_envelope(&valid_payload, true, NOW, TOLERANCE);
    assert!(matches!(accepted, Ok(true)));

    let missing_attenuation = payload(b"did:key:issuer", b"did:key:audience", Vec::new());
    let rejected = validate_envelope(&missing_attenuation, true, NOW, TOLERANCE);
    assert!(matches!(rejected, Err(ValidationError::MissingAttenuations)));

    let subject = principal();
    let roots = vec![principal()];
    let attestations = Vec::new();
    let revocations = Vec::new();
    assert!(!threshold_accepts(&roots, &attestations, &revocations, &subject, TRUST_THRESHOLD, TRUST_DEPTH, NOW,));

    println!("ucan/web-of-trust: valid UCAN accepted, empty trust graph rejected");
}
