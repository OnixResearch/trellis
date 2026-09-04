# Aeneas cross-verifier pilot

<!-- r[impl trellis.aeneas_cross_verifier.claim_boundary] -->
<!-- r[impl trellis.aeneas_cross_verifier.generated_boundary] -->
<!-- r[impl trellis.aeneas_cross_verifier.manifest] -->

## Scope

This pilot selects the runtime function at `src/arithmetic.rs:21-25`.

The selected function is `verified_logic::arithmetic::abs_diff`. Its source-file identity is `b3:3064658eb9d27a1c1c423abbad9e9c8878756594f07a1d01eeb6b3efef4fa8a4`.

Charon uses `--start-from crate::arithmetic::abs_diff` and Cargo `--lib`. It emits one function declaration.

The pilot does not copy, rewrite, or manually project the Rust function.

## Ownership boundary

The pure Rust checker validates loaded profiles, artifacts, receipts, and manifests. It does not run external tools or read ambient state.

The Nix check owns source copying, provider execution, bounded logs, generated files, and process results.

Trellis owns theorem intent, requirement mapping, its receipt, and its cross-verifier manifest.

Octet owns the provider package, adequacy policy, compatibility matrix, and consumer contract.

## Pinned provider

The provider comes from Octet revision `890f424cd7b1b2015f39c0eb5747f1bd1e576028`.

The relevant identities are:

- provider profile: `b3:cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625`
- compatibility matrix: `b3:403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5`
- consumer contract: `b3:63ea1e9e78916728a4a83719c29a6443a8952dbaadeeaae2a7420365afa7ff07`
- policy artifact: `b3:cc07811243f39a70c08d80b2d145fbc97596644d56b74284d5f870d93826f605`
- Aeneas revision: `bc2eb6f941369fa3c695e30598b5758b11e4f0de`
- Charon revision: `cb50ff16b9f1066b8a97dc06da704de2da2fa41c`
- Lean toolchain: `leanprover/lean4:v4.31.0`

Trellis keeps its existing Octet Verus pin separate. The Aeneas input cannot change Verus trial authority.

## Generated boundary

Aeneas-generated files are in `aeneas-pilot/generated/`.

- `llbc.llbc`: `b3:85ad114c09426020bd1c0ba56bc5e71ba301b7552a5f6b0b9219357b671cb872`
- `translation.json`: `b3:c7df4ccc0437a026d9dff206324ff5d42f3cf1c4e8663022cc4cba34ddc536a4`
- `Llbc.lean`: `b3:4fe6569363d2ffef0075c278a0a49b91ccfb83a9387826927df658a7e5e6644b`

Maintained theorem files are in `aeneas-pilot/theorems/`. Regeneration never writes to that directory.

`translation.json` joins LLBC declaration `def_id` 0 to the generated Lean declaration. This number is only a current-run join key.

The durable run identity uses the three generated artifact identities. It does not use `def_id`.

## Theorem mapping

| Property | Requirement | Verus symbol | Lean symbol | Maintained source identity |
| --- | --- | --- | --- | --- |
| symmetry | `arithmetic.abs-diff` | `arithmetic::abs_diff_symmetric` | `trellis_abs_diff_symmetric` | `b3:a67c10d4699ae7b0ddb5b35eb87552b893fcf038c93fd5c6b4c776d5b172e3fb` |
| self-zero | `arithmetic.abs-diff` | `arithmetic::abs_diff_self_is_zero` | `trellis_abs_diff_self_is_zero` | `b3:b3df314ff917bb6eb9e26f7a8d56b7c5bc8439da5e5c5076c47a4bee58c7ade4` |

Both Lean theorems use `Aeneas.Std.U64` on the recorded 64-bit little-endian target.

The branch selects only an in-range unsigned subtraction. Self subtraction returns zero.

## Trust closure

Lean reports these axioms for the selected theorem declarations:

- symmetry: `propext`
- self-zero: `propext`, `Classical.choice`, and `Quot.sound`

The profile records these names and deterministic trust-input identities. It records no selected opaque definition.

The wider provider library contains declarations outside this selected proof path. Their authority does not transfer into this theorem claim.

## Resource policy

`aeneas-pilot/resource-policy.ncl` owns the typed resource policy.

Each stage has a 900-second timeout. Each retained output stream is limited to 1,048,576 bytes.

Each retained generated artifact is limited to 8,388,608 bytes. The network namespace is denied.

A timeout, process error, policy error, or budget exhaustion is a failed result. It is not an unsupported-language result.

## Negative evidence

The positive rail checks both maintained theorems. The negative rail checks distinct failure paths.

- Lean rejects `FalseTheorem.lean` with an unsolved goal.
- Lean accepts `VacuousTheorem.lean` as syntax and proof.
- Octet adequacy rejects that vacuous theorem before proof promotion.
- The checker runs 18 profile, artifact, trust, receipt, and claim-boundary mutations.
- A complete reviewed unsupported observation maps to blocked status.
- Setup, process, artifact, timeout, and budget errors map to failed status.

## Commands

Run the pure checks:

```console
./scripts/check-aeneas-cross-verifier.rs --check
./scripts/check-aeneas-cross-verifier.rs --self-test
```

Run the exact provider rail:

```console
NIX_CONFIG='builders =' nix build .#checks.$(nix eval --raw --impure --expr 'builtins.currentSystem').aeneas-cross-verifier-pilot --no-link -L
```

The Nix check regenerates LLBC and Lean output in a private work root. It compares every generated byte with the checked artifacts.

## Non-claims

This pilot records two separately checked property statements over one exact runtime function.

It does not prove Aeneas, Charon, Lean, Verus, rustc, or solver soundness.

It does not prove semantic equivalence between the Verus specification and the Aeneas model.

It does not prove verifier independence, whole-program correctness, downstream behavior, or release eligibility.
