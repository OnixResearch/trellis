## Why

Trellis retains Verus `0.2026.03.22.5e66329` while Octet publishes `0.2026.04.03.21dfcd2`. Matching Rust versions and formatter packages do not establish proof compatibility. Promotion requires an exact local rerun over Trellis plus a policy-aware audit of all five pinned upstream corpus rows, with deterministic blockers preserved rather than edited away.

## What Changes

- Add a typed upgrade-trial matrix binding the current and candidate verifier identities, source snapshots, native probe commands, reference-only rows, limits, and non-claims.
- Execute Trellis's complete proof crate through the Octet production package.
- Probe verified IronKV, verified-memory-allocator, verified-node-replication, the SOSP artifact's available Verus microcases, and VerusBelt according to their actual tool and authority boundaries.
- Emit a deterministic promotion or blocked receipt and retain the current verifier unless every required native probe passes.

## Impact

- **Files**: `flake.nix`, typed trial configuration and fixtures under `upstream-verus-corpus/`, README documentation, saved Cairn evidence, and this change.
- **Testing**: current baseline, candidate Trellis proof run, five-row corpus trial, positive and negative trial fixtures, receipt freshness, existing corpus/formatter checks, and Cairn gates.
