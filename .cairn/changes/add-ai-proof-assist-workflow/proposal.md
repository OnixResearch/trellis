# Proposal: AI-assisted proof workflow with machine-checked verdicts

## Why

Trellis owns Verus-checked logic primitives and a proof corpus. Producing contracts, predicates, and loop invariants is the main cost of growing that corpus. MoonBit 0.9 proves a workflow where an AI agent proposes proof obligations and the prover machine-checks every step. Trellis has no equivalent first-class loop today. Candidate obligations arrive untyped, verdicts are not recorded as evidence, and failed goals disappear.

A typed proof-assist workflow turns the AI-guesses, prover-checks model into a Trellis-owned rail. The pinned prover stays the only acceptance authority. The change introduces no new language or verifier.

## What Changes

- Add a typed proof-obligation contract for contracts, predicates, loop invariants, and proof-assert steps. r[trellis.ai_proof_assist.obligations]
- Record every candidate with its proposing run, generator identity, and source hashes before the prover sees it. r[trellis.ai_proof_assist.candidates]
- Make the pinned Verus proof run the sole acceptance authority; a candidate is accepted only on an explicit prover success verdict. r[trellis.ai_proof_assist.machine_check]
- Preserve failed goals with their diagnostics so later candidates can iterate against recorded blockers. r[trellis.ai_proof_assist.machine_check]
- Emit bounded receipts that bind obligation identity, verifier and version, input hashes, verdict, and proof output identity without full transcripts. r[trellis.ai_proof_assist.evidence]
- Add positive and negative fixtures for accepted proofs, rejected goals, stale verifier identities, and malformed obligations. r[trellis.ai_proof_assist.fixtures]

## Impact

- **Core**: a focused family owns obligation shape validation, candidate admission, verdict normalization, blocker classification, and receipt payload construction without host effects.
- **Shell**: a separate command owns Verus and SMT invocation, source and binary reads, process exit mapping, and output rendering.
- **Existing owners**: the proof corpus, Valence evidence wrapping, and octet proof rails keep their claims.

## Lifecycle Prerequisites

None. The workflow uses the already-pinned Verus toolchain and existing Trellis proof artifacts.

## Out of Scope

- New provers, new proof languages, automated theorem search, proof generation inside the verifier, or whole-program verification claims.
- Changes to accepted proof semantics, Valence evidence semantics, or release eligibility.

## Affected Specs

- `ai-proof-assist`: obligations, candidates, machine_check, evidence, fixtures, and non-claims.
