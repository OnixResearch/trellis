# Runnable Proof Examples

These examples show the smallest Trellis proof loop: write a `spec fn`, refine a
runtime `exec fn` with `ensures`, and keep failing proof shapes isolated behind an
expected-failure command. They are local, deterministic, and do not add trusted
bodies or `assume(...)` soundness oracles.

## Command inventory

| Example | Source | Positive command | Runtime command | Negative / opt-in command | Expected outcome |
|---|---|---|---|---|---|
| Basic refinement | [`tests/proof_examples/basic_refinement.rs`](../tests/proof_examples/basic_refinement.rs) | `./scripts/verus.sh tests/proof_examples/basic_refinement.rs` | Not applicable; this is a proof-only Verus fixture. | `./scripts/verus.sh tests/proof_examples/fixtures/broken_refinement.rs` | The positive command verifies; the negative command is expected to fail with an `ensures`/postcondition diagnostic. |

Run the maintained wrapper when you want both the positive and negative evidence
in one command:

```bash
./scripts/check-proof-examples.sh
```

The wrapper stores logs under `target-test/proof-examples/`, verifies the
positive fixture, runs the broken fixture, and treats the documented Verus failure
as a successful negative check.

## Coverage

The positive fixture carries Tracey implementation and verification markers for
`runnable-proof-examples.basic` and `runnable-proof-examples.commands`. The broken
fixture intentionally carries no coverage markers so a failing file cannot satisfy
Tracey coverage by accident.
