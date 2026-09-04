# Nickel evaluator cohort

Trellis evaluates proof profiles and evidence contracts with Nickel CLI `1.17.0` from `nickel-lang/nickel` revision `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.

The Nix flake supplies this exact evaluator to proof profiles, evidence contracts, fixtures, and developer tools. The cohort guard rejects ambient package-set references and checks the resolved executable and full version output.

Nickel-dependent evidence records identity `nickel-lang-cli nickel 1.17.0 (rev 1320a98)`. Substituted evaluator identities and downstream-correctness overclaims fail closed.

The standalone export rail pins `OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49`. Its requests use the evaluator token `1.17.0` and preserve `contract = ""`.

Trellis still owns policy meaning and proof gates. See [the cutover guide](nickel-export-cutover.md) for requests, rollback, and non-claims.

## Validation

```console
nix build .#checks.x86_64-linux.nickel-cohort-exact --no-link -L
nix build .#checks.x86_64-linux.nickel-toolchain-fixtures --no-link -L
nix build .#checks.x86_64-linux.verification-policy --no-link -L
nix build .#checks.x86_64-linux.nickel-export-cutover --no-link -L
nix build .#checks.x86_64-linux.verus-proofs --no-link -L
```

A passing Nickel profile proves configuration shape only. The named proof tool must still establish its scoped result. Neither result proves downstream correctness.
