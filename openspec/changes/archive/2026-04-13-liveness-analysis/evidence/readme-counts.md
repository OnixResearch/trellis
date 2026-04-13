# README Count Derivation

README headline after landing:
- `6380 Verus proofs`
- `3474 tests`
- `1387 requirements`
- `296 modules`
- `15 trusted boundaries`

These counts were taken from the live tree after the liveness-analysis work landed.

## Proof count

Source: `evidence/verus-lib.txt`

```text
verification results:: 6380 verified, 0 errors
```

## Test count

Source: `evidence/cargo-test.txt`

```text
test result: ok. 3474 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Requirement count

Source: `evidence/tracey-uncovered.txt` and `evidence/tracey-untested.txt`

```text
verified-logic/rust: 0 uncovered out of 1387 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1387 rules
```

README uses the live Tracey rule count (`1387`), not a raw line count from
`docs/requirements.md`.

## Module count

Derived from `src/lib.rs` by counting `pub mod ` lines.

```text
modules=296
```

## Trusted-boundary count

Derived from the live source tree by counting `#[verifier::external_body]`.

```text
15
```
