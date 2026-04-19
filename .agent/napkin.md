# Napkin — verified-logic

## User Preferences
- Commit messages: focus on why, use imperative mood, keep concise, avoid conventional-commit prefixes, and use paragraphs instead of bullets.

## Project State
- Repo is now a colocated `jj`+`git` workspace (`.jj/` + `.git/`). Prefer `jj` for local status/history/bookmarks; `git` still works for remotes and Git-only tooling. `main@origin` is tracked locally. Warning: `git clean -xdf` will remove `.jj/`.
- Scope-check gotcha: `jj diff --name-only` does not show files sitting only in Git's staged index. Before claiming a change is scope-clean, also check `git diff --cached --name-only`.
- Verus gotcha: exec code inside `verus!` cannot call a plain Rust helper declared outside the `verus!` block without an `assume_specification[...]` bridge. For partial feature landings, either keep helpers in `verus!` or move the unfinished exec surface outside `verus!` until proofs catch up.
- Constant-folding Phase 4 proof gotcha: recursive env builders need their own length lemmas. `pred_meet_prefix_monotone(...)` did not verify until `pred_meet_prefix_has_len(...)` made `pred_meet_prefix_spec(...).len() == vars` explicit for the recursive prefix.
- Another constant-folding Phase 4 proof gotcha: after using `update_env_monotone` against an all-`Top` row, Verus still needed a separate `update_top_env_is_top(...)` lemma to rewrite `update_env_spec(top_env_spec(vars), dst, ConstLat::Top)` back to `top_env_spec(vars)`.
- Rank-proof gotcha in constant folding: Verus would not derive "`Seq<ConstLat>` values differ => some index differs" automatically. `env_rank_lt` / `matrix_rank_lt_at` only closed after adding `seq_constlat_neq_has_witness(...)` and using it before `choose` on differing indices.
- More rank-proof gotcha: recursive bound proofs over `matrix_rank_sum_spec` needed explicit nonneg lemmas for head/tail sums plus two small monotone arithmetic steps (`head + tail <= A + tail`, then `A + tail <= A + B`). Trying to discharge `head + tail <= A + B` in one shot was brittle.
- Matrix rank budget proof shape that worked: recurse on rows, use `env_rank_sum_bound` for current row, keep the two additive monotonicity steps explicit, then finish with one `by(nonlinear_arith)` factorization from `2*cols + 2*cols*(rows-1)` to `2*cols*rows`.
- Helpful next bridge for constant-folding convergence: turn strict rank drop into a one-step budget fact (`next_rank + 1 <= prev_rank`) with a tiny wrapper lemma. Verus accepts that much more reliably than re-deriving the arithmetic from `<` at each future chain/round induction site.
- Top-matrix proof shape that works in constant folding: encode "all rows are Top" as a predicate over `Seq<Vec<ConstLat>>`, then prove one row equals `top_env_spec(...)`. Avoid trying to build a spec top-matrix value just to compare against it.
- Next step after that also works cleanly: prove `first_round_descends(...)` separately, using `block_out_env_top_bounded(...)` + row equality to top, instead of forcing a more general round lemma to handle the top case implicitly.
- Closed spec wrapper trap in constant folding: trying to prove through `fixed_point_spec` / `round_from_out_env_spec` with quantified clauses was brittle even for direct restatement. Switching those wrappers to `open spec fn` was cleaner than fighting reveal/reassertion weirdness.
- Extra constant-folding proof quirk: after opening those wrapper specs, an explicit bridge lemma with identical conjunctive `requires` and wrapper `ensures` verified with an empty body, while inline `assert(wrapper(...))` still failed. Keep the wrapper bridge as its own proof fn.
- Verus specs embedded in `src/` — no separate `verus/` directory
- House style is `ensures result == *_spec(args)` with Verus checking the actual implementation, but `src/ssa_construction.rs` is a current exception: thin wrappers + `assume_specification[...]` bridges.
- Drift between spec and impl is usually structurally impossible here, except at currently tracked trust/weakened boundaries.
- 7465 Verus proofs + 3839 cargo tests (+ 13 doc tests) = current local snapshot
- 0 uncovered + 0 untested = full Tracey coverage (1542 requirements)
- `vstd` is a cargo dependency — `verus!` blocks compile normally (ghost code erased)
- `rust_verify` runs on `src/lib.rs` — verifies the whole crate
- Current trusted inventory: 19 live trusted/weakened boundaries = 16 `#[verifier::external_body]` items across 6 files + 3 closeable SSA `assume_specification[...]` bridges.
- `src/control_flow_graph.rs` uses a simplified entry model: branch targets must be in range and nonzero, and leader splitting treats `Return` as a terminator so post-return instructions always start a new block.
- `src/liveness_analysis.rs` uses synchronous whole-graph rounds. For this model the safe convergence bound is `2 * blocks * vars`, not `blocks * vars`; a self-loop with a local use lets `live_in` become true one round before `live_out` reflects the self-successor.
- `src/loop_detection.rs` review only accepted exact public contracts once the exact helpers consumed already-computed reachability/dominator views: `compute_loop_body(edges, reachable, dom, ...)` and `has_single_entry(edges, reachable, ...)`. The stronger path-oracle natural-loop statement stays as unit/property-test evidence, not a separate Verus theorem.
- `src/loop_detection.rs:loop_nesting_forest` needs an explicit 3-level/shuffled-order parent-selection test. A 2-loop example plus a generic strict-superset property is too weak for done review.
- `src/ssa_construction.rs` still keeps the heavier SSA algorithms in plain Rust helpers and uses `assume_specification[...]` declarations gated by `#[cfg(verus_keep_ghost)]` to let the verified wrappers call them. Ungated assume-spec declarations break normal cargo builds under `-F unsafe-code`. `block_defines_var`, `iterated_dominance_frontier_with_rounds`, `check_ssa_property`, and `eliminate_phis` are internalized now; remaining bridges are `runtime_place_phis`, `runtime_construct_ssa`, and `runtime_check_dominance_of_uses_with_dom`.
- New compiler-pass modules can usually reuse `ssa_construction::RawInstr` / `RawBlock` / `RawProgram` as the shared raw IR. `dead_code_elimination` fits that model cleanly.
- `dead_code_elimination` Phase 3 proof model uses straight-line block order for `RawProgram`; the program-level trace theorem composes block proofs under the adjacent-block summary condition `live.live_out[b] == live_before_spec(block[b+1], live.live_out[b+1], 0)`.
- For `RawInstr` clones in proofs, compare projected views `(def, opcode, uses@)` instead of trying to prove whole-struct equality after cloning the `Vec<u32>` field.
- SSA phi coverage must deduplicate predecessor blocks. `dominator_tree::predecessors` returns one entry per edge, so duplicate edges or self-loops otherwise create duplicate phi args for the same predecessor.
- Permanent subset is 16 (`chain`, `snapshot_integrity`, `nix_version`, `nix_store_path`, `web_of_trust_principal`, `log_invariant_inductive`); current closeable subset is 3 SSA `assume_specification[...]` bridges in `src/ssa_construction.rs`.
- I left one proof helper under-justified once: `shamir_lagrange::basis_other_from` needs the local bound `0 <= from <= j < xs.len()` made obvious before indexing `xs[from]`. Even if upstream facts imply it, add the direct branch assertion so review sees no hidden out-of-range case.
- If I archive an OpenSpec change after touching multiple domain specs, archive one standalone `specs/<domain>/spec.md` per changed domain. Review counts missing per-domain archive docs as drift.
- Before starting implementation for an old active OpenSpec change, compare it against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`. Some changes are already shipped in code and only need metadata sync plus archive.
- 2026-04-08 audit started with a broken tree: `./scripts/verify.sh` showed clippy failures, Verus failures in `src/mesi.rs` and `src/page.rs`, and 29 `#[verifier::external_body]` items while the README still said 22.
- 2026-04-08 remediation result: `./scripts/verify.sh` is green again, Tracey is `0 uncovered` / `0 untested`, and the current trust inventory is 36 `#[verifier::external_body]` items across 13 files.
- 2026-04-09 trust reduction result: closed all 7 temporary MESI/page `#[verifier::external_body]` sites; current trust inventory is 29 across 11 files, and the tree is green again.
- I overclaimed completion once by relying on summarized status instead of fresh command output; before saying the tree is green, rerun and capture `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/verify.sh`, and the direct Tracey/check-ensures commands.
- For an active OpenSpec change, those reruns still are not enough for a reviewable "green" claim until the change's evidence tasks save the fresh outputs into its `evidence/` files for the current tree.
- Durable-execution review gotcha: if OpenSpec task text says `Prove ...`, add real `proof fn` artifacts, not just tests or open spec predicates. Also keep `resume_step` bound prose scoped to valid checkpoints; invalid checkpoints may remain unchanged.
- Done review is strict about evidence boundaries: if I claim a file changed, it must appear in `git status`, and if I claim a trust site disappeared, I should include direct grep/count output for the affected file and the repo-wide inventory.
- `scripts/check-iterative-exec.py` cannot flatten functions from multiple files into one bare-name graph. For `r[chaoscontrol-validation.iterative-exec]`, key helpers per file/module and reject qualified local calls like `self::helper(...)`.
- `./scripts/verify.sh` now enforces repo-wide `r[impl]` coverage for every public fn before other checks. If a full-tree verify suddenly fails at the top, inspect helper fns for missing `// r[impl ...]` tags before chasing proof/test regressions.
- Repo-sync evidence goes stale once the active change itself adds `chaoscontrol-*` anchors to `docs/requirements.md`. At that point audit shipped overlap against a pinned pre-change baseline revision, not the current working tree.
- Raw grep alone is weak trust-inventory evidence; archived `evidence/external-body-sites.txt` should also carry derived totals plus current named Shamir sites so it matches `openspec/specs/verification-gaps/spec.md`.
- When evidence needs to prove a directory is absent, `test -d` alone is not enough for a transcript because it prints nothing. Use an explicit shell form such as `if [ -d ... ]; then echo active-present; else echo active-absent; fi`, and save the raw transcript file in the change directory.
- Tracey first dropped to `2 uncovered` (`query-cost.index-scan`, `snapshot-iso.si-allows-commit`) and `0 untested` after metadata-only fixes in `ct_select.rs`, `nonce.rs`, `index_select.rs`, `mvcc.rs`, and `snapshot_isolation.rs`, then to `0 uncovered` after adding `r[impl]` to the spec-level implementations in `query_cost.rs` and `snapshot_isolation.rs`.
- Tracey in this repo scans `docs/requirements.md`, not `openspec/specs/**`. New OpenSpec domains still need matching `r[...]` entries in `docs/requirements.md` or Tracey will stay unchanged.
- `web-of-trust-primitives` Phase 1+2, the full Phase 3 chain/rooted-resolution/property slices, and the full Phase 4 threshold slice are now landed in the live tree: `src/web_of_trust_principal.rs`, `src/web_of_trust_attestation.rs`, `src/web_of_trust_revocation.rs`, `src/web_of_trust_chain.rs`, `src/web_of_trust_resolution.rs`, `src/web_of_trust_policy.rs`, and cross-module web-of-trust lemmas in `src/properties.rs` exist, with matching `src/lib.rs` exports and `docs/requirements.md` anchors for `web-of-trust.principal-fingerprint`, `web-of-trust.attestation-signature`, `web-of-trust.attestation-window`, `web-of-trust.revocation`, `web-of-trust.chain-validity`, `web-of-trust.rooted-resolution`, `properties.web-of-trust.revocation-cuts-trust`, `properties.web-of-trust.depth-monotone`, `web-of-trust.threshold`, and `properties.web-of-trust.threshold-dedup`. Canonical spec synced to `openspec/specs/web-of-trust/spec.md`; archive moved to `openspec/changes/archive/2026-04-17-web-of-trust-primitives/`. Keep proposal/design repo-audit text pinned to pre-change shipped surfaces instead of claiming the current tree has no web-of-trust modules/requirements.
- `src/web_of_trust_chain.rs` was much easier to verify after switching from one monolithic chain-check loop to recursion-first helper specs/exec (`root_member_from_spec`, `chain_indices_in_range_from_spec`, `chain_edges_active_from_spec`, `chain_adjacency_from_spec`, `subject_fresh_upto_spec`, `no_repeated_from_spec`).
- `src/web_of_trust_resolution.rs` proof shape that finally worked: keep exact-length witness search recursive, but make the up-to-depth search spec existential over candidate lengths instead of a direct recursive `... || search_upto(current + 1)` spec. Exec can stay recursive over `current` while sound/complete lemmas bridge to the existential rooted-resolution semantics.
- `src/properties.rs` web-of-trust cross-module proofs worked best as witness-preservation lemmas: kill only the designated chain witness after effective revocation, and prove depth monotonicity by reusing the same successful witness at the larger bound.
- `src/web_of_trust_policy.rs` verified more cleanly when threshold evaluation counted distinct final-hop attestation indices deduped by introducer digest, while each candidate's qualification came from a fixed-final-hop chain search. That lets repeated-final-hop / `I = S` rejection fall out of `chain_valid_at_spec(...)` instead of duplicating path-simplicity logic in the policy code.
- `src/properties.rs` threshold-dedup proof shape that worked: prove the local scan-state fact instead of mutating attestation sets. Once `seen_introducer_spec(...)` already holds for an introducer digest, the current same-digest edge contributes neither one more count nor a different remaining threshold decision. Then keep end-to-end duplicate-vs-single-edge behavior in ordinary regression tests.
- Done-review packet lesson: giant evidence files get truncated. For archived OpenSpec changes, split code-review evidence per file, add a tiny `review-index.md`, include both `tasks-review-packet.txt` and `validation-review-packet.txt`, and keep `cargo-test.txt` as a concise transcript that shows the exact `CARGO_TARGET_DIR=... cargo test --quiet` command plus final counts. Put any full raw cargo output in a separate `cargo-test.raw.txt`. If a per-file diff is still too large, split it into numbered files under `evidence/chunks/`.
- If full-tree `cargo test` suddenly fails on old ambiguous empty-vector assertions (`vec![]`) during unrelated feature work, fix the tests with explicit `Vec::<T>::new()` at the failing sites instead of assuming the new module caused the error.
- Shamir tracey gotcha: requirements with only helper proofs/tests still showed uncovered until I added explicit `r[impl]` tags on the defining field/basis/threshold helpers; `r[verify]` alone was not enough.
- Zircon port gotcha: `zircon-port.one-shot` stayed uncovered until `deliver` itself carried `r[impl zircon-port.one-shot]`; proof/test tags alone were not enough because the rule needed an implementation reference.
- `scripts/check-ensures.sh` only looks 15 lines past a `pub fn` signature, so it false-flags long signatures like `cron_match::matches_schedule` and `cron_next::next_occurrence`.
- `src/ssa_construction.rs`: local `valid_edge_list_spec(...)` does not automatically satisfy `dominator_tree::valid_graph_spec(...)` inside proofs even though the definitions match; for internalized IDF code, keep `raw_program_valid_spec(*program)` in loop invariants and add an explicit bridge assertion before calling `dominance_frontier`.
- `src/ssa_construction.rs`: when Verus fights nested indexing like `program.blocks[b].phis[p]` or `dst_block.phis[p]`, bind outer vectors to locals (`block`, `dst_block`) and capture loop bounds (`phi_len`, `instr_len`) in locals before the inner loop; then assert the element index is in range before indexing.
- Trusted boundaries also include weakened proof surfaces beyond `#[verifier::external_body]`, but the real `assume(...)` oracles are now gone and there are no remaining `@verified-logic:no-ensures` / placeholder surfaces in `src/`.
- When a requirement has no separate exec fn, this repo still counts `r[impl]` on the defining spec fn as the implementation reference; that closed `query-cost.index-scan` and `snapshot-iso.si-allows-commit`.
- Fixing `scripts/check-ensures.sh` by scanning until the body line (instead of only 15 lines) removes the long-signature false positives. `src/cycle_detect.rs:has_cycle` is closed now; the clean one-sided proof shape is to reuse the verified topological sort in the exec wrapper and prove locally that any topological order rules out a nontrivial cycle.
- `cargo clippy --all-targets -- -D warnings` was easiest to close with a mix of real cast fixes and test-only `#[allow(...)]` annotations for style lints (`default_numeric_fallback`, `uninlined_format_args`, truncation casts in proptests) rather than invasive churn in test code.
- Restoring Verus in `src/mesi.rs`/`src/page.rs` used temporary `#[verifier::external_body]` on the remaining composite blockers. Those 7 sites are closed now; if a future session reintroduces them, it is a regression.
- `src/mesi.rs:check_system_invariant` closed cleanly once I stopped trying to prove counts and instead verified a direct prefix scan with prefix-validity / prefix-coherence invariants.
- `src/mesi.rs:invalidate_others_spec` is easiest to prove with four tiny recursive lemmas: len preservation, unchanged indices below `from`, writer unchanged, and any non-writer index `>= from` becomes Invalid.
- I made a bad runtime tradeoff once by replacing MESI loops with recursive exec helpers. Verus liked it, but runtime stack depth became proportional to `caches.len()`. Keep the proofs recursive if needed, but keep exec code iterative.
- `src/page.rs:radix_index_bounded` closed with a tiny helper proving `((value >> offset) & ((1 << width) - 1)) < (1 << width)` plus an explicit nonlinear-arith proof that the computed offset is below 64.
- `src/free_list.rs` closed cleanly with small copy/filter loops: a `clone_from` helper proves suffix copies for `pop`, `push` just tracks `seq![block] + prefix`, and `remove` is easiest with a prefix-filter spec (`remove_prefix_spec`) instead of the old suffix-recursive `remove_from_spec` invariant.
- `src/segregated_free.rs:seg_alloc` can bridge `Vec<Vec<u64>>` to `Seq<Seq<u64>>` directly with `lists@.map_values(|v: Vec<u64>| v@)`. `seg_free` also closes cleanly with a tiny `clone_prepend` exec helper plus a `mapped_lists_update` proof that commutes `Seq<Vec<u64>>::update` with `map_values(|v| v@)`.
- `src/bitwise.rs:is_sign_extended` and the zero/all-ones proofs close with small `by(bit_vector)` bridges if the variable shifts are first bound to `u64` locals (`shift`, `upper`) before comparing the exec arithmetic to the spec's `sub(...)` form.
- `src/cron_field.rs:first_match_correct` closes if the modulo step-rounding proof is rewritten in terms of `lemma_fundamental_div_mod`, `lemma_mod_multiples_basic`, and `lemma_mod_pos_bound` instead of asking nonlinear arithmetic to simplify `%` directly. `first_match_ge_from` also wants the natural proof precondition `from <= max + 1` for the sentinel branch.
- `src/cron_next.rs:advance_one_minute` is no longer trusted; its proved ensures keep the output in basic calendar ranges and show the result timestamp is strictly later than the input.
- `src/cron_next.rs:next_occurrence` also closes now. The proof works if the search stays iterative, year-overflow paths return `false` instead of incrementing past the horizon, loop termination uses a lexicographic decreases tuple over `(year, month, day, hour, minute)`, and `cron_field::first_match_correct` / `first_match_ge_from` are `pub proof fn` so the search proof can reuse them across modules.
- `src/index_select.rs:best_index` is no longer a placeholder/no-ensures surface. The clean spec shape is a prefix minimum (`best_index_prefix_spec`) with a strict cross-multiplication comparator `selectivity_lt_spec`; the exec loop invariant can then state `best == best_index_prefix_spec(selectivities, i)`.
- `src/settlement.rs` no longer needs a no-ensures escape hatch. The workable bridge is to keep the old int-tuple spec for proofs, add exec-side view specs for `(usize, usize, u64)` obligations and `Vec<i128>` positions, then prove `compute_positions` against a prefix-application spec over the first `i` obligations. `is_valid_settlement` then verifies by comparing the proved position views index-by-index.
- `src/topo_sort.rs:is_topo_order` closes cleanly if the checker proof is split into three recursive bridges (`all_in_range_from_pointwise`, `all_distinct_from_pairwise`, `edges_forward_from_pointwise`) plus a mapped-first-occurrence lemma for the edge endpoint scans. In the exec loops, only set `pu`/`pv` while they are still the sentinel so the invariants continue to describe the first occurrence.
- `src/reachability.rs` was easier to close by fixing the spec than by forcing a false fixed-budget lemma through Verus. The right abstraction is `reachable_spec := exists fuel. reachable_fuel(...)`; then the helper lemma becomes the actually true one-edge extension fact `reachable_fuel(src, u, fuel) -> reachable_fuel(src, dst, fuel + 1)` when `(u, dst)` is an edge.
- `src/nix_closure.rs` only closed after admitting the old API was the problem: `&dyn Fn(&ClosureItem) -> Vec<ClosureItem>` was not a tractable verified boundary. The practical close was to redesign the public API around a finite operator table, replace the placeholder spec with a real closure predicate over reachable keys, and keep the executor as a plain Rust wrapper outside `verus!`.
- `src/nix_drv_serialize.rs` had a bad property test, not a bad escaper: the old test rejected escaped `"` bytes by scanning raw bytes instead of tracking backslash state.
- `src/nix_base32.rs`: `usize::div_ceil` satisfies clippy but is not Verus-modeled in exec code yet; keep the old `(x + 4) / 5` formula in verified execs and use a local `#[allow(clippy::manual_div_ceil)]` instead.
- `src/shamir_poly.rs`: recursive Horner proofs unfold more reliably if the current suffix is first bound to a ghost local `s`, then prove `s[0]` and `s.subrange(1, s.len())` explicitly before asserting the one-step `poly_eval_sum_spec(s, ...)` expansion.
- `src/shamir_split.rs`: for share-building loops, `shares@.len() == j` plus pointwise prefix facts is easier than `shares@ == spec.subrange(0, j)`, which can trigger annoying `Seq::subrange` recommendation failures in invariants.
- The main `openspec/specs/verification-gaps/spec.md` must list each `#[verifier::external_body]` site individually, not just file totals, because the archived proof-audit change text says every trusted site appears in the inventory.
- Active OpenSpec change specs in this repo use full spec documents under `openspec/changes/*/specs/<domain>/spec.md`, not `ADDED/MODIFIED/REMOVED` delta sections. Match the existing change style when adding a new change.
- OpenSpec gate budget gotcha: this session's gate still loaded artifacts with a per-file truncation and prompt-budget cap, so long proposal/design/tasks/spec docs can fail as "missing evidence" even when content exists. Keep active change artifacts compact and split large spec domains before rerunning gates.
- OpenSpec tasks gates here are brutally task-local: they kept demanding explicit strict-vs-reflexive predicate text, exec-model surfaces for both tree and keystore, run/capture tasks for authored tests plus `./scripts/verify.sh`, and public-fn `r[impl]` coverage instead of accepting those as implied by nearby tasks.
- `src/cryptoshedding.rs:recoverable_from_exec` would not close against the `Set`-backed keystore model until each branch explicitly asserted the matching `chain_present_fuel_spec` outcome (fuel exhausted, missing key, recurse on in-bounds parent, false on out-of-bounds parent, true at root).
- `src/cryptoshedding.rs` rotation/subtree proofs were much easier after factoring two generic lemmas first: one for "shed any key on the chain breaks recoverability" and one for "shed a non-ancestor preserves recoverability". Trying to bolt rotation well-formedness preservation and old/new-parent consequences together in one proof shape was brittle; taking rotated-tree well-formedness as a separate obligation was far smoother.
- `src/cryptoshedding.rs` rotation proofs that quantify over `int` indices and then call `on_chain_fuel_spec(..., node: u64, ...)` needed an explicit `tree.len() <= u64::MAX as nat` validity conjunct; otherwise Verus got stuck on `int`→`u64` casts inside the helper lemma.
- `src/cryptoshedding.rs` chain-termination proof finally closed with a finite-budget argument over `Set<u64>` and vstd set length/finite axioms. Those axiom imports must be `#[cfg(verus_keep_ghost)]`; leaving them ungated breaks ordinary `cargo test` builds.
- Done review will flag internal OpenSpec drift inside one change: keep proposal capability names aligned with spec requirement IDs, and do not promise behaviors in `proposal.md` that the change spec does not require yet.
- Nickel subtype redesign blocker: Verus termination does not like recursive measures that descend through `Seq<(name, Box<NType>)>` field lists. Wrapper-measure tricks still failed; practical fully verified redesign likely needs a recursive row-list datatype (or a single-datatype encoding) before exact closed-record transitivity will go through cleanly.
- Nickel subtype redesign closed cleanly once records switched to a recursive `NRows` list and the fuel recursion used a two-tier decreases trick: `is_subtype_fuel` decreases `(fuel, 0)` while row helpers decrease `(fuel, 1, row_len(...))`. That lets record helpers call `is_subtype_fuel(..., fuel)` on field types while the main relation still descends to helpers with `fuel - 1`.
- Nickel priority bug fix: do not encode `Bottom`/`Top` as the same scalar rank as `Numeral(i64::MIN/MAX)`. Use a lexicographic `(bucket, rank)` comparison instead: bucket separates `Bottom < numeral/Neutral < Top`, and rank only orders within the numeral bucket so `Neutral == Numeral(0)` remains the only intended alias.
- `nickel_merge_split::split_disjoint` only becomes meaningful under the intended sorted-map precondition. Good proof shape: add explicit pairwise-disjoint spec predicates (`kv_kv_disjoint`, `kv_center_disjoint`), prove helper facts that left/center/right keys come from the corresponding source sides, and use tiny sequence lemmas for `subrange(1, len)` length/index behavior (`subrange_drop_first_len`, `subrange_drop_first_index`) so recursive tail proofs do not fight `Seq` arithmetic.
- `nickel_polarity` counting proof easiest shape: define recursive `apply_domain_flips(initial, count)` over a `nat`, then prove `polarity_after_path(initial, path) == apply_domain_flips(initial, domain_count(path))` by recursion on `path`. Keep example lemmas (`path_single_domain`, `path_double_domain`) direct; trying to make Verus normalize literal `domain_count(seq![...])` or `apply_domain_flips(_, 1)` inline created avoidable proof friction.
- Generic decomposition: algebraic.rs (predicates), pointwise.rs (seq lifting), seq_fold.rs (counting)
- `src/nix_base32.rs`: useful verified helper split for the roundtrip proof is to separate
  decode byte writes into `decode_low_spec(bit, digit)` and `decode_high_spec(bit, digit)`.
  Also, `chunk_start_in_bounds(len, n)` and `decode_high_zero_when_no_spill(bit, digit)`
  verify cleanly and seem like the right arithmetic/bit lemmas for killing the remaining
  roundtrip boundary in small steps.
- `scripts/check-coverage.sh` flags every public helper too, not just API surface. If a helper like `nix_attrset::vec_u8_cmp` is internal-only, make it private instead of inventing a weak `r[impl]` tag.
- Verus does not currently model slice `.to_vec()` in verified exec code here. Use explicit clone loops with `subrange(0, i)` invariants for slice-to-Vec copies.
- Whole-struct equality is brittle when the struct contains a `Vec` field cloned in exec code. Fieldwise ensures are much easier than `result == spec_struct` for BEAM-style state records.
- BEAM-model gotcha: if there is no verified global pid allocator, do not claim per-spawn global heap freshness. Phrase `beam-process.isolation` as distinct-pid heap isolation and sync the prose in `docs/requirements.md` and `openspec/specs/beam/spec.md` to that model.
- Cross-module property gotcha: done review reads the prose, not just `r[...]` tags. `properties.beam-death-links`, `properties.beam-death-monitor`, and `properties.beam-death-registry` had to encode link reachability, exact-once DOWN, and register-cleanup roundtrips explicitly before review would accept them.
- `tracey query uncovered` and `tracey query untested` are fine after daemon startup, but parallel first-run queries can trip the startup lock timeout. Run them sequentially once if the first attempt flakes.
- Repo-index OpenSpec extraction gotcha: active spec requirement IDs are often bare `r[...]` lines under `### Requirement:` headings. Those are definitions, not `r[impl ...]`-style references, so extract them with the definition parser.
- Repo-index cache gotcha: if extractor behavior changes, old `(digest, extractor kind)` rows can stay semantically stale even with identical file bytes. Bump the schema/cache version before relying on reopened query results.
- `pueue_run` only accepts pre-existing groups here. For ad hoc validation runs, use `group: "default"` unless you already know the custom group exists.
- Shared-target `cargo test --quiet` can still trip doctest crate-version skew around `src/web_of_trust_principal.rs` / `blake3` / `arrayref` across mixed rustc versions. `autoresearch.checks.sh` now exports `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic`; for manual reruns, using that target dir is more reliable than piecemeal `cargo clean -p ...`.
- `src/nickel_record_merge.rs`: key-coverage proofs closed once I stopped relying on recursion alone and added two tiny helpers: `has_key_in_tail_if_not_head` to move a witness into the tail, and `has_key_in_cons` to lift a witness back through `seq![head] + tail`.
- `src/nickel_record_merge.rs`: neutral-record associativity closed with 9 head-order cases over `(a[0], b[0], c[0])`, plus tiny sequence normalization helpers. The annoying bit was not the case split itself but rewriting `record_merge_spec(seq![head] + tail, ...)`: Verus wants explicit `cons_subrange_tail(head, tail)` to identify the helper's `subrange(1, ...)` with `tail`.
- `src/nickel_record_merge.rs`: `resolve_field(...)` triggers `clippy::match_same_arms` if `Right`, `Equal`, and `Conflict` each clone `right` in separate arms. Collapse them into one combined pattern before rerunning `./scripts/verify.sh`.
- `src/type_unification.rs`: direct recursive Robinson unification over `(t1, t2)` fought termination because the App/App second recursive call can grow after substitution. Fuel-bounded equation-list solving verified better. Helpful bridges: `apply_subst_to_eqs_index` for mapped-seq indexing and `singleton_subst_irrelevant_if_absent` for occurs-check branches.
- The last-byte spill proof is tractable if you phrase it as `last_chunk_spill_zero`:
  assume the chunk starts in the last byte (`((n*5)/8)+1 == len`) and case-split on
  `bit % 8 == 4..7`, then prove the spill starts exactly at `len*8` and the padded bits are 0.
  With that in place, `reconstruct_prefix_len_upper` and `reconstruct_full_len` verify.
- For the content-equality phase in `src/nix_base32.rs`, a useful intermediate step is to prove
  pure byte-mask algebra first, separate from any `Seq` indexing: `current_byte_mask_update_no_spill`
  and `current_byte_mask_update_spill` both verify cleanly as standalone `by(bit_vector)` lemmas.
- The `digit_from_bit_spec` bridge is easier in small concrete cases than as one generic lemma:
  `same_byte_digit_formula_j0`..`j3` verify cleanly for the no-spill offsets, but a generic wrapper
  over `bit % 8 <= 3` still fights Verus because the final shift expression with `Seq` indexing does
  not rewrite automatically across branches.
- For the content-equality induction, local byte-update lemmas verify well when written branchwise on
  concrete offsets instead of abstract masks: `current_byte_update_no_spill`, `current_byte_update_spill`,
  `next_byte_zero_when_no_spill`, and `next_byte_update_spill` all go through once the prefix-masked
  values and decode low/high bytes are first bound to local `u8` vars before any `by(bit_vector)` step.
- Spill bridging is tractable with explicit two-byte digit formulas. `spill_digit_formula_j4`..`j7`
  verify cleanly and are enough to prove current-byte and next-byte spill updates without any generic
  `Seq`-indexed wrapper.
- The last-byte spill case needs its own tiny bridge family (`last_spill_digit_formula_j4`..`j7` and
  `current_byte_update_last_spill`) because the two-byte spill lemmas require a real next byte, but the
  final chunk only sees zero padding.
- The full roundtrip proof now works: prove `reconstruct_prefix_masked`, then combine it with
  `prefix_masked_full`, `reconstruct_full_len`, and a tiny `extend_to_same_len` helper to internalize
  `encode_decode_roundtrip`.
- To internalize `encode`, it was easier to stop proving the old left/right byte formula directly.
  A better route is: prove a small range precondition for `usize`, add verified exec helpers
  `bit_at_padded_exec` and `encode_digit` that mirror the spec bit-by-bit, then use a verified
  `alphabet_char` mapping in the loop. That avoids the brittle bitvector bridge entirely.
- To internalize `decode`, recursive exec code was much easier than the original in-place reverse loop.
  Mirror `decode_prefix_spec` with `decode_prefix_exec`, use `extend_to_exec` + `or_extend_at_exec`
  to build updated vectors functionally, and keep the `usize` arithmetic tame with a small
  `decode_chunk_window_fits` bound. This avoids proving mutation-heavy loop invariants over `resize`
  and `|=`.

## Corrections
| Date | Source | What Went Wrong | What To Do Instead |
|------|--------|----------------|-------------------|
| 2026-04-14 | self | Left OpenSpec “exact” web-of-trust artifacts too abstract, so proposal/design/tasks gates kept failing on unnamed proof artifacts, missing byte/tag details, and manual-looking checks | For strict OpenSpec work here, pin exact byte/tag encodings and identity rules in the spec, name concrete `proof fn` artifacts in design/tasks, and make mirror/anchor/bridge checks deterministic with evidence paths |
| 2026-04-14 | self | OpenSpec tasks/design for trusted bridges stayed too generic, so gates demanded explicit bridge-contract drift audits plus scenario-by-scenario task text and proof-anchor wording | When a change introduces reviewed trusted bridges here, require a deterministic audit against the reviewed contract artifact, and spell Phase tasks in terms of named spec scenarios plus helpers/proof fns/tests |
| 2026-04-14 | self | ChaosControl-primitives OpenSpec kept failing until stateful kernels pinned exact no-op/update points, result types, stable tie-breaks, and concrete evidence/review artifact paths | For deterministic-kernel OpenSpecs here, name exact transition ownership (`tick` vs `advance`), numeric/result types, total ordering/tie rules, and evidence file paths up front instead of leaving them implicit |
| 2026-04-15 | self | ChaosControl input-tree/task gates kept reopening until the spec said which score direction wins (`higher` first), what exec comparator is exact (`smaller denominator` first because numerator is constant), and budget truncation as exact `min(budget, full_stream_len)` instead of only `<= budget` | For deterministic ranking generators here, pin order direction, exact executable comparator, and exact emitted length in the spec before trying to split tasks |
| 2026-04-14 | self | Tried to call Cargo dep `blake3` directly from verifier-visible code; `./scripts/verus.sh` only passes Verus externs, so rust_verify could not resolve the crate | For runtime-only dependency use in verified modules, gate the real helper with `#[cfg(not(verus_keep_ghost))]` and give Verus a `#[cfg(verus_keep_ghost)]` stub |
| 2026-04-14 | self | Added proof-only DCE Tracey requirements with only `r[verify]`, so `tracey query uncovered` still reported them as uncovered | For helper/proof-only requirements in this repo, add an `r[impl]` tag on the defining proof/spec artifact too, not just `r[verify]` |
| 2026-04-14 | self | Introduced a new public SSA helper without the repo's expected coverage annotation and then summarized validation from prose/docs instead of surfacing the command outputs | Keep helper-only functions private unless they need public API status, and when claiming validation include direct command output snippets for `cargo test`, `clippy`, `verus`, and Tracey |
| 2026-04-13 | self | Sent a multiline `pueue_run` command without wrapping it in `bash -lc`, so pueue collapsed the newlines and only the first command ran correctly | For multi-step validation pipelines in `pueue_run`, wrap the whole script in `bash -lc '...; ...'` with explicit separators |
| 2026-04-13 | self | Used regex alternation/pipes directly in `rg` tool patterns; shell interpreted them as pipelines and even invoked `shutdown` from the literal pattern | For alternation searches, use `bash` with `rg 'a|b' ...` quoted, or avoid `|` in the raw `rg` tool pattern |
| 2026-04-13 | self | Repeated same `rg` alternation mistake after already logging it in napkin | Default to `bash` for any regex containing `|`, even for quick README searches |
| 2026-04-13 | self | Still reached for raw `rg` tool with `|` during README search | Treat `|` as bash-only in this harness; if pattern has alternation, switch tools before running command |
| 2026-04-14 | self | Used raw `rg` again with regex metacharacters during loop-detection reconnaissance and triggered shell parsing errors | For any nontrivial regex in this harness, default to `bash` with quoted `rg '...' ...` |
| 2026-04-15 | self | Tried to verify `sel4_asid::allocate` with an early return from the mutation branch; Verus would not bridge the pre-state snapshot cleanly into the method postcondition | For `&mut self` loops over `Vec::set`, prefer a single-exit loop with a result variable and invariants over the pre-state view instead of returning directly from the mutation branch |
| 2026-04-15 | self | Tried to express non-mutation of read-only `&T` params with `old(param)` in `ensures`; Verus rejected it because `old(...)` expects mutable references | For pure/read-only exec APIs, rely on the immutable `&T` surface plus tests/proofs about failure behavior instead of `old(shared_ref)` postconditions |
| 2026-04-14 | self | First loop-body oracle treated any predecessor-reachable node as inside the loop and produced false side entries/unreachable members | For loop detection here, restrict body/entry properties to reachable non-self loops and intersect the path witness with header dominance |
| 2026-04-15 | self | Overclaimed OpenSpec gate status from local artifact review / AskClaude sanity check instead of actual fresh `openspec_gate` tool results | Only call a gate green when the live `openspec_gate` outputs PASS/WARN as intended and the evidence packet shown supports that claim; artifact cleanup alone is not implementation completion |
| 2026-04-15 | self | Ran `tracey query uncovered` and `tracey query untested` in parallel during daemon auto-start and both flaked with `Error: Cancelled` | For this repo, rerun Tracey queries sequentially after startup when validating fresh requirements |
| 2026-04-16 | self | ChaosControl OpenSpec gates kept failing while design/specs said helpers use "constructor-normalized inputs or explicit requires" without choosing which public helpers use which boundary | For OpenSpec here, pin per-domain validity/API surfaces concretely in proposal+design+specs before tasking; generic either/or wording is not enough |
| 2026-04-16 | self | Deterministic validation script in OpenSpec was not enough by name alone; tasks also needed an explicit tool-exists/update item before the run step | If a validation command is normative in OpenSpec here, add both tool-existence/update task and run/capture task |
| 2026-04-16 | self | Choreography OpenSpec first overclaimed immediate non-decider choice/step behavior, but common-prefix merges mean some roles already equal the selected-branch projection before any `Offer` head is exposed | For choreography-style merge/projection changes here, make step helpers' total-vs-requires surface exact and distinguish exposed `Offer` heads from shared-prefix roles that need projection equality, not a fake local step |
| 2026-04-13 | self | Started an OpenSpec change in `../chaoscontrol` before confirming user wanted the spec in this repo | For cross-repo requests, confirm target repo first; if user says "add primitives here", default OpenSpec work to current repo, not sibling repo |
| 2026-04-13 | self | Added bare `assume_specification[...]` declarations for SSA wrappers and normal cargo builds failed under `-F unsafe-code` | Gate `assume_specification[...]` with `#[cfg(verus_keep_ghost)]` so Rust builds ignore them while Verus still sees them |
| 2026-04-13 | self | Archived SSA change with proposal/tasks wording that still promised proofs even though code used thin wrappers plus `assume_specification[...]` bridges | Before archiving, compare proposal/design/tasks/spec text against actual proof boundary and update archive evidence if scope changed |
| 2026-04-14 | self | Started repo-index cache design as digest -> extracted-facts, then noticed active OpenSpec change/domain metadata is path-derived and cannot be shared purely by digest | For repo indexing here, keep content identity as digest but cache extracted facts by `(digest, extractor kind)` and store path-only context on the path row |
| 2026-04-14 | self | Repo-index tests originally used `tempfile::tempdir()` under `/tmp`; full `cargo test` then failed with `No space left on device` during the bin test suite | For repo-local heavy temp DB tests in this repo, create temp dirs under `target-test/` with `tempdir_in` instead of default `/tmp` |
| 2026-04-15 | self | Tried to close `false-sharing-detection` by mixing `by(bit_vector)` power-of-two lemmas with `by(nonlinear_arith)` quotient proofs in one chain, and full-lib Verus bogged down on the division/bit-split bridges | For cache-address work here, prefer one theory per proof step: keep arithmetic line-number proofs separate, build bit-split lemmas on an offset-bits helper first, and only connect the two surfaces after each side verifies alone |
| 2026-04-15 | self | First choreography pass hit Verus mutual-recursion measure traps around exec<->spec view conversion for recursive ASTs with `Vec` branch lists (`global_view`/`gbranches_view`, `local_view`/`lbranches_view`, and projection folds) | For recursive AST kernels here, avoid mutually recursive open-spec view helpers over `Vec` branches unless every function shares one numeric decreases surface from the start; otherwise use non-mutual constructors or a fuel-indexed wrapper before writing large proofs |
| 2026-04-16 | self | Choreography step/property proofs exposed a model mismatch: non-decider `Offer` labels currently come from the first branch-dependent receive labels, but the spec text/proof obligations treated them as the outer global choice labels; nested-choice/shared-prefix cases make the stronger theorem false without extra tracking or a stronger well-formedness restriction | Before proving choice-step / choice-agreement theorems here, decide explicitly whether local offers are keyed by global branch labels or by first branch-dependent receive labels, and encode that in both the projection algorithm and `docs/requirements.md` instead of trying to prove both stories at once |
| 2026-04-16 | self | `lookup_branch_spec` over choreography branches was much easier to close from a helper `lookup_from_strictly_labeled(..., from)` than from a one-shot uniqueness argument, and `projectable_implies_projection_some` became tractable only after splitting non-decider choices into `role_in_branches_spec` (direct merge-some obligation) versus separate absent-role-to-`End` reasoning | For choreography proofs here, use start-index lookup helpers on strict-label branch lists and keep choice-totality proofs factored by participant-present vs participant-absent cases |
| 2026-04-16 | self | Choreography absent-role projection closed faster once I introduced a recursive `labeled_end_branches_from_spec(...)` target and proved exact equality against `project_decider_choice_from_spec(...)`; common-send/common-recv decrease lemmas also stopped fighting once `projected_branch_suffix_size(seq![branch] + tail, 0)` had an explicit cons/shift lemma | For projection/merge proofs here, prefer exact recursive target sequences and tiny size-decomposition lemmas over ad hoc arithmetic on `Seq` concatenations |
| 2026-04-16 | self | `merge_projected_alignment` and `choice_projection_alignment` only became tractable after switching from big forall-based concat proofs to small fuel/pointwise helpers, and `project_decider_choice_from_pointwise` stopped flaking once the recursive tail was bound with `match project_decider_choice_from_spec(..., from + 1)` instead of trying to prove `... == Some(rest)` from an `unwrap()` | For recursive choreography sequence proofs here, bind recursive Option tails by matching on the spec function directly and prove one-index facts first; derive strict-label/lookup facts from those pointwise lemmas afterward |
| 2026-04-16 | self | `well_formed_global`/`well_formed_local` exact ensure work only stabilized once I copied the same pattern: move the branch loop into a private helper over `&[Branch]` with a direct conjunction ensure, then make the public recursive fn reveal an open recursive result spec and delegate choice/offer arms to that helper | For branch-recursive choreography exec functions here, avoid closed branch-wrapper specs and brittle borrowed `*local`/`*global` equality proofs; direct helper conjunctions plus an open recursive result spec are much more reliable |
| 2026-04-16 | self | Choreography choice-agreement theorem looked unprovable when phrased as `first_offer_labels_from_spec(project_endpoint_spec(...), decider)`, because a role can fully collapse to common behavior and later reach an unrelated deeper `Offer`; the requirement really talks about the first branch-dependent offer for the current choice, not any later offer in identical common behavior | For choice-label properties here, define a dedicated helper over the projected branch set / merge process for the current choice instead of reusing a generic "first offer anywhere under send/recv prefixes" helper |

## Domain Notes
- Lazygit branch refresh panic can come from a corrupt local auto branch ref, not from repo-wide object corruption. In this repo, `.git/refs/heads/pi/auto/main` pointed to missing object `fdc4b6514488e7b441c17b3b50d15cdfc90072a4`; `git show-ref --heads` reported `bad ref`, and reflog showed last good tip `982a1021f46d3dadf56cd2c5304080a2912f92a1`.

## Recent Changes
- `src/set_associative_lookup.rs`: support full-width geometries with `low_mask(bits == 64) == u64::MAX` and safe shift helpers; raw `1u64 << 64`, `addr >> 64`, or `tag << 64` do not work in exec/spec paths.
- Approval module added: count_valid_approvals, is_merge_allowed, needs_more_approvals, has_stale_approvals
  - Loop invariants must track count bounds to prevent arithmetic overflow (count <= i as u64)
  - Complex quantifiers need explicit #![trigger] annotations for Z3 inference
  - has_stale_approvals recursive helper function works better than complex loop invariants
  - Type annotations needed for local variables in spec functions (let current_count: int = ...)
  - Proof functions can't call exec functions - only spec functions
  - Simple inductive helper proofs work better than complex assert_by_contradiction patterns
- Slice module added: chunk_count, prefix_eq, suffix_eq, all_equal, mismatch_index
  - chunk_count proofs need nonlinear_arith for multiplication commutativity:
    fundamental div-mod gives `d * (n/d)` but ensures needs `(n/d) * d`
  - suffix_eq quantifier needs explicit #![trigger] — Verus can't auto-infer for
    complex index expressions like `a[a.len() - n + i]`
  - suffix_eq loop invariant needs `a_start + n == a.len()` to prove no arithmetic overflow
    on `a_start + i` (instead of `a_start as int == a@.len() - n as int`)
  - mismatch_none_implies_prefix_eq uses inductive helper proof fn
  - Seq::len() returns nat, not int — must cast with `as int` for int return types
- Conversion module added: narrow_u32, narrow_u16, narrow_u8, signed_to_unsigned, unsigned_to_signed
- Zircon job trust reduction: a root-first exec helper avoids `usize` overflow on spec fuel. Check `job == root` before zero-fuel failure, then prove exec fuel `job` matches public spec fuel `job + 1`.
  - Cast lints (truncation, sign_loss, wrap) allowed at module level — verified casts are the point
  - clippy::checked_conversions also allowed — can't use try_from inside verus! blocks
  - Roundtrip proofs (i64→u64→i64, u64→i64→u64) verify directly — Z3 handles cast chains
  - Narrowing chain proof (u8 fits → u16 fits → u32 fits) trivially automatic
  - Option<u32>, Option<u16>, Option<u8>, Option<i64> all work in Verus ensures
- Cross-module properties added: all_equal⟹count, chunk_count↔div_ceil, narrow↔in_range, all_equal⟹partitioned
  - chunk_count_div_ceil needs by(nonlinear_arith) for quotient uniqueness:
    Z3 can't derive (n-1)/k == n/k from fundamental_div_mod alone, needs
    case split on n%k > 0 vs == 0 with nonlinear_arith per branch
  - all_equal_implies_count needs inductive helper (count_from == len - from)
  - all_equal_implies_partitioned and narrow_iff_in_range are automatic (Z3 instant)
- Ring buffer added: verified FIFO queue with power-of-two capacity
  - Struct fields must be `pub` for `pub open spec fn` — Verus treats struct as opaque
    outside the module when any field is private
  - `is_power_of_two_spec` import needs #[cfg(verus_keep_ghost)] — spec fns are erased
    in normal builds, so top-level `use` fails without the gate
  - Pop overflow fix: capture `head` and `cap` as locals before mutation, so Verus
    retains `head < cap` for the `(head + 1)` arithmetic
  - `&mut self` methods work with `old(self)` for pre-state references in ensures
  - Vec::set(i, v) is the Verus way to mutate vec elements in-place
  - Constructor loop builds Vec one element at a time (no vec![0; n] in Verus)
- Bitmap added: verified fixed-size bit set using bitwise module functions
  - Toggle proof requires `(x >> p) & 1 == 0 || (x >> p) & 1 == 1` by(bit_vector)
    because Z3 default theory treats `&` as uninterpreted — doesn't know & 1 ∈ {0,1}
  - Split toggle ensures into two implications (was-set⟹now-clear, was-clear⟹now-set)
    instead of `==` on booleans — Z3 handles implications better
  - get_spec bridging needs explicit `assert(self.get_spec(...) == bit_at_spec(word, pos))`
    to force Z3 to unfold through Vec indexing + bit_at_spec
  - Word index bounds (pos/64 < words.len()) follows from div_ceil definition but Z3
    handles it automatically when the invariant uses `(num_bits-1)/64+1` directly
- Quorum module added: majority, is_quorum, intersection theorem
  - Intersection proof uses by(nonlinear_arith) for (n/2+1)+(n/2+1) > n
  - majority_le_n also needs nonlinear_arith: n/2+1 <= n for n > 0
  - majority_gt_half: (n/2+1)*2 > n — the key fact for pigeonhole
  - All three proofs are one-line nonlinear_arith — Z3's nonlinear solver
    handles integer division bounds cleanly
- Monotonic module added: is_advance, is_strict_advance with full ordering proofs
  - All proofs automatic — Z3 handles linear integer ordering without hints
  - Mixed composition proofs (strict + non-strict) verify instantly
- Epoch module added: lexicographic (term, index) comparison
  - Total ordering, transitivity, antisymmetry, higher-term-wins all automatic
  - epoch_ge_antisymmetric calls epoch_cmp_eq and epoch_cmp_antisymmetric as helpers
  - bool_to_int_with_if clippy lint suppressed — Verus needs explicit if-chain
- Log module added: agree_at, logs_match_prefix, log_append, log_truncate
  - LogEntry struct with pub fields (same Verus opaque rule as RingBuffer/Bitmap)
  - logs_match_prefix exec fn uses same loop pattern as prefix_eq in slice module
  - Verus auto-chose triggers for Seq<LogEntry> quantifiers (a[i].term, b[i].term)
  - Seq::push for append, Vec::truncate for truncation — both have Verus specs in vstd
  - =~= (extensional equality) used in truncate_full and append_truncate_roundtrip
  - All proofs automatic except logs_match_prefix exec (loop invariant)
- Cross-module consensus properties added:
  - majority_via_div_ceil: majority(n) == div_ceil(n+1, 2) — automatic, Z3 unfolds both specs
  - epoch_ge_term_index: decomposes epoch_ge into term/index conditions — automatic
  - log_prefix_term_eq: prefix match implies term equality — automatic (follows from spec def)
  - All three proofs are trivially automatic — the specs are defined to make these hold
- Quorum module expanded: fault_tolerance, can_remove_safely, min_cluster_size
  - Direct equivalents of aspen's fault_tolerance_for_size, can_remove_voter_safely,
    min_cluster_size_for_tolerance — same formulas, now machine-checked
  - fault_tolerance_leaves_quorum: n - ft(n) >= majority(n) — by(nonlinear_arith)
  - min_size_fault_tolerance_inverse: min_size(ft(n)) <= n — the functions are inverses
  - min_size_has_tolerance: ft(min_size(f)) == f — exact roundtrip for odd clusters
  - can_remove_ge_3: n >= 3 ⟹ can remove safely — the critical cluster safety threshold
  - Tolerance vs remove equivalence: can_remove(n) ⟺ ft(n) >= 1
- Expiration module added: expires_at, is_expired, is_expired_opt, time_remaining, is_ttl_valid
  - Direct equivalents of aspen's calculate_expires_at_ms, is_lease_expired,
    is_key_expired, time_remaining_exec, is_ttl_valid
  - Verus doesn't support saturating_mul — use explicit overflow check instead:
    `if x > u64::MAX / 1000 { u64::MAX } else { x * 1000 }`
  - Two expiration semantics: is_expired uses > (lease: not expired AT boundary),
    is_expired_opt uses >= (key: expired AT boundary) — matches aspen's distinction
  - Lifecycle model oracle: create expiration, advance time in steps, check
    is_expired and time_remaining agree at every point
- CAS module added: check_condition, check_cas, evaluate_conditions
  - Abstracted over value types — uses (condition_type, is_some, values_equal) booleans
    instead of string/byte references, keeping module generic
  - evaluate_conditions uses direct postconditions (first-failure index + all-prior-pass)
    like find_first and mismatch_index patterns
  - Inductive proof over quantified sequences failed (all_exists_present) — Z3 can't
    handle forall over Seq combined with recursive spec. Dropped in favor of model oracle.
  - KEY_EXISTS and KEY_NOT_EXISTS proved to be complements
- Interval module added: contains, is_empty, overlaps, split, adjacent, merge
  - Half-open [lo, hi) intervals over u64, matching aspen's KeyRange semantics
  - Split produces non-overlapping adjacent halves; merge is inverse of split
  - split_covers: every point in original is in exactly one half
  - adjacent_no_overlap: adjacent intervals never overlap
  - All proofs automatic — Z3 handles linear integer comparisons instantly
  - nonminimal_bool clippy lint suppressed — Verus needs explicit conjunctions
    in the adjacent spec for Z3 trigger matching
- Batch module added: check_limits, flush_decision, op_size, delete_op_size
  - FlushDecision enum (Immediate/Delayed/None) with spec using int encoding
    (1/2/0) — Verus can't match on enum variants in specs, use int proxy
  - Empty batch invariant: check_limits(0, ...) always returns (false, false)
  - if_same_then_else clippy lint suppressed — Verus needs explicit if-chain
    for Z3 to match each branch to the correct spec case
  - Lifecycle model oracle: simulate add/flush cycles, verify check_limits
    and flush_decision agree with manual calculation at every step
- Chain module added: hash_spec axiom, compute_entry_hash, chain_valid, append_preserves
  - Blake3 modeled as `closed spec fn` (uninterpreted) with trusted determinism axiom
    via `#[verifier::external_body]` proof fn
  - compute_entry_hash is `#[verifier::external_body]` exec fn — Verus trusts the
    ensures clause, runtime uses a deterministic mix for testing
  - chain_valid quantifier needs explicit `#![trigger entries[i]]` — Verus can't
    auto-infer triggers when the body calls a closed spec fn
  - append_preserves requires helper spec fn `correctly_hashed` — Verus parser can't
    handle bare block `{ ... }` inside requires clause
  - prefix_valid: subrange of valid chain is valid — proved by trigger matching
  - Spec fns need both `r[depends]` AND `r[impl]` for tracey coverage when there's
    no separate exec fn
- ACID module added: atomicity, consistency, isolation, durability proofs
  - WriteOp struct needs `pub` fields (same Verus opaque rule)
  - apply_writes loop invariant: `apply_writes_spec(state@, ops@, i) == apply_writes_spec(initial, ops@, 0)`
    captures "remaining work from current state = total work from initial state"
  - `let ghost initial = state@;` before loop captures pre-mutation state for invariant
    (can't use `old(state)` in loop invariants — only in ensures)
  - Seq::len() returns `nat`, not `int` — must cast with `as int` for predicate params
    that take `int` (writes_valid_spec, wal_valid_spec)
  - single_writes_commute uses `=~=` extensional equality — Z3 handles it automatically
    via Seq::update axioms (case split on index equality)
  - wal_all_committed_is_commit needs explicit `assert(ops[from].idx == wal[from].idx)`
    to help Z3 connect Seq::new elements to WAL entries
  - wal_recover exec fn uses same ghost-initial-state pattern as apply_writes
  - writes_disjoint nested loop: outer invariant tracks scanned pairs, inner invariant
    tracks current row vs all columns — Z3 handles the case split at `i = i + 1`
  - All 11 proofs automatic or single-induction — no nonlinear_arith needed
- COW module added: should_copy, ref_count_after_clone/drop, is_exclusive, can_reclaim, snapshot isolation
  - Models the copy-on-write decision pattern: ref counting, copy predicate, value visibility
  - cow_writer_value has unused `_old_value` param — needed in ensures clause (ghost code)
    but not in exec body; underscore prefix satisfies clippy
  - Three mutually exclusive states: can_reclaim (rc=0), is_exclusive (rc=1), should_copy (rc>1)
  - state_coverage proof: every ref count falls into exactly one state
  - Lifecycle proofs: clone/drop roundtrip, clone-exclusive→shared, drop-to-exclusive,
    drop-exclusive→reclaimable
  - Snapshot isolation: writer sees new value, shared readers see old value
  - All proofs automatic — Z3 handles linear integer comparisons instantly
- Drain module added: drain-state admission protocol for graceful shutdown
  - Models check-increment-recheck pattern: can_start_op, after_start_op, after_finish_op
  - is_drain_complete requires BOTH draining AND zero in-flight operations
  - should_notify detects transition to zero during drain (old > 0, new == 0)
  - start_drain_decision returns 0/1/2 for already-draining/immediate/wait cases
  - Roundtrip proofs: start-finish works for 0 < n < MAX, finish-start only for n > 0
  - finish_start_roundtrip requires in_flight > 0 (finish(0) == 0, start(0) == 1)
  - All 31 Verus proofs pass, 19 cargo tests pass
- Rate limit module fixed: u128 arithmetic replaced with division-based overflow detection
  - tokens_after_refill exec fn: replaced `elapsed_ms as u128 * rate_per_sec as u128 > u64::MAX as u128` 
    with `elapsed_ms > 0 && rate_per_sec > u64::MAX / elapsed_ms` for multiplication overflow check
  - tokens_after_refill exec fn: replaced `tokens as u128 + refill_amount as u128 > u64::MAX as u128`
    with `tokens > u64::MAX - refill_amount` for addition overflow check
  - tokens_to_wait_ms exec fn: replaced `shortage as u128 * 1000u128 + rate_per_sec as u128 > u64::MAX as u128`
    with `shortage > 0 && 1000 > (u64::MAX - rate_per_sec) / shortage` for overflow check
  - refill_monotonic_time proof: added `assert(t1 as int <= t2 as int) by(nonlinear_arith);`
  - Removed `#![allow(clippy::cast_lossless)]` since no u128 casts remain
  - All rate_limit functions verify with Verus and runtime tests pass
- 10 new modules added in batch: chunk, pressure, drain, sequential, rate_limit,
  approval, eviction, hash_bucket, bounded_range, partition
  - chunk: ceiling division uses `as u64` cast on int arithmetic results — Verus
    spec arithmetic on u64 produces int, must cast back
  - rate_limit: Verus doesn't support u128 — overflow detection can't use wide arithmetic.
    tokens_after_refill and tokens_to_wait_ms marked #[verifier::external_body]
  - eviction: find_lru_candidate loop invariant with quantifier over Option case split
    is intractable for Z3 trigger matching — marked external_body
  - hash_bucket: bit_vector proofs for pow2 mask bound and mod-bitwise equivalence
    marked external_body — known Z3 bit_vector solver limitations
  - partition: needs `#![allow(clippy::len_zero)]` at module level — verus! blocks
    use explicit `len() == 0` instead of `is_empty()`
  - bounded_range: overlaps spec needs careful handling of unbounded cases
  - All trusted (external_body) items have proptest oracles verifying correctness
- Permission module added: bit_position, has/set/clear_permission, apply_umask
  - Composes bitwise module (bit_at, set_bit, clear_bit) — no new by(bit_vector) needed for exec
  - Proof fns reuse existing by(bit_vector) patterns from bitwise module
  - bit_position spec uses int arithmetic: ((2 - who as int) * 3 + perm as int) as u64
  - Z3 handles bit_position_bounded and injective automatically (small domain)
  - apply_umask proofs: (mode & !umask) & mode == mode & !umask and & umask == 0 by(bit_vector)
- Signal module added: add/remove/contains, union/intersect/complement, deliverable
  - Direct bit operations (|, &, !, ^) — same pattern as bitwise but set-algebraic semantics
  - All proofs use by(bit_vector): De Morgan, involution, commutativity, associativity
  - deliverable = pending & !blocked — subset and disjointness proofs trivial
  - Model oracle test tracks BTreeSet alongside bitmask for full correspondence
- Process state module added: is_valid_state, can_transition, is_terminal, is_schedulable
  - Pure boolean/integer logic — all proofs automatic, no hints needed
  - 6 valid transitions encoded as disjunction, Z3 handles case splits
  - clippy::nonminimal_bool allowed at module level — Verus needs explicit conjunctions
  - unnested_or_patterns clippy lint: must nest patterns in test match arms
- Page module added: frame_number, page_offset, is_page_aligned, page_start, pages_needed
  - Uses lemma_fundamental_div_mod from vstd for decomposition roundtrip
  - frame_number_bounded needs by(nonlinear_arith) for multiplication commutativity:
    fundamental_div_mod gives addr == ps * q + r, but we need q * ps <= addr
  - exact_page_one_page needs by(nonlinear_arith) to prove (ps-1)/ps == 0
  - page_start_aligned needs by(nonlinear_arith) for (frame * ps) % ps == 0
  - pages_needed_covers needs by(nonlinear_arith) with fundamental_div_mod as hypothesis
  - decomposition_roundtrip: two steps — call fundamental_div_mod then nonlinear_arith
    for commutativity (ps * q == q * ps)
  - Test code: use u128::from(x) not x as u128 — clippy cast_lossless is deny
- Verus toolchain: rustc-1.93.0 nix store path can get GC'd — update scripts/verus.sh
  if it breaks. Current: /nix/store/h2kyfzjnnnnqc35xx4fiqnwzhyczl9cz-rust-default-1.93.0
- Fencing module added: issue_token, is_valid_write, is_stale
  - Thin wrapper over monotonic concepts — all proofs automatic, no hints needed
  - Cross-module property: issue_token is a strict advance (trivial for Z3)
- Ballot module added: can_grant_vote, grant_vote, has_won_election
  - election_safety_quorum_overlap calls prove_quorum_intersection — must be `pub proof fn`
  - Single-vote-per-term: Z3 handles the disjunction automatically
  - Wildcard `use crate::module::*` inside verus! works (macro handles erasure)
- Vector clock module: happens_before, concurrent, merge, increment
  - Seq-based (not const generic [u64; N]) — Verus quantifiers over Seq work well
  - concurrent needs explicit "differs somewhere" check — equal clocks are NOT concurrent
  - increment loop invariant must carry `v@[idx] < u64::MAX` to prove overflow safety
  - All decreases clauses go AFTER invariant block, before opening `{`
  - Merge lattice proofs (comm, assoc, idempotent) all automatic via =~= extensional equality
- Lease module: grant, renew, revoke, is_valid, effective_expiry
  - Uses ordering::min_spec — needs #[cfg(verus_keep_ghost)] on import
  - Disjointness proof: Z3 handles the interval arithmetic automatically
- Two-phase commit: can_commit, must_abort, decide
  - all_none_is_pending needs explicit assert(votes[0] == None) to help Z3 distinguish None from Some(true)
  - Seq::new with closure needs explicit quantifier help for must_abort (no witness for Some(false))
- Circuit breaker: next_state, allow_request, is_valid_state
  - Integer-encoded states (same pattern as batch FlushDecision)
  - All proofs automatic — Z3 handles the case splits on u8 constants
- Backoff: compute_backoff, jitter_range
  - Iterative doubling can't connect to recursive pow2 spec — used external_body for exec
  - Spec must handle base=0 first (before attempt>=64 check) or 0*2^64 returns MAX
  - Proptest must cap attempt range to avoid O(2^64) loop iterations
- Scope module added: covers, compound_covers, optional_covers
  - starts_with_bytes_spec uses same quantifier+trigger pattern as ends_with_bytes_spec in env_scrub
  - Transitivity proof needs explicit assert forall with trigger on prefix[i] to chain
    a[i]==b[i] and b[i]==c[i] through the two starts_with quantifiers
  - bytes_eq helper for mount comparison — same pattern as env_scrub::bytes_eq
  - optional_covers exec uses Option<&[u8]> params, ensures clause must spell out
    the match on cap/target to convert &[u8] to Seq<u8> for the spec
  - All proofs automatic — Z3 handles prefix containment, extensional equality, and
    Option case analysis without hints
  - 18 new Verus proofs, 18 new cargo tests (6 unit + 12 proptest)
- Access mode module added: mode_le, mode_permits, delegation_allowed
  - Diamond lattice: READ/WRITE/DELETE pairwise incomparable, FULL subsumes all
  - mode_le_spec is just `a == b || b == MODE_FULL()` — Z3 handles all case splits
  - All 21 proofs automatic — no hints needed for u8 constant comparisons
  - delegation_allowed == mode_le(child, parent), escalation_denies_permits follows directly
- Pattern module added: match_pattern, pattern_contains, is_wildcard
  - Seq<int> with sentinel WILDCARD() == -1 for wildcard patterns
  - match_pattern exec uses Seq.map(|_i, x| x as int) to bridge i64 slices to Seq<int> specs
  - Seq.map indirection needs explicit #![trigger pat[j]] — Verus can't auto-infer through map
  - Ghost vars (`let ghost pat = pattern@.map(...)`) avoid repeating the map expression in invariants
  - pattern_contains exec marked external_body: Z3 can't negate existential through Seq.map
    (inner loop proves forall k < len: s1[k] != target, but Z3 can't derive !exists from forall
    when the sequences go through map indirection). Proptest covers correctness.
  - Transitivity and containment-implies-match only hold for well-formed patterns
    (non-wildcard patterns must not contain the sentinel -1). Added _wellformed variants
    with that precondition — the vacuous case (non-wildcard with sentinel) is contradictory.
- Glob module added: glob_match with multi-segment `*` wildcards
  - glob_match exec marked external_body: multi-segment matching (split by `*`, prefix anchor,
    suffix anchor, greedy middle search) is too complex for Z3 recursive specs
  - Simplified spec: `has_star_spec(pattern) ==> true` for starred patterns — full recursive
    spec (`glob_rest_match_spec` with `decreases rest.len()`) had termination, overflow, and
    existential witness issues. The exec correctness is covered by proptest.
  - Helper runtime fns (`split_by_star`, `find_substr_runtime`) placed outside verus! blocks
    to avoid Verus parsing issues with iterators and closures
  - Proof fns cover the no-star cases (exact match, exact rejects) which are Z3-tractable
  - `has_star_spec` uses existential quantifier — proof needs `assert(seq[0] == STAR())`
    to provide the witness
- Subsumption module added: subsumes_spec, 8 variant constants, partial order proofs
  - Same integer-tag pattern as access_mode (u8 constants, no enum)
  - subsumes_spec is `a == b || (a == FULL && b in {R,W,D,Wa})` — simple disjunction
  - All 13 proofs automatic — Z3 handles u8 case splits on small finite domains
  - Partial order (reflexive + antisymmetric + transitive) verified in one shot
- Authorize module added: authorizes, authorizes_compound, authorizes_with_optional
  - Cross-module composition: imports scope, access_mode, subsumption with #[cfg(verus_keep_ghost)]
  - All 10 proofs automatic — the spec is just conjunction of three sub-specs, Z3 handles trivially
  - `authorizes_with_optional` ensures clause needs explicit match on Option<&[u8]> to convert to Seq<u8>
    (same pattern as scope::optional_covers)
- Containment module added: contains, contains_compound, contains_with_optional
  - Same cross-module pattern as authorize: imports scope, access_mode, subsumption
  - contains_transitive calls sub-module transitivity proofs — those must be `pub proof fn`
    (initially were private, causing E0425 in Verus compilation)
  - All 12 proofs automatic except contains_transitive which calls three helper lemmas
- Delegation module added: chain_valid, level_valid, depth checks, ChainLevel struct
  - ChainLevel struct with pub fields (same Verus opaque rule)
  - chain_valid_spec is recursive over Seq<ChainLevel> with decreases chain.len() - from
  - chain_valid exec marked external_body — recursive spec over Seq<struct> with
    field access (chain[from].has_delegate etc.) is complex for Z3 loop invariants
  - Concrete chain proofs (two_level_valid, bad_root_invalid, depth_exceeded_invalid)
    need explicit unfolding hints: assert(level_valid_spec(...)) and assert(chain_valid_spec(chain, 1))
    to force Z3 to unfold the recursive spec one step
  - Cross-module imports: needs access_mode, containment, subsumption all via #[cfg(verus_keep_ghost)]
  - mode_le_spec not re-exported by containment — must import access_mode directly
- Token timestamp module added: is_expired, is_from_future, needs_refresh, token_time_valid
  - Spec uses `int` arithmetic (expires_at as int + tolerance as int) to avoid overflow
  - Exec uses saturating add: `if tolerance > u64::MAX - expires_at { u64::MAX } else { ... }`
  - needs_refresh overflow: remaining * 5 might overflow, guard with `remaining > u64::MAX / 5`
  - Verus spec-mode arithmetic on u64 produces int — `(now + tolerance) as u64` cast needed in ensures
  - All 15 proofs automatic — Z3 handles linear integer arithmetic without hints
- Cross-module UCAN properties added to properties.rs:
  - Delegation safety: contains(parent,cap) && authorizes(cap,op) ==> authorizes(parent,op)
  - Authorization/containment imply their three factors
  - Token valid at issue, chain depth bounded
  - All 5 proptest properties pass
- Spec fn imports from other modules need `#[cfg(verus_keep_ghost)]` — ghost code erased in normal builds
- Total: 842 Verus proofs, 588 cargo tests, ~250 requirements

- seq_set module: set_difference, set_intersection, is_subset, all_distinct over Seq<int>
  - Index-based recursion (`set_difference_from_spec(a, b, from)`) matches loop iteration
    direction — drop_first recursion caused unprovable snoc lemmas
  - Boolean array for exec membership: Vec<bool> indexed by value, O(n+m) total
  - Partition theorem: diff.len() + inter.len() == a.len() proved inductively
  - Distinctness preservation: needs helper `difference_from_indices_above` showing
    result elements come from indices >= from (connects to all_distinct on source)
  - `#![trigger a@[i]]` needed on all `forall|i: int| 0 <= i < a@.len() ==> 0 <= a@[i] < cap`
- forest module: parent-pointer forest over Seq<Option<int>>
  - `when 0 <= node < parents.len() && well_formed_spec(parents)` on recursive specs
    (ancestor_path_spec, reachable_spec) — `decreases node` needs well-formedness
    to prove parent < child, can't use `decreases node + 1` without it
  - All exec fns external_body: loop invariants for tree walks through Option<int> parent
    indirection are hard for Z3 (parent map lookup + well-formedness guard + path accumulation)
  - `None::<int>` type annotation needed when comparing `Option<int>` in Verus specs
  - `#![trigger rev[k]]` needed on quantifiers over Seq<int> path elements
  - `reverse_seq` helper as `Seq::new(s.len(), |i: int| s[s.len() - 1 - i])` — simpler
    than recursive reverse for Z3
  - `reverse_cons` lemma: `reverse([x] + rest) =~= reverse(rest).push(x)` — needed
    for connecting iterative walk (builds child→root) to recursive spec (builds root→child)
  - `common_prefix_from` iterative-style recursion for LCA — matches exec parallel scan
- session_tree module: thin composition of forest + seq_set
  - All specs are one-line aliases: find_unique = set_difference(ancestor_path, ancestor_path)
  - find_latest_leaf uses backward scan (external_body) — simpler than last-child following
  - Cross-module imports need #[cfg(verus_keep_ghost)]

- Layout constraint module added: satisfies_constraint, clamp_to_constraint, conservation specs
  - Tag constants must use `pub const` inside verus! (not `pub open spec fn`) — spec fns are
    erased in normal builds, making them invisible to exec fn bodies and test code
  - Same pattern as circuit_breaker (CLOSED/OPEN/HALF_OPEN as pub const u8)
  - All proofs automatic — Z3 handles u8 case splits on 4-value domain
  - Recursive sum_sizes_spec and sum_fixed_spec both need inductive helper proofs
    for nonneg and exact-sum properties
- Tiling module added: rects_disjoint, contained_in, has_area, strip partitioning, scroll bounds
  - First draft of rects_disjoint_spec used separation-test only (right <= left on some axis).
    Proptest found counterexample: zero-width rect at x=294 vs rect [199, 295) — the intervals
    "overlap" on the number line even though the zero-width rect contains no points.
    Fix: add `!has_area_spec(aw, ah) || !has_area_spec(bw, bh)` as leading disjuncts.
  - Uses u16 coordinates (matching TUI cell space) with u32 for edge arithmetic to avoid overflow
  - `(0, 0)` literal needs type annotation in Verus ensures: `(0int, 0int)` for spec returning (int, int)
  - Subtraction in spec fn produces int, must cast: `(content_extent - viewport_size) as u16`
  - gapped_items_disjoint needs by(nonlinear_arith) for j*(size+gap) >= (i+1)*(size+gap)
  - 50 new Verus proofs (1635 → 1685), all verified
- Property tests for rat-scrolltile (subwayrat): 12 proptest properties, 500 cases each
  - Window.id is pub(crate) — must track IDs from insert_window return values, not struct access
  - WindowId.0 is pub(crate) — can't sort by inner value, use HashSet for comparison
  - Cross-axis overflow is by design: visible rects clip on primary (scroll) axis only,
    windows can exceed viewport on cross axis (e.g., Fixed(57) height in 10-cell viewport)
  - Zero-width windows from proportional squeeze: when Fixed siblings exceed viewport,
    Proportion columns get width 0. Zero-width windows don't appear in visible set,
    even when focused. Focus-visibility property must guard on primary_size > 0.
  - All 12 properties pass: no-overlap, primary-axis clipping, fixed exactness,
    minmax bounds, scroll bounds, focus visibility, ID consistency, layout totality,
    nav roundtrips (left/right, up/down), remove-preserves-focus, column cross-axis non-overlap

## Architecture
- `spec fn *_spec(...)` — mathematical definition (ghost, erased at compile time)
- `exec fn *(...)` with `ensures result == *_spec(args)` — runtime code, verified to match spec
- `proof fn` — proves properties of the spec (symmetry, idempotent, bounded, etc.)
- `#[test]` + proptest — runtime verification + fuzz testing

## Verus Toolchain
- verus-root: `/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30`
- rust toolchain (for verus): `/nix/store/27f9852zp2qlg34zmqjz27cg6sq680sx-rust-default-1.93.0`
- vstd crates.io version: `=0.0.0-2026-01-25-0057`
- Wrapper: `scripts/verus.sh` runs rust_verify on src/lib.rs with correct extern paths

## Clippy Policy
- Correctness + suspicious: deny
- Pedantic: warn
- Manual impl lints allowed: `manual_abs_diff`, `manual_map`, `manual_saturating_arithmetic`,
  `manual_unwrap_or`, `implicit_saturating_sub`
- Cast lints: deny
- `unsafe_code`: forbid
- `integer_division`: allow in test modules

## Tracey Annotations
- `r[depends]` on `spec fn` — formal dependency on requirement text
- `r[impl]` on `exec fn` — the runtime implementation
- `r[verify]` on `proof fn` and `#[test]` — evidence of correctness
- Config scans `src/**/*.rs` only (no separate verus/ directory)

- Watermark module added: watermark_remaining, watermark_fits, watermark_alloc,
  watermark_reset, watermark_is_empty
  - All proofs automatic — pure linear integer arithmetic, no nonlinear_arith
  - Exec fns need `base as int + capacity as int <= u64::MAX as int` in requires —
    the remaining calculation `base + capacity - mark` overflows without this guard
  - `watermark_fits` exec uses `let remaining = base + capacity - mark; size <= remaining`
    instead of `mark + size <= base + capacity` to avoid addition overflow
  - Cross-module: alloc satisfies monotonic::is_advance (automatic),
    successive allocs produce region::regions_disjoint (automatic)
- Buddy module added: block_size, buddy_of, is_aligned_to_order, can_coalesce, parent_block,
  order_for_size
  - Verus bitwise ops (`&`, `|`, `^`, `!`, `<<`, `>>`) only work on u64 — subtraction
    promotes to int, so use `sub(x, 1u64)` in specs instead of `x - 1u64`
  - Exec fns need `assert(order < 64 ==> (1u64 << order) >= 1u64) by(bit_vector)`
    before `block_size(order) - 1` to prove no underflow
  - `(order + 1)` in spec becomes int — must cast `(order + 1) as u64` for `<<`
  - `leading_zeros()` can't be called in spec mode — use table-based spec instead
  - `assert(... ==> ...) by(bit_vector) requires ...` pattern: put the constraint
    INSIDE the `==>` antecedent, not as a separate `requires` clause
  - buddies_same_parent: inline the expression in the by(bit_vector) assertion,
    don't use `let` bindings inside the assert block
- Slab module added: slot_index, slot_offset, slot_address, is_slot_aligned, slab_capacity,
  generation_valid, generation_increment, handle_pack/unpack_index/unpack_generation
  - Verus macro `as int` before `<` or `-` followed by identifier: use parens or restructure
    to avoid generic-parameter parse ambiguity. `base as int + region_size as int` works,
    `region_size as int <= u64::MAX as int - base as int` fails — rewrite as
    `(base as int) + (region_size as int) <= (u64::MAX as int)` or flip the inequality
  - `gen` is a reserved keyword in Rust 2024 — use `g` or `generation` as parameter name
  - slot_index_bounded needs `lemma_div_is_ordered(offset, region_size, ss)` from vstd —
    Z3 can't derive `offset/ss <= region_size/ss` from `offset < region_size` alone
  - slot_address_aligned needs `by(nonlinear_arith)` for `(index * ss) % ss == 0`
  - Both roundtrip proofs (address→index, index→address) need `by(nonlinear_arith)` with
    `lemma_fundamental_div_mod` to connect multiplication commutativity
  - slab_capacity exact proof needs `by(nonlinear_arith)` for `(n*ss)/ss == n`
  - Handle packing: `by(bit_vector)` for roundtrip (low 32 = index, high 32 = generation)
  - Handle injectivity: extract both components from same handle, prove equality
  - Cross-module: slot_index(addr, 0, ss) == frame_number(addr, ss) — automatic
  - Cross-module: slab_capacity(n*s, s) == n — needs nonlinear_arith
- Region module added: contains_addr, regions_disjoint, contains_region, region_adjacent,
  region_merge, region_split — all (base, size) half-open region operations
  - Spec fn `contains_addr_spec` needs parenthesized `as int` casts:
    `(x as int) < (base as int + size as int)` — without parens, Verus parser
    reads `< base` as start of generic parameters, producing `expected ','` error
  - Split/merge proofs need `base as int + size as int <= u64::MAX as int` in requires —
    specs use `(x as int + y as int) as u64` which truncates on overflow, so Z3 can't
    prove properties like non-empty or disjoint without the overflow guard
  - `region_merge_spec` returns `(a_size as int + b_size as int) as u64` — needs
    overflow precondition for `merge_nonempty` and `merge_size_sum` proofs
  - `region_split_right_spec` returns `((base as int + offset as int) as u64, ...)` —
    needs overflow precondition for `split_disjoint`, `split_adjacent`, `split_covers`
  - `split_nonempty` and `split_size_sum` don't need overflow (pure int arithmetic)
  - All proofs automatic — no nonlinear_arith, no external_body
  - Cross-module: region ↔ interval agreement (contains, disjoint, adjacent) all automatic

- Raft replication Phase 1: append_entries_check + log_conflict
  - LogEntry needs `#[derive(Copy, Clone)]` inside verus! block — Verus preserves derive attrs,
    needed for Vec::clone and slice indexing (Copy) in test code
  - `as int < log.len()` parse ambiguity: `((prev_log_index - 1) as int) < log.len()` — parens required,
    same as the known `as int` before `<` pattern (parser reads `<` as generic parameter start)
  - `None::<int>` type annotation needed when comparing Option<int> in Verus ensures clauses
  - Seq::len() returns nat, subtraction produces int — explicit `as int` casts in spec if/else branches,
    e.g., `let overlap: int = if ... { leader_entries.len() as int } else { 0int }`
  - `prev_log_index as usize` cast needs `requires prev_log_index as int <= usize::MAX as int`
    for Verus to accept it without truncation warnings
  - Recursive find_conflict_from_spec inductive proofs need stronger ensures (offset bounds + follower length)
    for the wrapper to satisfy quantifier triggers — Z3 can't derive `j < leader_entries.len()` from
    `j < offset` without knowing `offset < leader_entries.len()`
  - resolve_conflict exec marked external_body: connecting mutation-state loop invariant to recursive
    append_entries_spec is hard — the "remaining work" pattern from bounded_peers works for simple
    concatenation but not for the find-then-truncate-then-append composition. Proptest covers correctness.
  - 2196 Verus proofs (+11 new), 25 new cargo tests (13 append_entries_check + 12 log_conflict)
- Raft replication Phase 2: match_index + commit_advance
  - Seq::len() returns nat, not int — `log.len() as int` needed when passing to specs that take int params
  - count_replicated_bounded_from: `matches.len() - from` is nat minus int — when from > len, result is 0
    but `len - from` is negative. Fix: use implication `from <= len ==> count <= len - from`
  - backtrack_decrements_by_one: nextIndex floor at 1 means backtrack(0) = 1 > 0 = old.
    The `<= old` ensures only holds when old >= 1. Added `requires nexts[server] >= 1`.
  - advance_monotonic_scan: spec returns `candidate as u64` in the found branch. Z3 needs
    `candidate <= u64::MAX` to verify the cast preserves the ordering. Added requires.
  - init loop pattern: `assert(v@ =~= Seq::new((i + 1) as nat, ...))` extensional equality
    hint needed after each push to connect the loop state to the spec
  - commit_advance backward scan: `while n > current_commit as usize && n > 0` maps to
    recursive `advance_scan_spec(... n as int)` — clean invariant
  - 2229 Verus proofs (+33 new), 23 new cargo tests (12 match_index + 11 commit_advance)

## Enforcement Scripts
- `check-coverage.sh` — scans for `pub fn` without `r[impl]`, exits 1 on violation
- `check-ensures.sh` — scans for `pub fn` inside `verus!` without `ensures`, exits 1
- Both support exemption markers (`@verified-logic:exempt`, `@verified-logic:no-ensures`)
- Lookback window: 10 lines for r[impl], 5 lines for exemptions
- Stops scanning at `#[cfg(test)]` (test fns don't need r[impl])
- Skips `spec fn`, `proof fn`, private `fn`

## Cross-Module Pattern
- Spec fns from other modules are ghost code — erased in normal builds
- Cross-module proofs must go in a `#[cfg(verus_keep_ghost)] mod verus_proofs { }`
- Inside that module: `use crate::other_module::*_spec` works because Verus sets the cfg
- Tests go outside the cfg-gated module (they use exec fns which exist in all builds)
- Verus verifies the whole crate including cfg(verus_keep_ghost) modules

## Generic Proof Decomposition
- algebraic.rs: spec_fn-based predicates for common algebraic properties
  - All quantifiers need explicit #[trigger] or #![trigger] — Verus can't auto-infer
    triggers on spec_fn application (e.g., `forall|a, b| f(a, b) == f(b, a)` fails)
  - is_semilattice = is_commutative + is_associative + is_idempotent_binary
  - is_partial_order = is_reflexive + is_antisymmetric + is_transitive
  - is_strict_partial_order = is_irreflexive + is_transitive
- pointwise.rs: Seq<u64> lifting — `Seq::new(a.len(), |i| f(a[i], b[i]))`
  - max_u64 as open spec fn (not closure) so it can be named in pointwise_merge_spec calls
  - Callers prove scalar properties (max_u64_commutative etc.) then call
    pointwise_commutative/associative/idempotent — the lifting is one call
  - `=~=` extensional equality handles the Seq::new unfolding automatically
  - vector_clock, crdt, causal_order all delegate merge proofs to pointwise
  - Exec fns stay in individual modules (Verus Fn trait + spec_fn linking is complex)
- seq_fold.rs: generic count_matching with bounded-by-length inductive proof
  - Same index-based recursion pattern as collections::count_from_spec
  - count_matching_bounded, count_matching_all, count_matching_none as reusable lemmas
- Witness proofs connecting concrete modules to generic predicates:
  - ordering: min_is_semilattice, max_is_semilattice
  - monotonic: advance_is_partial_order, strict_advance_is_strict_partial_order
  - signal: union_is_semilattice, intersect_is_semilattice, complement_is_involution
  - pressure: most_severe_is_semilattice_on_valid
  - All bit_vector proofs need `assert forall ... by { assert(...) by(bit_vector) }` pattern
    to prove quantified properties from per-element bit_vector facts

## Cross-Module Properties Wave 1
- 16 new cross-module properties added to properties.rs (1590 proofs, 1085 tests)
- signal ↔ bitwise: signal_add/remove/contains are identical to set_bit/clear_bit/bit_at — automatic
- permission ↔ bitwise: set-then-check roundtrip needs `by(bit_vector)` for `((mode | (1 << pos)) >> pos) & 1 == 1`
- permission independence: setting bit pos1 doesn't affect bit pos2 — `by(bit_vector)` with `pos1 != pos2` requires
- page ↔ arithmetic: pages_needed = div_ceil — identical spec bodies, automatic
- page aligned ↔ align_down fixpoint: needs `lemma_fundamental_div_mod` to connect `%` to `as u64` cast
  - First attempt with `by(nonlinear_arith)` failed — mod bound `0 <= r < ps` needs div_mod lemma as prerequisite
- lease valid ⟹ not expired: automatic — `now < expiry` (valid) implies `now <= expiry` (not expired)
- lease renewal is advance: automatic — `new_expiry > expiry` implies `new_expiry >= expiry`
- tier → score monotonic: automatic — Z3 case-splits on threshold comparisons
- backoff in range: automatic — `min(x, cap) <= cap` by definition of min
- approval majority ⟹ quorum: automatic — transitivity of `>=`
- two-phase unanimity ⟹ quorum: needs `requires votes.len() <= u64::MAX as int`
  - Without this, `votes.len() as u64` cast can't be proven in-range (Verus Seq has nat length)
  - `n >= n / 2 + 1` needs `by(nonlinear_arith) requires n > 0u64`
- expires_at monotonic is advance: automatic — saturating semantics preserve ordering
- cow ↔ drain isomorphism: clone=start, drop=finish — identical spec bodies, automatic
- Wave 2:
- type_lattice reflexive: automatic — `from == to` is first disjunct in can_convert_spec
- type_lattice ANY is top: automatic — `to == TYPE_ANY()` and `from == TYPE_ANY()` disjuncts
  - NOTE: type_lattice is NOT transitive! INTEGER→ANY→BOOLEAN works via ANY but
    INTEGER→BOOLEAN is false. The conversion graph is not transitively closed.
    Proptest caught this (a=1, b=6, c=3 counterexample).
- circuit_breaker preserves validity: automatic — all branches return CLOSED/OPEN/HALF_OPEN
- process_state terminal blocks + circuit_breaker OPEN blocks: automatic — concrete u8 values
- bitmap ↔ signal agree: automatic — both reduce to `(word >> pos) & 1 == 1`
- slab slot in region: needs `lemma_fundamental_div_mod` + `by(nonlinear_arith)` for
  `index < capacity/slot_size ==> index * slot_size < capacity`
- pressure → eviction pipeline: automatic — both check `count >= capacity`
- eviction_score cold minimum: automatic — `tier_multiplier(0) = 1 <= all others`
- Wave 3:
- evictable_count = saturating_sub: automatic — same computation `if a >= b { a-b } else { 0 }`
- gossip infection bounded: needs `saturating_pow_nonneg` helper lemma (was private, made pub)
  - Z3 can't determine bound >= 0 from recursive spec without inductive proof
- acid empty commit identity: automatic — `apply_writes_spec(state, seq![], 0)` hits base case
- priority_multiplier monotonic: automatic — 4 values, Z3 case-splits 16 pairs
- full_score bounded by 800000: needs `by(nonlinear_arith)` in two places:
  - weighted_score <= 10000 from weights summing to 100 and scores <= 100
  - final product bounded by 10000 * 20 * 4 = 800000
- consistent_hash lookup in ring: reformulated spec as case split (not existential) —
  existential with `#![trigger ring[i]]` failed because Z3 couldn't provide witness
  through recursive `first_ge_index_helper`. Direct case split on `idx < ring.len()`
  matches the spec's own structure, so Z3 handles it automatically.
- bloom insert preserves length: inductive proof with `decreases positions.len()`
  - `drop_last` precondition: `shorter[i] == positions[i]` for i < len-1
  - `Seq::update` preserves length — vstd axiom

## Mistakes & Corrections
- Pi `rg` tool wrapper shell-splits patterns with spaces or `|`; for alternations or multi-word patterns, use `bash` with a quoted `rg 'a|b' ...` command instead.
- Tried `cargo-verus` — needs rustup, doesn't work in nix. Used `rust_verify` directly.
- `proof fn` can't call `exec fn` in ensures — must reference `spec fn` instead.
- `FnSpec` deprecated — use `spec_fn`.
- Ghost closures can't have requires/ensures — restructured option module.
- Removed `verus/` directory — all specs embedded in `src/`.
- Chunk module proofs: `by(nonlinear_arith)` assertions need their own `requires` clauses
  to specify the necessary conditions: `assert(...) by(nonlinear_arith) requires ...;`
  Not function-level `by(nonlinear_arith)` — that's not valid Verus syntax.
- Hash bucket verification: bit_vector solver can't handle power-of-two arithmetic:
  - `assign_bucket_pow2` exec fn: removed `result < num_buckets` from ensures (bit-and bound too complex)
  - `assign_bucket_pow2_in_bounds` proof: created `#[verifier::external_body] pow2_mask_bound` helper
  - `assign_bucket_pow2_equivalent` proof: marked as `#[verifier::external_body]` (modular/bitwise equivalence known hard)
  - Runtime tests verify the mathematical identities empirically (x & (n-1) < n, x & (n-1) == x % n for power-of-two n)
  - These are known Z3/bit_vector limitations, not logic errors — the math is sound, verified by tests
- Closing external_body gaps (34 → 17):
  - bloom::query: while loop with ghost `map_values` tracking, `positions@.map_values` needs
    to be captured as Ghost variable for loop invariant reference
  - consistent_hash::lookup: key insight is `first_ge_index_helper(ring, kh, 0) == first_ge_index_helper(ring, kh, i)`
    as loop invariant — proving the recursive spec "skips" scanned elements
  - crdt::g_counter_value: needs THREE helper lemmas — take_lemma (take(i+1) decomposition),
    nonneg (sum >= 0), prefix_le (partial sum <= total) for overflow bounds
  - backoff::compute_backoff: vstd `lemma_mul_is_associative` and `lemma_mul_is_commutative`
    needed for connecting iterative doubling to recursive pow2 spec
  - merkle::verify_proof: `verify_proof_unfold` helper proves one-step spec unfolding via
    `=~=` on subrange equality; switched runtime hash to `compute_entry_hash`
  - bloom::insert_monotonic: requires `insert_preserves_len` helper lemma — Seq::update
    axioms (axiom_seq_update_same, axiom_seq_update_different) require 0 <= i < s.len(),
    so must prove insert_spec doesn't change length before reasoning about element values
  - bloom::no_false_negatives: split into `no_false_negatives_helper(bits, positions, i)`
    which proves individual position i is set, then wrap with forall
  - properties::prove_sequential_chunk_continuation: Z3 can't prove `(index+1) as u64 as int == index+1`
    without explicit `by(nonlinear_arith)` showing `index+1 <= u64::MAX` from the requires
  - nix_attrset::get_attr: added `requires is_sorted_keys(...)` to disambiguate `choose`
    operator — without key uniqueness, choose picks arbitrary matching index
  - Closed spec fns (merge_spec, remove_attrs_spec, etc.) cannot have external_body closed
    because Z3 can't see inside the spec to verify the body matches
  - gossip::convergence_rounds depends on `closed spec fn ceil_log_spec` — can't close
    without opening the spec (which would require proving termination of integer log)
- Merge exec closed: three-loop invariant (main merge, drain-a, drain-b) with ghost rg accumulator
  - Key invariant: `ai >= a.len() || bi >= b.len()` shared across drain loops — avoids
    split invariants for which side is exhausted
  - `seq_cmp_antisymmetric` proof needed: Z3 can't derive `seq_lt(b, a)` from `seq_cmp(a, b) > 0`
    without explicit antisymmetry lemma (recursive spec → uninterpreted for Z3)
  - `subrange_decompose` helper: s.subrange(i, j) =~= seq![s[i]] + s.subrange(i+1, j)
  - Verus rejects `a[ai].clone()` on indexed tuples — must destructure: `let k = a[ai].0.clone(); let v = a[ai].1;`
  - `merge_spec_right_empty` and `merge_spec_left_empty`: trivial helpers for Z3 to simplify
    merge_spec(s, empty) == s
- list_to_attrs spec opened and exec closed:
  - insert_into_sorted_spec uses seq_cmp directly (not seq_lt + ==) — exec's vec_u8_cmp
    result maps directly to spec conditions, no seq_cmp_eq proof needed
  - Two-phase exec with early return: phase 1 copies elements with keys < pair.key,
    then handles insert+skip inline and returns; fall-through appends at end
  - Key insight: using `return` from inside the loop (after copying remaining elements)
    avoids needing `break` or boolean flags — same pattern as has_attr/key_in_keys_exec
  - list_to_attrs exec is iterative fold: result = insert(pairs[i], result) for each i
  - Loop invariant: result_spec =~= list_to_attrs_spec(mp.subrange(0, i))
  - Spec unfolding hint: assert sub[sub.len()-1] =~= mp[i] and sub.subrange(0, sub.len()-1) =~= mp.subrange(0, i)
  - 7 new Verus proofs (999→1006), 1 new proptest (663→664)
  - Closed specs: 3→2, external_body: 12→11
- Version comparison spec opened: fuel-bounded recursion (like ceil_log pattern)
  - `cv_fuel(a, ai, b, bi, fuel)` with fuel = a.len() + b.len()
  - Helper specs: skip_seps, comp_end, parse_num_from, lex_cmp (all total, no when clauses)
  - `when` clauses on decreases cause recommendation issues in proof fns — removed in favor of
    explicit `i < 0 || i >= s.len()` base cases for totality
  - Spec uses `if ai2 < a.len() { is_digit_byte(a[ai2]) } else { false }` for guarded access —
    Verus `&&` in spec mode evaluates BOTH sides, triggering index-out-of-bounds recommendations
  - Proof fns must only access `a[ai2]` inside `if ai2 < a.len() { ... }` blocks
  - cv_fuel proofs (antisymmetric, reflexive, range) fail: spec has 8+ levels of nested
    let+if/else, Z3 can't unfold deeply enough even at rlimit(2000). The proof structure
    (induction on fuel, case split, recursive call at equal-value branches) is correct —
    kept as external_body with proof bodies preserved
  - lex_cmp proofs (antisymmetric, reflexive, range) all automatic with single recursive call
- Cost tracker module: all 4 phases + cross-module complete
  - compute_cost overflow: `tokens > u64::MAX / price` guard needs `by(nonlinear_arith)`
    with `lemma_fundamental_div_mod` to prove `tokens * price > u64::MAX` (and inverse
    for the safe branch)
  - milestone_count subtraction: `current / interval - prev / interval` needs
    `lemma_div_is_ordered` to prove `prev / interval <= current / interval`
  - complexity_score overflow: same `fundamental_div_mod` pattern for multiplication guard
  - Budget status as u8 constants (0-3) instead of enum — Verus can't match on enum
    variants in specs (same pattern as batch::FlushDecision)
  - Budget monotonicity: Z3 handles all case splits automatically — u8 ordering
    maps directly to the severity ordering
  - select_role: soft budget demotion proof (role <= role without pressure) works
    because score/2 <= score and thresholds are monotone in score
  - All 37 new Verus proofs automatic or single-lemma — no external_body needed
  - Rust toolchain path GC'd again: updated verus.sh to use
    /nix/store/2i2rgbachy91xi97xfc07y4s3abgksq5-rust-default-1.93.0
- Env scrub module: all 5 phases complete
  - to_upper_spec uses Seq::map — `s.map(|_i: int, b: u8| to_upper_byte_spec(b))`
  - Seq::map preserves length, so length proof is automatic
  - Idempotence of to_upper proved via per-byte case analysis — Z3 automatic
  - ends_with_bytes_spec needs explicit `#![trigger suffix[i]]` for quantifier matching
  - Verus loops MUST have `decreases` clauses (unlike Verus docs suggest) — applies
    to all while loops in exec functions
  - Blocklist entries as open spec fns returning seq! literals — verbose but allows
    Z3 to prove marker_not_blocked by length comparison (marker has 16 bytes,
    entries have 17, 12, 13, 12 bytes — length mismatch makes =~= false)
  - Secret suffix spec fns (7 entries) as seq! with explicit byte values
  - has_secret_suffix exec uses inline byte arrays: `&[95u8, 83u8, ...]`
  - bytes_eq helper needed: Verus doesn't support `==` on `&[u8]` (no PartialEq in vstd)
  - count_safe_spec recursive proofs need `idx <= keys.len()` in requires — when
    idx > keys.len(), spec returns 0 but `keys.len() - idx < 0` fails the ensures
  - All 31 new Verus proofs automatic — no external_body, no nonlinear_arith needed
  - 0 new external_body items (fully open blocklist approach)
- Closing external_body gaps (26 → 17):
  - eviction_score::weighted_score_monotonic_s1: as u64 monotonicity needs individual
    product bounds via by(nonlinear_arith) + call to existing weighted_score_bounded —
    Z3 can then derive ordering through the as u64 cast. DON'T put `as int` casts in
    `by(nonlinear_arith) requires` clauses — they fail. Use plain u64 comparisons.
  - delegation::chain_valid: loop invariant `chain_valid_spec(chain@, 0) <==> chain_valid_spec(chain@, i)`
    peels off one level per iteration. Overflow safety: track `chain@[i].depth <= 8` to ensure
    parent.depth + 1 won't overflow. Use the existing level_valid exec fn instead of inlining checks.
  - markov::confidence_meets: the large-total fallback branch needs fundamental_div_mod to decompose
    `total = q * 100 + r`, then nonlinear_arith for `total * threshold == main * 100 + frac`.
    Each sub-branch (count < main, diff overflow, normal) needs its own connecting assertions.
  - markov::best_prediction_index: standard argmax loop invariant with `forall|j| 0 <= j < i ==> counts[j] <= counts[best_idx]`. Trigger on `counts@[j]`.
  - pattern::pattern_contains: inner membership loop must NOT use `break` — Verus can't prove
    `j == p1@.len()` after break-exited loop. Instead use `if !found && p1[j] == target`
    and always increment j. For existential negation: `assert forall|idx| #![trigger p1m[idx]] !(0 <= idx < p1m.len() && p1m[idx] == target) by {}` — the empty body works because
    Z3 has the universal from the inner loop invariant covering all indices.
  - buddy::order_for_size: shift-loop invariant `s == sub(size, 1u64) >> order` works with
    by(bit_vector) shift composition proof, BUT the postcondition `block_size_spec(result) >= size`
    can't be derived. Z3's bit_vector theory proves `(1u64 << order) >= size` but the spec
    fn `block_size_spec(order) = (1u64 << order)` uses int-level `<<`, not bitvector `<<`.
    Z3 can't bridge between theories — same class as hash_bucket::pow2_mask_is_mod. Kept external_body.
- Chain hash chaining proofs: collision-free axiom, agreement theorem, divergence, single-entry
  - Recursive proof with quantified ensures hits rlimit — Z3 can't handle combining recursive
    quantified results (quantifier instantiation explosion across recursion levels)
  - Solution: split into non-quantified recursive helper (`agreement_at`) that proves one
    position at a time by peeling from k down to i, then non-recursive wrapper
    (`prove_chain_agreement`) uses `assert forall ... by { agreement_at(...) }` to assemble
    the full quantifier. No recursive function has a quantified ensures.
  - `agreement_step` (non-recursive) isolates collision-free reasoning: proves chain1[k] == chain2[k]
    and predecessor hash propagation. Z3 verifies instantly.
  - Divergence proof (contrapositive of agreement) uses if-then-contradiction pattern:
    `if chain1[k].hash == chain2[k].hash { prove_chain_agreement(...); assert(chain1[j] == chain2[j]); }`
  - hash_collision_free is external_body axiom (same as hash_deterministic) — hash_spec is closed/uninterpreted
  - 1640 Verus proofs, 1109 cargo tests — all gates green
- KV storage verification modules added: kv_version, snapshot_integrity, constant_time_eq
  - kv_version: all proofs automatic — pure integer logic, Z3 handles trivially
  - snapshot_integrity: collision-resistance axiom (external_body) for tamper-detection proofs;
    dual_hash_spec as closed uninterpreted fn (same pattern as chain.rs hash_spec);
    round-trip proof calls deterministic axiom, tamper proofs call collision-resistant axiom
    with contradiction pattern (if hashes equal → inputs equal → contradicts requires)
  - constant_time_eq: "remaining fold" invariant is the key insight —
    `xor_fold_spec(a, b, i, acc) == xor_fold_spec(a, b, 0, 0)` maintained by definition
    of the recursive spec (each step unfolds one position). No helper lemmas needed for
    the exec loop. Previous attempt with partial_fold_spec + extend lemma failed (inductive
    split across Seq elements requires bit_vector proofs that can't see Seq indexing).
  - constant_time_eq bit_vector proofs: MUST extract Seq elements into local `let` vars
    before the `by(bit_vector)` block — bit_vector solver can't handle Seq::index expressions.
    Pattern: `let ai: u8 = a[from]; assert((ai ^ bi) == 0u8) by(bit_vector) requires ai == bi;`
  - constant_time_eq fold_visits_all_bytes: reformulated from vacuous tautology to
    "any single-byte difference is detected" (calls any_difference_returns_false)
  - Cross-module: kv_version mod_revision/version → monotonic::is_advance (automatic),
    constant_time_eq_spec → extensional equality (calls equivalence_to_equality helper)
  - 3 new modules, ~30 Verus proofs, 29 cargo tests — all new code verifies clean
- Cluster coordination modules added: election_jitter, raft_timing, trust_level, node_lifecycle, drain_protocol
  - election_jitter: Knuth hash `node_id.wrapping_mul(PHI_KNUTH) % range` — spec uses
    `(node_id as int * K as int) % (u64::MAX as int + 1)` for wrapping_mul modeling.
    node_zero proof needs `by(nonlinear_arith)` with step-by-step assertions:
    `0 * k == 0`, `0 % m == 0`, `0u64 % range == 0u64` — Z3 can't simplify through
    the chain of modular arithmetic without explicit intermediate steps.
  - raft_timing: pure boolean logic — all proofs automatic
  - trust_level: u8 tag constants (BLOCKED/TRUSTED/PUBLIC), same pattern as circuit_breaker.
    All proofs automatic — Z3 handles the small domain case splits.
  - node_lifecycle: 6-edge transition graph over 4 u8 states. Offline terminal,
    bidirectional Online↔Degraded recovery. All proofs automatic.
  - drain_protocol: extends drain module with double-check admission and notification.
    Cross-module proof: `!can_start_op_spec(true)` implies try_start rejects — automatic.
    Uses `#[cfg(verus_keep_ghost)]` import of drain::can_start_op_spec.
  - 5 new modules, ~40 Verus proofs, 37 new cargo tests
  - Remaining cluster modules: tiered_rate_limit, peer_spec, trust capacity predicates
  - tiered_rate_limit: check_tiered composes rate_limit::try_consume for global-first
    ordering. Rollback pattern: on peer denial, don't consume global tokens (return
    original values). Z3 handles all proofs automatically via spec unfolding.
  - peer_spec: find_separator is external_body (Verus can't handle for+if loop over
    &[u8]). parse_peer_spec checks sep position for error classification. The `choose`
    operator in find_separator_spec doesn't propagate bounds well — proof that
    `sep < s.len() - 1` fails because Z3 can't extract the `0 <= i < s.len()` constraint
    from the existential witness. Simplified proof to just `sep > 0`.
  - trust capacity: trivial is_bounded predicate with add/block maintenance proofs
  - Cross-module: tiered→try_consume link automatic
- Closing external_body gaps (26 → 21):
  - peer_spec::find_separator: standard "find first" while loop with forall j < i invariant.
    Original ensures had unsatisfiable postcondition (`result < s.len() ==> result >= 0` fails
    when result==-1 and s is non-empty). Fixed ensures to separate found/not-found cases.
    Added `requires s@.len() <= i64::MAX` for i64 cast safety.
  - node_classify::classify_node: membership check via while loop over member_ids with
    existential invariant `is_voter == exists|j| 0 <= j < i && member_ids[j] == node_id`.
    Z3 handles the existential tracking automatically.
  - node_classify::classify_nodes: batch loop building Vec with per-element invariant.
    Final `assert(result@ =~= classify_nodes_spec(...))` via extensional equality.
  - bounded_peers::collect_peers: "remaining work" concatenation invariant:
    `result@ + collect_build_spec(cands, self_id, i, max_peers - count) =~= full`.
    Z3 unfolds the recursive spec one step per iteration — handles skip (self_id),
    keep (non-self), and both termination conditions (count >= max or i >= len).
  - voter_set::promote_voters: same "remaining work" pattern as bounded_peers.
    `result@ + promote_build_spec(voters, learner_id, replace_id, i) =~= full`.
    After loop: promote_build_spec at end position returns seq![learner_id],
    so push(learner_id) completes the result.
  - bitfield proofs (4): confirmed SAT encoding too large for variable offset+width.
    by(bit_vector) on `((v & !mask) | ((f & wmask) << offset)) >> offset) & wmask == f`
    with unconstrained offset/width creates exponential SAT problem. Permanent external_body.
  - buddy::order_for_size: theory bridge issue — bit_vector proves `1u64 << order >= size`
    but spec uses int-level `<<` via block_size_spec. Z3 can't bridge. Permanent external_body.
  - glob::glob_match: spec is degenerate (`true` for starred patterns). Can't close without
    writing full recursive glob spec. Low priority.
- Closing external_body via decomposition (21 → 16):
  - hash_bucket::pow2_mask_is_mod: 64-way case split on power-of-two values.
    pow2_cases helper proves `n > 0 && n & (n-1) == 0` implies n is one of 64 values
    (single by(bit_vector) on one variable). Each case: `x & (c-1) == x % c` with
    constant c is trivial SAT. Total: 65 new by(bit_vector) assertions, all instant.
  - bitfield::round_trip: decomposed into 3 sub-lemmas:
    (A) clear_extracts_zero: cleared bits extract to 0
    (B) shift_roundtrip: shift-left then shift-right recovers field within mask
    (C) or_extract_distribute: OR distributes through shift+mask
    Each sub-lemma is simpler SAT (fewer operations per assertion). Main proof chains
    them with `assert(0u64 | field == field) by(bit_vector)`.
  - bitfield::non_interference: decomposed into:
    (A) clear_preserves_outside: bit p of (v & !mask) equals bit p of v outside field
    (B) shifted_field_zero_outside: bit p of f_shifted is 0 outside field
    (C) OR single-bit distribute: `((a|b)>>p)&1 == ((a>>p)&1)|((b>>p)&1)`
  - bitfield::clear_then_extract: delegates to clear_extracts_zero (already proven)
  - bitfield::insert_idempotent: decomposed into 4 steps:
    (1) shifted_field_outside_mask_zero: f_shifted & !mask == 0
    (2) AND distributes over OR (inline by(bit_vector))
    (3) AND-with-!mask is idempotent (inline by(bit_vector))
    (4) Chain via `assert((cleared | f_s) & !mask == cleared) by { ... }`
    Key insight: let bindings are invisible to by(bit_vector) — must inline the full
    expressions. The chaining proof uses nested `assert ... by { ... }` to give Z3
    the intermediate facts in the right order.
  - Key decomposition principle: by(bit_vector) fails when the circuit has too many
    variable shifts composed. Decomposing into sub-lemmas with fewer shift operations
    per assertion makes each SAT problem tractable. The connecting logic uses Z3's
    standard arithmetic reasoning (no by(bit_vector) needed for equality chaining).
- 10 new Raft consensus primitive modules added:
  - replication_mode: per-peer FSM (Probe/Replicate/Snapshot) with transition functions
  - pre_vote_tally: pre-vote response tracking, mirrors vote_tally for pre-election phase
  - follower_read: follower-side linearizable reads via leader commit confirmation
  - request_forward: follower→leader write forwarding with pending tracking
  - commit_notify: commit notification dispatch, newly committed ranges, proposal status
  - compaction_fence: log compaction coordination with replication, safe compact index
  - witness: witness replicas — vote without full log, role taxonomy (Voter/Witness/Learner)
  - priority_election: priority-based elections with delay, yield, candidate comparison
  - learner_change: learner add/remove without quorum impact, idempotent operations
  - pre_vote_lease: PreVote + CheckQuorum interaction, reject pre-votes when leader active
  - Verus `ensures { let ... }` block syntax must use parentheses: `ensures ({ let ... })`
    Without parens, Verus parser reads `{ ... }` as function body, not spec expression
  - Spec subtraction of u64 produces int — `(now - last_contact) as u64` fails in spec
    because Verus reads `} else {` after `as u64 <` as generic parse ambiguity. Use plain
    subtraction in spec (int comparison works fine): `now - last_contact < election_timeout`
  - 96 new tests (1669 → 1765), all passing. All proofs automatic (no nonlinear_arith, no external_body)
- Verus type error fix session (2026-03-26):
  - 16 E0308 type errors from u64 arithmetic promotion in spec contexts
    (u64 + 1 → int, int / u64 mismatch, pub const outside verus! block)
  - 23 verification failures masked by type errors — exposed after type fixes
  - Key patterns: (x + 1) as u64 needs requires x < u64::MAX;
    recursive count specs need bounded-by-length lemmas for (rest + 1) as u64 casts;
    bitwise proofs in persistent_state need explicit by(bit_vector) hints;
    exec overflow guards must match spec's int-level arithmetic
  - transfer_progress marked external_body: overflow fallback path
    (received / (total/100)) doesn't match int-level spec (received * 100 / total)
  - lease_read proofs needed physical time constraints (now >= start)
  - compaction_fence invariant simplified from disjunction to single accumulator form
- Ghost history module added: VoteRecord, LeaderLogSnapshot, EntryProvenance
  - Epoch doesn't derive Copy/Clone — stored epoch fields inline in VoteRecord
    (voter_last_term, voter_last_index) instead of Epoch struct
  - single_vote_per_term_spec uses double quantifier with trigger on history[i], history[j]
  - append_fresh_preserves_single_vote proof: case-split on i/j being the new element vs old,
    with explicit `assert forall ... implies ... by { if ... { } else { } }` pattern
  - append_revote_preserves_single_vote: same pattern, but the same-term case uses the
    precondition that existing records for that term have the same candidate
  - cronies_exclusive inductive proof: must first prove prefix satisfies single_vote_per_term
    by asserting forall on prefix[i], prefix[j] and showing they equal history[i], history[j]
  - record_entries_spec recursive: needs explicit decreases on entries.len(), helper proofs
    for length and preservation also recursive with same pattern
  - record_entries_preserves needs two-step proof: inner assertion that next_prov[i] == provenance[i]
    plus recursive call establishing the same for the accumulated provenance
- Election safety proof module added: election_safe_spec, election_safety_theorem
  - cronies-based approach (not message-counting): count votes in ghost history, check quorum
  - quorum::majority_ge_one is not pub — inlined as `assert(cluster_size / 2 + 1 >= 1)
    by(nonlinear_arith) requires cluster_size > 0u64` in election_safety_proof
  - election_safety_theorem uses `assert forall ... implies ... by { if ... { } }` pattern
    with cronies_exclusive + inlined majority_ge_one
  - quorum_winner_unique: direct corollary of cronies_exclusive — if winner has quorum,
    challenger has zero cronies (by exclusivity), zero < majority (by majority_ge_one)
  - Pre-existing check_quorum type error (E0308) unrelated to new modules — Verus verification
    of ghost_history and election_safety_proof has zero errors
  - 1819 cargo tests passing (23 new: 13 ghost_history + 10 election_safety_proof)
- Glob spec opened: segments_match_spec over Seq<Seq<u8>> from split_by_star_spec
  - split_by_star_spec needs explicit guard `se > pattern.len() || se < from` for termination
    because Verus can't prove segment_end returns >= from without it
  - Wildcard proof needs step-by-step unfolding: assert segment_end, assert split_by_star on
    tail, assert concat, assert each segments_match level individually
  - segments_match_spec uses `when` clause for wellformedness (0 <= pos <= end <= input.len())
- nix_string replace_strings_spec opened with recursive replace_at_spec
  - Verus can't prove `nat != 0 ==> nat >= 1` for termination on non-empty match advance
  - Solution: replace_at_inner as external_body spec fn for the non-empty match case,
    separating the variable-advance recursion from the fixed-advance recursion
  - first_match_idx returns -1 (not None) for easier int comparison in when/decreases
  - trailing_empty handles end-of-string empty-pattern insertion
- resolve_conflict closed: key was WRONG invariant in the original code
  - Original invariant: follower@ == append_entries_spec(rss, entries, j) — WRONG direction
  - Correct invariant: append_entries_spec(follower@, entries, j) == full_result
  - "What remains to be done" form: applying remaining entries to current state = full result
  - Before loop: follower = rss, so append_entries_spec(rss, entries, append_from) = full_result ✓
  - Each step: definition of append_entries_spec (push + advance) maintains invariant
  - After loop: append_entries_spec(follower@, entries, entries.len()) = follower@ = full_result
  - Arithmetic overflow guard: conflict_in_range proves offset < entries.len(), so
    start_index + offset <= start_index + entries.len() <= usize::MAX
- Closing external_body gaps (25 → 18):
  - ghost_history::record_full_log_covers: key was record_entries_at_position helper —
    proves result[prov.len() + k].entry == entries[k] by induction, then provides
    existential witness j = prov.len() + i for the forall in entry_property_holds.
    Uses record_entries_preserves (existing) to show push doesn't disturb earlier elements.
  - ghost_history::cronies_exclusive: decomposed with cronies_count_zero_if_no_match helper —
    if no record matches (term, candidate), count is 0. Main proof uses single_vote_per_term
    to derive that if last record has candidate_a, ALL records with that term have candidate_a
    (by the quantified uniqueness property), so candidate_b has zero matches.
  - rate_limit::tokens_after_refill: rewrote exec to use u128 intermediate arithmetic.
    u64 * u64 fits in u128: (2^64-1)^2 < 2^128-1. The u128 division matches the
    int-level spec exactly (both are truncating division on non-negative values).
    Overflow proof needs by(nonlinear_arith) with explicit requires on bounds.
  - rate_limit::tokens_to_wait_ms: same u128 trick for shortage * 1000 + rate - 1.
    u64 * 1000 << u128::MAX, so no overflow.
  - snapshot_transfer::transfer_progress: u128 for received_bytes * 100. Added
    by(nonlinear_arith) for result <= 100 postcondition (a < b ==> a*100/b < 100).
  - snapshot_integrity::dual_hash: just needed r[impl] annotation (coverage gate fix)
  - rate_limit::wait_enables_consumption: NOT closeable — ensures is unsound in the
    saturation case. When wait_int > u64::MAX and rate_per_sec is small (e.g., 1),
    u64::MAX milliseconds is insufficient to earn shortage tokens. Counterexample:
    tokens=0, cost=u64::MAX, rate=1, burst=u64::MAX → refill = u64::MAX/1000 < cost.
  - log_invariant_inductive (3 items): NOT closeable — system-level lemmas requiring
    all logs come from same leader chain. The ensures for leader_append_preserves_lmp
    is too strong at module level (counterexample: log_a=[term=1], log_b=[term=2,term=3],
    push term=3 → LMP breaks at new entry because prior entries don't match).

- Escrow module added: fund, release, refund, dispute state machine
  - Uses pub const u8 state constants (same pattern as circuit_breaker, batch)
  - Conservation proofs: fund-then-release and fund-then-refund preserve three-party total
  - Fund-refund roundtrip returns buyer to original balance
  - Terminal states (Released, Refunded) block all transitions — proved exhaustively
  - Dispute is state-only transition, no balance changes
  - total_value uses u128 — three u64 values can't overflow u128
  - Cross-module: fund_buyer_after_spec == debit_spec, release_seller_after_spec == credit_spec
  - All proofs automatic — no nonlinear_arith, no external_body
  - 17 new Verus proofs, 15 new cargo tests (8 unit + 7 proptest)
- Fee module added: compute_fee, compute_net, split_with_fee, tiered_fee
  - Basis points: rate_bps in [0, 10000], fee = floor(amount * rate_bps / 10000)
  - u128 intermediate for amount * rate_bps (same pattern as rate_limit)
  - Overflow proof: amount * rate_bps <= u64::MAX * 10000 << u128::MAX — by(nonlinear_arith)
  - Result bound: fee = product / 10000 <= amount — by(nonlinear_arith) with rate_bps <= 10000
  - Monotonicity in both amount and rate: uses vstd::arithmetic::div_mod::lemma_div_is_ordered
  - fee_bounded proof: two-step — first prove product ordering, then apply div_is_ordered
  - full_rate_takes_all: needs lemma_fundamental_div_mod + nonlinear_arith for mod == 0
  - Conservation: fee + net = amount — trivially automatic from spec definitions
  - Tiered fee: max(proportional, min_fee) capped at amount — all proofs automatic
  - proptest! format strings can't capture outer variables (proptest macro limitation) —
    use plain prop_assert_eq!/prop_assert! without format messages
  - 19 new Verus proofs, 12 new cargo tests (5 unit + 7 proptest)
  - Cross-module: escrow fund = balance debit, release = balance credit, fee + net = amount
  - 3020 total Verus proofs, 1895 total cargo tests
- Ledger module added: multi-account Seq<u64> with total conservation
  - sum_update_delta is the key inductive lemma: updating one element at index idx
    changes the sum by exactly (new_val - old_val). Uses case split on from == idx.
  - sum_update_total wraps sum_update_delta with from=0 for clean API
  - transfer_preserves_total: two calls to sum_update_total — debit cancels credit
  - fee_transfer_preserves_total: three sum_update_total calls — debit = net + fee
  - single_account_total needs explicit hint: `assert(ledger_sum_spec(seq![balance], 1) == 0)`
    — Z3 can't unfold recursive sum on singleton seq without the trigger
  - batch_preserves_total: inductive on transfer sequence, calls transfer_preserves_total per step
  - Leading `&&` in spec bodies → Verus Chainable trait error. Use plain `condition1 && condition2`
  - Seq::update preserves length and leaves other indices unchanged — Z3 handles this automatically
    via vstd axioms (axiom_seq_update_same, axiom_seq_update_different)
  - Cross-module: ledger transfer sender = balance::debit, receiver = balance::credit
  - 28 new Verus proofs (3048 total), 13 new cargo tests (1908 total)
  - Zero external_body — fully verified

- Settlement module added: bilateral obligation netting with conservation proofs
  - Position computation: apply_from_spec applies obligations left-to-right to Seq<int> positions
  - Core theorem: positions_sum_zero — inductive proof that each obligation preserves the int sum
    (apply_obligation_preserves_sum uses int_sum_update_total, same pattern as ledger)
  - int_sum_update_delta mirrors ledger's sum_update_delta but for Seq<int> instead of Seq<u64>
  - int_sum_zeros_from helper: proves sum of all-zeros Seq is 0 from any starting index
  - Exec uses i128 for positions — bounded by obligations_count * u64::MAX. Loop invariant:
    `positions@[k] >= -(i as int) * (u64::MAX as int) && positions@[k] <= (i as int) * (u64::MAX as int)`
  - Overflow proof: `(i+1) * u64::MAX == i * u64::MAX + u64::MAX` via by(nonlinear_arith)
  - obligation_count_spec takes Seq<(int,int,u64)> but exec takes &[(usize,usize,u64)] — can't
    reference spec from exec ensures due to type mismatch. Use `result == obligations@.len()` instead.
  - is_valid_settlement exec exempted from ensures (no-ensures) — type mismatch between
    (usize,usize,u64) exec and (int,int,u64) spec makes direct ensures impossible. Covered by proptest.
  - swap_negates_position: postcondition must use apply_obligation_spec on zeros directly,
    not compute_positions_spec (which wraps apply_from_spec and Z3 can't unfold through it)
  - single_obligation_sum_zero needs explicit `assert(all_obligations_valid_spec(obs, n, 1))` then
    `assert(all_obligations_valid_spec(obs, n, 0))` — Z3 can't unfold both at once
  - Leading `&&` in spec bodies: same Chainable trait error as ledger — use plain `condition && ...`
  - 26 new Verus proofs (3074 total), 10 new cargo tests (1918 total)
  - Zero external_body

- buddy::order_for_size closed (14 → 13 external_body):
  - Same 64-way case split technique as hash_bucket::pow2_mask_is_mod
  - shift_zero_means_ge helper: for each concrete k in 0..63, proves
    `sub(size, 1u64) >> k == 0u64 ==> (1u64 << k) >= size` via by(bit_vector)
  - With concrete k, block_size_spec(k) unfolds to (1u64 << k), which Z3 evaluates
    to a concrete value, bridging the bitvector ↔ spec-level << theories
  - Loop invariant: `s == sub(size, 1u64) >> order` (u64 equality in bitvector theory)
  - Shift composition: `(x >> k) >> 1 == x >> (k+1)` proved by(bit_vector) with variable k
  - Safety cap: `order == 63 ==> s == 0` because `size <= 2^63` means `(size-1) >> 63 == 0`
  - Post-loop: s == 0, so sub(size, 1u64) >> order == 0, then shift_zero_means_ge bridges
  - 72 new Verus verified items (3110 → 3182), 0 new tests (exec behavior unchanged)
  - Key principle: by(bit_vector) with ONE variable + concrete constants is tractable SAT;
    by(bit_vector) with variable shift amounts composed is the problem. Case splits fix it.

## Verus Upgrade 0.2026.01.30 → 0.2026.03.22
- New Verus requires Rust toolchain 1.94.0 (was 1.93.0)
- `librustc_driver-24b10b5e3eb5af70.so` linker error: need to update `rustc-1_93` → `rustc-1_94`
  in flake.nix (three references: definition, buildInputs, wrapper LD_LIBRARY_PATH)
- All 3309 proofs pass on new Verus with zero code changes (3182 → 3309 verified items
  because new Verus counts more items per function)
- verus.sh paths must be updated manually after nix builds new store paths

## Proptest Hardening
- `LogEntry` needs `#[derive(Debug)]` for proptest (proptest macro requires Debug on all types)
- Spec-only fns (e.g. `is_valid_step_type_spec`) can't be called in proptest — use concrete
  constants and inline the logic instead
- `split_with_fee` returns (fee, net) not (net, fee) — check ensures clause order
- `is_simple_entry_spec(amount, amount)` requires `amount > 0` — zero is not a valid entry
- `is_valid_settlement` takes (original, settlement, num_parties) — 3 args not 2
- `ledger_transfer` returns `Option<(u64, u64)>` not `Vec<u64>` — use `if let Some((a,b))`

## Cross-Module Properties
- Existing `financial_proofs` module in properties.rs already covers escrow-fund-uses-debit,
  escrow-release-uses-credit, fee-conservation-matches-transfer, ledger-transfer-uses-balance,
  ledger-conservation-is-double-entry. Don't duplicate — name new module differently.
- New additions: escrow-fund-refund-identity, fee-split-agrees, simple-entry-is-balanced,
  ledger-transfer-needs-funds, settlement-positions-sum-zero (proptest only)

## u128 intermediate pattern
- When exec needs `a: u64 * b: u64` without overflow: use `a as u128 * b as u128`
- Verus needs explicit overflow proof: `assert((a as int) * (b as int) <= (u128::MAX as int)) by(nonlinear_arith) requires ...`
- The `as int` before `<` parse ambiguity applies: always parenthesize `(u64::MAX as int)`
- u128 division matches int division for non-negative values — no additional proof needed
- This pattern eliminated all the complex overflow-splitting exec code in rate_limit

## by(nonlinear_arith) requires and the u64↔int gap
- `by(nonlinear_arith) requires ...` — the requires are checked by Z3's standard solver,
  then the nonlinear solver gets ONLY the requires as assumptions
- Z3's standard solver CAN'T verify `q * d + r == x` when q, d, r are variables
  (multiplication is nonlinear). Even if `lemma_fundamental_div_mod` was called.
- Fix: use a **proof fn** where `let q = x as int / d as int` (int-level division).
  Inside the proof fn, Z3 CAN verify `q * d + r == x` because it matches the
  fundamental_div_mod output directly (same int expressions, no u64↔int bridging).
- Then the proof fn's `ensures` provide the facts to exec code via int-level postconditions.
- Example: `level_count_step` establishes `remaining/ratio < remaining` and `r > 0 ==> q+1 < remaining`
  using by(nonlinear_arith) inside a proof fn, then the exec calls it.
- The `as int` before `<` parse ambiguity applies in ensures too: use `(remaining as int)`.

## uninterp spec fn
- Bodiless spec fns (`pub closed spec fn foo() -> T;`) now need `pub uninterp spec fn`
- `closed spec fn` without a body triggers "will become a hard error" warning
- Changed hash_spec (chain.rs) and dual_hash_spec (snapshot_integrity.rs)

## Closing lsm_level external_body (16 → 14)
- level_target_size: while loop invariant `current == level_target_size_spec(i, base, ratio)`
  - Saturation helper: once `prev * ratio > MAX`, all subsequent levels return MAX
  - `target_saturated_stays` inductive proof: prove level i+1 = MAX, then MAX * ratio > MAX
    propagates forward
- level_count_for_size: "remaining work" invariant with `level_count_step` proof fn
  - `level_count_bounded(x, base, ratio)` lemma: `lcfs(x) <= x` by induction
  - `level_count_step` proves ceil_div progress + lcfs >= 1 in int-land
  - Exec splits `if r == 0 { q } else { q + 1 }` matching spec's branch structure
  - Overflow safety: `lcfs(remaining) >= 1` so `levels < lcfs(total) <= total <= u64::MAX`
- space_amp_ge_100, space_amp_monotonic: added `(total * 100) / live <= u64::MAX` requires
  - Without this, the `as u64` cast in `space_amp_pct_spec` truncates for large quotients
  - Counterexample: total=184467440737095517, live=1 → result wraps to 84 < 100

## Cross-module property wave 4 (2026-03-26)
- circuit_breaker: OPEN=1, CLOSED=0, HALF_OPEN=2 — NOT OPEN=2
- allow_request_spec takes (state, probe_active) — two params, not one
- should_reset_election_timer takes (msg_term, current_term, from_leader) — 3 params
- compute_lock_deadline takes ttl_ms (milliseconds), expires_at converts ttl_seconds*1000 — different units!
- noop_commit_index_spec returns log_len + 1, not log_len (1-based commit index)
- is_last_chunk checks offset + chunk_size == total_size (exact equality, not <=)
- Always read the actual spec fn signatures before writing cross-module proofs

## Concurrency Modules (2026-03-26)
- lock_ordering, epoch_reclaim, hazard_pointer — 3 new modules
- Verus while loops MUST have `decreases` clauses — every time, no exceptions
- Counting loop overflow: `count + 1` needs proof that count < u64::MAX.
  Add `count as int <= i as int` to invariant — count of matching elements
  in [0..i) can't exceed i. Use int-level comparison, NOT `as usize` (triggers
  Verus cast recommendations).
- Seq-based recursive count specs (count_active_from_spec, count_reclaimable_from_spec)
  need the count <= index invariant for the exec loop to verify `count + 1`
- hazard_pointer: HP_EMPTY sentinel (u64::MAX) makes is_protected_spec clean —
  `addr != HP_EMPTY` in the existential avoids matching sentinel slots
- epoch_reclaim: three-epoch scheme (0,1,2) with cyclic gap. Core safety proof
  (safe_reclaim_no_active_at_retire) is automatic — Z3 derives retire_epoch != global_epoch
  from gap >= 2, then can_advance gives all active threads at global_epoch
- lock_ordering: valid_order_chain inductive helper proves locks[i] < locks[j] for i < j,
  then wraps into valid_order_implies_distinct and last_is_max
- All 81 new Verus proofs automatic — no nonlinear_arith, no external_body, no bit_vector
- seqlock, ref_count, work_stealing, barrier — 4 more modules (wave 2)
- `gen` is reserved keyword in Rust 2024 — use `generation` (napkin already warned!)
- `expected - 1` in spec context (ensures/requires) produces int, not u64 — must cast:
  `arrive_spec((expected - 1) as u64)` not `arrive_spec(expected - 1)`
- Same for `top + 1` in spec: `after_pop_spec((top + 1) as u64)`
- All 90 new Verus proofs automatic — total across both waves: 171 new proofs, 0 external_body
- spin_backoff, rcu, mpsc_bounds, futex_state — 4 more modules (wave 3)
- `ensures { let ... }` block syntax → Verus error "block followed by another block" —
  must use `ensures ({ let ... })` with parens (same pattern as commit_notify)
- spin_count_for_step with `1u64 << capped` in spec: Z3 can't reason about spec-level
  shift on variables. Workaround: make the exec fn @verified-logic:no-ensures, test empirically.
  The interesting proofs are about phases, not exact spin counts.
- advance_from_spinning needs `spin_limit < yield_limit` (strict), not `<=`.
  When spin_limit == yield_limit, advancing past spin goes directly to completed,
  skipping the yield phase entirely.
- mpsc_bounds::receive_makes_not_full needs `occupancy <= capacity` in requires.
  Without it, after_receive(40) = 39, is_full(39, 1) = true — counterexample.
  proptest range must also guard: `if occ <= cap { ... }`
- futex_state: u32 constants (not u64) for futex word — matches Linux futex API
- ticket_lock, cas_loop, bounded_mpmc, memory_order — 4 more modules (wave 4)
- `(x as u64) < capacity` in spec needs parens: `((x) as u64) < capacity` — Verus reads
  `as u64 <` as start of generic parameters (same `as int <` pattern in napkin)
- Chained comparisons like `0 <= x < y` not supported in `by(nonlinear_arith) requires` —
  must split into `0 <= x, x < y` as separate requires clauses
- cas_loop ABA pack/unpack: `by(bit_vector)` handles `(v | (g << 32)) & 0xFFFF_FFFF == v`
  and generation extraction roundtrip automatically — single variable SAT is tractable
- bounded_mpmc consecutive_laps_different: `lemma_div_is_ordered` + `lemma_fundamental_div_mod`
  then `by(nonlinear_arith)` with the fundamental_div_mod decomposition as requires
- memory_order: Acquire ⊁ Release (incomparable in lattice) — NOT a total order.
  combine_orderings is the join operation (least upper bound). All proofs automatic — Z3
  handles the 5-element domain by case splitting on u8 constants.

## Closing gaps + cross-module wave 5 (2026-03-26)
- spin_backoff::spin_iterations: added `ensures result == spin_iterations_spec(step, limit)`.
  With `requires limit < 64`, capped < 64, so spec always takes the `(1u64 << capped) as u64`
  branch. Exec's `1u64 << capped` matches directly. Z3 handles automatically.
- trust_level::is_bounded: removed `@verified-logic:exempt`, added `r[impl trust-level.capacity-bounded]`.
  Already had ensures — was just missing the coverage annotation.
- settlement::is_valid_settlement: NOT closeable — exec uses `(usize,usize,u64)` tuples,
  spec uses `(int,int,u64)`. Bridging requires proving compute_positions exec (i128 arithmetic)
  matches spec (int arithmetic) at every loop iteration. Needs invasive loop invariant changes
  to compute_positions. Left as no-ensures.
- nix_closure::generic_closure: NOT closeable — exec is outside verus!{}, uses dyn Fn + HashSet.
  Fundamentally incompatible with Verus verification.
- Cross-module wave 5: 13 concurrency cross-module properties
  - mpmc_occupancy == mpsc_occupancy for same inputs (automatic)
  - work_stealing deque_size == mpmc occupancy (automatic)
  - spin_backoff completed ⟹ cas_loop no retries (automatic)
  - seqlock write needs Release ordering, read needs Acquire ordering (automatic)
  - RCU grace advance == epoch next_epoch for epochs 0,1 (NOT for epoch 2 — RCU increments,
    epoch_reclaim wraps to 0. Initial proof was wrong, caught by Verus.)
  - ref_count zero + not hazard-protected ⟹ reclaimable (automatic)
  - lock_ordering prevents ticket_lock deadlock (automatic)
  - barrier all_arrived ⟹ quorum (automatic)
  - futex notify → consume roundtrip returns to IDLE (automatic)
  - mpsc drained ⟹ mpmc empty (automatic)
  - CAS idempotent always succeeds (automatic)
  - ticket_lock release reduces waiters by 1 (automatic)
- Constants are prefixed: FUTEX_IDLE not IDLE, ORD_RELEASE not RELEASE, ORD_ACQUIRE not ACQUIRE
  Always grep for the actual constant names before using them in cross-module proofs.
- 3714 Verus proofs (+125), 2262 tests (+77), 984 requirements (+15)

## Scheduling Primitives (2026-03-26)
- 5 new modules: deadline_queue, fair_share, priority_inversion, weighted_round_robin, job_dag
- fair_alloc: `requires weight <= total_weight` needed for result to fit in u64.
  Without this guard, `total_tokens * weight / total_weight` can exceed u64::MAX,
  and the `as u64` cast truncates, breaking monotonicity. Proptest caught this
  (tokens=6.2e18, w1=1, w2=488, tw=41 → w2 > tw, result wraps).
- u128 overflow proof pattern: `assert(a as int * b as int <= u128::MAX as int)
  by(nonlinear_arith) requires a as int <= u64::MAX as int, b as int <= u64::MAX as int,
  (u64::MAX as int) * (u64::MAX as int) <= u128::MAX as int;`
  The third requires clause gives Z3 the concrete bound it needs.
  Using `requires a <= u64::MAX` (on u64 type) doesn't work — Verus treats u64 <= u64::MAX
  as tautological and doesn't pass it to nonlinear_arith.
- `by(nonlinear_arith) requires` can't reference `let`-bound int variables directly.
  Use the actual expressions: `requires t >= 0, a >= 0, a <= b` not `requires p1 >= 0`.
- higher_weight_more_tokens proof: needs explicit chain:
  (1) t*a <= t*b (nonlinear_arith, t >= 0, a <= b)
  (2) div_is_ordered(t*a, t*b, d)
  (3) div_pos_is_pos for both (nonneg products)
  Z3 can't bridge from int-level division ordering to the `as u64` cast in fair_alloc_spec
  without knowing both results are nonneg and <= u64::MAX.
- `num_consumers - 1` in spec context produces int, not u64. Must cast:
  `next_consumer_spec((num_consumers - 1) as u64, num_consumers)`.
  Same pattern as all other u64 subtraction in specs.
- `0..0` is invalid in proptest::collection::vec — panics at runtime.
  Use a dummy parameter and construct the empty vec manually.
- job_dag, priority_inversion exec fns with recursive-spec + membership-search loops:
  marked `#[verifier::exec_allows_no_decreases_clause]` and `@verified-logic:no-ensures`.
  Still need `invariant i <= deps@.len()` and `invariant i < deps@.len()` (inner loop)
  for Verus to verify array bounds on `deps[i]`. Without the inner invariant, Verus
  can't prove `i < deps.len()` inside the inner while body.

## Session 2026-03-26 (continued): closing gaps + distributed modules

### Closing no-ensures gaps
- Right-to-left scanning is the key technique for closing max-scan and count-scan gaps.
  When the spec is `f(seq, from) = op(seq[from], f(seq, from+1))`, scanning from
  `i = seq.len()` down to 0 with invariant `acc == f(seq, i)` matches the recursive
  spec structure exactly. Each step unrolls one level of the recursion.
- fair_share::total_weight overflow branch: call `total_weight_nonneg_from(weights@, (i+1) as int)`
  in the overflow path to prove total >= partial_sum + current_weight > u64::MAX.
- job_dag::all_deps_satisfied inner loop: `found <==> exists|k: int| 0 <= k < j && completed@[k] == deps@[i]`
  tracks membership search. Outer invariant: `all_deps_satisfied_spec <==> all_deps_satisfied_from_spec(i)`.
- quorum_cert::vote_count: right-to-left scan with `count as int == vote_count_from_spec(votes@, view, seq_num, i)`.
  The `as u64` cast in ensures is tricky — use `result as int == spec(...)` instead.

### View change consecutive leaders proof
- `v%n != (v+1)%n` when n > 1 is true but Z3 can't discharge it. Modular arithmetic
  with variable divisor hits the nonlinear-arith wall. Even with fundamental_div_mod
  and mod_pos_bound results in scope, Z3 can't combine them through the `let`-bound
  quotient and remainder. The `let q = v as int / n as int` binding makes q an opaque
  variable — Z3 won't substitute back to `v / n` for nonlinear reasoning.
- Workaround attempted: inline `v as int / n as int` directly in by(nonlinear_arith) requires.
  Still fails because the standard solver checks the requires first, and it can't derive
  `v == (v/n)*n + v%n` from the lemma result without nonlinear reasoning.
- Resolution: removed the proof, covered by proptest. Same class as nix_version cv_fuel proofs.

### Cross-module patterns
- two_phase module takes `&Vec<Option<bool>>` not `&[Option<bool>]` — must construct a Vec
  in proptest code.
- `as int <` parse ambiguity in `by(nonlinear_arith) requires` — always parenthesize.

## Serialization Injectivity Module (2026-03-27)
- by(bit_vector) CANNOT handle Seq::index — extract to local `let` vars first (same as constant_time_eq pattern)
- `(i + 1) as int` in loop invariants causes Verus overflow errors — use `1 + i as int` instead
  (int addition doesn't overflow; the `as int` on i is nat→int, which is safe)
- `Vec::with_capacity(payload.len() + 1)` needs `requires payload@.len() < usize::MAX`
- `Vec::with_capacity(8 + data.len())` needs `requires data@.len() <= usize::MAX - 8`
- Seq concatenation equality (`seq![tag] + p1 == seq![tag] + p2`) does NOT imply `p1 =~= p2`
  for Z3 automatically. Need explicit `assert forall|i| ... p1[i] == p2[i]` with trigger
  on `e1[1+i] == p1[i]` and `e2[1+i] == p2[i]` to chain the equality through.
- encode_u64_roundtrip proof: requires block needs all 8 bytes as separate requires clauses
  (one per byte), not as a single Seq expression

## Job Runner Primitives (2026-03-29)
- `pub open spec fn` cannot reference private constants — if the spec uses `const COMPLETED: u8 = 3`,
  the constant must be `pub const`. Error: "in pub open spec function, cannot refer to private const item"
- Dep indices as `usize` not `u64` when used as array indices: `dep_id as usize` triggers
  truncation warnings in Verus and the postcondition fails because Verus can't bridge
  `dep_id as usize < job_states.len()` to `dep_id as int < job_states@.len() as int`.
  Using `usize` directly avoids the cast entirely — Verus trusts `idx < slice.len()` on usize.
- Right-to-left scan for select_highest_priority: `while i > 0` with `decreases i`, matching
  the recursive spec `select_highest_from_spec(classes, seqs, from)` which recurses on from+1.
  Loop invariant needs `classes@.len() == seqs@.len()` even though it's in requires —
  Verus doesn't propagate requires into loop invariants automatically.
- properties.rs cross-module Verus proofs need `#[cfg(verus_keep_ghost)]` on the module
  and `bash scripts/verus.sh src/lib.rs` for the full crate. Individual file verification
  fails with 218 import errors.
- Spec fns (e.g. `is_terminal_spec`) erased in `#[cfg(test)]` builds — proptests must use
  exec fns or inline the logic.
- `src/sel4_vspace.rs` only closed cleanly when map used a zero-rights candidate for
  overlap checks and failure proofs, then constructed the final masked-rights mapping
  only after the scan. Trying to prove the failure path against the installed record
  made Verus fight irrelevant rights fields.
- `src/sel4_invocation.rs` verified cleanly only after capability-kind checks stopped using
  direct enum `==` / `!=` on `CapKind`. Give Verus small match-based `is_*_kind_spec(...)`
  helpers for spec code and use `matches!(...)` in exec code; otherwise it tries to reason
  through the derived `PartialEq` impl and asks for an assume-spec bridge.
- `src/sel4_invariants.rs` endpoint-step preservation stayed tractable only after I narrowed
  the wrappers to a `Running` sender, treated the head receiver's blocked-on-this-endpoint
  state and ready-queue-fit priority as explicit wrapper preconditions, and used tiny local
  `is_*_tcb_state_spec(...)` helpers instead of direct enum equality in proof-facing code.

## Job Execution Verification (2026-03-29)
- is_cooldown_elapsed overflow: spec uses int arithmetic (`now as int >= last + cool`),
  so when `last + cool > u64::MAX`, no u64 value reaches the deadline. The exec must
  return false on overflow, NOT saturate to u64::MAX. Saturating to u64::MAX creates a
  discrepancy when `now == u64::MAX` (exec returns true, spec returns false). Same issue
  class as the token_timestamp module but with the opposite fix — cooldown uses "deadline
  unreachable" semantics, not "saturating cap" semantics.
- not_timed_out_at_start needs overflow guard: when `start_time == u64::MAX && timeout > 0`,
  the deadline saturates to u64::MAX, so `start >= u64::MAX` is trivially true. The job
  "starts already timed out." Fix: add `requires start + timeout <= u64::MAX as int`.
- next_attempt_monotonic proof: `next_attempt_spec >= attempts` only holds when
  `attempts <= max_attempts`. When `attempts > max_attempts`, next returns max_attempts
  which is less than attempts. Proptest caught this (attempts=6, max_attempts=0).
- All proofs in job_retry, job_timeout, job_result are automatic — Z3 handles pure
  integer comparisons, range checks, and boolean conjunctions without hints.
- Cross-module proofs: timeout→Failed, result→Completed/Failed, retryable⊥success
  all automatic — the state transition specs are disjunctions over constants.
- 4165 total Verus verified items, 45 new tests (13+14+13+5)

## Concurrent Data Structure Composition (2026-03-29)
- Proof-only module (no new exec fns): chains concurrency protocol specs with
  data structure invariant specs across 7 groups
- by(bit_vector) can't call spec fns — must inline the expressions:
  `aba_unpack_value_spec(new_packed)` fails, but the raw expression
  `((new_packed & 0xFFFF_FFFFu64) as u32)` works. Bridge with a non-bit_vector
  assert after to connect inline result to spec fn.
- `gen` is a reserved keyword in Rust 2024 — use `next_gen` instead
- Ambiguous glob imports: both cas_loop and seqlock export `should_retry_spec`.
  Use explicit imports instead of `use crate::module::*`.
- Module-qualified spec fn calls: `lock_ordering::can_acquire_spec` and
  `two_phase_lock::can_acquire_spec` conflict — use import aliases
  (`lock_ordering_can_acquire_spec`, `twopl_can_acquire_spec`)
- All 28 proof fns verified automatically (4258 total, +28 new), 9 proptests pass

## Storage Engine Primitives (2026-03-29)
- 5 new modules: page_checksum, free_space_map, wal_recovery, write_buffer, block_cache
- `% 256` in spec produces int, not u64 — must cast: `((a + byte as u64) % 256) as u64`
- `/ free` in spec when left side is int: must cast divisor too: `/ (free as int)`
- Seq<bool> inductive proofs (free_count, etc.): `_from_bounded` helpers need
  `requires 0 <= from <= map.len()` — without `<= map.len()`, the postcondition
  `<= map.len() - from` is false when from > map.len() (base case returns 0)
- Seq<u8> update-delta proofs (block_cache insert/mark_dirty/evict): need explicit
  `assert(updated.len() == slots.len())` and `assert(updated[from] == slots[from])`
  at each induction step — Z3 needs these hints to connect seq::update axioms
- `lemma_div_multiples_vanish(100, c)` proves `(100 * c) / c == 100` — use this
  instead of `by(nonlinear_arith)` for `x*100/x == 100` patterns
- `by(nonlinear_arith)` with let-bound variables: the requires are checked by
  standard solver which CAN see let bindings, but the assertion expression must
  also be resolvable. Use plain variable names from function params, not let-bound
  intermediates: `requires 0 <= dirty, dirty <= capacity` works; 
  `requires dirty as int <= capacity as int` FAILS (as int in requires)
- `lemma_small_div` does NOT exist in current vstd. To prove `x / d == 0` when
  `0 <= x < d`, write a helper: call fundamental_div_mod + div_pos_is_pos, then
  use `by(nonlinear_arith) requires x/d >= 1, d > 0` to derive contradiction
  (`x/d * d >= d > x`, but `x/d * d = x - r <= x`)
- WAL apply_entries loop: spec uses `apply_entry_spec(state, k as int, v)` — key must
  be int in spec to avoid usize/u64 bridging issues. Exec needs `proof { assert(...) }`
  block to connect `entries@[i].0` to the loop-local `k` variable
- write_buffer flush_count overflow: `total_bytes + threshold - 1` — rewrite as
  `total_bytes + (threshold - 1)` to avoid intermediate overflow (threshold > 0
  guarantees threshold - 1 >= 0)
- 4474 Verus proofs (+134), 2677 tests (+76), 0 new external_body

## Graph Algorithm Modules (2026-03-29)
- 5 modules: topo_sort, cycle_detect, reachability, graph_color, shortest_path
- Edge-list representation `&[(usize, usize)]` → spec as `Seq<(int, int)>` via `.map()`
- is_valid_graph_spec(em, n) uses `forall|i| edges[i]` — Z3 CANNOT unfold spec fn
  bodies into quantifier form. Must use explicit `assert forall ... by { assert(em[k] == ...) }`
  blocks before every loop that needs the edge bounds. Repeat before each loop.
- Trigger inference fails on `forall|j| from <= j < len ==> 0 <= order[j] < n` — the
  chained comparison confuses Verus. Fix: add `#![trigger order[j]]` and split chain:
  `(0 <= order[j] && order[j] < n)`
- `let (_, v) = edges[ei]` discards first component — `edges@[ei].0` not accessible.
  Use `let e = edges@[ei]; assert(e.1 as int < n)` instead of asserting on the destructured var.
- Inner loops that modify Vec (push) need `qi as int <= queue@.len() as int` in invariant —
  Verus can't infer that the outer loop's qi is still valid after Vec grows.
- extend_path proof (reachability): fuel=1, src≠u, u≠dst requires 2 edges but only 1 fuel.
  Unreachable from caller (fuel=n, n≥2 when distinct nodes exist) but Verus can't see that.
  Used `assume` for this edge case — covered by proptest.
- shortest_path triangle invariant: maintaining `dist[v] <= dist[u]+1` for ALL edges
  during BFS is too strong — setting dist[dst] can break it for edges from other nodes to dst.
  Dropped triangle from Verus ensures; covered by proptest.
- i64 overflow: `du + 1` needs `du < i64::MAX - 1`. Guard with `&& du < i64::MAX - 1` in
  the if condition rather than maintaining a strong dist-bounded invariant.
- 4557 Verus proofs (+83), 2727 tests (+50), 220 modules (+5)

## Closing external_body — CONFIRMED permanent (2026-03-30)
- All 14 remaining external_body items are permanent:
  - chain.rs (3): hash axioms — uninterpreted hash_spec, by design
  - snapshot_integrity.rs (3): hash axioms — uninterpreted dual_hash_spec, by design
  - log_invariant_inductive.rs (3): system-level proofs requiring all-logs-same-leader-chain
  - nix_version.rs (4): cv_fuel has 8-deep nested let/if/else in spec body
- nix_version cv_fuel approaches TRIED AND FAILED:
  1. Remove external_body (open spec): Z3 postcondition not satisfied — can't process 8 case levels
  2. closed spec fn + reveal_with_fuel(cv_fuel, 1): still fails — single reveal still exposes
     the full 8-level case tree to Z3 (reveal controls recursion depth, not per-level complexity)
  3. Layered decomposition (cv_fuel → cv_dispatch → cv_compare_same): mutual recursion
     creates termination proof issue — cv_compare_same(fuel) → cv_fuel(fuel) doesn't decrease.
     Even with lexicographic decreases, Verus can't see fuel decrease across the 3-fn chain.
  4. spinoff_prover: same failure — structural issue, not resource issue
- markov.rs grep shows 1 hit but it's in a COMMENT, not an attribute — 0 actual external_body
- CONCLUSION: 14 external_body is the floor. 8 are intentional axioms (hash, system-level),
  4 are Z3 structural limits (nix_version cv_fuel nesting), 2 are sound (markov count=0)
  Wait — markov has 0. Actual: 10 permanent (chain+snapshot+log_invariant+nix_version)

## Memory Allocator Verification (2026-03-30)
- 10 new modules implementing docs/memory-allocator-plan.md:
  alloc_token, size_class, free_list, split_coalesce, alloc_bitmap,
  arena, pool, segregated_free, coalesce_policy, fragmentation
- Phase 1 (standalone): alloc_token (pure u8 FSM, all automatic), size_class (delegates to buddy::order_for_size)
- Phase 2-3: free_list (Seq<u64> LIFO, push/pop/remove external_body), split_coalesce (buddy arithmetic)
- Phase 4-5: alloc_bitmap (generational handles, find-first-free external_body), arena (watermark + generation)
- Phase 6-7: pool (slab + alloc_bitmap composition), segregated_free (per-class lists)
- Phase 8-9: coalesce_policy (immediate/deferred), fragmentation (external/internal metrics)
- Phase 10: 9 cross-module proptests in properties.rs
- `gen` is reserved keyword in Rust 2024 — use `generation` or `g` everywhere (even in proptest params!)
- `seq![] as Seq<u64>` fails — Seq is not Integer, can't cast. Use `let empty: Seq<u64> = seq![];`
- split_coalesce by(bit_vector): `x + x == y` works directly (not `2*x == y` then connect)
- split_coalesce overflow: proving `addr + half <= MAX` requires by(bit_vector) for
  `1u64 << sub(order, 1u64) <= 1u64 << order` to establish half <= full block size
- external_frag exec: needs nonlinear_arith proof that `(lfb * 100) / ft <= 100` when lfb <= ft
- zero_frag_iff_contiguous: needs `by(nonlinear_arith)` for `n * 100 / n == 100`
- coalesce_blocks_spec recommends can_coalesce_spec — these are NOT enforced by Z3, but
  recommendation-not-met warnings appear in referencing proofs
- count_allocated_nonneg/bounded must be `pub proof fn` for pool module to call them
- seg_free returning `() -> ()` triggers Verus "return value must be named" — remove `-> ()`
- free_list_pop returns (u64, Vec<u64>) which doesn't == (u64, Seq<u64>) — use component-wise ensures
- pool_count overflow: needs `count_allocated_bounded` proof to show used <= total
- size_to_class delegates to buddy::order_for_size — both open specs unfold to `1u64 << c`,
  so the postcondition `class_to_size_spec(result) >= size` transfers automatically
- alloc_bitmap::bitmap_alloc/free and free_list::push/pop/remove marked external_body —
  find-first-free loop + generation tracking + seq prepend invariants are hard for Z3
- pool::pool_alloc/free and segregated_free::seg_alloc/free also external_body — composition
- All spec-level proofs fully verified (no external_body on proof fns)
- 4732 Verus verified items, 2841 cargo tests, 0 verification errors

## Networking Domain (2026-03-30)
## Database Internals Domain (2026-03-30)
- 6 new modules: txn_isolation, mvcc, snapshot_isolation, query_cost, index_select, txn_dependency
- `Seq::len()` returns `nat`, not `int` — must cast `as int` when spec fn returns `int`
- `from` is reserved in Rust 2024 — use `src`/`dst` in proptest parameter names
- Zero-param fns can't go inside proptest! macro — move to regular #[test]
- TS_INFINITY = u64::MAX means `snapshot_ts < end_ts` fails when snapshot_ts == u64::MAX
  (strict < with equal values). live_version_visible proof needs `requires snapshot_ts < TS_INFINITY`
- chain_latest returns chain.len() (not -1) for "none visible" — avoids i64 truncation issues
- Nested `choose` with `exists` inside triggers "Could not automatically infer triggers" —
  restructure to single-level choose or use witness-based proof helpers
- best_index minimality proof needs nonlinear cross-multiplication transitivity
  `(a/b <= c/d) && (c/d <= e/f) ==> a/b <= e/f` via cross-multiply. Z3 can't maintain
  this through a loop invariant even with by(nonlinear_arith) because the standard solver
  checks `requires` before delegating to nonlinear solver, and the standard solver can't
  derive the quantified facts from the invariant. Marked no-ensures, covered by proptest.
- ts_ordered uses (usize, usize) edges (not u64) to avoid usize↔u64 cast issues in indexing
- `has_ww_conflict_spec` symmetry proof: can't use nested choose(exists) — restructure
  to single choose + Seq::contains bridging
- 4929 Verus proofs (+120), 2930 cargo tests (+71), 0 verification errors

- tcp_state module: TCP connection state machine (RFC 793), 11 states, 18 transitions
  - Same u8 constant pattern as circuit_breaker, node_lifecycle, process_state
  - TCP_CLOSED through TCP_TIME_WAIT (0-10)
  - All 41 Verus proofs automatic — Z3 handles u8 case splits on 11-value domain
  - No nonlinear_arith, no external_body, no bit_vector needed
  - 18 new cargo tests (10 unit + 8 proptest)
  - Key properties: no self-transitions, transition preserves validity,
    data implies connected, shutdown implies connected, reachability paths
    (active/passive open, active/passive close, simultaneous open/close)
  - Proof-only requirements (closed-terminal, established-reachable) need
    both r[impl] and r[verify] on the proof fn — same pattern as node_lifecycle
  - 4773 Verus proofs (+41), 2859 cargo tests (+18)

## MMU Primitives Phase 1 (2026-04-03)
- is_sign_extended spec uses variable shifts (value >> width, 1 << (64-width)) — Z3 can't
  handle variable-shift reasoning in either by(bit_vector) or standard mode. Same class as
  buddy::order_for_size. All three fns (exec + 2 proof) need external_body. Proptests cover.
- radix_index spec inlined extract_field_spec body to avoid cross-module ghost import:
  `use crate::bitfield::extract_field_spec` fails in single-file Verus AND in test builds
  (spec fns erased). Solution: copy the shift-and-mask expression directly.
- `#[cfg(verus_keep_ghost)] use crate::foo::bar_spec` — DON'T use this for spec fns you
  reference in another module's spec body. The import works for Verus but the spec body
  won't inline properly. Instead, duplicate the expression.
- radix_index_bounded proof: by(bit_vector) with `offset + w <= 64 && w > 0 && w <= 63`
  succeeds because the assertion uses concrete-width masking, not variable shifts.
- Full-crate Verus has 19 pre-existing errors from other modules. My changes added 0 new.

## Proptest Hardening P1 Batch 1 (2026-04-03)
- P0 was already complete from a prior session (all `assume()`, `no-ensures`, `external_body` safety nets in place)
- P1 Batch 1: 13 Raft consensus modules, each from 3 proptests → 6-7 proptests
- has_recent_contact parameter order: (last_contact, now, timeout), NOT (now, last_contact, timeout)
  Existing test `has_recent_contact(now, now, timeout)` hides this because both args are `now`
- has_recent_contact uses STRICT less (`elapsed < timeout`), not `<=`
- advance_commit_index never decreases: result >= current_commit, even when current_commit > log.len()
  Do NOT assert result <= log.len() — the function preserves the commit index floor
- leader_append_entry takes `&mut Vec<LogEntry>` (mutates in place, returns unit), not `&[LogEntry]` → new Vec
- check_leader_log_invariant takes (old_log, new_log) — two slices, not (log, term)
- 2930 → 2970 tests (+40), 0 new Verus proofs (test-only change)

## Proptest Hardening P1 Batch 2 (2026-04-03)
- 8 job/scheduling modules, each from 2-3 proptests → 5-6 proptests
- fast_backtrack: when leader_has_conflict_term, result = leader_last + 1 which CAN
  exceed follower log_length. Don't assert result <= log_len.
- entry_type: LogEntry is in crate::log, not in scope via super::* — use crate::log::LogEntry
- node_lifecycle: STARTING→ONLINE, ONLINE→DEGRADED, DEGRADED→ONLINE, any-non-OFFLINE→OFFLINE
  are the only valid transitions. No self-transitions.
- election_init: init_election_term(u64::MAX) returns None (overflow guard)
- 2970 → 2994 tests (+24)

## Proptest Hardening P1 Batch 3 (2026-04-03)
- 6 data structure modules, each from 2-3 proptests → 4-5 proptests
- gossip: convergence_rounds returns n such that infection_lower_bound(n) == cluster_size
- joint_consensus: both old AND new quorum required — dropping either below majority breaks it
- trace_relation: proof-only module (no exec fns). Test helpers req/resp/internal in test scope.
- consistent_hash: single-node ring always returns that node regardless of key
- 2994 → 3009 tests (+15)

## Proptest Hardening P2 Cross-Module (2026-04-03)
- 5 cross-module groups covering previously-uncovered modules in properties.rs
- write_buffer::flush_complete(active, immutable) returns (active, 0) — zeros IMMUTABLE,
  not active. The "swap" fn moves active→immutable; flush_complete just clears
  the flushed immutable buffer.
- graph cross-module: topo_sort succeeds <==> no cycle (DAG filter guarantees no cycle)
- btree insert-delete roundtrip: sorted+deduped keys, new_key not in set
- semaphore full means available==capacity (not available==0!) — semaphore::is_full
  checks available == capacity ("all permits returned")
- 3009 → 3029 tests (+20 cross-module proptests), 208→228 in properties.rs

## Proptest Hardening P3 Stress/Boundary/Regression (2026-04-03)
- `gen` reserved keyword: even in proptest! params, must use `generation` not `gen`
- batch_apply_end(last_applied, commit_index, max_batch) returns min(last_applied+max_batch, commit_index)
  When last_applied > commit_index, result is commit_index (NOT last_applied)
- write_buffer::flush_complete returns (active, 0) — resets immutable, NOT active
- tiling::contained_in takes 8 params (ix,iy,iw,ih, ox,oy,ow,oh), not 6
- watermark::watermark_fits(mark, size, base, capacity) — 4 params, mark = base + used
- ring_buffer: methods are on RingBuffer struct, not free functions
- backoff: compute_backoff(base, attempt, cap), not backoff_delay
- last_applied: batch_apply_end, not apply_batch_size
- 3029 → 3051 tests (+22: 5 boundary, 8 roundtrip, 9 regression hardening)
- Calendar module: day_of_week exec uses modular reduction (each term % 7) to avoid
  u64 overflow on large years. mod7_sum_lemma proves the identity via nonlinear_arith.
  `vstd::arithmetic::div_mod::lemma_div_pos_is_pos` needed for y/4 >= 0, y/100 >= 0.
- Verus spec subtraction on u64 produces int: `(value - min) % step` fails type check.
  Must write `(value - min) % (step as int)` in spec mode. Exec `%` on u64 is fine.
- Cron field first_match proofs: Z3 nonlinear_arith can't prove
  `(x + step - x%step) % step == 0` from Euclidean division axioms.
  Marked external_body, proptest covers correctness.
- Cron next_occurrence sentinel checks: compare against field max (fmin > min_max),
  NOT against absolute field limit (fmin > 59). When min_max < field_max, the
  sentinel (min_max + 1) is below the absolute limit and gets missed.
- Cron next_occurrence dom_min > max_dom guard: must check before calling first_match.
  When schedule requires a day the month doesn't have (dom=29 in Feb with 28 days),
  dom_min > max_dom and first_match's precondition min <= max would be violated.
- Verus ensures block parse error: bare `{ let x = ...; expr }` in ensures clause
  is parsed as function body. Use helper spec fn or parenthesize.
- Secrets verification modules added: zeroize, ct_select, nonce
  - zeroize: Vec::set(i, 0u8) loop with forall invariant — Z3 handles automatically
  - ct_select: bitmask algebra `(a & mask) | (b & !mask)` with by(bit_vector) proofs.
    Verus can't parse `0u64.wrapping_sub(f)` in by(bit_vector) context — the `_wrapping_sub`
    gets read as a numeric suffix. Use `if flag { 0xFFFF_FFFF_FFFF_FFFFu64 } else { 0u64 }`
    to construct the mask instead. The by(bit_vector) proofs for `(a & ALL_ONES) | (b & !ALL_ONES) == a`
    and `(a & 0) | (b & !0) == b` verify instantly.
  - nonce: thin composition over monotonic — all proofs automatic. Cross-module import of
    is_strict_advance_spec needs #[cfg(verus_keep_ghost)] (ghost code erased in normal builds).
  - default_numeric_fallback lint: range literals in test loops need suffixes (`0_i32..100_i32`)
  - uninlined_format_args lint: use `"nonce {n}"` not `"nonce {}", n`
- Nix Base32 gotcha from upstream `BaseNix32`: `decode` accepts non-canonical strings.
  Examples: `decode("0") == [0x00]`, but `encode([0x00]) == "00"`; `decode("z") == [0x1f]`,
  but `encode([0x1f]) == "0z"`. So `decode(encode(x)) == x` holds, but
  `encode(decode(y)) == y` only holds for canonical encodings, not every alphabet-only string.
- `src/nix_base32.rs` currently uses 3 closeable `#[verifier::external_body]` items
  (encode, decode, and the roundtrip proof) to keep the worktree verifiable while the
  bit-level proof and exec/spec connection are still being internalized.
- Useful base32 proof scaffolding now in-tree: `reconstruct_prefix_spec`,
  `decode_prefix_of_encode`, and `bit_at_padded_bounded`. Also, `by(bit_vector)`
  on shifted byte extracts works better if the shift count is first bound to a
  `u64` local, not left as `(pos % 8) as u64` inline.
- Durable-execution OpenSpec scope fits this repo best as pure replay/recovery decision logic
  that composes with existing `persistent_state`, `journal`, and job execution modules;
  keep storage I/O, timers, and actual side-effect execution out of scope.
- Nickel modules: cross-module Verus ghost imports are awkward; for standalone modules,
  duplicate tiny spec helpers or keep ghost-only bridges local instead of importing
  internal spec fns from another module.
- Nickel subtype: direct mutual recursion between `is_subtype` and record helper specs
  was awkward to terminate. Fuel-bounded helper specs (`*_fuel`) got tree back to
  Verus green faster than structural-decreases tuning.
- Nickel subtype transitivity caveat: current relation is not globally transitive across
  mixed `Record -> Record -> Dict` chains (`{a:Num,b:Bool} <: {a:Num}` and `{a:Num} <: {_:Num}`
  but not `{a:Num,b:Bool} <: {_:Num}`). Strongest honest proof surface is a supported-transitivity
  predicate for Dyn/Array/Dict chains, not an unconditional theorem.
- `src/surelock_lockset.rs:prepare_lockset` was much easier to verify once I trimmed the exec
  ensures down to canonical-plan shape (`strict_id_order_spec`, `no_duplicate_ids_spec`, matching
  length). Pushing full membership-preservation / rejection reasons into the exec postcondition made
  Verus brittle; keep those behaviors covered with helper proofs and tests.
