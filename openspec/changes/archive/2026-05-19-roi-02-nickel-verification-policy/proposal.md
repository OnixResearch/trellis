## Why

Verification checker behavior is currently encoded directly in Rust scripts and Nix snippets. That works for enforcement, but policy details such as allowed script paths, expected checker names, bounded exemptions, and diagnostic severity are not represented as typed data. Nickel is a better fit for that policy/config layer than for replacing the scanners themselves.

## What Changes

- Introduce a Nickel-authored verification policy contract for checker configuration and bounded exemptions.
- Export a generated JSON policy artifact consumed by Rust/Nix checks or compared for freshness.
- Keep scanning and parsing logic in Rust scripts.

## Scope

In scope:
- Nickel schema/contracts for verification checker policy.
- Generated JSON output and freshness check.
- Rust/Nix integration only where it reads or validates the generated policy.

Out of scope:
- Replacing checker parsers with Nickel.
- Broad exemption additions.
- Changing full verification semantics without fixture-backed proof.

## Impact

- **Files**: `verification-policy/*.ncl` or similar, generated JSON policy, `flake.nix`, Rust checker config loading if needed.
- **Dependencies**: Nickel CLI available through Nix.
- **Testing**: `nickel typecheck/export`, generated freshness check, checker runs, full verification.
