# AI Proof Assist Specification

## ADDED Requirements

### Requirement: Proof obligations are typed candidates

r[trellis.ai_proof_assist.obligations] Trellis MUST define a typed proof-obligation contract for contracts, predicates, loop invariants, and proof-assert steps such that every candidate carries declared prover-facing shape, source scope, and failure-iteration linkage without host effects.

#### Scenario: Complete obligation facts are admitted

- GIVEN a caller supplies complete bounded in-memory obligation facts
- WHEN obligation validation runs
- THEN the core MUST return deterministic admission facts or typed blockers
- AND it MUST perform no file, process, network, clock, or verifier effect.

#### Scenario: Obligation omits declared shape

- GIVEN an obligation candidate omits prover-facing shape or source scope
- WHEN obligation validation runs
- THEN the core MUST reject the candidate.

### Requirement: Candidates carry provenance before checking

r[trellis.ai_proof_assist.candidates] Every candidate MUST bind the proposing run or generator identity, source revision, and input hashes before the prover checks it, and the workflow MUST admit no candidate whose provenance is missing or stale.

#### Scenario: Candidate has complete provenance

- GIVEN a candidate binds a proposing run, source revision, and input hashes
- WHEN candidate admission runs
- THEN the workflow MAY schedule the candidate for checking.

#### Scenario: Candidate provenance is stale

- GIVEN a candidate references a source revision that no longer matches the checked tree
- WHEN candidate admission runs
- THEN the workflow MUST reject the candidate before any prover run.

### Requirement: Machine check is the sole acceptance authority

r[trellis.ai_proof_assist.machine_check] A candidate obligation MUST be accepted only on an explicit success verdict from the pinned prover over the recorded inputs; failed goals MUST be preserved with their diagnostics as recorded blockers for later iteration.

#### Scenario: Prover confirms the goal

- GIVEN an admitted candidate completes an explicit pinned-prover success run
- WHEN verdict handling runs
- THEN the workflow MAY mark the obligation accepted
- AND acceptance MUST NOT rest on generation or formatting confidence.

#### Scenario: Prover rejects the goal

- GIVEN an admitted candidate receives a rejected or inconclusive verdict
- WHEN verdict handling runs
- THEN the workflow MUST record the failed goal and its diagnostics as blockers
- AND it MUST NOT mark the obligation accepted.

### Requirement: Receipts are bounded and safe

r[trellis.ai_proof_assist.evidence] Proof-assist receipts MUST bind obligation identity, verifier and version, input and source hashes, verdict class, and normalized proof-output identity while excluding full transcripts, prompts, and raw solver dumps beyond declared bounds.

#### Scenario: Receipt candidate carries a transcript beyond bound

- GIVEN a receipt candidate packs a full transcript or raw solver dump beyond bounds
- WHEN receipt validation runs
- THEN validation MUST reject or redact the payload before persistence.

### Requirement: Non-claims are preserved

r[trellis.ai_proof_assist.nonclaims] Passing proof-assist evidence MUST NOT claim verifier soundness, compiler correctness, whole-program safety, Rust semantic correctness, downstream system correctness, or release eligibility.

#### Scenario: Passing receipt is promoted to verifier soundness

- GIVEN a passing proof-assist receipt is labeled as verifier soundness or whole-program correctness
- WHEN non-claim validation runs
- THEN the evidence MUST fail.

### Requirement: The rail has positive and negative fixtures

r[trellis.ai_proof_assist.fixtures] The proof-assist rail MUST include positive accepted-proof, deterministic-replay, and receipt-identity fixtures plus negative rejected-goal, stale-provenance, wrong-verifier-identity, malformed-obligation, receipt-leak, and non-claim-overclaim fixtures.

#### Scenario: Rail is proposed for product use

- GIVEN obligations, candidates, verdicts, receipts, docs, and fixtures are complete
- WHEN focused Cargo, no-std, Wasm-isolation, octet, Cairn, and Nix validation runs
- THEN every positive fixture MUST pass
- AND every negative fixture MUST fail at its declared boundary.
