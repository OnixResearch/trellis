## Why

Trellis proof evidence should not be source-only. Source bytes are necessary for human review and verifier reruns, but downstream stack tools need a canonical, typed proof evidence IR for stable hashing, linkage, querying, and readiness gates.

## What Changes

- Define a Trellis proof evidence IR that preserves original source envelopes alongside canonical proof payloads.
- Record theorem/property IDs, requirement IDs, verified item locators, spans, assumptions, dependencies, verifier obligations, verifier/toolchain identity, command identity, result status, and non-claims.
- Produce a typed evidence-chain handoff suitable for Kamacite canonical Preserves envelopes.
- Add positive and negative fixtures for verified proof receipts, reference-only receipts, stale source identity, missing assumptions, and overclaims.

## Impact

Trellis remains the verified-logic producer while enabling Kamacite, Valence, Mantle, and Cairn to consume proof evidence without parsing Verus source or treating IR extraction as a proof by itself.
