# Value-Transfer Baseline

Recorded before semantic implementation on 2026-08-12.

- Focused command: `cargo test choreography`
- Focused result: 30 passed, 0 failed.
- Proof command: `NIX_CONFIG='builders =' ./scripts/verus.sh src/lib.rs`
- Proof result: 10,720 verified, 0 errors.
- Proof-gap inventory: 16 `external_body` sites, 0 other admitted bridge markers.
- Tracey result: 0 uncovered and 0 untested out of 2,044 requirements.

The baseline already included the published `PayloadTag` newtype pilot. It did not include `ValueId`, ownership state, or owned-transfer syntax.
