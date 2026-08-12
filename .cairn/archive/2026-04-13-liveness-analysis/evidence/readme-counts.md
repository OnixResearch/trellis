# README Count Derivation

README headline after landing:
- `6380 Verus proofs`
- `3474 tests`
- `1387 requirements`
- `296 modules`
- `15 trusted boundaries`

Archive-stable sources of truth are preserved in:
- `evidence/verus-lib.txt`
- `evidence/cargo-test.txt`
- `evidence/tracey-uncovered.txt`
- `evidence/tracey-untested.txt`
- `evidence/src-lib-pub-mod-lines.txt`
- `evidence/external-body-sites.txt`
- `evidence/requirements-marker-lines.txt`

Exact derivation commands and their original outputs are also preserved in:
- `evidence/readme-count-commands.txt`

The pinned evidence files above are the archive source of truth. The command
transcript is provenance only; rerunning those commands against a future live
worktree may drift.

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
- pinned raw marker listing: `evidence/requirements-marker-lines.txt`
- pinned raw marker count: `1410`
- README is meant to report the live Tracey rule count for the `verified-logic/rust`
  implementation, not the raw marker-line count in the markdown file
- Tracey is therefore the source of truth for the README requirement headline

## Module count

Pinned archive input:
- `evidence/src-lib-pub-mod-lines.txt`

Command/output provenance:
- `evidence/readme-count-commands.txt`

Chosen source of truth:
- `296` lines in `evidence/src-lib-pub-mod-lines.txt`

Derivation rule:
- count lines in `src/lib.rs` that start with `pub mod `

## Trusted-boundary count

Pinned archive input:
- `evidence/external-body-sites.txt`

Command/output provenance:
- `evidence/readme-count-commands.txt`

Chosen source of truth:
- `15` lines in `evidence/external-body-sites.txt`

Derivation rule:
- count `#[verifier::external_body]` occurrences under `src/`
