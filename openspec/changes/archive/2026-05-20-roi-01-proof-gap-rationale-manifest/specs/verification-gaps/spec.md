# Verification Gaps Specification

## ADDED Requirements

### Requirement: Proof-Gap Manifest Rationale Source
The proof-gap site manifest MUST include a concise rationale for every expected trusted or weakened boundary so the checked inventory explains why each remaining boundary is acceptable.

r[verification-gaps.manifest-rationale-source]

#### Scenario: Manifest rows include rationale

- GIVEN `docs/proof-gap-sites.tsv` is parsed
- WHEN a non-comment row lists an expected proof-gap site
- THEN the row contains path, function signature, classification, and rationale fields
- AND the rationale field is non-empty after trimming whitespace

#### Scenario: Rationale remains tied to exact sites

- GIVEN a live `#[verifier::external_body]` site is expected
- WHEN the manifest row for that site is checked
- THEN the rationale is associated with the same path and function signature used for exact-site drift detection

### Requirement: Proof-Gap Manifest Rationale Drift Gate
The proof-gap inventory checker MUST reject rationale manifest drift before full verification can pass.

r[verification-gaps.manifest-rationale-drift-gate]

#### Scenario: Current rationale manifest passes

- GIVEN the current source tree and rationale-bearing proof-gap manifest
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/` and reads `docs/proof-gap-sites.tsv`
- THEN every expected live site has a non-empty rationale
- AND the checker exits successfully only when counts, exact sites, classifications, and rationales are structurally valid

#### Scenario: Missing rationale fails closed

- GIVEN the checker runs a controlled fixture or self-test with a manifest row missing its rationale field
- WHEN manifest validation executes
- THEN the checker exits non-zero with a manifest validation diagnostic
- AND no source files are mutated

#### Scenario: Blank rationale fails closed

- GIVEN the checker runs a controlled fixture or self-test with an empty or whitespace-only rationale field
- WHEN manifest validation executes
- THEN the checker exits non-zero with a manifest validation diagnostic
- AND no source files are mutated

#### Scenario: Full verification enforces rationale validation

- GIVEN `./scripts/verify.rs` runs
- WHEN it invokes the proof-gap inventory stage
- THEN rationale validation runs before full verification can pass

### Requirement: Published Proof-Gap Rationale Alignment
Published proof-gap prose MUST point reviewers to the checked manifest as the authoritative per-site rationale source after rationale fields are introduced.

r[verification-gaps.manifest-rationale-publication]

#### Scenario: README describes the checked rationale source

- GIVEN the rationale manifest change is complete
- WHEN README proof-gap documentation is reviewed
- THEN it identifies `docs/proof-gap-sites.tsv` as the checked source for per-site trusted-boundary rationale

#### Scenario: Canonical spec records rationale validation

- GIVEN the change is archived
- WHEN `openspec/specs/verification-gaps/spec.md` is reviewed
- THEN it contains the rationale manifest and drift-gate requirements
