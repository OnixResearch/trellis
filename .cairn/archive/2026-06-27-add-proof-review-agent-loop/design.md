## Context

`./scripts/proof-review-candidates.rs` already provides deterministic candidate discovery and VibeThinker packet generation. Recent R1CS hardening work showed that the remaining manual loop is highly structured: one source target is selected, a native Cairn change is created, a known proof/test hardening pattern is applied when safe, validation logs are captured, the change is synced and archived, then one commit records the evidence.

## Decisions

### 1. Add a bounded loop wrapper, not autonomous open-ended editing

**Choice:** Implement `scripts/proof-review-loop.rs` as a Rust cargo-script wrapper with explicit `--max-iterations`, `--mode dry-run|assisted|apply`, and `--no-push` default behavior.

**Rationale:** A bounded wrapper gives repeatability without turning proof repair into an unreviewable background process. One candidate per iteration keeps commits and evidence small enough to audit.

### 2. Keep target selection deterministic and policy-backed

**Choice:** Define loop policy in Nickel under `proof-review/`, export checked-in JSON, and have the loop consume that generated JSON. The policy will rank actionable findings, skip permanent trust boundaries by default, prefer vacuous coverage proofs, and stop on ambiguous or unsupported patterns.

**Rationale:** Policy-backed selection makes safety decisions explicit and avoids hard-coded magic thresholds in the loop driver.

### 3. Separate functional core from imperative shell

**Choice:** Keep candidate filtering, mode planning, slug generation, stop-condition classification, and receipt rendering as pure functions. The shell performs file reads, command execution, Cairn mutation, evidence writes, formatting, and commits.

**Rationale:** The loop needs strong regression coverage for safety behavior without standing up Verus, Tracey, or Cairn in unit tests.

### 4. Validation remains authoritative

**Choice:** VibeThinker output is optional advisory evidence. A loop iteration succeeds only if the configured repo checks pass: focused tests where available, Verus over `src/lib.rs`, proof-review scanner, Tracey uncovered/untested, Cairn validation/gates, and the repo verification wrapper.

**Rationale:** Advisory model output can be wrong. The loop must never treat a critique as proof of correctness.

### 5. Stop before unsafe side effects

**Choice:** The loop MUST stop on dirty unrelated files, active Cairn changes, unsupported patch patterns in apply mode, validation failure, attempted trust-boundary additions, missing evidence, failed archive, or a request to push without an explicit `--push` flag.

**Rationale:** Fail-closed behavior prevents the automation from hiding uncertainty or expanding trust.

## Risks / Trade-offs

- Fully automatic proof repair is brittle. The first implementation should support dry-run and assisted mode before enabling narrow apply recipes.
- Running full validation every iteration is slower, but it keeps archive evidence trustworthy.
- A policy file adds one generated artifact to maintain, but it makes loop behavior reviewable and testable.
