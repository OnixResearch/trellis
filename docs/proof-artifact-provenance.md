# Proof Artifact Provenance

Trellis proof artifacts are local proof facts. An exportable artifact records the
identity of the exact Trellis content and verifier receipt that produced the fact,
plus a boundary that prevents downstream overclaiming.

## Required metadata

Each exportable artifact carries:

- stable artifact ID;
- canonical BLAKE3 metadata digest;
- BLAKE3 source/proof input digests;
- BLAKE3 verifier receipt digest;
- verifier/toolchain identity and normalized invocation;
- declared Trellis-local property ID;
- exact content scope checked by the verifier;
- verification status, with a failure reason when the local proof did not pass;
- explicit non-claims excluding downstream certification, release eligibility,
  verifier soundness, whole-stack correctness, and production approval.

Paths and display names are review metadata only. They are not primary identity.

## Handoff boundary

Octet, Valence, Mantle, and Cairn may cite Trellis proof artifact identities as
manual-review or reference inputs. They do not inherit a Trellis proof as their
own certification. A downstream repo must rerun its own verifier or record its
own receipt before labeling the fact downstream-certified.

Trellis rejects metadata that treats a Trellis-local proof as Octet source-code
certification, Mantle release eligibility, Cairn gate satisfaction, or whole-stack
correctness.

## Example

```text
artifact_id: trellis.properties.depth_monotone_resolution
local_property_id: properties.web-of-trust.depth-monotone
content_scope: src/properties.rs::depth_monotone_resolution
status: passing
role: manual_review_input
non_claims:
  - not downstream-certified
  - not release eligibility
  - not whole-stack correctness
  - not verifier soundness proof
```

A downstream consumer can reference this identity during review. It becomes a
downstream-certified claim only after the downstream repo records its own verifier
receipt and states the narrower downstream role in its own evidence bundle.

## Validation

The pure validation core lives in `src/proof/artifact/provenance.rs`. The runtime
shell supplies digest-match facts and parsed metadata; the core admits or rejects
identity, local-property scope, non-claims, and handoff roles without reading
files, running tools, or computing hashes.
