# Tasks

## Phase 1: Baseline and prerequisites

- [ ] [serial] Record the proof corpus, Verus toolchain pin, and existing proof-evidence baselines before core changes. r[trellis.ai_proof_assist.obligations]
- [ ] [serial] Record accepted non-claim boundaries for proof evidence and keep them unchanged. r[trellis.ai_proof_assist.nonclaims]

## Phase 2: Core and shell

- [ ] [serial] Define obligation, candidate, verdict, blocker, and receipt DTOs in a focused `no_std + alloc` family. r[trellis.ai_proof_assist.obligations] r[trellis.ai_proof_assist.evidence]
- [ ] [serial] Implement pure obligation admission, provenance binding, verdict normalization, blocker classification, and receipt payload construction. r[trellis.ai_proof_assist.candidates] r[trellis.ai_proof_assist.machine_check]
- [ ] [serial] Add the separate command shell that invokes the pinned prover, reads source and binary inputs, maps process exits, and renders output. r[trellis.ai_proof_assist.machine_check]

## Phase 3: Evidence and isolation

- [ ] [parallel] Add positive accepted-proof, deterministic-replay, and receipt-identity fixtures. r[trellis.ai_proof_assist.fixtures]
- [ ] [parallel] Add negative rejected-goal, stale-provenance, wrong-verifier-identity, malformed-obligation, receipt-leak, and non-claim-overclaim fixtures. r[trellis.ai_proof_assist.fixtures] r[trellis.ai_proof_assist.evidence] r[trellis.ai_proof_assist.nonclaims]
- [ ] [serial] Run focused tests before and after changes, no-std and Wasm-isolation builds, Clippy with warnings denied, octet, Cairn validation and gates, and relevant Nix checks. r[trellis.ai_proof_assist.fixtures]
