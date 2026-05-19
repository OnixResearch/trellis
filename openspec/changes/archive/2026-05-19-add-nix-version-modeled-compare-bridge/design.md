## Context

The current `nix_version` module already has spec-level Nix version comparison logic and proofs around helper properties. The full executable `compare_versions` surface remains a `#[verifier::external_body]` trusted boundary because matching arbitrary Nix comparator behavior and Rust string/runtime behavior is broader than a small proof-gap closeout slice.

The canonical verification-gaps spec has two adjacent requirements:

- `verification-gaps.nix-version-bridge-decomposition`: define a bounded structural decomposition or preserve the full trusted bridge explicitly.
- `verification-gaps.nix-version-modeled-compare-bridge`: compute admitted finite-model comparisons through verified executable helpers and return no claim for out-of-model inputs.

This change implements the second requirement as a focused, reviewable wrapper.

## Goals / Non-Goals

**Goals:**
- Define admitted finite byte sequences and explicit out-of-model rejection.
- Implement verified executable helpers corresponding to separator skipping, component bounds, numeric parsing, lexicographic/component comparison, and the modeled compare loop.
- Prove admitted modeled results align with `compare_versions_spec` and are limited to `-1`, `0`, or `1`.
- Preserve proof-gap inventory visibility for the original trusted `compare_versions` boundary.

**Non-Goals:**
- No proof that the full Nix/Rust `compareVersions` implementation is trust-free.
- No support for unbounded numeric components, locale/collation behavior, Unicode, or non-ASCII version syntax beyond the existing finite model.
- No removal or hiding of the existing trusted boundary unless separately proven and reflected in proof-gap inventory updates.

## Decisions

### 1. Model admission before comparison

**Choice:** The public modeled wrapper returns `None` for inputs outside the finite model instead of trying to coerce or partially compare them.

**Rationale:** This makes the trust boundary explicit and avoids replacing one broad trusted claim with another implicit one.

**Alternative:** Treat unknown bytes as separators or alphabetic bytes. Rejected because it would expand semantics beyond the canonical finite model.

### 2. Keep helper proofs small and compositional

**Choice:** Implement separator, component, numeric, and lexicographic helpers as separate verified functions with local postconditions, then compose them in `compare_versions_modeled`.

**Rationale:** Smaller helpers fit existing Verus proof patterns and are easier to diagnose than a monolithic comparator proof.

**Alternative:** Prove the existing runtime comparator directly. Rejected as too broad for this slice.

### 3. Inventory remains authoritative

**Choice:** The full `compare_versions` trusted bridge remains in the manifest/counts unless the implementation actually removes it.

**Rationale:** The value of this slice is explicit narrowing, not overclaiming full trust removal.

## Risks / Trade-offs

**Spec overclaiming** → Requirements and tasks must say admitted finite model only, and evidence must keep the original trusted boundary visible.

**Verifier complexity** → Keep the modeled comparator iterative/bounded and use helper lemmas for range/bounds facts rather than nonlinear arithmetic-heavy proofs.

**Inventory drift** → Run `scripts/check-proof-gap-inventory.rs` and full `./scripts/verify.sh` before archive.

## Validation Plan

1. `nix develop -c openspec validate add-nix-version-modeled-compare-bridge --strict`
2. Focused tests: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c cargo test --quiet nix_version`
3. `nix develop -c ./scripts/verus.sh src/lib.rs`
4. `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c bash -lc './scripts/check-proof-gap-inventory.rs'`
5. `nix develop -c tracey query uncovered` and `nix develop -c tracey query untested`
6. `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c ./scripts/verify.sh`
7. Sync canonical specs and archive once all tasks are complete.
