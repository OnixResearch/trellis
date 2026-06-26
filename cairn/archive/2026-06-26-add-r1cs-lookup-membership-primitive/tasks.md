## Phase 1: Spec and implementation

- [x] [serial] Inspect current R1CS witness/element surfaces and confirm native Cairn change scope. r[rust-primitives.r1cs-lookup-shape]
- [x] [serial] Author `add-r1cs-lookup-membership-primitive` proposal, design, tasks, and rust-primitives delta spec. r[rust-primitives.r1cs-lookup-active-membership]
- [x] [serial] Implement `src/r1cs_lookup.rs` with verified shape admission, active table membership, and satisfaction helpers. r[rust-primitives.r1cs-lookup-satisfaction]
- [x] [serial] Update `src/lib.rs`, README, `docs/requirements.md`, and canonical `openspec/specs/rust-primitives/spec.md`. r[rust-primitives.r1cs-lookup-verification-coverage]

## Phase 2: Evidence and archive

- [x] [serial] Run focused lookup tests and Verus verification over `src/lib.rs`. r[rust-primitives.r1cs-lookup-satisfaction]
- [x] [serial] Run full clippy, full cargo tests, Tracey coverage, Cairn validation, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[rust-primitives.r1cs-lookup-verification-coverage]
- [x] [serial] Mark tasks complete, sync/archive the change, commit, push, and confirm clean synced status. r[rust-primitives.r1cs-lookup-verification-coverage]
