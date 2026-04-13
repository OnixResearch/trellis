# README Count Derivation

README headline after landing:
- `6380 Verus proofs`
- `3474 tests`
- `1387 requirements`
- `296 modules`
- `15 trusted boundaries`

Exact derivation commands and outputs are preserved in:
- `evidence/readme-count-commands.txt`

## Proof count

Command/output source: `evidence/readme-count-commands.txt`

Chosen source of truth:
- `verification results:: 6380 verified, 0 errors`

Underlying transcript source:
- `evidence/verus-lib.txt`

## Test count

Command/output source: `evidence/readme-count-commands.txt`

Chosen source of truth:
- `test result: ok. 3474 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

Underlying transcript source:
- `evidence/cargo-test.txt`

## Requirement count

Command/output source: `evidence/readme-count-commands.txt`

Chosen source of truth:
- `verified-logic/rust: 0 uncovered out of 1387 rules`
- `verified-logic/rust: 0 untested (impl but no verify) out of 1387 rules`

Underlying transcript sources:
- `evidence/tracey-uncovered.txt`
- `evidence/tracey-untested.txt`

Why README uses `1387`, not the raw `docs/requirements.md` marker count:
- the raw shell count of lines starting with `r[` is `1410`
- README is meant to report the live Tracey rule count for the `verified-logic/rust`
  implementation, not the raw marker-line count in the markdown file
- Tracey is therefore the source of truth for the README requirement headline

## Module count

Command/output source: `evidence/readme-count-commands.txt`

Chosen source of truth:
- `modules=296`

Derivation rule:
- count lines in `src/lib.rs` that start with `pub mod `

## Trusted-boundary count

Command/output source: `evidence/readme-count-commands.txt`

Chosen source of truth:
- `15`

Derivation rule:
- count `#[verifier::external_body]` occurrences under `src/`
