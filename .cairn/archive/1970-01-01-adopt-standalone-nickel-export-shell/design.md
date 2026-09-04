## Context

Trellis invokes external Nickel from Rust and Nix checker shells to materialize policy/config artifacts, compare checked bytes, and exercise invalid fixtures. Those shells are outside the Verus-checked logic corpus. The standalone `nickel-export` CLI can centralize request normalization, exact BLAKE3 identity, diagnostics, multi-format output, and `--check` freshness while Trellis retains all proof-policy meaning.

## Decisions

### 1. Release inputs use one immutable source and evaluator cohort

Nix and any Rust dependency metadata will select `github.com/OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` and a declared Nickel evaluator version. Local path overrides are diagnostic-only and cannot satisfy release checks.

### 2. Only checker shells gain the external tool

A thin Trellis-owned wrapper translates existing checker declarations into canonical requests and chooses repository roots, evaluator binaries, dependencies, selectors, contracts, and destinations. Verified modules, proof specifications, and pure models do not gain filesystem, process, environment, or evaluator dependencies.

### 3. Trellis semantic validation runs after export

The standalone shell establishes exact request/output/dependency identities, structured diagnostics, and freshness. Trellis continues to interpret verification policy, lifecycle YAML, artifact contracts, proof-corpus expectations, and invalid fixtures. Export success never bypasses a Trellis proof or policy gate.

### 4. Dual-run is byte- and failure-class exact

Legacy and standalone paths run over the same source, declared dependencies, evaluator version, selector/contract inputs, and destination expectations. The comparison covers verification-policy bytes, lifecycle YAML, JSON artifact-contract bytes, and invalid-fixture rejection. Differences are classified before authority changes.

### 5. Rollback remains available through validation

Existing checker scripts remain authoritative for at least one complete positive and negative validation cycle. On drift, Trellis retains both outputs and identities, keeps the legacy result authoritative, adds paired regression fixtures, and retries only after the difference is explained.

### 6. The functional-core boundary remains explicit

Canonical normalization, identity, admission, receipt/manifest construction, and freshness are pure standalone-core operations. Trellis wrappers own file reads, process execution, checked destinations, proof-policy parsing, Verus gates, and evidence rendering. No proof claim is derived solely from the shared receipt.

## Risks / Trade-offs

- External evaluator diagnostics can vary across Nickel versions, so the evaluator cohort must be pinned and compared explicitly.
- Exact-byte comparison intentionally exposes serialization and newline drift.
- Dual-running temporarily increases checker time.
- Standalone admission cannot establish the truth of Trellis proof-policy semantics.

## Non-Goals

- Modifying Trellis verified algorithms or proof obligations.
- Importing filesystem/process authority into reusable verified modules.
- Replacing Verus, Tracey, proof-corpus, or release gates.
- Claiming evaluator equivalence, proof correctness, or release readiness from matching exports.
