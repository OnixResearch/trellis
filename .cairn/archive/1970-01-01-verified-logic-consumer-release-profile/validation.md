# Validation Evidence

## Bound identities

- Implementation commit: `5c57e9f97975594cd09775d0295e121f6475263d`
- Consumer release revision: `1f03111b92cc36aade07359c96773e086908c95f`
- Consumer release input: locked `trellis-consumer-release-src` Git source at the same revision
- Profile: 39 UCAN-facing modules and 42 selected artifacts
- Vendored compatibility report BLAKE3: `9116ff2cc033683f134dba98ef6b9420709d90175fa03094c7a6928b6658137e`
- Direct compatibility report BLAKE3: `a77b4ec25df5ba39fb712ab86fd0d4bfeefa5ada0934c855c3d14f3c77e62031`

## Focused profile validation

The following checks passed:

- `./scripts/check-verified-logic-consumer-profile.rs`
- `./scripts/check-verified-logic-consumer-profile.rs --self-test`
- Direct and vendored report generation from Nickel-authored example manifests
- `nix build .#checks.x86_64-linux.consumer-release-profile --no-link -L --option secret-key-files ''`
- UCAN's seven Josh checker unit tests
- UCAN's full vendored-tree comparison against Trellis revision `1f03111b92cc36aade07359c96773e086908c95f`

The checked fixture matrix contains two positive fixtures and eight negative fixtures. It rejects stale or missing revisions, missing source metadata, missing selected modules, modified snapshot files, unapproved patches, unverified local additions, and weakened non-claims. The in-memory self-test additionally rejects a weakened dirty-state policy and a mismatched immutable source revision.

## Repository validation

The following broad rails passed on the implementation:

- The dev-shell-compiled `scripts/verify.rs` pipeline, ending with `ALL CHECKS PASSED`
- 4,849 library tests, proof-evidence tests, repo-index tests, HDL output-matrix tests, Clippy, Verus, and direct Tracey checks in the full verifier
- `nix flake check -L --option secret-key-files ''`, including 4,871 passing nextest cases and the new immutable-source consumer profile derivation
- Policy-compatible Cairn repository validation and current proposal, design, and tasks gates

The empty `secret-key-files` override accommodates this machine's absent configured Nix signing key. It does not change source inputs, derivations, profile decisions, or proof claims. The failed remote-builder connection fell back to the local builder and did not change the successful results.

## Lifecycle compatibility

Current proposal, design, and tasks gates pass for this change. Current-schema root validation does not: it is blocked by eleven unrelated active changes with duplicate requirement IDs. Current sync preflight is additionally blocked by unrelated accepted specs that predate canonical substance IDs. Trellis's checked local policy uses the immediately preceding policy schema. Validation and sync therefore used policy-compatible Cairn revision `8774a1f8fa2bea8c8e7bc6741ae31d56563c8802`, the parent of the substance-gate introduction. Postconditions remain checked by current change gates and exact accepted-spec ID inspection.

Executed sync used plan hash `3a3e1751ecbfd72d95d0688e11c6db52fef2af4d715c5ef76049443aa15e78e4` and receipt hash `43489bfce1e4bcee91d1b0a4afd4c1b78279eeef46d3652c565851e824bc7f7e`. All ten new `trellis.crate_packaging.verified_logic_consumer_profile*` requirement and scenario IDs are present in `cairn/specs/crate-packaging/spec.md` after execution.

## Non-claims

These checks establish exact release/profile and selected-source alignment only. They do not prove UCAN token correctness, downstream authorization policy, verifier soundness beyond Trellis's stated boundary, external consumer readiness, or whole-system security.
