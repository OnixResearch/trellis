# OpenSpec Gate Summary

## Proposal
- Command: `openspec_gate stage=proposal change=ucan-signature-primitives`
- Current status: WARN
- Notes: proposal content is now coherent for primitive scope; remaining reviewer notes are about archive-readiness evidence not being present in the proposal-only gate packet, not about proposal text contradictions.

## Design
- Command: `openspec_gate stage=design change=ucan-signature-primitives`
- Current status: WARN
- Notes: current reviewer notes are about archive-readiness artifacts not being present in a design-only packet; substantive design contradictions were resolved by syncing requirement-to-proof/test mapping and explicit gate/validation checks.

## Tasks
- Command: `openspec_gate stage=tasks change=ucan-signature-primitives`
- Current status: FAIL while closeout work remains unchecked
- Notes: remaining blockers are execution blockers, not artifact-shape blockers: run/capture fresh gate outputs, archive evidence, move the change, and recount active changes.
