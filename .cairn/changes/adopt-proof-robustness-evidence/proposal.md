# Proposal: Adopt proof robustness evidence

## Why

Trellis records exact Verus proof results, source identities, assumptions, and trust boundaries. One successful run does not reveal sensitivity to controlled solver-seed changes.

The current proof evidence IR also cannot distinguish declared assumptions from provider-observed axiom use. A bounded pilot can add that evidence without changing theorem claims.

## What Changes

- Consume the Octet proof-robustness profile and receipt contract after that producer change is accepted.
- Define a typed Trellis pilot corpus with clean, trusted-boundary, and proof-complexity classes.
- Start with `src/arithmetic.rs`, `src/chain.rs`, and `src/choreography_projection.rs`, each with a recorded selection reason.
- Use strict no-cheating mode where the selected proof has no accepted boundary.
- Use audited-boundary mode where accepted cryptographic or runtime assumptions remain.
- Preserve Verus axiom-use output as partial proof-use evidence.
- Link the robustness receipt into the existing Trellis proof evidence IR chain.
- Add stable positive fixtures and expected unstable, stale, missing-assumption, and overclaim negative fixtures.

## Impact

- **Specs**: extend `proof-evidence-bundles` with proof robustness adoption.
- **Files**: pilot Nickel profile, proof-run adapters, proof evidence chain fields, fixtures, receipts, documentation, and Nix checks.
- **Claims**: theorem and formal-proof claims remain owned by existing Verus receipts. Robustness evidence is supplemental.

## Dependencies

- Octet `add-proof-robustness-evidence` must publish an accepted provider and receipt contract.
- Trellis `proof-evidence-ir-chain` must provide the source, IR, verifier, assumption, and handoff layers.
- Valence proof-robustness role registration must be available before stack promotion.

A blocked dependency remains a valid planning outcome. Trellis must not reconstruct a shadow Octet schema.

## Non-goals

- Do not run Dafny in Trellis.
- Do not claim complete proof dependency coverage from `-V axiom-usage-info`.
- Do not replace Verus proof status with a stability classification.
- Do not convert stable resource use into theorem or release evidence.
- Do not make the pilot part of normal Cargo builds.

## Completion Evidence

The change is complete when every pilot member has current proof and robustness receipts, exact assumption accounting, and positive and negative fixture evidence.
