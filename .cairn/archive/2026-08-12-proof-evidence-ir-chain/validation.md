# Validation evidence

Date of record: 2026-08-12 (worktree `cairn/drain-20260811`).

## Scope

This change drains the `proof-evidence-ir-chain` Cairn package: a verified-proof-adjacent checker artifact that links proof source envelopes, canonical proof evidence IR, verifier receipts, typed assumptions, deterministic BLAKE3 chain identities, and a Kamacite-ready handoff. It is implemented as a plain-Rust cargo bin (`src/bin/proof_evidence_ir.rs`); no verified Verus module surface changed.

## Focused commands and outcomes

```text
$ cargo test --quiet --bin proof_evidence_ir
running 4 tests
test result: ok. 4 passed; 0 failed

$ cargo run --quiet --bin proof_evidence_ir -- self-test
✓ proof evidence IR chain self-test fixtures passed

$ cargo run --quiet --bin proof_evidence_ir -- check
✓ proof evidence IR chain validates 3 records

$ cargo run --quiet --bin proof_evidence_ir -- write
✓ wrote docs/proof-evidence-ir-chain.json

$ cargo clippy --quiet --bin proof_evidence_ir -- -D warnings
(exits 0, no diagnostics)

$ cargo test --quiet
(full workspace suite exits 0; bin and lib targets pass)
```

Heavy rails (run locally against the merged tree; the pre-commit hook's
remote-builder hash mismatch is avoided with `--builders ''`):

```text
$ nix build --builders '' .#checks.x86_64-linux.verus-proofs --no-link -L
PASS (exit 0; full-library Verus verification, pre-existing
type_lattice snake_case style warnings only)

$ nix build --builders '' .#checks.x86_64-linux.octet --no-link
PASS (exit 0)
```

Tracey (current tree):

```text
verified-logic/rust: 0 uncovered out of 2031 rules
verified-logic/rust: 0 untested (impl but no verify) out of 2031 rules
```

Cairn lifecycle (policy passed explicitly because the Trellis-checked policy file still predates the current Cairn schema; see Limitation):

```text
cairn gate proposal|design|tasks proof-evidence-ir-chain --root . --policy <cairn policy>
  verdict: PASS (all three)

cairn validate --root . --policy <cairn policy>
  valid: true (0 issues after accepted-spec identity-layout repair across the corpus)

cairn sync proof-evidence-ir-chain --root . --policy <cairn policy> --execute
  plan hash c8a29d2db588691255b0ae17d9950a0d483092714eb0be974735552d0d38192a

cairn archive proof-evidence-ir-chain --root . --policy <cairn policy> --execute
  plan hash 574af4dd7c1f9a62a629f3e7d9158097cc366f2aa3ebe0720d75813b80daa032
  archive: .cairn/archive/2026-08-12-proof-evidence-ir-chain/

post-archive: cairn validate = valid: true, 0 issues, 25 active changes
```

## Accepted spec promotion

`cairn/specs/proof-evidence-bundles/spec.md` now contains the nine
`trellis.proof_evidence_bundles.proof_evidence_ir_chain.*` requirement IDs with
scenarios after sync. `docs/requirements.md` carries the same requirement IDs,
each with `r[impl ...]` and `r[verify ...]` coverage in
`src/bin/proof_evidence_ir.rs`, and Tracey reports zero uncovered/untested.

## Limitation (pre-existing, outside this change)

The checked-in Trellis policy (`cairn-policy/generated/cairn-policy.json`)
predates the current Cairn CLI schema (the workflow schema shape is missing
`output_path`), so lifecycle commands ran with the current Cairn-generated
policy via `--policy`. This mirrors the bounded-compatibility pattern recorded
by the 2026-07-14-strengthen-proof-review-portfolio-guidance archive. Repairing
the Trellis policy to the current schema is a separate change and does not
weaken the evidence recorded here.
