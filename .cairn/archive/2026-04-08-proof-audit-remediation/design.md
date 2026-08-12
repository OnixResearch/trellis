## Context

The repo already has a `verification-gaps` spec, but it is stale and too
narrow for the current tree. It only inventories 13 `external_body` items
across 4 files. The current audit found 29 `external_body` items across 11
modules, plus additional trust boundaries that are not represented in the
main spec:

- `assume(...)` soundness oracles
- `@verified-logic:no-ensures` public functions
- real missing postconditions
- placeholder or simplified specs that weaken exec/spec correspondence
- Tracey annotation drift (`uncovered` and `untested` rules)
- tooling false positives that hide the difference between a real contract
  hole and a parser limitation

At the same time, the repository is not currently green: Clippy fails,
Verus fails in `src/mesi.rs` and `src/page.rs`, and Tracey reports both
uncovered and untested rules.

## Goals / Non-Goals

**Goals:**
- Capture the proof-audit findings as a concrete remediation plan
- Broaden the verification-gap inventory to include all trusted or weakened
  proof surfaces
- Restore alignment between code, docs, audit notes, and Tracey output
- Fix tooling that currently produces misleading audit output

**Non-Goals:**
- Eliminate permanent cryptographic trust boundaries
- Rewrite the full verification architecture
- Archive this change before the proof posture is actually restored

## Decisions

### 1. Track proof boundaries by category, not just by attribute

**Choice:** The remediation work will classify findings into explicit
categories:

- `external_body`
- `assume-oracle`
- `no-ensures`
- `missing-ensures`
- `placeholder-spec`
- `tracey-gap`
- `tooling-false-positive`

**Rationale:** The audit showed that `external_body` alone is not a complete
measure of trust. A proof audit that ignores `assume(...)` or weakened
contracts will undercount the real soundness surface.

### 2. Fix the live proof state before refreshing the public narrative

**Choice:** The remediation order starts with the failing gates:
Clippy, Verus, and Tracey.

**Rationale:** Updating `README.md` and the gap inventory before the tool
outputs are stable would just create another stale snapshot.

### 3. Treat Tracey gaps as proof-audit failures, not doc polish

**Choice:** `tracey query uncovered` and `tracey query untested` findings are
part of the proof posture and must be addressed in the same remediation
track.

**Rationale:** The audit surfaced real mismatches between requirement text,
implementation annotations, and verification evidence. Those are proof
coverage defects, not cosmetic issues.

### 4. Separate permanent trust boundaries from closeable regressions

**Choice:** Every audited item must be classified as either:

- **Permanent**: architectural trust boundary that remains by design
- **Closeable**: should be removed or tightened
- **Tooling**: not a proof hole, but an audit/checker defect that must be
  fixed so audits stay credible

**Rationale:** The repo already contains permanent trust boundaries for
crypto axioms and some Z3 limits. This change should not blur those with
regressions such as stale counts, missing annotations, or missing ensures.

### 5. Carry evidence into the change itself

**Choice:** The remediation tasks require capturing `verify.sh`, Tracey, and
inventory scans as evidence for the change.

**Rationale:** The audit findings were large enough that future sessions need
artifact-level evidence, not just conversational memory.

## Risks / Trade-offs

**[Broad scope]** -> This touches verification, documentation, Tracey, and
helper scripts. The mitigation is to keep the work split into phases and to
preserve the audited item list in the change.

**[Moving target]** -> Counts can change while the remediation is in flight.
The mitigation is to regenerate the inventory from scans before updating the
main spec and README.

**[Permanent-vs-closeable disagreement]** -> Some items may be arguable.
The mitigation is to record the rationale for each classification in the
updated inventory rather than relying on a raw count.
