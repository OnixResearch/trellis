## Why

Unison's durable storage pitch is that persisted values can be loaded later with type/code identity preserved, avoiding ad hoc schema glue. `verified-logic` has durable execution and codec/storage primitives, but not a finite typed durable reference envelope tying value identity, type identity, schema version, and dependency availability together.

## What Changes

- Specify typed durable reference envelopes with type hash, value hash, schema/version metadata, and dependency-closure identity.
- Specify unpersist admission only when expected type identity matches the persisted reference.
- Specify explicit migration/conversion boundaries for schema or type changes.
- Exclude real serialization, storage media durability, and database query behavior.

## Scope

In scope: finite structural predicates for typed durable references.

Out of scope:
- Runtime storage APIs, fsync, crash recovery, or redb/SQL semantics.
- Proving codec correctness or deserializing arbitrary bytes.
- Automatic type migration without explicit conversion evidence.

## Verification

Validate this OpenSpec package, then implement finite Rust/Verus contracts, tests, Tracey coverage, and repository verification.
