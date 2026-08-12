## Why

A web of trust lets a relying party accept keys or principals through
signed attestations from already trusted introducers instead of a
central CA. This repo already verifies delegation, revocation,
quorum-like thresholds, and chain-style invariants, but it does not yet
have the graph semantics needed for authenticated trust edges,
activation windows, exact revocation, chain validity, or threshold-based
trust resolution.

A verified web-of-trust layer would give downstream crates reusable
logic for package-signing trust roots, peer bootstrapping, artifact
provenance, and decentralized identity checks without dragging in packet
formats, keyservers, or UI policy.

## Current Tree Audit

- Checked pinned pre-change shipped surfaces `src/lib.rs`,
  `docs/requirements.md`, `README.md`, and `openspec/specs/`.
- At change inception, those shipped surfaces exported no
  `src/web_of_trust*.rs` modules, no `web-of-trust.*` or
  `properties.web-of-trust.*` requirements, and no shipped web-of-trust
  domain spec.
- The live working tree now contains landed Phase 1+2, the full Phase 3
  chain/rooted-resolution/property slices, and the full Phase 4
  threshold slice from this active change: `web_of_trust_principal`,
  `web_of_trust_attestation`, `web_of_trust_revocation`,
  `web_of_trust_chain`, `web_of_trust_resolution`,
  `web_of_trust_policy`, plus cross-module web-of-trust lemmas in
  `src/properties.rs`, along with mirrored
  `web-of-trust.principal-fingerprint`,
  `web-of-trust.attestation-signature`,
  `web-of-trust.attestation-window`, `web-of-trust.revocation`,
  `web-of-trust.chain-validity`, `web-of-trust.rooted-resolution`,
  `properties.web-of-trust.revocation-cuts-trust`,
  `properties.web-of-trust.depth-monotone`, `web-of-trust.threshold`,
  and `properties.web-of-trust.threshold-dedup` requirements.
- The main `openspec/specs/` tree still has no shipped web-of-trust
  domain spec.

This remains net-new functionality, not archive cleanup or spec-sync-only
work.

## What Changes

- **`web_of_trust_principal`** — principal identifiers and canonical
  BLAKE3 fingerprints over public-key bytes.
- **`web_of_trust_attestation`** — signed issuer -> subject
  attestations whose canonical payload binds issuer, subject, explicit
  trust role, validity window, and sequence metadata exactly.
- **Operational trust-role semantics** — this change models one
  operational role, `trust`. Only active attestations with role `trust`
  form trust edges for chains, rooted resolution, and threshold policy.
- **`web_of_trust_revocation`** — authenticated same-issuer revocations
  with two exact target kinds: canonical-payload identity, which
  disables all identical-payload copies, or an exact attestation family
  keyed by `(issuer, subject, role)`.
- **`web_of_trust_chain`** — simple-path chain validity over active
  `trust` edges.
- **`web_of_trust_resolution`** — rooted, depth-bounded trust
  resolution where a trust root resolves with an empty chain at depth
  `0`.
- **`web_of_trust_policy`** — threshold aggregation over distinct
  rooted-trusted introducers, with the direct introducer -> subject edge
  consuming the final depth hop, `k = 0` or `d = 0` rejecting every
  subject, no special root-subject exemption, and counted witnesses
  preserving simple-chain validity.
- **Cross-module properties** — proofs that revocation cuts derived
  trust, depth bounds are monotone, and duplicate attestations from one
  introducer do not inflate threshold counts.

## Capabilities

### New Capabilities

- `web-of-trust.principal-fingerprint`: Verified canonical BLAKE3 principal IDs
- `web-of-trust.attestation-signature`: Verified exact attestation payload binding
- `web-of-trust.attestation-window`: Verified activation-window semantics
- `web-of-trust.revocation`: Verified exact, monotone revocation semantics
- `web-of-trust.chain-validity`: Verified active simple-path trust-chain validity
- `web-of-trust.rooted-resolution`: Verified rooted depth-bounded trust resolution
- `web-of-trust.threshold`: Verified distinct-introducer threshold aggregation
- `properties.web-of-trust.revocation-cuts-trust`: Verified a revoked edge can no longer justify trust via that same witness chain
- `properties.web-of-trust.depth-monotone`: Verified larger depth bounds preserve successful trust resolutions
- `properties.web-of-trust.threshold-dedup`: Verified duplicate attestations from one introducer count once

## Impact

- **Files**: new `src/web_of_trust_*.rs` modules, `src/lib.rs`
  exports, `src/properties.rs` lemmas, one-to-one mirrored
  `docs/requirements.md` entries for every new `r[...]` ID, and this
  OpenSpec change
- **APIs**: public spec/exec/proof functions for principal IDs,
  attestation validation, revocation, chain checks, rooted resolution,
  and threshold aggregation
- **Dependencies**: no new runtime dependencies expected; may reuse
  existing sequence, revocation, delegation, and quorum helpers where
  they fit
- **Testing**: Verus proofs, explicit `proof fn` artifacts for the
  `properties.web-of-trust.*` requirements, one-to-one mirrored
  `docs/requirements.md` entries for every new `r[...]` ID, explicit
  source-level `r[impl]` / `r[verify]` tags on the defining source
  artifacts for every new requirement, unit tests for concrete trust
  graphs, and proptest model oracles for chain validity, revocation,
  rooted resolution, and threshold aggregation

## Out of Scope

- PGP packet parsing, OpenPGP keyring formats, or keyserver sync
- Public-key cryptography internals beyond the abstract hash/signature
  boundary
- Human-identity normalization such as email, DNS, or profile lookups
- Additional non-`trust` role semantics beyond keeping the role field
  explicit and signed
- Reputation scoring, gossip convergence, or UX policy tuning beyond a
  basic distinct-introducer threshold
- Distributed revocation transport, transparency logs, or consensus on
  trust roots
