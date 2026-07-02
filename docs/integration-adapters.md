# Integration Adapters

Integration adapters are thin deterministic shells over existing Trellis verified
primitives. They provide domain-shaped inputs and explicit accept/reject metadata
for downstream users who do not want to call every low-level predicate directly.

## Supported initial slice

| Adapter | Public function | Composed primitives | Supported subset | Non-goals |
|---|---|---|---|---|
| Consensus read admission | `integration_adapters::consensus_read_adapter` | `raft_consensus_core::raft_openraft_read_linearizer_admission_result` | finite OpenRaft-inspired read-index catch-up metadata | no network quorum, lease-clock, persistence, or OpenRaft conformance claim |
| UCAN authorization admission | `integration_adapters::ucan_authorization_adapter` | `ucan_token::validate_envelope` | finite UCAN envelope shape, temporal window, self-delegation, required fields, and caller-supplied signature verdict | no cryptographic signing/verification claim beyond the supplied signature verdict |
| Nix/Nickel closure policy admission | `integration_adapters::nix_closure_policy_adapter` | `nix_eval_closure_policy::nix_eval_closure_policy` | sorted finite attrsets, closure reachability, version/substitution normalization, and disjoint Nickel record merge side condition | no full Nix evaluator, store, flakes, or Nickel language conformance claim |
| Storage/WAL recovery admission | `integration_adapters::storage_wal_recovery_adapter` | `wal_recovery::{replay_range,replay_count,needs_recovery,can_append}` | finite checkpoint/WAL-end replay metadata and append-overflow rejection | no fsync, device, crash-timing, or storage-engine durability claim |

## Commands

Run the example:

```bash
cargo run --example integration_adapters
```

Run focused adapter tests:

```bash
cargo test --quiet integration_adapters
cargo clippy --all-targets -- -D warnings
```

The example and tests include positive and negative cases for every public
adapter. Unsupported external behavior is rejected or left as an explicit
non-goal instead of being accepted implicitly.
