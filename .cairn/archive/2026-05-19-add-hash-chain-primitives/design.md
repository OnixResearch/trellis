## Context

The existing chain module models each entry hash as `hash(prev_hash, index, term, data_id)`, proves chain validity and append preservation, and uses a trusted collision-resistance axiom for agreement/divergence. Downstream modules such as snapshots, replication, receipts, and storage facades often need slightly higher-level facts: the chain tip commits to a prefix, one chain extends another, a checkpoint hash authenticates a prefix boundary, and forks are detectable without open-coding index arithmetic.

## Goals / Non-Goals

**Goals:**
- Add named, reusable chain predicates and proof obligations for the ten proposed primitives.
- Keep cryptographic trust explicit: domain separation and runtime BLAKE3 alignment may name trusted crypto/serialization bridges, but must not hide new assumptions.
- Separate hash-chain validity from log-shape validity so callers can choose the right strength.
- Prefer pure/spec/proof helpers before adding public executable surfaces.

**Non-Goals:**
- No proof of BLAKE3 internals.
- No replacement of the base linear chain with a Merkle or skip-chain structure.
- No broad Raft log semantics beyond index/term shape predicates named here.
- No public runtime CID/receipt/snapshot API changes in this slice unless separately scoped.

## Decisions

### 1. Tip and checkpoint first

**Choice:** Implement `chain_tip_spec` and `checkpoint_valid_spec` before larger fork/batch helpers.

**Rationale:** These are the smallest reusable commitments and give snapshots/receipts a stable anchor.

**Alternative:** Start with skip/Merkle proofs. Rejected because compact proofs are higher cost and compose better after tip/checkpoint semantics are stable.

### 2. Extension is a prefix relation, not a second validity model

**Choice:** `chain_extends_spec(prefix, full)` should mean `prefix` is exactly a prefix of `full`; validity remains a separate predicate.

**Rationale:** This avoids overloading extension with hash validity and lets callers prove `valid full ==> valid prefix` separately.

**Alternative:** Define extension only for valid chains. Rejected because malformed data-shape checks should remain explicit.

### 3. Batch append folds the existing append theorem

**Choice:** Batch append should be specified as a left fold over correctly-hashed entries and proved by reusing `prove_append_preserves`.

**Rationale:** This prevents another independent chain-validity proof and keeps the proof shape predictable.

**Alternative:** Define batch validity by a standalone all-indices quantifier. Rejected because it duplicates existing append reasoning.

### 4. Domain separation is an input contract

**Choice:** Domain tags become part of the hash input contract, either via a domain-tagged spec function or an explicit domain parameter in entry-hash wrappers.

**Rationale:** Hashes for entries, checkpoints, receipts, and snapshots must not be accidentally comparable when they are semantically different.

**Alternative:** Rely on caller naming conventions. Rejected because the error is silent and cross-domain equality can look like valid evidence.

### 5. Compact history proofs are optional overlay

**Choice:** Skip/Merkle/MMR-style compact proofs should compose with base linear chain tips and checkpoints rather than changing `chain_valid_spec`.

**Rationale:** Consumers that only need linear chains should not inherit higher proof complexity.

**Alternative:** Replace linear chain validity with a tree accumulator. Rejected as too disruptive for current modules.

## Risks / Trade-offs

**Verifier load** → Implement in small helper lemmas: tip, prefix, checkpoint, fork, batch, then shape/tamper/domain overlays.

**Trust drift** → Any runtime BLAKE3 bridge or serialization bridge must be counted in proof-gap inventory and documented as a crypto/runtime boundary.

**Spec overclaiming** → The compact-history requirement is optional and must not be treated as required for basic chain validity.

## Validation Plan

1. `nix develop -c openspec validate add-hash-chain-primitives --strict`
2. Focused tests: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c cargo test --quiet chain`
3. `nix develop -c ./scripts/verus.sh src/lib.rs`
4. `nix develop -c tracey query uncovered` and `nix develop -c tracey query untested`
5. `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c ./scripts/verify.sh`
6. Sync `docs/requirements.md` and canonical `openspec/specs/chain/spec.md`, then archive when tasks are complete.
