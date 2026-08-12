# Change: Kani complementary verification boundary

## Why

Trellis is a Verus-checked proof corpus. Kani can still add value as external model-check evidence for runtime wrappers, panic-freedom, and boundary regressions, but it must not dilute Trellis proof claims or replace Verus obligations.

## What Changes

- Define how Trellis may reference Kani receipts as complementary external evidence.
- Keep Verus proof artifacts as the source of Trellis verified-logic claims.
- Require Kani evidence to carry scope, source identity, non-claims, and external-evidence status.
- Add guidance for using Kani to harden runtime wrappers and boundary tests without promoting it to Trellis proof.

## Impact

- **Files**: verification-boundary specs/docs, evidence templates, and optional runtime-wrapper examples.
- **Consumers**: Octet and Valence can cite Trellis proof artifacts and Kani receipts without confusing their roles.
- **Non-claims**: Kani receipts do not prove Trellis ghost/spec theorems or replace Verus verification.
