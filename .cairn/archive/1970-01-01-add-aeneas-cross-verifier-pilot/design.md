## Context

`abs_diff` is a useful first target because its runtime body is safe, sequential, allocation-free, dependency-free, and already has a Verus postcondition plus symmetry and self-zero proofs. It also exercises conditional unsigned subtraction without requiring string, collection, crypto, unsafe, or concurrency models.

The key uncertainty is not theorem difficulty. It is whether Charon can select and extract the exact regular-Cargo form emitted after `verus!` macro erasure without forcing the entire Trellis crate through unsupported features. This change therefore starts with a discriminating feasibility gate.

## Decisions

### 1. Exact source or explicit blocker; no shadow implementation

**Choice:** The pilot targets the runtime function defined at `src/arithmetic.rs::abs_diff`. The source and selected item identity are measured from the repository. A copied plain-Rust function, manually synchronized projection, or theorem over a lookalike fixture cannot satisfy the pilot.

If Charon/Aeneas cannot process the exact function under a bounded item/crate selection, the pilot records a blocked feasibility receipt and stops before proof promotion.

**Rationale:** Proving a duplicate would introduce the semantic-equivalence obligation the pilot is supposed to avoid.

### 2. Consume Octet's provider contract

**Choice:** Trellis pins a reviewed Octet revision that exposes the `aeneas-lean` package/profile and runs it through `cargo octet prove` or the exact provider entrypoint. The selected provider must supply its reviewed effective Charon extraction contract, measured `translation.json`, theorem-adequacy policy identity, and fixture-gated compatibility receipt. Trellis owns its source selection, theorem source, property mapping, receipt, and review. It does not fork Aeneas/Charon/Lean package logic.

**Rationale:** Octet owns checked Rust proof execution; Trellis owns the proof corpus and local claim.

### 3. Make feasibility a deterministic result

**Choice:** The feasibility core classifies the attempt as `supported` or `blocked` from typed stage observations. Supported status requires non-errorful LLBC, borrow-check-preserving reviewed extraction options, exact target metadata, a measured `translation.json`, and a complete declaration/file mapping to generated Lean. A blocked result names the exact source identity, provider identity, failing stage, stable diagnostic class, bounded diagnostic artifact identity, unsupported construct when known, and next smallest check. Tool crashes or missing packages are failures, not unsupported-language conclusions.

**Rationale:** A failed setup must not be misreported as an Aeneas language limitation, and an unsupported feature must not be reported as a failed theorem.

### 4. Prove the existing runtime properties directly in Lean

**Choice:** When translation succeeds, the hand-written Lean layer states and checks:

- symmetry of the generated `abs_diff` function; and
- self-difference equals zero.

The theorem map records the existing Trellis requirement ID, approved Lean theorem-statement BLAKE3, claim class, Verus proof symbol, Lean theorem symbol, exact runtime source identity, generated-model identity, assumptions/trust closure, and both proof receipts. It does not assert that Verus specifications and Aeneas models are definitionally or semantically equivalent.

**Rationale:** Matching human-readable property intent is useful evidence; claiming model equivalence would require a separate proof.

### 5. Require theorem adequacy before proof promotion

**Choice:** Each positive Lean theorem has a reviewed statement hash, requirement ID, claim class, exact integer/overflow model, and complete assumption/model/axiom/opaque-definition closure. The pilot rejects `sorry`, `admit`, undeclared trust inputs, and vacuous `True`-style statements for the declared property class before a passing Lean result can enter the cross-verifier manifest.

**Rationale:** A checker success says the encoded theorem is derivable; it does not establish that the statement is the intended Trellis property.

### 6. Separate generated and maintained Lean files

**Choice:** Aeneas-generated definitions are regenerated into an owned output path. Hand-maintained theorems import those definitions from a separate path that regeneration never overwrites. A freshness checker binds exact source, reviewed extraction contract, LLBC, measured `translation.json`, generated-file inventory, and theorem source. Manifest `def_id` values are retained only as current-run joins; durable Trellis identities remain BLAKE3-bound.

**Rationale:** Hand proof survival and generated-source freshness are different obligations.

### 7. Require negative theorem and tamper evidence

**Choice:** Intentionally false and vacuous `True` theorems live outside positive selection. The false theorem must fail in Lean; the vacuous theorem may check in Lean but must fail Octet/Trellis adequacy admission. Negative fixtures also cover partial/errorful extraction, `skip_borrowck`, target/profile drift, source changes after LLBC generation, stale or mismatched `translation.json`, cross-run `def_id` reuse, stale generated Lean, theorem-symbol or statement-hash mismatch, changed/undeclared trust inputs, swapped receipts, missing Octet artifact verification, and overclaim text.

**Rationale:** The pilot needs evidence that failure paths do not accidentally produce passing cross-verifier status.

### 8. Cross-verifier status remains a conjunction of bounded facts

**Choice:** A passing manifest means:

- the existing Verus receipt passed for its declared source/spec/property;
- the Octet Aeneas/Lean receipt passed for its declared source/generated-model/theorem/property;
- both receipts bind the same runtime source identity; and
- the theorem map and required non-claims validate.

It does not merge assumptions, transfer soundness, or prove the two semantic models equivalent.

**Rationale:** Independent mechanisms reduce correlated implementation risk only when their differences remain visible.

### 9. Keep rollout tiny

**Choice:** No second Trellis function is admitted in this change. The pilot uses named execution, log, and artifact budgets inherited from the provider and records their identities and outcomes. Expansion requires reviewing the pilot evidence, model burden, resource cost, failure modes, and receipt stability in a later Cairn.

**Rationale:** A successful arithmetic microcase does not establish that parsers, allocators, concurrency primitives, unsafe code, or large dependency graphs are suitable.

## Risks / Trade-offs

- Exact-source extraction may be blocked by macro expansion or crate selection. That is a valid feasibility outcome, not permission to add a duplicate implementation.
- The two theorem statements may look equivalent while differing in integer model or overflow semantics. The theorem map must show exact types and assumptions and must not claim semantic equivalence.
- A second verifier increases maintenance and toolchain cost. The pilot records named execution/log/artifact budget identities and outcomes but does not claim performance suitability.
- Aeneas declaration IDs are not stable across runs. The pilot uses them only to check the frozen LLBC-to-Lean mapping and never as Trellis source or theorem identity.
- A passing independent proof can encourage whole-library overclaims. Scope and non-claim checks remain release blockers for the manifest.
