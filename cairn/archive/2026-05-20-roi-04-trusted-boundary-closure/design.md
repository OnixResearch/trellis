## Context

Recent proof work has reduced closeable gaps, but 16 permanent trusted/proof-gap sites remain. Some are architectural crypto/hash assumptions; the remaining structural Nix version bridge is narrowable because admitted finite-model inputs already have verified executable helpers.

## Goals / Non-Goals

**Goals:**
- Choose exactly one closeable or narrowable trusted boundary.
- Preserve honest classification of permanent cryptographic assumptions.
- Update inventory counts only after source and proof evidence support the change.

**Non-Goals:**
- No blanket removal of `external_body` markers.
- No replacing real cryptographic assumptions with unsound proofs.
- No broad refactor across unrelated modules.

## Selected Target

- **Target:** `src/nix_version.rs::compare_versions` trusted exec/spec bridge.
- **Current classification:** manifest-listed `permanent-trust-boundary` at `pub fn compare_versions(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)`.
- **Strategy:** narrow the public API so admitted finite-model inputs route through verified executable helpers and only out-of-model or length-overflow inputs call a private trusted fallback, `compare_versions_unmodeled_trusted`.
- **Expected inventory result:** trusted-boundary count remains `16`, but the manifest site moves from public `compare_versions` to the private fallback, reflecting that the broad public bridge no longer claims full exec/spec equality for all inputs.

## Decisions

### Inventory-first target selection

**Choice:** Start by reading `docs/proof-gap-sites.tsv`, `scripts/check-proof-gap-inventory.rs` output, and nearby source requirements before selecting the target.

**Rationale:** The highest-ROI target is the one that is closeable without weakening the trust model.

### Closure or narrowing both count as useful

**Choice:** The implementation may either fully remove a closeable boundary or narrow/reclassify an overly broad boundary with stronger checked wrappers.

**Rationale:** Some crypto/hash boundaries must remain trusted, but better contracts still reduce review risk.

## Risks / Trade-offs

**Unsound proof closure** → Require Verus success plus inventory and requirement alignment; do not use `assume(...)` or hidden replacement trust.

**Scope creep** → Limit the change to one selected boundary and defer other candidates.
