# Verification Gaps Specification

## Purpose

This specification tracks source-derived proof-gap and trusted-boundary inventory requirements. It preserves the closed UCAN DID varint boundary requirement and records the current audit policy for remaining trusted cryptographic, codec, runtime, and solver-limit bridges.
## Requirements
### Requirement: UCAN DID Varint Decode Boundary Is Closed
r[verification-gaps.close-ucan-did-varint-decode]

`src/ucan_did.rs::varint_decode` MUST no longer be a trusted or weakened proof boundary. The implementation MUST remove its `#[verifier::external_body]` attribute and MUST NOT replace it with `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or a runtime-only `@verified-logic:exempt` marker.


#### Scenario: Source audit finds no trusted varint decoder bridge

- GIVEN the implementation is complete
- WHEN `src/ucan_did.rs` is audited around `varint_decode`
- THEN `varint_decode` has a normal verified postcondition tying the result to `varint_decode_spec`
- AND no trusted or exempt marker appears on that function

#### Scenario: Decoder helper path has no replacement trust

- GIVEN the implementation is complete
- WHEN the `src/ucan_did.rs` decoder helper path is audited, including `varint_decode`, bounded decode helpers, and bounded decode lemmas
- THEN none of those items uses `#[verifier::external_body]`, `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or `@verified-logic:exempt`

#### Scenario: Verus proves the decoder

- GIVEN the implementation is complete
- WHEN `./scripts/verus.sh src/ucan_did.rs` and `./scripts/verus.sh src/lib.rs` run
- THEN both verifier runs pass without a `varint_decode` external body

### Requirement: Proof-Gap Inventory Reflects The Closed Boundary
r[verification-gaps.inventory-after-ucan-did-varint]

The repo-wide verification-gap inventory MUST remove the closeable row for `src/ucan_did.rs::varint_decode` and update all source-derived published totals. With the UCAN attenuation-chain checker, ability-subsumption checker, principal-link DID extraction bridge, SSA runtime-helper wrappers, instruction-selection public API surfaces, equal-input hash determinism / SHA-256 width facts, recursive `cv_fuel` proof lemmas, log-matching leader-append preservation lemmas, CID multibase round-trip bridge, and web-of-trust attestation/revocation signature-input checks verified, the current expected inventory is `16` total trusted/weakened/runtime-only boundaries: `16` `#[verifier::external_body]` sites, `0` `assume_specification[...]` bridges, and `0` instruction-selection runtime-only exemptions. No `@verified-logic:no-ensures` markers remain.


#### Scenario: Counts are derived from source

- GIVEN the implementation is complete
- WHEN the count evidence runs `rg` audits for `external_body`, `assume_specification[...]`, `@verified-logic:no-ensures`, and instruction-selection runtime-only exemptions
- THEN `README.md`, `openspec/specs/verification-gaps/spec.md`, and captured evidence publish the same non-double-counted total
- AND the evidence lists any overlapping category sites as non-additive

#### Scenario: Tracey coverage stays green

- GIVEN `docs/requirements.md` and code annotations are updated for this change with the new or clarified `r[...]` IDs
- WHEN `tracey query status` starts or confirms the daemon and then `tracey query uncovered` and `tracey query untested` run sequentially
- THEN both coverage queries report zero missing coverage items

### Requirement: Source-Derived Proof-Gap Boundary Inventory
r[verification-gaps.boundary-audit.inventory]

The system MUST maintain a source-derived inventory of remaining trusted or weakened proof boundaries before any proof-gap closeout attempt updates published counts.


#### Scenario: Inventory lists every trusted boundary

- GIVEN the audit command scans `src/` for `#[verifier::external_body]`
- WHEN proof-gap inventory evidence is captured
- THEN the evidence lists every matching file, function, and total count
- AND it separately reports counts for `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and runtime-only exemption markers

### Requirement: Permanent And Closeable Boundary Classification
r[verification-gaps.boundary-audit.classification]

The system MUST classify each remaining proof boundary as permanent trust or a closeable structural candidate before implementation work removes, rewrites, or re-counts the boundary.


#### Scenario: Cryptographic runtime trust is not overclaimed

- GIVEN a boundary represents runtime cryptography, digest implementation behavior, codec correctness, or collision resistance
- WHEN the boundary is classified
- THEN the classification records it as permanent trust unless a concrete structural wrapper proof is scoped without claiming cryptographic strength

#### Scenario: Structural candidates include proof plan

- GIVEN a boundary is classified as closeable
- WHEN implementation begins
- THEN the audit records the finite structural property to prove, the touched module, and the focused Verus/Tracey evidence required for closure

### Requirement: Bounded Closeout Slice
r[verification-gaps.boundary-audit.closeout-slice]

The system MUST drain at most one selected closeable boundary family per proof-gap audit closeout slice, and MUST NOT introduce `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, runtime-only exemption markers, or new `#[verifier::external_body]` sites as replacements.


#### Scenario: Failed closeout is reclassified rather than hidden

- GIVEN the selected closeable boundary cannot be closed with a focused proof
- WHEN the closeout attempt is abandoned
- THEN the change records the blocker and classification update
- AND no replacement weakened marker is added

### Requirement: Published Inventory Synchronization
r[verification-gaps.boundary-audit.inventory-sync]

The system MUST synchronize README, requirements, and canonical OpenSpec proof-gap totals from source-derived count evidence whenever the trusted-boundary inventory changes.


#### Scenario: Published counts match source audit

- GIVEN a closeout changes the number of trusted or weakened proof boundaries
- WHEN the change is verified
- THEN README, `docs/requirements.md`, `openspec/specs/verification-gaps/spec.md`, and captured evidence report the same non-double-counted totals
- AND Tracey uncovered and untested queries remain green

### Requirement: Proof-Gap Inventory Drift Gate
r[verification-gaps.inventory-drift-gate]

The system MUST provide a Rust-script inventory gate that scans source files for trusted or weakened proof-gap markers and fails when live counts drift from the published expected inventory.


#### Scenario: Current source inventory passes

- GIVEN the current source tree and expected inventory counts
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN it reports 16 `#[verifier::external_body]` sites
- AND it reports zero `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and `@verified-logic:exempt` markers
- AND it exits successfully

#### Scenario: Added trusted marker fails closed

- GIVEN the Rust inventory script runs in its controlled extra-marker fixture mode
- WHEN the simulated live inventory contains one additional `#[verifier::external_body]` marker
- THEN the script exits non-zero
- AND the evidence records the expected-count mismatch without mutating source files

#### Scenario: Full verification includes the drift gate

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.rs` reaches the proof-gap inventory stage
- THEN it invokes the Rust inventory script before cargo, Verus, and Tracey checks claim success

### Requirement: Proof-Gap Site-List Drift Gate
r[verification-gaps.inventory-site-list-drift-gate]

The system MUST validate the exact current `#[verifier::external_body]` site list, not only the aggregate count, so same-count proof-gap substitutions fail closed.


#### Scenario: Current external-body site list passes

- GIVEN the current audited external-body baseline of 16 file/function sites
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN the script reports the expected 16 sites
- AND it exits successfully when every live site matches the expected file/function list

#### Scenario: Same-count site substitution fails closed

- GIVEN the Rust inventory script runs in a controlled same-count site-substitution fixture mode
- WHEN the simulated live inventory preserves the external-body count but replaces one expected site with an unexpected site
- THEN the script exits non-zero
- AND the evidence records the missing expected site and unexpected simulated site without mutating source files

#### Scenario: Full verification enforces exact sites

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.rs` invokes the proof-gap inventory drift stage
- THEN the Rust checker validates both marker counts and the exact external-body site list before full verification can pass

### Requirement: Proof-Gap Classification Drift Gate
r[verification-gaps.inventory-classification-drift-gate]

The system MUST validate the classification attached to every expected proof-gap site, so permanent trust and closeable-candidate labels cannot silently drift independently of the exact site list.


#### Scenario: Current classifications pass

- GIVEN the current audited 16-site proof-gap baseline with classifications
- WHEN the Rust inventory checker validates classifications
- THEN every live site matches its expected classification and the checker exits successfully

#### Scenario: Classification substitution fails closed

- GIVEN a controlled fixture that preserves the exact site list but changes one classification
- WHEN the Rust checker evaluates classification drift
- THEN it exits non-zero and reports the changed classification without mutating source files

#### Scenario: Full verification enforces classifications

- GIVEN `./scripts/verify.rs` runs
- WHEN it invokes the proof-gap inventory drift stage
- THEN classification drift is checked before full verification can pass

### Requirement: Proof-Gap Site Manifest Source
r[verification-gaps.inventory-site-manifest-source]

The system MUST load the expected proof-gap external-body site baseline from a checked repository manifest and fail closed when the manifest is malformed, stale, or inconsistent with live source.


#### Scenario: Manifest and source match

- GIVEN a checked manifest listing the current 16 expected external-body sites
- WHEN the Rust inventory checker scans source and reads the manifest
- THEN validation succeeds only if the manifest and live site list match exactly

#### Scenario: Malformed manifest fails closed

- GIVEN a manifest with duplicate, missing, malformed, or unsorted site entries
- WHEN the Rust checker reads the manifest
- THEN it exits non-zero with a manifest validation diagnostic

#### Scenario: Stale manifest fails closed

- GIVEN live source changes one external-body site but the manifest is not updated
- WHEN the Rust checker compares manifest and source
- THEN it rejects the drift before full verification can pass

### Requirement: Nix Version Bridge Decomposition
r[verification-gaps.nix-version-bridge-decomposition]
The system MUST provide a bounded structural decomposition of `src/nix_version.rs::compare_versions` that routes admitted finite-model inputs through verified executable comparison helpers while keeping out-of-model inputs on an explicit fallback trusted boundary.


#### Scenario: Finite comparator agrees with modeled cases

- GIVEN admitted finite version byte sequences within the modeled token rules
- WHEN the modeled comparison wrapper compares them
- THEN the result is delegated through the existing compare specification with an explicit admitted-input precondition

#### Scenario: Out-of-model input boundary is explicit

- GIVEN version bytes outside the admitted finite model
- WHEN comparison admission is evaluated
- THEN the wrapper returns no ordering claim instead of pretending the input was verified by the finite model

#### Scenario: Public API narrows the trusted bridge

- GIVEN admitted finite-model inputs within the public `compare_versions` API preconditions
- WHEN `compare_versions` evaluates them
- THEN it uses the verified modeled executable path and the trusted inventory retains only the unmodeled fallback boundary

### Requirement: Nix Version Modeled Compare Bridge
r[verification-gaps.nix-version-modeled-compare-bridge]
The system MUST compute admitted finite-model Nix version comparisons through verified executable helpers for separator skipping, component bounds, numeric parsing, and lexicographic comparison. The modeled wrapper MUST return an ordering matching `compare_versions_spec` for admitted inputs, return no ordering claim for out-of-model inputs, and keep only the unmodeled fallback external-body trusted boundary visible for the broader API.


#### Scenario: Admitted modeled comparison is executable evidence

- GIVEN two admitted finite-model version byte sequences
- WHEN `compare_versions_modeled` evaluates them
- THEN the admitted branch computes through the verified modeled comparator path
- AND the result is `Some(-1)`, `Some(0)`, or `Some(1)` matching `compare_versions_spec`

#### Scenario: Out-of-model input boundary remains explicit

- GIVEN either version byte sequence is outside the admitted finite model
- WHEN `compare_versions_modeled` evaluates admission
- THEN it returns `None` with no ordering claim

#### Scenario: Trusted inventory remains visible

- GIVEN the public `compare_versions` bridge has been narrowed to admitted finite-model correctness
- WHEN proof-gap inventory evidence is captured
- THEN the trusted-boundary count is unchanged and the site list includes `src/nix_version.rs::compare_versions_unmodeled_trusted`
- AND no replacement weakened marker or additional external body is introduced

### Requirement: Single Trusted Boundary Closure Slice
r[verification-gaps.single-boundary-closure]
A trusted-boundary closure change MUST select one tracked proof-gap site or one tightly related site family, justify why it is closeable or narrowable, and update source, inventory, and evidence together.


#### Scenario: Target is selected from current inventory

- GIVEN the current proof-gap inventory lists trusted boundaries
- WHEN this change begins implementation
- THEN it records the selected target site, current classification, and reason it is suitable for closure or narrowing

#### Scenario: Closure removes or narrows trust honestly

- GIVEN a selected trusted boundary
- WHEN implementation changes are complete
- THEN the source either removes the trusted marker with verified replacement logic or narrows the trusted contract without hiding an equivalent new trust boundary elsewhere

#### Scenario: Inventory matches source

- GIVEN source proof-gap markers and the proof-gap manifest
- WHEN the proof-gap inventory checker runs
- THEN expected counts, site names, and classifications match the source tree

#### Scenario: Verification evidence supports the count

- GIVEN the closure or narrowing is claimed complete
- WHEN focused tests, Verus, Tracey coverage, and full verification run
- THEN the evidence supports the updated trusted-boundary count and no uncovered or untested requirements remain

### Requirement: Proof-Gap Manifest Rationale Source
r[verification-gaps.manifest-rationale-source]
The proof-gap site manifest MUST include a concise rationale for every expected trusted or weakened boundary so the checked inventory explains why each remaining boundary is acceptable.


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
r[verification-gaps.manifest-rationale-drift-gate]
The proof-gap inventory checker MUST reject rationale manifest drift before full verification can pass.


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
r[verification-gaps.manifest-rationale-publication]
Published proof-gap prose MUST point reviewers to the checked manifest as the authoritative per-site rationale source after rationale fields are introduced.


#### Scenario: README describes the checked rationale source

- GIVEN the rationale manifest change is complete
- WHEN README proof-gap documentation is reviewed
- THEN it identifies `docs/proof-gap-sites.tsv` as the checked source for per-site trusted-boundary rationale

#### Scenario: Canonical spec records rationale validation

- GIVEN the change is archived
- WHEN `openspec/specs/verification-gaps/spec.md` is reviewed
- THEN it contains the rationale manifest and drift-gate requirements
