## Context

`proof-review-candidates.rs` deterministically selects source findings and renders focused advisory packets. Its current contract prevents trust-boundary expansion and asks for concrete proof, specification, and executable-test gaps, but a hard target can still receive several paraphrases of one mechanism or a reduction whose missing lemma is effectively the original target.

## Decisions

### 1. Preserve deterministic packet construction

**Choice:** Keep every new instruction as a fixed Rust string covered by the existing script self-test.

**Rationale:** Packet generation remains reproducible and does not add runtime prompt loading, model classification, network access, or ambient configuration.

### 2. Group routes by mechanism

**Choice:** Ask the reviewer to group materially different proof proposals by induction, invariant, algebraic identity, decomposition, finite search, refinement, or another concrete mechanism and to collapse paraphrases.

**Rationale:** Diversity should measure distinct reasoning mechanisms, not wording variation.

### 3. Expose reduction strength

**Choice:** Require each proposed helper lemma or reduction to be labeled `simpler`, `equivalent`, `stronger`, or `unknown` relative to the selected target. Equivalent and stronger gaps are reported as blocked.

**Rationale:** This prevents an unresolved theorem-strength restatement from being presented as near-completion.

### 4. Require an adversarial proof audit

**Choice:** Add a fixed checklist for precondition satisfiability, vacuity, quantifier scope, arithmetic and index bounds, decreases, exec/spec correspondence, and trust-boundary expansion. Rejected sublemmas should include a concrete counterexample or failed obligation when feasible.

**Rationale:** These are recurring Trellis proof failure modes and produce actionable evidence instead of generic skepticism.

### 5. Keep advisory classifications non-authoritative

**Choice:** The packet states that the target or patch must not be assumed correct, while all route and gap classifications remain advisory until checked by repository validation.

**Rationale:** Prompt pressure must not manufacture proof success or replace Verus.

## Risks / Trade-offs

- The packet grows slightly; the added contract remains bounded and source excerpts retain existing limits.
- A reviewer can misclassify gap strength. The classification is triage metadata only and must be reconciled against actual proof obligations.
- Mechanism grouping is model-supplied rather than semantic deduplication. A future typed portfolio registry may validate stronger structure without changing this script's advisory boundary.
