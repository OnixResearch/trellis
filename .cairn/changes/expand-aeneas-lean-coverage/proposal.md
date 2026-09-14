## Why

Trellis completed the exact-source Aeneas/Lean pilot for one function. The
archive is `.cairn/archive/1970-01-01-add-aeneas-cross-verifier-pilot/`, the
outcome is `supported`, and the reviewed Octet `aeneas-lean` provider rail with
its reusable consumer contract is archived in Octet. The next question is
scale.

Aeneas translation cannot see Verus ghost code. Charon extracts the
Cargo-erased runtime MIR, so `spec fn`, `proof fn`, `requires`, and `ensures`
never reach Lean. Measured facts from this tree:

- 596 top-level `src/*.rs` files; 579 contain `verus!`; 578 import `vstd`.
- 374 files use only primitives: no `Vec<`, no `&mut`, and no reference to
  blake3, ed25519, redb, regex, serde_json, bs58, or anyhow.
- 212 files mention `Vec<`; 62 mention `&mut`.
- 16 `#[verifier::external_body]` sites in 6 files carry SMT trust.

The README headline is 624 modules, 11098 Verus proofs, 2270 requirements, and
16 trusted boundaries. A blanket claim like "all of Trellis is proved in Lean"
cannot work. Every Lean theorem needs its own approved statement, requirement
ID, claim class, and trust closure. Coverage must come from measured class
counts, not from a plan promise.

## What Changes

- Add a measured Aeneas/Lean coverage frontier. Each top-level module gets an
  eligibility class from recorded extraction facts, not from a hand-written
  list.
- Add a requirement-to-statement registry keyed by the existing `r[...]` IDs.
  Each targeted requirement records its Lean statement BLAKE3, claim class,
  tier, and either a proof reference or an explicit non-target reason.
- Run the pilot pipeline over one bounded Tier-1 batch of primitive-only,
  dependency-free modules. Reuse the pilot provider consumption, freshness,
  adequacy, budget, and tamper gates.
- Emit a coverage receipt with counts by class and tier. The receipt never
  claims whole-library proof coverage.
- Keep the pilot claim boundary and the Octet rail non-claims.

## Capabilities

### New Capabilities

- `aeneas-lean-coverage`: measured module eligibility classes, the requirement
  statement registry, bounded batch execution, and the coverage receipt.

### Modified Capabilities

- None. The pilot owns cross-verifier manifest semantics. This change consumes
  that manifest contract and does not redefine it.

## Impact

- **Scope**: the frontier instrument plus one bounded Tier-1 batch. No second
  batch, no Tier-2 campaign, and no trust-boundary closure in this change.
- **Out of scope**: unsafe Rust, concurrent Rust, modules with unsupported
  nested-loop control transfer, hand-written external models, Verus/Aeneas
  semantic equivalence, Verus replacement, and closure of the 16 trust
  boundaries.
- **Dependencies**: the Octet `aeneas-lean` provider rail, the archived Octet
  consumer contract, and the archived pilot result.
- **Testing**: eligibility classifier fixtures, registry validation, batch
  positive theorems, false and vacuous negatives, freshness checks, tamper
  fixtures, budget cases, and overclaim rejection.
