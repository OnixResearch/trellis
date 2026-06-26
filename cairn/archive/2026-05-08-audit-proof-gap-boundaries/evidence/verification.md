# Verification evidence

Status: captured

## Commands / Oracles

- command: `cargo test --quiet proof_gap_boundary_audit`
- command: `openspec validate audit-proof-gap-boundaries --strict --json`
- command: `openspec validate verification-gaps --strict --json`
- command: `./scripts/verus.sh src/lib.rs`
- command: `tracey query uncovered`
- command: `tracey query untested`
- command: `./scripts/verify.sh`
- oracle: source-derived marker scan in `proof-gap-inventory.md` reports unchanged counts and zero replacement weakened markers.

## Outcomes

- result: pass `cargo test --quiet proof_gap_boundary_audit` — no matching unit tests; crate test harness completed successfully for the filter.
- result: pass `openspec validate audit-proof-gap-boundaries --strict --json` —         "items": 1,         "passed": 1,         "failed": 0       }     }   },   "version": "1.0" }
- result: pass `openspec validate verification-gaps --strict --json` —         "items": 1,         "passed": 1,         "failed": 0       }     }   },   "version": "1.0" }
- result: pass `./scripts/verus.sh src/lib.rs` —       pattern specified by one of the quantifier's triggers.)     --> src/nix_base32.rs:2537:9      | 2537 |         assert forall|k: int| 0 <= k < len as int implies result@[k] == extend_to_spec(acc@, len as nat)[k] by {      |         ^^^^^^  verification results:: 9811 verified, 0 errors warning: 18 warnings emitted
- result: pass `tracey query uncovered` — verified-logic/rust: 0 uncovered out of 1870 rules  --- → Run `tracey query rule <rule-id>` → Run `tracey query uncovered --prefix <prefix>`
- result: pass `tracey query untested` — verified-logic/rust: 0 untested (impl but no verify) out of 1870 rules  --- → Run `tracey query rule <rule-id>` → Run `tracey query untested --prefix <prefix>`
- result: pass `./scripts/verify.sh` — verified-logic/rust: 0 untested (impl but no verify) out of 1870 rules  --- → Run `tracey query rule <rule-id>` → Run `tracey query untested --prefix <prefix>` ✓ no untested requirements  === ALL CHECKS PASSED ===
- result: pass source audit — unchanged `16` external-body sites and zero replacement weakened markers in `evidence/proof-gap-inventory.md`.
