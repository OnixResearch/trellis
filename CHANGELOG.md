# Changelog

## v0.2.0 - 2026-05-16

This release is a major proof-surface and model-coverage expansion since `v0.1.0`.

### Highlights

- Expanded the library to 575 verified/modelled modules with 10101 Verus proofs, 4587 tests, and 1894 Tracey requirements.
- Reduced and documented the remaining trusted/proof-gap boundary inventory to 16 explicit sites.
- Landed broad domain primitive coverage across arithmetic, storage, concurrency, distributed systems, UCAN/web-of-trust, kernel/OS, compiler, GPU, graph, geometry, filesystem, network, observability, and composite facade models.
- Closed multiple former proof gaps by replacing trusted bridges with verified executable/modelled decompositions, including Nix version comparison slices, UCAN DID/CID/signature structure, graph/topological/reachability helpers, cache and memory models, and composite facade traceability.
- Completed and archived the OpenSpec backlog; the active OpenSpec queue is empty at release cut.

### Verification baseline

The release candidate is expected to pass the maintained repository gate:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/verify.sh
```

plus clean OpenSpec status and a clean jj/git worktree.
