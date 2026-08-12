## Why

Trellis owns reusable verified logic, but downstream systems and Octet release gates need a stable way to inspect what was proved, which requirement IDs were covered, which verifier/toolchain produced the evidence, and which trusted boundaries remain. Today those facts are spread across README counts, Tracey, Verus output, proof-gap inventory, module docs, and Cairn specs.

Trellis should export deterministic proof-evidence bundles that downstream tools can consume without reinterpreting raw logs or broad marketing claims. The bundle should describe verified building blocks and their boundaries; downstream gates such as Octet can decide how to admit or reject a release artifact.

## What Changes

- Define an exportable proof-evidence bundle contract for Trellis modules and examples.
- Include requirement IDs, module/source/proof paths, Verus command identity, toolchain identity, proof outcome, Tracey coverage status, trusted-boundary references, module maturity facts, and BLAKE3 content identities.
- Add fail-closed validation for missing required fields, stale content identities, unsupported schema versions, untracked proof gaps, and overbroad claims.
- Document the boundary between Trellis evidence export and downstream release approval.

## Scope

In scope: bundle schema/contract, deterministic generator or checker, docs, positive and negative fixtures, BLAKE3 content identity, and integration notes for downstream consumers.

Out of scope:
- Octet release-gate implementation or artifact-verifier behavior.
- New proof targets solely for bundle optics.
- Cryptographic signatures, transparency logs, or remote registry publication.
- Treating a bundle as certification or whole-system correctness evidence.

## Verification

Run bundle generator/checker tests, positive and negative fixture validation, Tracey, Verus, cargo/clippy, Cairn validation/gates, and repository verification appropriate for touched files.
