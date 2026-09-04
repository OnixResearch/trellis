let Contracts = import "../contracts.ncl" in
{
  schema_version = "trellis-proof-evidence-v1",
  generated_by = "src/bin/proof_evidence.rs",
  records = [{
    scope = "module:fixture",
    evidence_tier = "local-proof-summary",
    requirement_ids = ["fixture.req"],
    source_paths = ["src/fixture.rs"],
    proof_functions = [],
    algorithm_profile = "not-applicable",
    executable_oracle_symbol = "not-applicable",
    downstream_validation_requirement = "not-applicable",
    proof_command = "./scripts/verus.sh src/lib.rs",
    verifier_toolchain = "fixture Verus",
    proof_outcome = "verified",
    tracey_coverage = "covered",
    proof_gap_references = [],
    maturity_facts = ["test-backed"],
    content_identities = [{ path = "src/fixture.rs", algorithm = "blake3", digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" }],
    claims = ["records local Trellis proof facts for the named scope"],
    non_claims = ["not release approval", "not whole-program correctness", "not third-party conformance"],
  }],
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
