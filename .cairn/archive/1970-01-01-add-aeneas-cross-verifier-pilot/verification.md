# Verification: add-aeneas-cross-verifier-pilot

r[verify trellis.aeneas_cross_verifier.final_validation]

Date: 2026-09-04

## Exact source and provider

The pilot selects `verified_logic::arithmetic::abs_diff` at `src/arithmetic.rs:21-25`.

The source file is 8,458 bytes. Its BLAKE3 identity is `b3:3064658eb9d27a1c1c423abbad9e9c8878756594f07a1d01eeb6b3efef4fa8a4`.

No copied or rewritten Rust function is used.

Octet revision `890f424cd7b1b2015f39c0eb5747f1bd1e576028` supplies the provider and consumer contract.

The checked provider identities are:

- profile `b3:cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625`
- compatibility matrix `b3:403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5`
- consumer contract `b3:63ea1e9e78916728a4a83719c29a6443a8952dbaadeeaae2a7420365afa7ff07`
- policy artifact `b3:cc07811243f39a70c08d80b2d145fbc97596644d56b74284d5f870d93826f605`

The profile uses Aeneas `bc2eb6f941369fa3c695e30598b5758b11e4f0de`, Charon `cb50ff16b9f1066b8a97dc06da704de2da2fa41c`, and Lean 4.31.0.

## Translation result

Charon runs with `--start-from crate::arithmetic::abs_diff` and Cargo `--lib`.

The LLBC result has `has_errors = false`, preserves borrow checking, and contains one function.

The target is `x86_64-unknown-linux-gnu`, with an 8-byte pointer and little-endian data.

The generated artifact identities are:

- LLBC `b3:85ad114c09426020bd1c0ba56bc5e71ba301b7552a5f6b0b9219357b671cb872`
- translation manifest `b3:c7df4ccc0437a026d9dff206324ff5d42f3cf1c4e8663022cc4cba34ddc536a4`
- generated Lean `b3:4fe6569363d2ffef0075c278a0a49b91ccfb83a9387826927df658a7e5e6644b`

The LLBC and translation rows share `def_id` 0 only within the measured run. Durable identities do not use that value.

## Theorem result

Lean passes both maintained theorems.

- symmetry source: `b3:a67c10d4699ae7b0ddb5b35eb87552b893fcf038c93fd5c6b4c776d5b172e3fb`
- self-zero source: `b3:b3df314ff917bb6eb9e26f7a8d56b7c5bc8439da5e5c5076c47a4bee58c7ade4`

The axiom audit reports `propext` for symmetry.

It reports `propext`, `Classical.choice`, and `Quot.sound` for self-zero.

The profile records those exact trust inputs. It records no selected opaque definition.

The Trellis proof receipt is `b3:da3840e394b1c671ec8b908e98a6ac52a054ab20b55a1cb48cceb1edae2c8d9b`.

The cross-verifier manifest is `b3:93a5d7aa36a941648edce19b1862c1fae847143cdf8e0e43092992564b91b95d`.

## Negative evidence

Lean rejects the intentionally false theorem with an unsolved goal.

Lean accepts the vacuous `True` theorem as an encoded theorem.

Octet adequacy rejects that vacuous theorem before proof promotion.

The pure checker passes one positive profile and 18 negative cases.

The cases cover provider, command, source, LLBC, target, translation, `def_id`, generated output, theorem, trust, budget, receipt, artifact, and claim drift.

The outcome classifier maps only a complete reviewed unsupported observation to blocked status.

It maps setup, process, policy, artifact, timeout, and budget errors to failed status.

## Passing gates

- The focused arithmetic Cargo tests pass.
- The focused arithmetic Verus proof passes.
- The checker passes `--check` and `--self-test`.
- The checker passes temporary-package Clippy with `-D warnings`.
- The full Trellis wrapper passes 11,098 Verus checks and 5,216 library tests.
- README metrics confirm 2,264 requirements, 624 modules, and 16 trusted or proof-gap boundaries.
- Tracey reports 0 uncovered and 0 untested rules out of 2,264 rules.
- The proof bundle validates 7 records at `b3:c2497e655340de0a52cc85bd72e0b7c627dc2f881ce87110a8b2e3c7a1dc8be6`.
- The proof IR chain validates 7 records at `b3:0f21d73530dca19d25ccb1a48a2b639357b19b245d0f8d54e6d081274c1cf43d`.
- The exact Aeneas Nix check regenerates and compares LLBC, translation, Lean, theorem, axiom, and negative outputs.
- Octet artifact verification accepts the pinned provider fixture bundle.
- The pinned Trellis Octet gate reports 3,867 accepted and current findings, with zero new and zero stale findings.
- Cairn proposal, design, tasks, review-completeness, and strict validation gates pass with the canonical policy.

The crate-root Tracey bridge changed one existing `excessive_file_length` identity.

A guarded migration replaces `b3:69dd8f7e6f7922b2` with `b3:0a9b4eca7243e493`. No other accepted finding changes.

## Non-claims

The manifest links two separate scoped proof facts to one source identity.

It does not prove Verus and Aeneas semantic equivalence or verifier independence.

It does not prove Aeneas, Charon, Lean, Verus, rustc, or solver soundness.

It does not prove whole-program correctness, downstream behavior, or release eligibility.
