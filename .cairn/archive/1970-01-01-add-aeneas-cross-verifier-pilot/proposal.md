## Why

Trellis verifies Rust implementations and specifications with Verus, while normal Cargo builds erase ghost code. Aeneas offers a materially different route: Charon extracts compiled Rust into LLBC, Aeneas translates it into Lean, and Lean checks hand-written theorems over that generated model. A small exact-source pilot can test whether the ordinary Trellis runtime path is independently tractable through that route and can expose source/model drift that a single verifier path would not reveal.

The pilot must not assume success. Verus macro erasure, crate breadth, rustc features, external definitions, or Aeneas's safe-Rust subset may block exact-source translation. A checked blocked result with the precise unsupported boundary is preferable to a copied shadow implementation or an unsupported cross-verifier claim.

## What Changes

- Add a bounded Aeneas/Lean feasibility and proof pilot for the exact runtime implementation of `src/arithmetic.rs::abs_diff` and its existing symmetry and self-zero properties.
- Consume the reviewed Octet `aeneas-lean` provider package and receipt contract rather than reconstructing toolchain pins or proof authority locally.
- Attempt Charon extraction from the normal Cargo-erased Trellis source path without copying or rewriting `abs_diff` into a separate fixture, and require Octet's reviewed effective-extraction contract plus non-errorful LLBC.
- Bind Aeneas `translation.json` from the selected LLBC declaration to generated Lean declarations/files while treating `def_id` as a within-run join key rather than durable Trellis identity.
- If exact-source extraction is supported, maintain separate generated Lean definitions and hand-written theorem source, require approved theorem-statement hashes/requirement IDs/claim classes and complete trust closure, run positive theorems plus intentionally false and vacuous negative theorems, and emit a Trellis-owned cross-verifier manifest bound to both Verus and Octet/Aeneas receipts.
- If exact-source extraction is unsupported, emit a deterministic blocked feasibility receipt naming the source scope, stage, tool identities, unsupported construct, and next smallest discriminating check; do not emit formal-proof or equivalence claims.
- Add freshness, tamper, theorem-mapping, and claim-boundary checks with positive and negative fixtures.

## Impact

- **Pilot scope**: one existing dependency-free arithmetic function and two already-declared properties; no broad Trellis corpus migration.
- **Toolchain**: exact Octet provider consumption with local source/proof receipt ownership.
- **Evidence**: a cross-verifier manifest links distinct Verus and Aeneas/Lean claims to the same runtime source identity while preserving separate theorem encodings and assumptions.
- **Generated files**: Aeneas-generated Lean definitions remain regenerable output; hand-written theorem files are separate and never overwritten by regeneration.
- **Testing**: exact-source feasibility, extraction-policy and manifest validation, successful theorem checks when supported, intentionally false theorem rejection, vacuity rejection, trust-closure drift, stale generated output, source drift, receipt mismatch, bounded proof-budget outcomes, and non-claim validation.
- **Claims**: agreement supports only two separately checked theorem claims over one exact function. It does not prove Verus/Aeneas/Charon/Lean/rustc soundness, equivalence between their models, whole-program correctness, absence of trusted assumptions, or downstream runtime behavior.
