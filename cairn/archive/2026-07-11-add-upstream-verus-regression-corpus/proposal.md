# Proposal: Add an upstream Verus regression corpus

## Summary

Add a typed, pinned intake corpus for selected `verus-lang` repositories and a ratcheted `verusfmt` rail for Trellis source. The corpus validates source identity, license-review status, sentinel files, intended execution mode, assumptions, and non-claims without treating upstream verification as Trellis proof.

## Motivation

Verifier upgrades need representative external inputs that exercise semantic foundations, systems proofs, distributed-host reasoning, raw-pointer allocation, and local NUMA concurrency. Trellis currently has no single deterministic inventory for those inputs. Formatting the existing proof corpus wholesale would create an unreviewable mechanical change, so formatter adoption also needs an exact ratchet baseline.

## Scope

- Add a typed Nickel corpus catalog for VerusBelt, the SOSP 2024 artifact, verified memory allocator, verified IronKV, and verified node replication.
- Pin exact upstream revisions and Nix fixed-output source hashes.
- Add positive and negative catalog fixtures and a pure checker.
- Add a Nix source-intake check that fetches exact revisions and validates declared sentinels.
- Add a pinned `verusfmt` ratchet: existing nonconforming files are listed exactly; new drift or stale baseline entries fail.
- Add upstream references and explicit proof/non-proof boundaries.

## Non-goals

- Do not vendor upstream source into Trellis.
- Do not run every upstream verifier or benchmark in the default Trellis check set.
- Do not treat source identity, sentinel presence, formatting, or upstream README claims as Trellis proof.
- Do not claim VerusBelt covers a Verus release other than its recorded snapshot.

## Validation

Catalog contract checks, positive/negative checker self-tests, fixed-output source/sentinel checks, formatter positive/negative fixtures, exact ratchet comparison, existing Trellis proof verification, and Cairn validation.
