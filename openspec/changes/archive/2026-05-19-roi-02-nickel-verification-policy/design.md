## Context

The new Rust scripts are effective scanners, but their policy boundaries are implicit. Nickel can provide typed contracts and export stable JSON while preserving Rust for parsing and command behavior.

## Goals / Non-Goals

**Goals:**
- Make verification policy inspectable as data.
- Add freshness checks so generated JSON cannot drift from Nickel sources.
- Bound exemptions with owner/reason/expiry fields if exemptions are introduced.

**Non-Goals:**
- No Nickel parser replacement.
- No dynamic runtime policy fetch.
- No open-ended exemption expansion.

## Decisions

### Nickel owns policy, Rust owns scanning

**Choice:** Nickel defines declarative policy and exports JSON; Rust scripts continue to inspect source files.

**Rationale:** This matches Nickel's strength as a typed configuration language and avoids weakening deterministic scanner behavior.

### Generated policy freshness gate

**Choice:** Add a Nix check that exports Nickel policy to a temp file and compares it against the checked-in generated artifact.

**Rationale:** Reviewers can inspect generated JSON while the source of truth remains Nickel.

## Risks / Trade-offs

**Tool availability** → Use the Nix flake to provide Nickel; do not require host-global `nickel`.

**Policy indirection** → Keep the initial schema small and document every field.
