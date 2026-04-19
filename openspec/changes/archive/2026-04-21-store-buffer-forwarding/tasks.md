## Phase 1: Core model and implementation

- [x] Run the exact pre-change repo-audit commands in `evidence/repo-sync.txt`: `git show HEAD:src/lib.rs | rg -n '^pub mod store_buffer;' || true`, `git show HEAD:docs/requirements.md | rg -n 'store-buffer\\.' || true`, `git ls-tree -r --name-only HEAD openspec/specs | rg 'store-buffer/spec.md' || true`, and `git show HEAD:README.md | rg -n 'store buffer|store-buffer' || true`; record the output excerpt showing no matches.
- [x] Create `src/store_buffer.rs` with `StoreEntry { addr, size, data, seq }`, `valid_access_size`, `well_formed_buffer`, and `enqueue(buffer, entry, capacity)` with valid-size and strict-`seq` input-domain contracts plus fail-closed `len >= capacity` rejection.
- [x] Add the remaining exec/spec surfaces in `src/store_buffer.rs`: FIFO `drain_oldest`, overflow-safe offset-arithmetic `can_forward`, youngest-match `forward_lookup`, and exact fence emptiness.
- [x] Export the module from `src/lib.rs` so targeted and repo-level validation run against the wired crate surface.

## Phase 2: Proof artifacts, targeted tests, and targeted validation

- [x] Add input-domain proofs: successful enqueue/drain preserve valid sizes and strict `seq` order; `can_forward` requires valid store/load widths in `1..=8`; `forward_lookup` requires valid load widths plus a well-formed buffer; `drain_oldest` requires a well-formed buffer; `enqueue` models non-increasing `seq` as out of domain.
- [x] Add capacity proofs: `enqueue_full_returns_none`, `enqueue_full_preserves_buffer`, `enqueue_nonfull_grows`, and `drain_nonempty_shrinks`.
- [x] Add forwarding proofs: `can_forward_implies_range_coverage`, `can_forward_nonwrapping_boundary`, `forward_index_prefix_maximal`, and `forward_lookup_youngest_match`.
- [x] Add drain/fence proofs: `drain_empty_preserves_buffer`, `drain_order_is_identity`, `drain_preserves_sequence_numbers`, `empty_buffer_satisfies_fence`, `nonempty_buffer_blocks_fence`, `drain_all_remaining_is_empty`, and `post_fence_store_enters_empty_buffer`.
- [x] Add unit tests and proptests for enqueue/drain roundtrips, full-buffer rejection, empty-drain rejection, exact/subrange forwarding, partial rejection, reversed-address rejection, a high-address non-wrapping forwarding boundary case, youngest-match selection, cache fallback, fence emptiness, brute-force forwarding equivalence, FIFO drain replay, and capacity preservation.
- [x] Run targeted module validation with fresh captured output: `cargo test --quiet store_buffer` and `./scripts/verus.sh src/store_buffer.rs`. Save evidence to `evidence/cargo-test-store-buffer.txt` and `evidence/verus-store-buffer.txt`.

## Phase 3: Requirement sync, validation, and archive

- [x] Mirror `r[store-buffer.input-domain]`, `r[store-buffer.full-coverage]`, `r[store-buffer.youngest-match]`, `r[store-buffer.cache-fallback]`, `r[store-buffer.drain-ordering]`, `r[store-buffer.fence]`, and `r[store-buffer.capacity]` into `docs/requirements.md`, add explicit `r[impl]`/`r[verify]` anchors for each requirement in the defining APIs, proof artifacts, and tests in `src/store_buffer.rs`, and sync `openspec/specs/store-buffer/spec.md`.
- [x] Run `openspec_gate` for `proposal`, `design`, and `tasks`, capture outputs in `evidence/openspec-gate-proposal.txt`, `evidence/openspec-gate-design.txt`, and `evidence/openspec-gate-tasks.txt`, and resolve any findings before archive.
- [x] Run repo-level validation with fresh captured output: `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Save evidence to `evidence/cargo-test.txt`, `evidence/clippy.txt`, `evidence/verus.txt`, `evidence/check-ensures.txt`, `evidence/verify.txt`, `evidence/tracey-uncovered.txt`, and `evidence/tracey-untested.txt`, and resolve uncovered/untested results to zero before archive.
- [x] Assemble the archive review packet: `evidence/review-index.md`, `evidence/tasks-review-packet.txt`, `evidence/validation-review-packet.txt`, concise `evidence/cargo-test.txt`, optional `evidence/cargo-test.raw.txt`, and chunk oversized evidence under `evidence/chunks/` if needed.
- [x] Archive `openspec/changes/store-buffer-forwarding` under `openspec/changes/archive/` with the review packet in place.
