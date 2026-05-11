## Why

Aspen now has many small Verus-verified slices across coordination, Nostr relay, federation, CI, storage, network, jobs, and cluster crates. This repo should be able to reference that work without turning chat summaries into the source of truth or copying a drifting tree of proofs.

A bounded OpenSpec is needed to define the local contract: keep a curated evidence index for Aspen verified logic, optionally snapshot only reusable proof kernels, and make source-of-truth and drift boundaries explicit.

## What Changes

- Add a downstream verified-logic snapshot policy for Aspen evidence.
- Require an index that records source repository path, Aspen commit, proof module path, verified property summary, Verus command/result, focused Rust test command/result, and remaining trusted boundaries.
- Permit local proof-code snapshots only when this repo needs independent composition or verification, with a source-commit manifest and drift check.
- Prevent overclaiming: Aspen crate-local `crates/*/verus/` remains source of truth unless a local snapshot is explicitly imported and verified here.

## Scope

In scope:
- Documentation and machine-checkable inventory for Aspen verified-logic evidence.
- Optional local snapshot layout for selected reusable proof kernels.
- Drift and validation gates that keep the local index/snapshot aligned with recorded Aspen commits.

Out of scope:
- Wholesale mirroring all Aspen Verus modules.
- Claiming Aspen runtime, networking, crypto, I/O, or external-system behavior is verified by this repo.
- Replacing Aspen's crate-local Verus proofs as the source of truth.
- Automatically importing code from sibling repositories without an explicit operator action.

## Verification

Validate this active OpenSpec first. Implementation must later capture evidence for index generation, drift failure cases, local snapshot verification when snapshots are present, and the normal repo gates before archive.
