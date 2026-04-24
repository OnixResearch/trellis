## Phase 1: Nameless core

- [x] Add `src/debruijn.rs` with `Term = Var | Abs | App`
- [x] Add ghost `SpecTerm` and `term_view`
- [x] Define `well_scoped_spec`, `shift_spec`, `subst_spec`, and `beta_reduce_spec`
- [x] Add executable `shift`, `subst`, and `beta_reduce` wrappers with explicit `Option` overflow handling
- [x] Export the module from `src/lib.rs`

## Phase 2: Proofs

- [x] Prove well-scoped variable validity/invalidity cases (`well_scoped_var_valid`, `well_scoped_var_invalid`)
- [x] Prove shift-by-zero identity (`shift_zero_identity`)
- [x] Prove same-cutoff shift composition (`shift_composition`)
- [x] Prove shift preserves well-scopedness (`shift_preserves_scope`)
- [x] Prove lift/substitute cancellation for the matching variable (`subst_var_cancels_lift`)
- [x] Prove top-level substitution preserves well-scopedness (`subst_preserves_scope` with `index = 0` consumer)
- [x] Prove beta reduction preserves scope for well-scoped redexes and returns `None` on non-redexes (`beta_reduce_preserves_scope`, `beta_reduce_non_redex_none`)

## Phase 3: Tests

- [x] Add positive unit tests for shift, substitution, and beta reduction (`test_shift_zero_identity_runtime`, `test_shift_under_binder`, `test_subst_top_cancels_lift_runtime`, `test_beta_reduce_identity`, `test_beta_reduce_nested_binder`)
- [x] Add negative unit tests for overflow and non-redex cases (`test_shift_overflow_returns_none`, `test_subst_overflow_returns_none`, `test_beta_reduce_non_redex_returns_none`)
- [x] Add proptests for shift identity and composition (`prop_shift_zero_identity`, `prop_shift_composition`)
- [x] Add proptests for runtime scope preservation through shift, substitution, and beta reduction on bounded well-scoped terms (`prop_shift_preserves_runtime_scope`, `prop_subst_preserves_runtime_scope`, `prop_beta_reduce_preserves_runtime_scope`)

## Phase 4: Spec sync

- [x] Sync `docs/requirements.md` with `r[debruijn.*]` requirements
- [x] Sync the active change spec to the landed nameless-core scope
- [x] Add the canonical spec at `openspec/specs/debruijn-substitution/spec.md`
- [x] Update README module tables for `debruijn`

## Phase 5: Validation and archive

- [x] Run `cargo test --quiet debruijn` and capture output (`evidence/cargo-test.txt`)
- [x] Run `./scripts/verus.sh src/debruijn.rs` and capture output (`evidence/verus-debruijn.txt`)
- [x] Run `openspec_gate` for proposal, design, and tasks and capture output (`evidence/openspec-*.txt`)
- [x] Archive the change under `openspec/changes/archive/` with evidence
