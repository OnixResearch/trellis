# README Count Derivation

README headline after landing:
- `8277 Verus proofs`
- `4106 tests`
- `1683 requirements`
- `368 modules`
- `19 trusted boundaries`

Pinned archive sources of truth:
- `verus-lib.txt`
- `cargo-test-full.txt`
- `tracey-uncovered.txt`
- `tracey-untested.txt`
- `src-lib-pub-mod-lines.txt`
- `readme-count-commands.txt`

## Proof count

Source of truth:
- `verification results:: 8277 verified, 0 errors`
- captured in `verus-lib.txt`

## Test count

Source of truth:
- `cargo-test-full.txt` reports `4093` library/property tests and `13` integration tests
- README total is `4106`

## Requirement count

Source of truth:
- `tracey-uncovered.txt`: `0 uncovered out of 1683 rules`
- `tracey-untested.txt`: `0 untested (impl but no verify) out of 1683 rules`

README uses the live Tracey rule count rather than the raw markdown marker
count.

## Module count

Source of truth:
- `src-lib-pub-mod-lines.txt` contains `368` `pub mod` lines from `src/lib.rs`

## Trusted-boundary count

This change does not touch the live trust inventory. README therefore keeps the
existing `19` trusted boundaries (`16` permanent `#[verifier::external_body]`
items + `3` closeable `assume_specification[...]` bridges), matching the live
inventory tracked in `openspec/specs/verification-gaps/spec.md` and `AGENTS.md`.
