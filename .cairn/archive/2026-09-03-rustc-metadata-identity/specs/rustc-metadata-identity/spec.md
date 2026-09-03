# Rustc Metadata Identity Delta

## ADDED Requirements

### Requirement: Rustc Library Kind Metadata
r[rustc-metadata.library-kind] The system MUST distinguish finite `rlib`, `dylib`, and `rmeta` library-kind metadata and MUST declare whether each candidate carries metadata, linkable object code, and optional codegen payloads.

#### Scenario: rlib metadata is admitted
r[rustc-metadata.library-kind.rlib]
- GIVEN an `rlib` candidate with declared rustc metadata and linkable-code capability metadata
- WHEN library-kind admission is evaluated
- THEN the candidate is accepted for metadata and linking use.

#### Scenario: rmeta is metadata-only
r[rustc-metadata.library-kind.rmeta]
- GIVEN an `rmeta` candidate with declared rustc metadata and no linkable-code capability
- WHEN library-kind admission is evaluated
- THEN the candidate is accepted only for metadata-only usage modes.

### Requirement: Rustc Identity Fields
r[rustc-metadata.identity-fields] The system MUST model compiler-version, strict-version-hash, and stable-crate-id fields as rustc-mandated identity values and MUST compare them by declared compatibility rules.

#### Scenario: Matching identity fields are accepted
r[rustc-metadata.identity-fields.match]
- GIVEN a candidate whose compiler version, SVH, and stable crate ID match the requested dependency metadata
- WHEN identity compatibility is evaluated
- THEN the candidate is accepted as identity-compatible.

#### Scenario: Wrong compiler version is rejected
r[rustc-metadata.identity-fields.wrong-version]
- GIVEN a candidate whose compiler-version field differs from the requested compiler version
- WHEN identity compatibility is evaluated
- THEN the candidate is rejected with deterministic version metadata.

#### Scenario: Wrong SVH is rejected
r[rustc-metadata.identity-fields.wrong-svh]
- GIVEN a candidate whose strict-version-hash field differs from the requested SVH
- WHEN identity compatibility is evaluated
- THEN the candidate is rejected with deterministic SVH metadata.

### Requirement: Metadata Compatibility
r[rustc-metadata.compatibility] The system MUST admit metadata candidates only when library kind, crate name, compiler version, SVH, stable crate ID, and requested usage mode are compatible.

#### Scenario: Check usage accepts metadata-only candidate
r[rustc-metadata.compatibility.check]
- GIVEN a compatible `rmeta` candidate and a metadata-only check or documentation usage mode
- WHEN compatibility admission is evaluated
- THEN the candidate is accepted.

#### Scenario: Link usage rejects metadata-only candidate
r[rustc-metadata.compatibility.link-rmeta]
- GIVEN an `rmeta` candidate and a link usage mode requiring object code
- WHEN compatibility admission is evaluated
- THEN the candidate is rejected with deterministic linkability metadata.

### Requirement: Encoded MIR Availability
r[rustc-metadata.admission] The system MUST model encoded MIR as optional metadata and MUST reject uses that require encoded MIR when the selected candidate declares it unavailable.

#### Scenario: Encoded MIR requirement is satisfied
r[rustc-metadata.admission.encoded-mir-present]
- GIVEN a compatible metadata candidate declaring encoded MIR availability
- WHEN a codegen usage requiring encoded MIR is evaluated
- THEN the use is accepted.

#### Scenario: Missing encoded MIR is rejected when required
r[rustc-metadata.admission.encoded-mir-missing]
- GIVEN a compatible metadata candidate without encoded MIR availability
- WHEN a usage mode requiring encoded MIR is evaluated
- THEN the use is rejected with deterministic encoded-MIR metadata.

### Requirement: Dependency Loading
r[rustc-metadata.dependency-loading] The system MUST select exactly one compatible metadata candidate from a finite candidate set for each dependency request, or reject deterministically if no candidate or multiple candidates match.

#### Scenario: Unique compatible candidate is selected
r[rustc-metadata.dependency-loading.unique]
- GIVEN a finite candidate set containing exactly one candidate compatible with the dependency request
- WHEN dependency loading is evaluated
- THEN that candidate is selected.

#### Scenario: No compatible candidate is rejected
r[rustc-metadata.dependency-loading.none]
- GIVEN a finite candidate set containing no compatible candidate
- WHEN dependency loading is evaluated
- THEN loading is rejected with deterministic missing-candidate metadata.

#### Scenario: Ambiguous compatible candidates are rejected
r[rustc-metadata.dependency-loading.ambiguous]
- GIVEN a finite candidate set containing more than one compatible candidate for the same request
- WHEN dependency loading is evaluated
- THEN loading is rejected with deterministic ambiguity metadata.

### Requirement: Rmeta Pipelining Boundary
r[rustc-metadata.rmeta-pipelining] The system MUST allow `rmeta` candidates for declared metadata-only check, documentation, or pipelining usage, and MUST reject claims that `rmeta` alone satisfies final linking.

#### Scenario: Pipelined check can use rmeta
r[rustc-metadata.rmeta-pipelining.check]
- GIVEN a compatible `rmeta` candidate and a metadata-only pipelined check usage
- WHEN pipelining admission is evaluated
- THEN the use is accepted.

#### Scenario: Final link requires linkable candidate
r[rustc-metadata.rmeta-pipelining.link]
- GIVEN only `rmeta` metadata for a dependency and a final link usage
- WHEN pipelining admission is evaluated
- THEN the use is rejected until a linkable candidate is available.

### Requirement: Metadata Evidence Coverage
r[rustc-metadata.evidence] The implementation MUST include positive and negative evidence for library kind, identity fields, compatibility, dependency loading, encoded MIR availability, rmeta pipelining, and failure metadata.

#### Scenario: Positive fixtures cover compatible metadata behavior
r[rustc-metadata.evidence.positive.fixtures]
- GIVEN the rustc metadata identity test suite
- WHEN positive fixtures run
- THEN matching rlib, dylib, rmeta metadata-only use, optional encoded MIR, and unique dependency lookup are covered.

#### Scenario: Negative fixtures cover incompatible metadata behavior
r[rustc-metadata.evidence.negative.fixtures]
- GIVEN the rustc metadata identity test suite
- WHEN negative fixtures run
- THEN wrong version, wrong SVH, stable-crate-id mismatch, missing metadata, invalid link use, missing encoded MIR, and ambiguous lookup are rejected deterministically.

### Requirement: Metadata Documentation Sync
r[rustc-metadata.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public metadata-identity helpers.

#### Scenario: Requirement IDs are traceable
r[rustc-metadata.docs.traceable]
- GIVEN a public metadata identity helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Metadata Validation Evidence
r[rustc-metadata.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[rustc-metadata.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Metadata Non-Claim Boundary
r[rustc-metadata.non-claims] The metadata identity primitive MUST NOT claim real archive parsing, rustc hash construction correctness, hash collision resistance, linker correctness, object-code correctness, LTO correctness, metadata compression correctness, or whole build-system correctness.

#### Scenario: Rustc hash boundary is explicit
r[rustc-metadata.non-claims.hash-boundary]
- GIVEN a consumer cites metadata identity evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to equality and compatibility of declared rustc identity fields and does not replace rustc-mandated hash algorithms with BLAKE3.
