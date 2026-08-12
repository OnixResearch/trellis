## Success Contract

The exact goal is to decide whether Trellis may replace its local Verus pin with Octet's production package. Valid promotion evidence requires the complete Trellis proof crate to pass under the candidate and every corpus row classified as a required native Verus probe to complete without verifier error, unsupported feature, timeout, or failed obligation. Reference-only and meta-artifact rows must be audited under their declared tool boundary but cannot be counted as Verus proof passes.

False completion includes source intake, formatting, upstream README claims, partial-file success, VerusBelt Rocq success, diagnostic-only microcases, matching version strings, or a candidate run that skips a required native probe. Allowed outcomes are `promote` and `blocked`; a reproducible blocker is successful completion of the trial but not successful verifier promotion.

## Portfolio Registry

| Family | Mechanism | Claim | State before execution |
|---|---|---|---|
| complete-trellis | Run `src/lib.rs` through the Octet package | Candidate preserves all Trellis obligations | active |
| upstream-native | Run repository-owned Verus roots with admitted dependencies | Candidate accepts native upstream proof workloads | active |
| artifact-microcase | Run only Verus microcases physically present in the SOSP meta-artifact | Candidate accepts those bounded files, not the full artifact | active |
| semantic-reference | Audit VerusBelt as Rocq semantics tied to its recorded snapshot | Semantic reference remains visible without false Verus execution | audit |
| source-only | Treat hashes, sentinels, or formatting as compatibility | No verifier compatibility claim | falsified |

## Design

A typed Nickel matrix records the candidate Octet revision and profile, current Trellis verifier, per-row mechanism, command identity, promotion role, expected bounded result, limits, and non-claims. A Nix check fetches sources from the existing catalog, validates positive and negative matrix fixtures, executes bounded probes, normalizes diagnostics, and writes one canonical BLAKE3 decision receipt.

The native probes use Octet's exact locked `octet-production-verus` package. The allocator probe builds only its pinned `libc` dependency with the candidate's Rust toolchain before running the repository verification entrypoint. IronKV uses its Verus source root without requiring unrelated .NET products. Node replication executes both the no-flag and required-feature paths. The SOSP row runs its included positive Verus microcases and separately records that full artifact reproduction remains tied to external historical repositories. VerusBelt is audited as Rocq, not passed to Verus.

Promotion logic is pure data reduction over row results. Any required native row that is not `passed` produces `blocked` and leaves Trellis's verifier and Octet's `local-pin` row unchanged.

## Audit Risks and Budgets

Risks are stale source identity, toolchain ABI mismatch, hidden generated dependencies, native scripts that compile rather than verify, timeout misclassification, trusted-body overclaims, and treating non-Verus artifacts as Verus successes. The budget is one baseline, one candidate Trellis run, one bounded probe per corpus row, one adversarial negative matrix fixture, and one rerun for freshness. Full benchmark reproduction and arbitrary upstream patching are excluded.

## Evidence Boundaries

The receipt establishes only observed compatibility outcomes for exact source, tool, argv, limits, and dependencies. It does not prove verifier, solver, compiler, Rocq, or upstream soundness; transfer upstream proof authority; establish runtime correctness; or confer release eligibility.
