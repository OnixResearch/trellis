## Why

Trellis becomes more useful when downstream systems can call verified admission helpers without learning every internal model. Thin integration adapters can connect existing verified predicates to common domains while preserving deterministic, dependency-light behavior.

## What Changes

- Define an adapter contract for user-facing integration surfaces.
- Add an initial adapter slice for selected domains such as Raft/OpenRaft-like admission, UCAN authorization, Nix/Nickel policy checks, and storage/WAL recovery checks.
- Require examples, tests, and clear non-conformance boundaries for every adapter.

## Scope

In scope: thin deterministic adapters over existing Trellis primitives, examples, and positive/negative tests.

Out of scope:
- Full external library compatibility promises.
- Networked services or async runtimes.
- New cryptographic trust claims.

## Verification

Run baseline tests before implementation, then focused adapter tests, examples, clippy, Tracey, Verus, and full repository gates after implementation.
