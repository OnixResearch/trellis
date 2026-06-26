## Why

Unison's core model treats definitions as immutable content-addressed values: names are metadata, while definition identity comes from a deterministic hash over normalized syntax and dependency identities. `verified-logic` already has hash, Merkle, Nix, and repo-index primitives, but lacks a focused finite model for code definition identity and dependency closure.

## What Changes

- Specify finite content-addressed definition envelopes for terms/types, names, dependencies, and recursive groups.
- Specify dependency-closure and missing-dependency admission for a root definition hash.
- Exclude cryptographic hash strength, parser correctness, compiler correctness, and runtime code execution.

## Scope

In scope: requirements, design notes, and implementation tasks for finite structural code-identity primitives inspired by Unison.

Out of scope:
- Proving BLAKE3/SHA collision resistance or real syntax-tree canonicalization.
- Runtime compiler, interpreter, or linker behavior.
- Network code sync, ability checks, or durable value loading, which are separate follow-up changes.

## Verification

Validate this OpenSpec package, then implement Rust/Verus structs, executable admission predicates, proof functions, positive/negative tests, Tracey coverage, and the repository verification gate.
