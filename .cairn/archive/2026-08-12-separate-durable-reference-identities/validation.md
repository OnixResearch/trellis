# Validation evidence

Date of record: 2026-08-12 (worktree `drain-separate-durable-identities`, branch `cairn/drain-separate-durable-identities-20260812`).

## Scope

Drains the `separate-durable-reference-identities` Cairn package: the typed durable reference identity migration. `src/typed_durable_reference.rs` now models reference, value, type, codec, closure, and schema identities as distinct public structures with raw-`u64` fields, keeps malformed zero inputs representable, and preserves finite-metadata, closure, unpersist, and conversion-witness admission semantics. All comparisons occur within one identity domain via raw projection. Cross-domain substitution fails to compile (compile-fail doctests).

## Commands and outcomes

```text
$ VERUS_ROOT=<octet-production-verus> ./libexec/verus/rust_verify ... src/typed_durable_reference.rs
verification results:: 23 verified, 0 errors   (per-file baseline 17 verified before migration)

$ cargo test --quiet
(full workspace suite exits 0; module lib tests 5 passed, compile-fail doctests 3 passed)

$ cargo clippy --quiet --all-targets -- -D warnings
(exits 0)

$ nix build --builders '' .#checks.x86_64-linux.verus-proofs --no-link -L
PASS (exit 0; full-library Verus verification)

$ /home/brittonr/git/OnixResearch/cairn/result/bin/cairn validate --root . --policy <cairn policy>
valid: true (0 issues)

$ cairn gate proposal|design|tasks separate-durable-reference-identities --root . --policy <cairn policy>
verdict: PASS (all three)

$ cairn sync separate-durable-reference-identities --root . --policy <cairn policy> --execute
plan hash 1dbea1dc6919ca41f19d976d9e949f1251f520a8b18632872c2c79ff57dfe497

$ CAIRN_ARCHIVE_DATE=2026-08-12 cairn archive separate-durable-reference-identities --root . --policy <cairn policy> --execute
plan hash fb72f27c45b61b90f098c40b1275ed7f7c1b22540b925af175744520ba29372a
archive: .cairn/archive/2026-08-12-separate-durable-reference-identities/

post-archive: cairn validate = valid: true, 0 issues, 24 active changes
```

Tracey (current tree):

```text
verified-logic/rust: 0 uncovered out of 2038 rules
verified-logic/rust: 0 untested (impl but no verify) out of 2038 rules
```

## Accepted spec promotion

`cairn/specs/domain-primitives/spec.md` now contains the seven
`newtypes.durable-identities.*` requirement IDs and their scenarios after
sync. `docs/requirements.md` carries the same requirement IDs, each with
implementation and verification marker coverage in
`src/typed_durable_reference.rs`, and Tracey reports zero uncovered/untested.

## Corpus normalization prerequisite

The sync merge preflight exposed a stricter accepted-identity rule than
`cairn validate`: requirement and scenario headings that still carried inline
`[r[id]]` tokens produced duplicate identity counts. All 468 remaining inline
heading tokens in accepted specs were stripped to the canonical bare-ID layout
(a separate commit), after which sync unblocked and whole-tree validation
stayed green.

## Claim boundary

Domain separation and preserved finite-model admission behavior only. The
wrappers are not cryptographic digests, collision-resistance evidence,
persistence evidence, codec correctness, type-checker correctness,
dependency retrieval, or migration execution.
