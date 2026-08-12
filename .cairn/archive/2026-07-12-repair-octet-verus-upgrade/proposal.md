# Repair the blocked Octet Verus upgrade

## Why

The exact Octet candidate rejects one Trellis serialization postcondition and three required-native upstream probes. The existing blocked receipt is correct, but leaving the failures as static expected diagnostics prevents the trial from distinguishing a real compatibility repair from evidence drift.

## What changes

- Replace the brittle platform-field proof with explicit sequence-context cancellation while preserving its exact postcondition and trust boundary.
- Make the upgrade rail record repaired outcomes without treating source patches, compilation, or partial verification as proof success.
- Investigate each required-native upstream blocker through a distinct compatibility mechanism, with immutable source/patch identities and no hidden assumptions.
- Promote only if the complete Trellis candidate and every required-native probe verify; otherwise retain the current pin with the exact remaining blocker.

## Success

Success requires both Trellis verifier versions to pass the unchanged serialization theorem, the complete candidate crate to pass, every required-native corpus probe to pass under its recorded source boundary, a fresh deterministic receipt, positive and adversarial-negative policy checks, and Cairn validation. A checked local proof with blocked corpus rows is progress, not completion.
