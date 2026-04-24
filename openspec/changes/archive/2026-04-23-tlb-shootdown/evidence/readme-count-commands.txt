# README count command provenance

```bash
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet \
  > openspec/changes/archive/2026-04-23-tlb-shootdown/evidence/cargo-test-full.txt 2>&1

./scripts/verus.sh src/lib.rs \
  > openspec/changes/archive/2026-04-23-tlb-shootdown/evidence/verus-lib.txt 2>&1

tracey query uncovered \
  > openspec/changes/archive/2026-04-23-tlb-shootdown/evidence/tracey-uncovered.txt 2>&1

tracey query untested \
  > openspec/changes/archive/2026-04-23-tlb-shootdown/evidence/tracey-untested.txt 2>&1

rg '^pub mod ' src/lib.rs \
  > openspec/changes/archive/2026-04-23-tlb-shootdown/evidence/src-lib-pub-mod-lines.txt
```

Trusted-boundary headline stayed unchanged at `19` for this change because
`src/tlb_shootdown.rs` adds no `#[verifier::external_body]` items and no
`assume_specification[...]` bridges; the existing live trust inventory remains
`16` permanent + `3` closeable per `openspec/specs/verification-gaps/spec.md`
and `AGENTS.md`.
