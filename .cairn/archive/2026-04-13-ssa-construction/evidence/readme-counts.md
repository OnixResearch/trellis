# README headline count derivation

Current README headline:

- `6450 Verus proofs`
- `3517 tests`
- `1406 requirements`
- `301 modules`
- `22 trusted boundaries`

Derived inputs:

- modules = `301` from `evidence/src-lib-pub-mod-lines.txt`
- requirements = `1406` from `evidence/requirements-marker-lines.txt`
- trusted boundaries = `22` total = `15` external-body sites from
  `evidence/external-body-sites.txt` + `7` SSA `assume_specification[...]`
  bridges from `evidence/ssa-assume-spec-sites.txt`
- tests = `3517` from `evidence/cargo-test.txt`
- Verus proofs = `6450` from `evidence/verus-lib.txt`
