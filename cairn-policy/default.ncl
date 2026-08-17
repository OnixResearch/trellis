let Contracts = import "contracts.ncl" in
let Base = import "generated/cairn-policy.base.json" in
let TrellisTraceabilityPolicy = {
  default_profile = "trellis-rust",
  profiles = [
    {
      id = "cairn-default",
      requirement_sources = [
        { root = ".cairn/specs", source_kind = "accepted_specs", extensions = ["md"] },
      ],
      evidence_sources = [
        { root = "src", source_kind = "rust_source", extensions = ["rs"] },
        { root = "scripts", source_kind = "tool_source", extensions = ["rs", "sh"] },
        { root = "cairn-policy", source_kind = "nickel_source", extensions = ["ncl"] },
      ],
      marker_verbs = ["impl", "verify", "depends", "related"],
    },
    {
      id = "trellis-rust",
      requirement_sources = [
        { root = "docs/requirements.md", source_kind = "requirements_doc", extensions = ["md"] },
      ],
      evidence_sources = [
        { root = "src", source_kind = "rust_source", extensions = ["rs"] },
        { root = "tests", source_kind = "rust_source", extensions = ["rs"] },
      ],
      marker_verbs = ["impl", "verify", "depends", "related"],
    },
  ],
} in
let TrellisStackProvenanceGate = {
  mode = "optional",
  allowed_claim_scopes = ["stack_provenance.v1.lifecycle_policy_conformance"],
  accepted_valence_policy_hashes = ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
  required_boundary = "Cairn consumed Valence and Mantle stack provenance receipts for lifecycle and release policy conformance only; it did not prove source-code correctness, Valence verifier soundness, Mantle build correctness, or Trellis or Octet proof validity",
} in
let TrellisFunctionAddressGate = {
  mode = "optional",
  allowed_claim_scopes = ["function_address.v1.lifecycle_policy_conformance"],
  accepted_valence_policy_hashes = ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
  required_roles = [
    "function-address-evidence-sidecar",
    "valence-function-address-evidence-profile",
    "kamacite-function-address-receipt",
  ],
  required_schemas = [
    "valence.function-address-evidence.v1",
    "function-address-evidence-v1",
    "kamacite.function-address-receipt.v1",
  ],
  required_boundary = "Cairn consumed Valence, Mantle, and Kamacite function-address evidence for lifecycle and release policy conformance only; it did not prove Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility",
} in
(Base & {
  name | force = "trellis-default",
  traceability_policy | force = TrellisTraceabilityPolicy,
  stack_provenance_gate | force = TrellisStackProvenanceGate,
  function_address_gate | force = TrellisFunctionAddressGate,
}) | Contracts.Policy | Contracts.PolicyIntegrity
