## Context

Aspen uses a two-file architecture: production helpers live in crate source, while formal proofs live under crate-local `verus/` directories. Recent work closed many small `#[verifier(external_body)]` proof gaps and added several Verus proof modules. This repository can benefit from a durable record of those verified properties, but copying all proof files would create a second source of truth and likely drift.

## Goals

- Make Aspen verified-logic evidence discoverable from this repo.
- Keep the default local artifact as a compact index/evidence contract, not a full proof mirror.
- Allow selected proof-kernel snapshots only when local composition or independent verification needs them.
- Require source-derived drift checks before the local index or snapshot is trusted.

## Non-Goals

- No automatic broad import of Aspen source trees.
- No claim that this repo verifies Aspen runtime shells, networking, serialization, crypto, or storage engines.
- No hidden trust transfer from sibling checkout state into this repo's canonical proof inventory.

## Decisions

### 1. Curated index is the default artifact

**Choice:** Add a checked Aspen verified-logic evidence index as the durable local artifact.

**Rationale:** An index can record proof facts, commands, commits, and boundaries without duplicating proof code. This keeps review diffs small and lets future drains pick reusable kernels deliberately.

**Alternative rejected:** Wholesale copying `../aspen/crates/*/verus`. That creates drift and makes this repo appear to own proofs that Aspen still owns.

### 2. Snapshots are opt-in and manifest-backed

**Choice:** Local proof snapshots are permitted only for selected reusable kernels and must carry a manifest with Aspen source commit, source paths, local paths, verification command, and boundary notes.

**Rationale:** Some primitives may become useful for local composition, but importing them should be intentional and auditable.

### 3. Source-of-truth boundary is explicit

**Choice:** Until a proof kernel is locally snapshotted and verified here, Aspen's crate-local proof remains the source of truth and the local index is evidence metadata.

**Rationale:** This prevents stale local docs from being treated as machine-checked local proof.

## Risks / Trade-offs

**Index drift** → Mitigate with a script that validates recorded commits/paths and fails on stale or malformed index rows.

**Overclaiming** → Mitigate by requiring every row to classify remaining trusted boundaries and by excluding runtime/crypto/I/O claims unless explicitly proven.

**Snapshot maintenance cost** → Mitigate by keeping snapshots opt-in and limited to kernels this repo composes directly.
