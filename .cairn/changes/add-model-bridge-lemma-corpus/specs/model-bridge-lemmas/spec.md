# Model Bridge Lemma Corpus Specification

## Purpose

Define a bounded corpus of Verus-checked bridge lemmas that connect code-enforced invariants to model invariants through one documented proof boundary, with exact provenance and explicit non-overclaim.

## ADDED Requirements

### Requirement: Bridge lemmas follow one handshake contract

r[trellis.bridge_lemma.handshake]

A bridge lemma MUST bind a model invariant identified by its requirement ID to a Verus lemma over a code-level invariant through exactly one declared proof boundary, and MUST record imported model-layer facts as assumptions rather than deriving them.

#### Scenario: Handshake contract is complete
- GIVEN a lemma declares its model requirement ID, code identifier, Verus proof identifier, assumption list, and proof boundary
- WHEN handshake validation runs
- THEN the lemma MUST be admitted to the corpus.

#### Scenario: Handshake boundary is missing
- GIVEN a lemma lacks a declared proof boundary or an assumption list
- WHEN handshake validation runs
- THEN Trellis MUST reject the lemma.

### Requirement: The corpus is bounded and traced

r[trellis.bridge_lemma.corpus]

Trellis MUST maintain a bounded bridge-lemma corpus for selected protocol surfaces, with one lemma per declared invariant pair and an explicit corpus scope recorded in the repository.

#### Scenario: Corpus scope is declared
- GIVEN the corpus declares its selected surfaces and invariant pairs
- WHEN corpus validation runs
- THEN every lemma MUST map to a pair inside the declared scope.

#### Scenario: Lemma outside scope appears
- GIVEN a lemma references an invariant pair outside the declared scope
- WHEN corpus validation runs
- THEN Trellis MUST flag the lemma as out of scope.

### Requirement: Lemmas carry exact provenance

r[trellis.bridge_lemma.provenance]

Each bridge lemma MUST record the model requirement ID, the code invariant identifier, the Verus proof identifier, and the full assumption list, and MUST NOT reference a requirement ID or code identifier that does not exist.

#### Scenario: Provenance is exact
- GIVEN a lemma whose identifiers all resolve
- WHEN provenance validation runs
- THEN the lemma MUST bind its proof artifact identity to those identifiers.

#### Scenario: Requirement ID does not resolve
- GIVEN a lemma references a model requirement ID that is not admitted
- WHEN provenance validation runs
- THEN Trellis MUST reject the lemma.

### Requirement: Denial fixtures are first class

r[trellis.bridge_lemma.fixtures]

The bridge-lemma suite MUST include positive verified lemmas and negative fixtures for an invariant pair that does not hold, a mismatched requirement ID, an unrecorded assumption, and a missing bounded non-claim.

#### Scenario: Suite runs
- GIVEN all fixtures are committed with their declared identities
- WHEN the verification harness executes
- THEN positive lemmas MUST verify and each negative fixture MUST fail at its declared boundary.

### Requirement: Bounded non-overclaim is mandatory

r[trellis.bridge_lemma.nonclaim]

Every bridge lemma MUST carry the non-claim that it proves the code invariant implies the model invariant under the recorded assumptions, and MUST NOT claim model validity, engine correctness, or arbitrary semantic equivalence.

#### Scenario: Non-claim is recorded
- GIVEN a lemma includes its bounded non-claim
- WHEN the corpus is audited
- THEN the non-claim MUST be present in the lemma metadata.

### Requirement: Bridge changes run full validation

r[trellis.bridge_lemma.validation]

Changes to the handshake contract, corpus, provenance checks, or fixtures MUST run the verification harness with positive and negative cases, cleanup of proof artifacts, lifecycle validation, Cairn gates, and relevant flake checks.

#### Scenario: Change is validated
- GIVEN a bridge-lemma change passes all focused checks
- WHEN lifecycle validation is assembled
- THEN it MUST include both positive and negative results and MUST identify the exact model requirement IDs and proof artifact identities.
