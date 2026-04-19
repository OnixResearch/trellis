## Context

The existing subsumption module uses integer variant tags with a
hardcoded relation table. UCANs use string-based abilities like
`crud/read` with hierarchical glob matching. Both subsumption
surfaces must satisfy the same authorization decomposition contract.

## Goals / Non-Goals

**Goals:** Verified hierarchical string ability matching. Reflexive,
transitive subsumption. Namespace isolation. Single-level glob.
Integration with the existing authorization composition proof.

**Non-Goals:** Recursive glob (`**`). Regex-based abilities.
Runtime ability registration or dynamic hierarchies.

## Decisions

### 1. Abilities Are Slash-Separated Sequences

**Choice:** An `Ability` is modeled as `Seq<Seq<u8>>` — a sequence
of segment byte strings split on `/`.

**Rationale:** Segment-level matching is cleaner than byte-level
parsing with `/` detection. The split is done once at parse time.

### 2. Single-Level Glob Only

**Choice:** `*` matches exactly one segment. No `**` recursive glob.

**Rationale:** UCAN 0.10/1.0 specs use single-level wildcards for
ability hierarchies. Recursive glob adds complexity without matching
the current UCAN practice. Can be added later.

**Implementation:** `ability_subsumes` checks segment-by-segment:
literal segments must match exactly, `*` matches any single segment.
Length must match (no variable-depth matching).

### 3. Top Is Single Star

**Choice:** `top_ability()` is the pattern `["*"]` (a single
wildcard segment).

**Rationale:** UCAN top capability is `*`. Since single-level glob
requires matching segment count, top must be special-cased to match
any ability regardless of depth. Implemented as: if pattern is
exactly `["*"]`, subsume everything.

### 4. Parallel to Integer Subsumption

**Choice:** `ucan_ability.rs` is a new module alongside `subsumption.rs`,
not a replacement.

**Rationale:** The integer model is used by existing capability-auth
consumers. Both models satisfy the same `authorizes` decomposition
contract. Code that uses integer variants keeps working unchanged.

## Risks / Trade-offs

**[Segment encoding]** → Abilities are compared as raw bytes per
segment. No Unicode normalization. This matches UCAN practice (abilities
are ASCII identifiers) but could bite if someone uses non-ASCII abilities.
