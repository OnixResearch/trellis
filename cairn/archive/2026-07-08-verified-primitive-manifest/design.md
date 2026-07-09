## Context

Trellis already tracks proof-gap boundaries, requirement coverage, and Verus validation, but downstream consumers need a small machine-readable export boundary. Cargo revisions and vendored snapshots identify bytes but not proof intent, trusted-boundary status, or evidence-tier boundaries.

## Design

### Manifest shape

Author the manifest contract in Nickel and export deterministic JSON for Rust or downstream checks. Each primitive row includes:

- stable primitive ID;
- module path and public API surface;
- requirement IDs covered by the primitive;
- source artifact BLAKE3 and byte count;
- proof artifact BLAKE3 and byte count when a proof artifact exists;
- verifier command argv and verifier identity;
- trusted-boundary classification;
- evidence tier recommendation;
- downstream non-claims.

### Freshness check

A pure checker receives parsed manifest rows and measured file identities. The shell reads files and computes BLAKE3. The checker rejects stale identities, missing exported APIs, missing verifier commands for proof rows, formal-proof tier on trusted-only rows, and non-claim weakening.

### Downstream use

UCAN vendor sync can record the Trellis manifest digest next to `vendor/verified-logic-revision.txt`. Octet proof imports can reference primitive IDs and keep manual-review evidence separate until an Octet-owned proof rail reruns. Valence and Cairn can consume the row as identity/linkage metadata only.

## Alternatives

### Keep relying on Cargo revisions

Rejected. A revision is necessary but not sufficient to identify the proof claim, trusted boundary, Verus command, or downstream non-claims.

### Emit one large proof bundle without row-level IDs

Rejected. Downstream consumers need stable, row-level IDs so they can bind only the primitives they actually consume.

## Risks

- **Manifest drift**: addressed by measuring source/proof bytes and rejecting stale rows.
- **Proof overclaiming**: addressed by required non-claims and evidence-tier rules.
- **Downstream lockstep pressure**: consumers can pin a manifest digest independently from adopting every new primitive.
