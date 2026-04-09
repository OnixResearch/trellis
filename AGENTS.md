# Agent Notes

## Verification
- `./scripts/verify.sh` is green as of 2026-04-09.
- `cargo test` passes (`3191 passed`), `cargo clippy --all-targets -- -D warnings` passes, and `./scripts/verus.sh src/lib.rs` passes (`5372 verified, 0 errors`).
- Tracey currently reports `0 uncovered` requirements and `0 untested` requirements (`1197/1197` covered and verified).

## Tooling gotchas
- `scripts/check-ensures.sh` now scans until the function body starts; the old 15-line false positives on `src/cron_match.rs:matches_schedule` and `src/cron_next.rs:next_occurrence` are fixed.
- `src/cycle_detect.rs:has_cycle` is now an explicit `@verified-logic:no-ensures` closeable boundary instead of an unmarked gap.
- `README.md`'s proof/trust counts should match the live tree. The current code has 29 `#[verifier::external_body]` items across 11 modules.
- `tracey query status` can still print a transient daemon auto-start warning inside `./scripts/verify.sh`; capture the full rerun output before claiming green, because the later uncovered/untested checks may still succeed and the script can still end with `=== ALL CHECKS PASSED ===`.
- Raw proof-audit rerun transcripts are archived at `openspec/changes/archive/2026-04-08-proof-audit-remediation/rerun-logs/`; use an explicit shell command for absence checks (`if [ -d ... ]; then echo ...; fi`) instead of `test -d` when the evidence must show stdout.
- Active OpenSpec changes in this repo use standalone spec docs under `openspec/changes/*/specs/<domain>/spec.md` instead of `ADDED/MODIFIED/REMOVED` delta sections; match the existing change style when adding new change specs.
- Nix Base32 gotcha for `openspec/changes/nix-derivation-primitives`: upstream `BaseNix32::decode` accepts non-canonical alphabet strings, so `decode(encode(x)) == x` is valid but `encode(decode(y)) == y` only holds for canonical encodings (`"0" -> [0x00] -> "00"`, `"z" -> [0x1f] -> "0z"`).
- The in-progress `src/nix_base32.rs` worktree no longer needs closeable base32 trusted boundaries: the canonical roundtrip proof, `encode`, and `decode` are all internalized now, so older notes that counted 1, 2, or 3 base32 trusted boundaries are stale.
- Base32 proof scaffolding that already verifies: `reconstruct_prefix_spec`, `decode_prefix_of_encode`, and `bit_at_padded_bounded`. For `by(bit_vector)` lemmas over shifted bytes, bind the shift count to a `u64` local first; inline `(pos % 8) as u64` can trigger `IntRange` failures.
- The base32 content-equality path now has verified local update lemmas for both cases: `current_byte_update_no_spill`/`next_byte_zero_when_no_spill` for `bit % 8 <= 3`, and `spill_digit_formula_j4`..`j7` plus `current_byte_update_spill`/`next_byte_update_spill` for spill cases. These verify best when the relevant `Seq` bytes and masked-byte expressions are first bound to local `u8` variables before any `by(bit_vector)` step.
- `encode_decode_roundtrip` is now fully internalized in `src/nix_base32.rs`. The key bridge was `reconstruct_prefix_masked`, which proves `extend_to_spec(reconstruct_prefix_spec(bytes, upto), bytes.len()) == prefix_masked_bytes_spec(bytes, upto)`, combined with `prefix_masked_full` and `reconstruct_full_len`.
- The `encode` exec wrapper can also be internalized cleanly by avoiding the old left/right byte formula in proofs: add a verified exec helper `bit_at_padded_exec`, build `encode_digit` from five padded bits, and map digits with a tiny verified `alphabet_char` helper. This required a precondition `bytes@.len() <= ((usize::MAX as int) - 4) / 8` so the expanded output length and `bit + 4` window stay in `usize` range.
- The `decode` exec wrapper can be internalized without loop invariants by switching to a recursive helper `decode_prefix_exec` that mirrors `decode_prefix_spec` exactly. The hard update step becomes manageable if you first internalize `extend_to_exec` and `or_extend_at_exec`, then build `decode_step_exec` out of those pure vector constructors.

## Proof audit hotspots
- Trusted boundaries are not limited to `external_body`: there are also `assume(...)` soundness oracles in `src/topo_sort.rs` and `src/reachability.rs`, plus 5 `@verified-logic:no-ensures` / placeholder-spec surfaces (`index_select.rs`, `nix_closure.rs`, `settlement.rs`, `segregated_free.rs`, `cycle_detect.rs`).
- Tracey is currently green (`0 uncovered`, `0 untested`).
- The recent Tracey drift was cleared with metadata-only fixes: add `r[verify]` tags for untested rules, and add `r[impl]` tags to spec-level implementations when a requirement has no separate exec fn (for example `query_cost::index_scan_cost_spec` and `snapshot_isolation::si_allows_commit_spec`).
- The temporary `src/mesi.rs` (6) and `src/page.rs` (1) trusted boundaries from the 2026-04-08 remediation are now closed; stale notes or inventories that still count MESI/page `external_body` sites are wrong.
- `src/mesi.rs:check_system_invariant` verifies best as a direct prefix-scan with prefix invariants, not by post-hoc count summaries.
- `src/mesi.rs:invalidate_others_spec` closed cleanly with small recursive lemmas for length preservation, unchanged indices below `from`, writer preservation, and target-index invalidation.
- Keep MESI exec paths iterative. A recursive exec helper over cache entries verifies, but it is a runtime regression because recursion depth grows with `caches.len()` and can blow the Rust stack.
- `src/page.rs:radix_index_bounded` closed with a tiny bit-vector helper over `(value >> offset) & ((1 << width) - 1)` plus an explicit arithmetic proof that the computed offset is `< 64`.
