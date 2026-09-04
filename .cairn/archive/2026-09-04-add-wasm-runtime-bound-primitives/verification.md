# Verification

## Result

The finite WebAssembly runtime-bound component is implemented, synchronized,
and archived. It provides pure transition admission, exact fuel, deterministic
segmentation, identity-bound continuation, capped allocation, explicit growth
policy, and exact chunk schedules.

The component does not call a host or concrete runtime. It does not verify
SpaceWasm, Wasmtime, decoding, unsafe allocation, scheduling, migration,
sandboxing, certification, or downstream adapters.

## Evidence

| Check | Result |
|---|---|
| Existing Wasm baseline | 45 tests passed |
| Focused runtime-bound tests | 27 passed, including 3 property tests |
| Focused Verus control proofs | 7 verified, 0 errors |
| Focused Verus resource proofs | 7 verified, 0 errors |
| Focused Verus schedule proofs | 6 verified, 0 errors |
| Strict Clippy | passed |
| Full `scripts/verify.rs` wrapper in the pinned Nix shell | `ALL CHECKS PASSED` |
| Full-library Verus | 11,032 verified, 0 errors |
| README metrics | 11,032 proofs, 5,174 tests, 2,226 requirements, 621 modules, 16 boundaries |
| Tracey uncovered | 0 of 2,226 |
| Tracey untested | 0 of 2,226 |
| Proof artifact manifest | 9 primitives, 1 Preserves envelope, 11 fixtures passed |
| Proof evidence bundle and IR chain | 5 records passed |
| Consumer profile | 39 modules, 42 artifacts, 10 fixtures passed |
| Pinned Nix Octet gate | 3,867 accepted/current, 0 new, 0 stale |
| Cairn initial validation and gates | passed |
| Cairn final validation, tasks gate, and accepted-spec sync | passed |
| Cairn archive execution | passed |

Evidence files are under `evidence/baseline-2026-09-03/`,
`evidence/final-2026-09-04/`, and `evidence/octet/`.

## Architecture and authority

All new domain decisions are deterministic functional-core code. Inputs carry
the model, observations, budget, identities, policy, and current state.

The core performs no filesystem, network, clock, random, process, or host I/O.
The growth wrapper only delegates to Trellis's existing checked in-memory Wasm
primitive after explicit policy and shape admission.

Callers retain runtime, authority, observation, serialization, migration,
resource-enforcement, and evidence-mapping responsibility.

## Proof search and audit

Three correlated serial lenses were used because subagent consent was absent.

| Family | Mechanism | Result |
|---|---|---|
| callback transition | opaque callback-driven stepping | rejected because it hides deterministic facts from Verus |
| total state table | one implicit transition per state | rejected because it cannot represent or reject duplicate observation keys |
| keyed finite facts | bounded `(state, observation)` rows with unique-key admission | selected and validated |

The adversarial pass checked malformed tables, exhausted observations, missing
transitions, fuel underflow, model and observation drift, forged continuation
boundaries, all stale identity fields, allocation overflow, closed phases,
invalid memory, denied growth, chunk overflow, gaps, overlap, reordering,
trailing bytes, ambiguous empty input, and claim promotion.

## Bounded broad-gate blocker

`NIX_CONFIG='builders =' nix flake check -L` reaches the immutable
`consumer-release-profile` check and fails against its old pinned source.
That source reports `src/lib.rs` as
`47826:a8d89d79f66dbf6ca14869a8384c71dfd807898c5b81e7f6c020f64a6ab94fde`.
The current checked profile records
`50852:99a8938764b5705c11925252fd5b5b8da1f600ce73ed77ed5d1bd57578ba08ed`.
The same immutable source also has the old proof manifest.

This release-pin drift is outside this change. The focused required Nix Octet
gate and the current-tree consumer-profile check both pass.

## Baseline tool note

The host-only baseline Verus command found an old ambient Cargo and Verus
combination. Its failure is preserved. Final Verus and wrapper evidence use the
repository's pinned Nix shell.
