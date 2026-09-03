# Baseline evidence

The baseline source is `b957d58f7c55f1d3d77c1043b4927d9e64863e25`.
No MIR-core implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The first direct `./scripts/verus.sh src/lib.rs --triggers-mode silent` run
  reached 10,875 proofs, then hit two resource limits in the pre-existing
  `src/nix_drv_serialize.rs` module.
- `nix build .#checks.<system>.verus-proofs --no-link --print-out-paths`
  succeeded for the unchanged Git source. Its stored log reports 10,877
  verified obligations and zero errors.

The failed direct transcript is retained because it records the transient
resource limit. The successful Nix transcript remains the baseline verdict.
