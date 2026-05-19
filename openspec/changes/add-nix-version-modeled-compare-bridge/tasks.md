## Phase 1: Finite Admission Surface

- [x] [serial] Define the finite modeled-input admission boundary for `src/nix_version.rs`, including maximum component/length assumptions needed for executable numeric parsing and explicit out-of-model rejection. ✅ completed: 2026-05-19T22:01:31Z
- [x] [depends:verification-gaps.nix-version-modeled-compare-bridge] Add `r[impl]`-tagged executable helpers for separator skipping, component-end discovery, numeric parsing, and lexicographic/component comparison over admitted inputs. ✅ completed: 2026-05-19T22:01:31Z
- [x] [depends:finite-admission] Add `compare_versions_modeled` that returns `Some(-1)`, `Some(0)`, or `Some(1)` exactly for admitted inputs and `None` for rejected/out-of-model inputs. ✅ completed: 2026-05-19T22:01:31Z

## Phase 2: Proofs and Tests

- [x] [depends:compare_versions_modeled] Add `r[verify]` proof coverage showing admitted modeled results match `compare_versions_spec` and stay in the allowed result set. ✅ completed: 2026-05-19T22:01:31Z
- [x] [depends:compare_versions_modeled] Add focused positive tests for representative numeric, alphabetic, separator, equality, and mixed-component admitted comparisons. ✅ completed: 2026-05-19T22:01:31Z
- [x] [depends:compare_versions_modeled] Add focused negative tests for out-of-model bytes or bounds that must return `None` without making an ordering claim. ✅ completed: 2026-05-19T22:01:31Z

## Phase 3: Inventory and Spec Sync

- [x] [depends:proofs-tests] Keep the original `src/nix_version.rs::compare_versions` trusted boundary visible in proof-gap inventory unless this slice actually removes it; add no replacement `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, `@verified-logic:exempt`, or new `#[verifier::external_body]` sites. ✅ completed: 2026-05-19T22:01:31Z
- [x] [depends:proofs-tests] Sync `docs/requirements.md` and `openspec/specs/verification-gaps/spec.md` wording if implementation details refine the admitted finite model or evidence language. ✅ completed: 2026-05-19T22:01:31Z

## Phase 4: Validation and Evidence

- [x] [serial] Run and capture focused evidence: `cargo test --quiet nix_version`, `./scripts/verus.sh src/lib.rs`, proof-gap inventory gate, Tracey uncovered/untested, and strict OpenSpec validation for this change. ✅ 2m (started: 2026-05-19T22:01:31Z → completed: 2026-05-19T22:03:23Z)
- [x] [serial] Run final full-tree verification with `./scripts/verify.sh`, reconcile any transient Tracey output, then sync/archive the change when all tasks are complete. ✅ 2m (started: 2026-05-19T22:03:23Z → completed: 2026-05-19T22:05:51Z)
