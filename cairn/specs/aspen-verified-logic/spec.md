# aspen-verified-logic Specification

## Purpose

This specification defines how verified-logic records, validates, and optionally snapshots Aspen verified-logic evidence while keeping Aspen crate-local proofs as the source of truth unless a local snapshot is explicitly imported and verified.

## Requirements
### Requirement: Aspen Verified Logic Evidence Index [r[aspen-verified-logic.evidence-index]]

The system MUST provide a local evidence index for Aspen verified-logic slices before this repository relies on Aspen proofs for planning, composition, or operator-facing claims.

#### Scenario: Index row records source and evidence [r[aspen-verified-logic.evidence-index.scenario.row-records-source-evidence]]

- GIVEN an Aspen verified-logic slice is included in the local index
- WHEN the index is validated
- THEN the row records the Aspen source commit, source proof paths, verified property summary, Verus command and result, focused Rust test command and result, and remaining trusted-boundary notes

#### Scenario: Index does not replace proof source of truth [r[aspen-verified-logic.evidence-index.scenario.source-of-truth]]

- GIVEN an index row points to an Aspen crate-local proof
- WHEN this repo has not imported and verified a local snapshot for that row
- THEN the row is treated as evidence metadata only
- AND Aspen's `crates/*/verus/` proof remains the source of truth

### Requirement: Opt-In Aspen Proof Snapshots [r[aspen-verified-logic.opt-in-snapshots]]

The system MUST import Aspen proof code into this repository only through an explicit manifest-backed snapshot selected for local composition or independent verification.

#### Scenario: Snapshot carries provenance [r[aspen-verified-logic.opt-in-snapshots.scenario.provenance]]

- GIVEN a selected Aspen proof kernel is copied into this repository
- WHEN the snapshot manifest is validated
- THEN the manifest records the Aspen source commit, source paths, local paths, import rationale, verification command, and known trusted boundaries

#### Scenario: No wholesale mirror [r[aspen-verified-logic.opt-in-snapshots.scenario.no-wholesale-mirror]]

- GIVEN this repository records Aspen verified-logic evidence
- WHEN no local composition need exists for a proof module
- THEN the module is not copied solely to mirror Aspen
- AND the local index remains the durable artifact

### Requirement: Aspen Evidence Drift Gate [r[aspen-verified-logic.drift-gate]]

The system MUST fail closed when the local Aspen verified-logic index or optional snapshot manifest is malformed, stale, or inconsistent with the recorded source commit/path contract.

#### Scenario: Current index validates [r[aspen-verified-logic.drift-gate.scenario.current-index-validates]]

- GIVEN the local Aspen verified-logic index and any snapshot manifests
- WHEN the drift checker runs
- THEN every row has required provenance and verification fields
- AND every snapshot row has matching local files and a recorded verification command

#### Scenario: Stale or malformed row fails closed [r[aspen-verified-logic.drift-gate.scenario.stale-malformed-fails]]

- GIVEN an index row omits a source commit, names a missing proof path, lacks verification evidence, or claims local proof ownership without a snapshot manifest
- WHEN the drift checker runs
- THEN it exits non-zero with a diagnostic identifying the invalid row

### Requirement: Aspen Boundary Non-Overclaiming [r[aspen-verified-logic.boundary-non-overclaiming]]

The system MUST distinguish verified finite logic from Aspen runtime shell, networking, serialization, storage engine, crypto, and external-system behavior in every local index or snapshot claim.

#### Scenario: Trusted boundaries are recorded [r[aspen-verified-logic.boundary-non-overclaiming.scenario.trusted-boundaries-recorded]]

- GIVEN an indexed Aspen slice leaves crypto, codec, runtime I/O, solver-limit, or external-system behavior outside the proof
- WHEN the row is validated
- THEN the remaining boundary is recorded explicitly
- AND the row does not claim this repository has verified that excluded behavior
