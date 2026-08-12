## Context

The Shamir stack already matches the intended runtime behavior, but six
modules still cross the exec/spec boundary through `#[verifier::external_body]`
wrappers and trusted proof stubs. Goal is to internalize those bridges
without changing the public API or the requirement surface.

Affected modules:

- `src/shamir_field.rs`
- `src/shamir_poly.rs`
- `src/shamir_lagrange.rs`
- `src/shamir_split.rs`
- `src/shamir_reconstruct.rs`
- `src/shamir_threshold.rs`

## Goals / Non-Goals

**Goals:**
- Remove all `38` closeable Shamir `external_body` sites
- Keep unit tests, proptests, and public behavior unchanged
- Preserve existing Tracey requirement IDs and coverage
- Reduce the global trust inventory to the `15` permanent boundaries
- Sync README and verification-gap inventory to the reduced counts
- Sync the secret-sharing uniqueness prose to the verified Lagrange model already tracked in `docs/requirements.md`

**Non-Goals:**
- Expanding the Shamir feature surface beyond the current modules
- Reworking the permanent trust boundaries
- Introducing probabilistic or cryptographic assumptions beyond the current
  finite-field model

## Decisions

### 1. Prove the algebra bottom-up

**Choice:** Close `shamir_field` and `shamir_poly` first, then use those
lemmas to internalize interpolation, split, reconstruct, and threshold.

**Rationale:** The higher modules are algebraic wrappers over field and
polynomial facts. Bottom-up closure keeps later proof obligations small and
avoids duplicating modular-arithmetic reasoning.

### 2. Keep exec APIs unchanged and strengthen local helpers

**Choice:** Preserve current exec signatures and public behavior. Replace
trusted wrappers with verified helpers local to each module.

**Rationale:** This change is trust reduction, not API redesign. Local proof
helpers let the runtime path stay stable while moving proof burden into
ordinary Verus lemmas.

### 3. Treat Lagrange proofs as the central bridge

**Choice:** Internalize Kronecker-delta, partition-of-unity, interpolation,
and uniqueness in `src/shamir_lagrange.rs` before touching the public
reconstruct proof.

**Rationale:** `split`, `reconstruct`, and the threshold theorems all depend
on interpolation facts. If the basis and interpolation lemmas verify cleanly,
the remaining modules should become thin wrappers.

### 4. Model threshold security through witness construction

**Choice:** Keep `src/shamir_threshold.rs` in the current constructive style:
for any `k - 1` visible shares and target secret, prove existence of a
completion consistent with those shares.

**Rationale:** That matches the requirement text already in
`docs/requirements.md` and avoids adding probabilistic reasoning to the
formal model.

### 5. Refresh inventory only from fresh scans

**Choice:** Update `README.md` and `openspec/specs/verification-gaps/spec.md`
only after a fresh trusted-site scan and fresh rerun of the green pipeline.

**Rationale:** This repo's done review is strict about proof-posture claims.
The reduced counts must come from live evidence, not arithmetic done from
memory.

### 6. Narrow uniqueness prose to the verified requirement surface

**Choice:** Keep the Lagrange uniqueness proof at the fixed-x / equal-values
level and sync `openspec/specs/secret-sharing/spec.md` to that statement.

**Rationale:** The verified proof and Tracey requirement already talk about
uniqueness of the value vector on a chosen interpolation set. The stronger
coefficient-equality wording was spec drift, not intended new scope.

## Risks / Trade-offs

**[Field inverse proof brittleness]** -> Extended-GCD or inverse-witness
proofs can trigger solver instability. Mitigation: prove witness properties
separately from the exec wrapper and reuse small modular-arithmetic lemmas.

**[Lagrange explosion]** -> Basis and interpolation proofs can fan out into
many similar helper lemmas. Mitigation: keep suffix/prefix helper structure
aligned with existing runtime recursion instead of forcing one giant theorem.

**[Inventory count drift]** -> Removing Shamir boundaries shifts line numbers
in the remaining trusted files. Mitigation: regenerate the full inventory
from a fresh scan before updating published counts.
