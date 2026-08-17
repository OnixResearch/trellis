# Nickel 1.17 proof-configuration evidence

## Result

Trellis now supplies Nickel CLI `1.17.0` from `nickel-lang/nickel` revision `1320a983e6c3d1e2fb53dd2464b084b4903b1426` to proof profiles, evidence contracts, fixtures, and developer tools.

Nix generated the lockfile update. The cohort guard checks the source revision, resolved executable, full version output, and absence of ambient `pkgs.nickel` references.

## Compatibility and evidence

`checks.x86_64-linux.nickel-toolchain-fixtures` passed.

Positive fixtures cover proof profiles, evidence contracts, imports, and the exact evaluator identity. Negative fixtures cover malformed source, missing imports, contract failures, unknown fields, identity mismatch, cohort drift, evaluator substitution, and downstream-correctness overclaims.

`verification-policy/generated/nickel-evaluator.json` records:

- identity: `nickel-lang-cli nickel 1.17.0 (rev 1320a98)`
- version: `1.17.0`
- source commit: `1320a983e6c3d1e2fb53dd2464b084b4903b1426`
- proof authority: `named-proof-tools`

The `verification-policy` Nix check passed under Nickel 1.17. It refreshed and checked verification, proof-review, and Cairn policy outputs plus their positive and negative fixtures.

## Proof and code checks

`checks.x86_64-linux.verus-proofs` passed with:

```text
verification results:: 10752 verified, 0 errors
```

This establishes that accepted Nickel configuration did not replace the named Verus proof tool.

`checks.x86_64-linux.clippy` passed with warnings denied.

The repository format check reached inherited formatting drift in unchanged `src/bin/proof_evidence_ir.rs` and `src/typed_durable_reference.rs`. No changed file appeared in the format diff. This change does not reformat unrelated implementation work.

## Cairn policy compatibility repair

The previous generated Cairn policy used an obsolete workflow-schema shape. Current Cairn first rejected it for missing `output_path` and then for missing `workflow_profile_policy`.

Trellis now layers its traceability and optional stack-evidence gates over the current checked Cairn runtime projection from Cairn revision `b6759012db4dab0158936e995b7bad39ac10465b`.

The local Nickel contract admits current workflow artifacts and markers while remaining closed over Trellis-owned legacy fields. Policy freshness, Cairn validation, proposal gate, and design gate pass.

## Non-claims

Nickel acceptance proves tested configuration shape and evaluator identity only.

It does not prove a theorem, replace Verus or Rust checks, prove proof-tool soundness, establish evidence truth, or prove downstream correctness.
