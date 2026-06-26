# README Count Derivation

README headline after landing:
- `8305 Verus proofs`
- `4116 tests`
- `1687 requirements`
- `369 modules`
- `19 trusted boundaries`

Pinned sources of truth in this archive:
- `verus-lib.txt`
- `cargo-test-full.txt`
- `cargo-test.txt`
- `tracey-uncovered.txt`
- `tracey-untested.txt`
- `src-lib-pub-mod-lines.txt`
- `readme-count-commands.txt`

## Proof count

Source of truth:
- `verification results:: 8305 verified, 0 errors`
- captured in `verus-lib.txt`

## Test count

Source of truth:
- `cargo-test-full.txt` reports `4103` library/property tests and `13` integration tests
- README total is `4116`

## Requirement count

Source of truth:
- `tracey-uncovered.txt`: `0 uncovered out of 1687 rules`
- `tracey-untested.txt`: `0 untested (impl but no verify) out of 1687 rules`

## Module count

Source of truth:
- `src-lib-pub-mod-lines.txt` contains `369` `pub mod` lines from `src/lib.rs`

## Trusted-boundary count

This change does not touch the trust inventory. README therefore keeps the
existing `19` trusted boundaries (`16` permanent `#[verifier::external_body]`
items + `3` closeable `assume_specification[...]` bridges).
