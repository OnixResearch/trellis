# Design: Add an upstream Verus regression corpus

## Catalog

`upstream-verus-corpus/catalog.ncl` owns the reviewable corpus. Each entry records repository id, URL, revision, Nix source hash, license status, corpus role, verifier policy, sentinels, known limitations, and non-claims. The catalog is exported deterministically and validated without network access.

The initial roles are:

- semantic foundation: VerusBelt;
- reproducibility/upgrade benchmark: SOSP 2024 artifact;
- raw-pointer and allocator proof stress: verified memory allocator;
- host-program/distributed-boundary stress: verified IronKV;
- local multicore/NUMA proof stress: verified node replication.

## Source intake

Nix fixed-output fetches establish exact source bytes under the required SHA-256 protocol. The source-intake check validates declared sentinel paths and emits a bounded source manifest. BLAKE3 remains the stack-owned identity for generated manifests and receipts.

Source intake is not proof execution. An entry can be `reference-only`, `opt-in-verifier`, or `blocked-compatibility`; promotion requires an exact local verifier run and Trellis-owned receipt.

## Formatter ratchet

Running `verusfmt` over the complete current Trellis corpus would rewrite roughly sixty-five thousand lines. Instead, the required rail compares the exact set of nonconforming existing files against `verusfmt-baseline.txt`.

- a new nonconforming file fails because it is absent from the baseline;
- a formatted baseline file fails as stale until its baseline entry is removed;
- positive and negative fixtures prove formatter pass/fail behavior;
- the baseline is sorted, duplicate-free, and contains only existing `src/*.rs` paths.

This is a migration ratchet, not a claim that baseline files are formatted.

## Boundaries

The checker core validates in-memory catalog values and normalized paths. The Nix shell performs fixed-output fetches and filesystem sentinel checks. No network, subprocess, or filesystem logic enters Trellis verified modules.
