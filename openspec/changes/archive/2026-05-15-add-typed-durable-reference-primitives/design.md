## Context

Unison can persist typed values because code/type identities are stable. This change captures the finite structural admission part: a persisted reference must carry type identity, value identity, and dependency-closure metadata that later unpersist requests can check.

## Goals / Non-Goals

**Goals:**
- Add typed durable reference envelopes.
- Prove exact type-match and closure-availability admission.
- Require explicit conversion witnesses for schema/type migration.

**Non-Goals:**
- Real serialization/deserialization or storage durability.
- Database query/index performance semantics.
- Automatic migration inference.

## Decisions

### 1. Type identity is explicit metadata

**Choice:** Store and compare opaque finite type-hash identifiers.

**Rationale:** Exact identity checks are useful and verifiable without claiming compiler/typechecker correctness.

### 2. Dependency closure is a referenced prerequisite

**Choice:** Unpersist admission requires an available dependency closure identity.

**Rationale:** This composes with the content-addressed code primitives and mirrors Unison's dependency-stable storage story.

### 3. Migration requires a witness

**Choice:** Different type/schema identities are rejected unless a finite conversion witness is supplied.

**Rationale:** This prevents accidental overclaiming that old data can be read as new types without explicit migration logic.

## Risks / Trade-offs

**Storage overclaiming** → Keep all claims at admission/metadata level; no fsync, storage media, or codec correctness claims.

**Circular dependency with content-addressed code** → Treat closure identity as opaque until the content-addressed implementation lands, then compose later.

**Schema semantics drift** → Keep schema/version metadata finite and exact rather than modeling full schema evolution.
