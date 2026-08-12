## Why

The current proof-review workflow is repeatable but still manually orchestrated: run the scanner, choose a candidate, create a Cairn change, consult VibeThinker, patch known proof/test patterns, collect validation evidence, sync/archive, and commit. The repeated manual steps increase drift risk and make it harder to run several safe proof-hardening iterations consistently.

## What Changes

- Add a repo-owned bounded proof-review loop that drives the existing scanner, Cairn lifecycle commands, evidence capture, and validation gates.
- Keep VibeThinker advisory-only: generated packets may be submitted for critique, but only repo checks decide success.
- Support dry-run/assisted/apply modes with one candidate per iteration, explicit stop conditions, no trust-boundary expansion, and no push by default.
- Use a Nickel-backed loop policy with checked-in generated JSON so target selection and safety rules are deterministic and reviewable.
- Save loop receipts and per-change evidence so each iteration can be audited after archive.

## Impact

- **Files**: expected implementation in `scripts/proof-review-loop.rs`, loop policy under `proof-review/`, Cairn spec artifacts, and tests/evidence.
- **Testing**: loop self-tests for candidate selection and stop conditions, focused dry-run execution, Cairn validation/gates, Tracey direct queries where relevant, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.
