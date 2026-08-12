# Verification evidence

## Baseline and focused result

- Before the source edit, `./scripts/proof-review-candidates.rs --self-test` passed.
- After the source edit, the same self-test passes with positive assertions for mechanism grouping, gap-strength classification, counterexample/failed-obligation guidance, and adversarial audit coverage plus negative assertions that target correctness is not assumed and equivalent/stronger gaps remain blocked.
- The script self-test compiles and executes the Rust cargo script, including packet rendering and bounded excerpt fixtures.

## Cairn lifecycle evidence

The repository's canonical command was run:

```sh
nix run path:../cairn#cairn -- validate --root .
```

It is blocked before lifecycle inspection because `cairn-policy/generated/cairn-policy.json` predates the current Cairn schema and is missing `function_address_gate`. This policy drift existed outside this change and the change does not rewrite Trellis policy contracts or generated policy.

As a bounded compatibility check, current Cairn was invoked with its own explicit generated policy via `--policy ../cairn/cairn-policy/generated/cairn-policy.json`. Repository validation and the proposal, design, and tasks gates for `strengthen-proof-review-portfolio-guidance` all pass under that explicit policy. Sync executed under that policy, the accepted scenarios were verified in `cairn/specs/proof-review-automation/spec.md`, and archive dry-run reaches only the intentionally open closeout task. This is not reported as canonical Trellis-policy validation, so the change remains active rather than manufacturing archive completion.

## Canonical policy repair and closeout

The shared blocker was repaired in the Nickel source of truth rather than bypassed. `cairn-policy/contracts.ncl` now types the current function-address gate, `cairn-policy/default.ncl` declares it in optional mode with explicit roles, schemas, and non-claims, and `cairn-policy/generated/cairn-policy.json` was regenerated from that source.

Current closeout evidence:

- `cairn policy export --check`: PASS (`policy fresh`).
- canonical `cairn validate --root .`: PASS with 21 active changes and 121 accepted specs.
- proposal, design, and tasks gates: PASS; the terminal tasks receipt is `c48242161dc76c6c5a5d5aa6392c2288dbb03787c6cea858c487e93db8fc2758`.
- `./scripts/proof-review-candidates.rs --self-test`: PASS.
- sync dry-run: unblocked under canonical `cairn-default` policy.
- accepted text: `cairn/specs/proof-review-automation/spec.md` contains the mechanism-family, gap-strength, adversarial-audit, and regression scenarios.

## Post-archive validation

```text
$ nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
{
  "change_issues": [],
  "changes": 20,
  "issues": [],
  "layout": "cairn",
  "policy": "cairn-default",
  "spec_issues": [],
  "specs_validated": 120,
  "valid": true
}

$ nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- policy export --source cairn-policy/default.ncl --output cairn-policy/generated/cairn-policy.json --check
policy fresh: cairn-policy/generated/cairn-policy.json

$ ./scripts/check-verification-policy.rs
verification policy fixture checks: PASS
verification policy check: PASS

$ ./scripts/proof-review-candidates.rs --self-test
proof-review-candidates self-test: PASS
```

## Claim boundary

The generated prompt and model classifications remain advisory. The checks prove deterministic packet text and fixture assertions only; they do not prove mechanism distinctness, gap-strength semantics, VibeThinker correctness, Verus proof success, or release readiness.
