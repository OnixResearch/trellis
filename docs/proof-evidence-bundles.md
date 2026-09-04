# Proof-Evidence Bundles

Trellis exports a deterministic proof-evidence bundle for downstream consumers
that need machine-checkable proof facts without reinterpreting raw logs. The
bundle is an input to a policy engine or release gate; it is not release
approval, certification, whole-program correctness, third-party conformance, or
production-safety evidence by itself.

## Bundle shape

The checked-in bundle lives at
[`docs/proof-evidence-bundle.json`](proof-evidence-bundle.json). Each record
contains:

- `schema_version`: currently `trellis-proof-evidence-v1`.
- `scope`: the module or proof-example scope summarized by the record.
- `requirement_ids`: Tracey requirement IDs covered by both implementation and
  verification markers.
- `source_paths`: local files whose proof facts are summarized.
- `proof_functions`: exact local proof functions for profile-specific records.
- `algorithm_profile`: the exact algorithm profile, or `not-applicable`.
- `executable_oracle_symbol`: the downstream executable seam, or `not-applicable`.
- `downstream_validation_requirement`: the required consumer-side validation, or `not-applicable`.
- `proof_command`: the verifier command that checks the relevant proof surface.
- `verifier_toolchain`: the local verifier/coverage command family.
- `proof_outcome` and `tracey_coverage`: fail-closed status fields.
- `proof_gap_references`: required references into
  [`docs/proof-gap-sites.tsv`](proof-gap-sites.tsv) when a source file owns
  trusted-boundary entries.
- `maturity_facts`: labels derived from [`docs/module-index.md`](module-index.md)
  when available.
- `content_identities`: BLAKE3 identities for source/proof inputs and supporting
  requirement/proof-gap metadata.
- `claims` and `non_claims`: explicit scope boundaries so consumers do not treat
  the bundle as a release decision.

## Commands

Regenerate the checked-in bundle:

```bash
cargo run --quiet --bin proof_evidence -- write
```

Validate the checked-in bundle against local files and BLAKE3 identities:

```bash
cargo run --quiet --bin proof_evidence -- check
```

Run pure positive and negative validation fixtures:

```bash
cargo run --quiet --bin proof_evidence -- self-test
cargo test --quiet --bin proof_evidence
```

## Downstream seam

Trellis owns only the exported local proof facts and boundary references.
[`docs/proof-artifact-provenance.md`](proof-artifact-provenance.md) defines the
artifact identity and handoff contract for those local facts. Downstream gates
such as Octet own admission policy, artifact provenance, release approval,
publication, and any product-specific acceptance decision.
