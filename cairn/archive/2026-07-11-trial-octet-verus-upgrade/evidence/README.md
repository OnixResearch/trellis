# Octet Verus upgrade trial evidence

The deterministic receipt in `trial/receipt.json` records terminal result
`blocked` with BLAKE3 digest
`5e653411b60f42356d3d6f43b43d477785ac18817e6f9d468a1b36b4157f6c4a`.

Run the hermetic rail with:

```console
nix build .#checks.x86_64-linux.verus-upgrade-trial --no-link -L
```

The rail validates the Nickel matrix and adversarial fixtures, checks the exact
Octet profile, runs the current Trellis baseline and candidate over one
immutable proof-source input, executes or audits all five catalog rows, creates
a canonical BLAKE3 receipt, and recursively compares every generated artifact
to `trial/`.

## Result

- Current Trellis verifier: `10710 verified, 0 errors`.
- Octet candidate verifier: `10709 verified, 1 errors`.
- Candidate failure: postcondition at `src/nix_drv_serialize.rs:761:9`.
- IronKV: removed `vstd::imap` and `vstd::iset_lib` APIs.
- Verified memory allocator: removed `ptr_mut_ref` API after exact libc admission.
- Verified node replication: `var_local_id failed: (800, 132)` internal error.
- SOSP microcases: diagnostic-only source API failures; no full artifact claim.
- VerusBelt: Rocq semantic-reference audit only; candidate Verus was not run.

No probe timed out. The Trellis source was not mutated, no trigger was changed,
and no proof or trusted boundary transfers. Source-level assumption drift is
excluded by the identical immutable source input. Verifier-library and blocked
corpus assumption drift remain explicitly unevaluated rather than being claimed
away.

## Smallest next checks

- Trellis: isolate the unequal-length serialization branch at line 761, prove
  the missing candidate-era sequence-length fact without weakening the
  postcondition, then rerun the candidate crate command.
- IronKV: port only the `vstd::imap` and `vstd::iset_lib` imports in a disposable
  source copy, then rerun `ironsht/src/lib.rs` before considering wider changes.
- Allocator: map the first `ptr_mut_ref` use to the candidate's current raw
  pointer API, then rerun `verus-mimalloc/lib.rs` with the admitted libc rlib.
- Node replication: reduce the `rwlock.rs` `var_local_id` failure and test that
  reproducer against the exact Octet verifier before any runtime or benchmark
  work.

## Artifact classes

- `upgrade-trial.json`, `catalog.json`, and `octet-profile.json`: typed inputs.
- `*-identity.txt`: verifier/toolchain identity evidence.
- `*.stdout` and `*.stderr`: bounded, path-normalized execution transcripts.
- `payload.json` and `payload.canonical.json`: receipt payload forms.
- `receipt.json`: BLAKE3-bound decision receipt.
- `portfolio.md`: approach-family registry and adversarial audit.

These artifacts establish only the bounded compatibility result. They do not
establish verifier, solver, compiler, Rocq, or upstream soundness.
