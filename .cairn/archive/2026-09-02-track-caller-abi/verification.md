# Track Caller ABI Verification

## Result

The finite track-caller metadata implementation is complete. Strict validation and all three lifecycle gates pass. Sync created the accepted specification, and archive execution completed under `2026-09-02-track-caller-abi`.

The public `track_caller_abi` facade composes separate model, admission, chain, trace, proof, and test components under `src/track/caller/`. Every production file has fewer than 300 lines.

The core models:

- Rust ABI, closure, naked, item-kind, and annotation-source admission;
- call-site, forwarded, and definition-site location metadata;
- direct tracked and untracked calls;
- bounded tracked-call chains;
- reify shims for tracked function pointers;
- prototype, default-body, and local trait markers; and
- bounded const/codegen trace agreement.

`MAX_METADATA_ENTRIES` is the named chain and trace bound. Invalid inputs return deterministic admission classes or `false` before any state change. The implementation performs no I/O.

## Baseline

The baseline source was `644603b`. Baseline Cargo tests, full-library Verus, and Tracey commands returned status `0`.

The first baseline Tracey daemon start printed its known cancellation message. An immediate direct rerun reported `0 uncovered out of 2062 rules`. See `evidence/baseline-2026-09-02/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 14 passed |
| Focused Verus | 39 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,791 Verus proofs; 4,877 tests; 2,072 requirements; 606 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,072 |
| Tracey untested | 0 of 2,072 |
| Octet | 3,867 accepted; 0 new; 0 stale |
| JSON artifact contracts | positive and negative fixtures passed |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures passed |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |

The full wrapper ran inside the repository Nix development shell. A temporary `.worktrees/aspen` symlink restored the existing `../aspen` sibling relation for the Aspen evidence-index check. The command removed the symlink when it finished.

The full wrapper includes Cargo tests, examples, Clippy, Verus, Tracey, proof evidence, consumer-profile checks, positive fixtures, and negative fixtures. Its Tracey section recorded a transient daemon-start cancellation before the wrapper obtained clean uncovered and untested results. Separate final direct logs contain the clean `0` results.

See `evidence/final-2026-09-03/`.

Cairn defaults an absent archive date to `1970-01-01` for deterministic operation. The first uncommitted archive rename used that default. It was rolled back, its receipt was retained as superseded evidence, and the official archive reran with `CAIRN_ARCHIVE_DATE=2026-09-02`.

## Octet Ratchet

The new `src/track/caller/` files have zero Octet findings. The untouched base already had 20 stale accepted IDs.

Two inherited ambiguous conversions prevented a fresh Cargo build. They now use explicit `u64::from(HDL_EMPTY_COUNT)` conversions. The new module declaration also changes `src/lib.rs` identity.

These three existing source files produced six replacement IDs. The guarded migration removed 26 stale IDs, added only the six exact replacements, and reduced the accepted count from 3,887 to 3,867. It did not add a warning budget or accept a new track-caller finding.

See `evidence/octet/`.

## Generated and Bound Artifacts

The change refreshed these repository-owned artifacts after source identity changed:

- `docs/module-index.md`;
- `consumer-profile/generated/profile.json`, from its Nickel source;
- `docs/proof-evidence-bundle.json`; and
- `docs/proof-evidence-ir-chain.json`.

The checked consumer profile still selects the same 39 UCAN-facing modules. It updates only the measured crate-root artifact.

## Boundaries

This evidence proves finite metadata and runtime-helper refinement for the checked source and toolchain. It does not prove:

- runtime stack walking;
- panic formatting;
- allocation of `core::panic::Location` values;
- concrete ABI lowering or intrinsic implementation;
- platform debug information;
- rustc correctness;
- verifier soundness; or
- whole-codegen correctness.
