## Phase 1: Sync artifact scope to the landed MOESI slice

- [x] Add explicit non-goals to `proposal.md` and pin the model to one-line coherence rather than multi-line or directory protocols
- [x] Define the event/system model in `design.md`: processor read/write, snoop `BusRead` / `BusReadX` / `BusUpgrade`, arbitrary cache-count `Seq<u8>`, and a separate `memory_up_to_date` flag
- [x] Ensure every requirement and scenario in `specs/moesi-protocol/spec.md` has an `r[...]` ID line and that the design maps each `r[...]` to concrete code artifacts

## Phase 2: Implement the MOESI module

- [x] Add `src/moesi.rs` and export it from `src/lib.rs`
- [x] Implement state validity, processor/snoop transition helpers, dirty-owner predicates, and whole-line `full_read_spec(...)` / `full_write_spec(...)`
- [x] Implement owner-eviction helpers and memory-source predicates (`memory_consistent_spec`, `read_source_spec`, `read_fresh_spec`)
- [x] Prove single-writer preservation, the `Modified -> Owned` remote-read path, dirty-owner data sourcing, owner-eviction writeback, and MESI refinement outside Owned-producing paths
- [x] Add focused unit tests covering positive and negative transition cases

## Phase 3: Tracey and requirement sync

- [x] Add `r[moesi.*]` requirements to `docs/requirements.md`
- [x] Tag the new public spec/proof/exec surfaces in `src/moesi.rs` with `r[impl]` / `r[verify]` references so `tracey` can cover the new rules
- [x] Keep the no-stale-read requirement precise at the read-source level instead of claiming full payload semantics the module does not model

## Phase 4: Validation evidence

- [x] Run the relevant baseline before the change: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib mesi` and `./scripts/verus.sh src/mesi.rs`
- [x] Run fresh focused validation for the new module: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib moesi`
- [x] Run fresh `openspec_gate` checks for `proposal`, `design`, and `tasks`
- [x] Capture fresh proof-oriented validation evidence with `./scripts/verus.sh src/lib.rs`

## Phase 5: Archive

- [x] Capture fresh archive evidence for the validated landed scope, including gate results, proof/runtime transcripts, archive move result, and post-archive active-change recount output
- [x] Move the synced change under `openspec/changes/archive/<date>-moesi-protocol/`
- [x] Recheck active change count after archive so the autoresearch metric reflects the closeout
