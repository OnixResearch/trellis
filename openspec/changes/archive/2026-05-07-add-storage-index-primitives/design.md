## Context

Storage and indexing proofs repeatedly need the same finite facts: page IDs are
inside a bounded page set, slots are either occupied or free, key ranges are
ordered and half-open, cursors stay within a scan range, segment maps cover
logical spans, lookup results match admission decisions, compaction preserves
live records, and versioned slots select deterministic visible entries.

## Goals / Non-Goals

**Goals:**

- define exact finite storage/index primitive requirements
- separate reusable storage/index metadata from database/filesystem engine claims
- state checked-constructor versus `requires` validity boundaries
- keep future implementation tasks small enough for Verus proofs and Tracey
  evidence

**Non-Goals:**

- implement storage/index modules now
- prove durability, crash consistency, fsync semantics, or write ordering
- specify runtime database/filesystem performance
- provide complete B-tree, LSM, filesystem, or log-structured engines

## Decisions

### 1. Model storage positions as finite IDs and half-open ranges

**Choice:** Page IDs, record slots, keys, and segments use finite identifiers and
half-open ranges with explicit lower/upper bounds.

**Rationale:** Half-open ranges avoid off-by-one ambiguity and are easy to
compose with cursor movement, lookup admission, and compaction remaps.

**Alternative:** Use unbounded integer domains. Rejected because finite bounds
are needed for executable admission checks and Verus-friendly proofs.

### 2. Treat lookup results as admitted decisions, not engine behavior

**Choice:** Lookup-result primitives specify when present, absent, and
out-of-range outcomes are structurally admitted by key/range/table metadata.

**Rationale:** This captures reusable safety facts without claiming any concrete
index implementation returns optimal or storage-engine-complete results.

### 3. Specify compaction as a finite preservation envelope

**Choice:** Compaction primitives describe a finite remap envelope that preserves
live logical record identity and admitted key/slot membership while allowing
physical page/segment spans to change.

**Rationale:** Many storage systems compact or rewrite data, but the reusable
primitive is the logical-preservation condition, not a specific algorithm.

### 4. Versioned slots choose deterministic latest-visible entries

**Choice:** Versioned-slot primitives model finite version metadata and a
selection predicate for the latest entry visible under an admitted read version.

**Rationale:** The fact composes with MVCC/snapshot-style changes while avoiding
transaction-isolation or storage-engine claims.

## Risks / Trade-offs

**Accidental persistence claims** → Requirements explicitly avoid durability,
crash recovery, fsync/write ordering, and filesystem behavior.

**Duplicate engine-specific requirements** → This child owns reusable finite
storage/index facts; future database, cache, log, or filesystem changes should
reference these IDs when they need the same metadata.

**Over-broad implementation slice** → Future work splits page/range, slots,
cursors, segment maps, lookup admission, compaction, and versioned slots into
separate proof/evidence tasks.

## Validation Plan

- `openspec validate add-storage-index-primitives --strict --json`
- validate every active primitive-family change after umbrella updates
- future implementation slices run focused cargo tests, `./scripts/verus.sh
  src/lib.rs`, Tracey uncovered/untested checks, and final repo validation
