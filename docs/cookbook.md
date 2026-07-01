# Trellis Cookbook

These recipes are short workflows for common Trellis tasks. They complement the
intent-oriented [module guide](module-guide.md) and the public
[contributor guide](../CONTRIBUTING.md). Each command snippet is marked with a
snippet status so `scripts/check-cookbook.rs` can verify recipe metadata and
local links without treating illustrative commands as a full execution gate.

## Recipe: Use a verified helper in downstream code

Goal: Call a small public helper from normal Rust while relying on its verified
contract in Trellis source.

Files: Start with the [module guide](module-guide.md), then inspect a focused
source module such as [`arithmetic`](../src/arithmetic.rs) or
[`quorum`](../src/quorum.rs).

Commands:

```bash
cargo test --quiet arithmetic
```

Success: The downstream call uses the public `verified-logic` crate API, and the
matching source function has an `ensures result == *_spec(...)` contract or a
module-level requirement explaining its runtime surface.

Failure mode: If the helper is model-only, trusted-boundary-adjacent, or missing
a public runtime function, do not depend on it as a concrete runtime API; choose
a different module or add a Cairn change first.

Snippet status: illustrative; run the command locally when touching this path.

## Recipe: Add a verified primitive

Goal: Add a small deterministic primitive with a requirement, spec, exec body,
proof evidence, and tests.

Files: Follow [CONTRIBUTING.md](../CONTRIBUTING.md), copy the shape in
[`docs/module-template.md`](module-template.md), update [`requirements.md`](requirements.md),
and add the source module under [`src/`](../src/lib.rs).

Commands:

```bash
cargo test --quiet <module_filter>
./scripts/verus.sh src/lib.rs
tracey query uncovered
tracey query untested
```

Success: The module exports only intentional public APIs, every public function
has Tracey coverage, Verus proves the exec/spec contracts, and focused tests
cover accepted and rejected inputs.

Failure mode: A new public function without `r[impl ...]`, an exec function
without a useful `ensures`, or a test suite with only happy-path cases is not
complete.

Snippet status: illustrative; command arguments depend on the module name.

## Recipe: Write positive and negative tests

Goal: Make tests demonstrate both valid behavior and safe rejection.

Files: Put module-local unit tests beside the implementation or integration
coverage under [`tests/`](../tests/). Use the pattern in
[`docs/module-template.md`](module-template.md).

Commands:

```bash
cargo test --quiet <module_filter>
```

Success: Positive tests prove accepted inputs produce the expected result, while
negative tests cover malformed, boundary, missing-field, overflow, or rejection
cases and assert the safe failure result.

Failure mode: A test file that only checks accepted inputs leaves the rejection
contract undocumented and should not close the validation task.

Snippet status: illustrative; test filters are module-specific.

## Recipe: Run a focused Verus check

Goal: Validate proof changes without immediately running every repository gate.

Files: Use [`scripts/verus.sh`](../scripts/verus.sh) for verifier execution and
check the module source linked from the [module guide](module-guide.md).

Commands:

```bash
./scripts/verus.sh src/lib.rs
```

Success: Verus reports zero errors for `src/lib.rs`, which is the reliable path
for modules that import siblings through the crate root.

Failure mode: A single-file Verus invocation can fail on unresolved sibling
imports even when the full crate root succeeds; capture the full-lib run for
archive evidence when imports are involved.

Snippet status: illustrative; use the full-lib command for archive evidence.

## Recipe: Update Tracey coverage

Goal: Keep requirements connected to implementation and verification evidence.

Files: Requirement prose lives in [`requirements.md`](requirements.md). Public
Rust APIs and tests live under [`src/`](../src/lib.rs) and [`tests/`](../tests/).

Commands:

```bash
tracey query uncovered
tracey query untested
./scripts/check-coverage.rs
```

Success: Tracey reports zero uncovered and zero untested requirements, and the
coverage checker accepts every public function.

Failure mode: Adding only Cairn/OpenSpec prose does not affect Tracey; code
traceability requires `docs/requirements.md` plus `r[impl ...]` and
`r[verify ...]` anchors in Tracey's configured source/test paths.

Snippet status: illustrative; run directly when traceability changes.

## Recipe: Handle a proof-gap boundary

Goal: Avoid hiding trusted runtime or proof assumptions.

Files: Review [`proof-gap-sites.tsv`](proof-gap-sites.tsv),
[`scripts/check-proof-gap-inventory.rs`](../scripts/check-proof-gap-inventory.rs),
and the accepted verification-gaps spec before adding any trusted marker.

Commands:

```bash
./scripts/check-proof-gap-inventory.rs
./scripts/check-ensures.rs
```

Success: The live source inventory matches the checked manifest, and any
intentional new trusted boundary is explained in the spec and site manifest.

Failure mode: Introducing `#[verifier::external_body]`, an assume bridge, or a
no-ensures marker without updating the inventory/spec leaves an untracked proof
gap and must fail review.

Snippet status: illustrative; run directly when proof policy changes.
