# Verification: adopt-standalone-nickel-export-shell

r[verify trellis.nickel_export_cutover.validation]

Date: 2026-09-04

## Exact source and evaluator

Trellis pins `OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49`.

The release rail uses Nickel `1.17.0` from revision `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.

The evaluator reports `nickel-lang-cli nickel 1.17.0 (rev 1320a98)`. Every request preserves `contract = ""`.

The typed profile source is `b3:1711fb7af1eb01e550798ebd86cda08f7608e57a921b0efe9374c1330f8bff20`.

The generated profile is `b3:5deb0ddfd9cdedab868a67f9d438fca823980f675d48256c54ec968f92c9f98a`.

## Exact output parity

The standalone checker validates nine output families and 31 existing invalid fixtures.

Important exact outputs include:

- verification policy: 1,245 bytes, `b3:064615f9e4034e337e102e51e5fb9c010d82137b9f4a4f64ea9113aee4e35ab3`;
- lifecycle YAML: 742 bytes, `b3:9d4c6f2b7db5b6960de54faf69367ab45a85b53e6a41c35975b476b77d539daf`;
- proof-review policy: 6,886 bytes, `b3:7e658d97c1c7959b237e6dc0be2a3665825dd3264221d1b5bdfbd8c592cc6aba`;
- Cairn policy: 162,466 bytes, `b3:0502fecf2ca0a41716048fd51484553d86b68a14e62218aeb3715fb60d886941`;
- Octet baseline contract: `b3:fb0cce6ce1fe45cd678b55e646ce4c1430eefa86901a9dd09560088b80fd2deb`;
- proof-evidence contract: `b3:a16019a11aa30f3e1b8d267d839ff215639e90843ccae6a5e14d68ad5d69c376`.

The checker compares each output by exact bytes and BLAKE3 identity. It validates each canonical manifest identity independently.

Nickel diagnostics contain process-local closure addresses. The checker removes only `0x…` addresses before it compares the remaining diagnostic text.

## Authority and rollback

The standalone shell owns generic request admission, evaluator execution, identities, manifests, writes, errors, and freshness.

Trellis retains policy meaning, contracts, destinations, invalid fixtures, Verus, Tracey, proof claims, and release decisions.

The authority baseline is `b3:71c7f2c3a4d1d5152c57ebbf7146c0d9bac811d7dfcdd8ae609e47e828d4e57d`.

Trace references live in `src/proof.rs`. They add no executable filesystem, process, environment, or evaluator authority.

Octet remains unchanged at 3,867 accepted and current findings. It reports zero new and zero stale findings.

The legacy policy and lifecycle checkers pass their explicit `--legacy-parity` modes. Their semantic and negative fixture checks remain required.

Six typed exceptions allow only reviewed Cairn-policy vocabulary. They bind exact family, path, literal, occurrence, and rationale values.

## Negative evidence

The standalone self-test passes 12 negative controls.

It covers malformed requests, invalid contracts, stale outputs, manifest tampering, path escape, symlinks, evaluator drift, secret markers, overclaims, authority drift, and exception drift.

The current invalid fixture cohort has 31 entries. Legacy and standalone paths agree on failure class and normalized diagnostic text.

## Passing gates

- The full Trellis wrapper ends with `=== ALL CHECKS PASSED ===`.
- The library suite passes 5,216 tests, plus positive and negative fixture suites.
- Strict Clippy passes with `-D warnings`.
- Verus passes the unchanged 11,098-proof corpus.
- Tracey reports 2,270 covered and verified requirements, with zero uncovered and zero untested.
- The proof bundle validates seven records at file identity `b3:e58a90561093f7ae28adfe8eb56c16a618c4e99e056af8822633609d5201a70e`.
- The proof IR chain validates seven records at file identity `b3:8e7ae10563470774e20188bd2fbbaab527b73511bd71209340b7e0818ea914e9`.
- The pinned Nickel export Nix rail passes nine outputs, 31 invalid fixtures, and 12 negative controls.
- The pinned verification-policy and Octet Nix rails pass.
- The exact Aeneas Nix rail passes with receipt `b3:baa04429d7c1eab4e0a984017f5a29bd02de7ced8aeeebe07bfb003027bcbc04`.
- The refreshed cross-verifier manifest is `b3:1c1ffd9ccbd8c2cd8de455ed4da6b9cc685fd589d0dc5e8c3c007c34046d49f9`.

## Non-claims

Matching exports do not prove evaluator equivalence, policy correctness, theorem correctness, build correctness, deployability, or release eligibility.

The shared receipt does not replace any Trellis proof, semantic, fixture, or release gate.
