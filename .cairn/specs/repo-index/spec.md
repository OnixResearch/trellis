# Repository Index Specification

## Purpose

Defines a persistent local index for the active `verified-logic` repo. The
index uses BLAKE3 digests as content identity and redb as the local store.
It covers source files, requirement definitions, and active OpenSpec docs so
local tooling can answer path, symbol, digest, and requirement queries
without reparsing the whole tree for every lookup.

## Requirements

### Requirement: repo-index.content-digest
r[repo-index.content-digest]

The index MUST compute a BLAKE3 digest from the raw bytes of each tracked
file and use that digest as the content identity key for extracted facts.

#### Scenario: Stable digest across rescans

- GIVEN the same file bytes on two refreshes
- WHEN the digest is computed both times
- THEN the stored digest is identical

#### Scenario: Shared content across paths

- GIVEN two tracked paths with identical file bytes
- WHEN both paths are indexed
- THEN both path records point at the same content digest

#### Scenario: Changed bytes update the path record

- GIVEN a tracked path whose bytes change
- WHEN the next refresh commits
- THEN the active path record points at the digest computed from the new bytes

### Requirement: repo-index.persistent-store
r[repo-index.repo-index-persistent-store]

The index MUST persist committed rows in redb so a later process can reopen
the database and answer queries from the last successful refresh.

#### Scenario: Reopen after successful build

- GIVEN a committed index database
- WHEN the index is reopened in a new process
- THEN path, digest, symbol, and requirement queries return the committed results

#### Scenario: Restart without immediate rebuild

- GIVEN no files changed after a successful refresh
- WHEN a query runs after reopening and before a new refresh
- THEN it is answered from the committed rows already in the database

### Requirement: repo-index.atomic-refresh
r[repo-index.repo-index-atomic-refresh]

A refresh MUST become visible as one committed generation. Readers MUST
never observe a mix of old and new generation rows.

#### Scenario: Successful refresh swaps generations

- GIVEN an existing committed generation and a newer successful refresh
- WHEN the write transaction commits
- THEN all queries observe only the new generation

#### Scenario: Failed refresh preserves prior generation

- GIVEN an existing committed generation
- WHEN a refresh fails before commit
- THEN queries still observe the prior committed generation

### Requirement: repo-index.incremental-refresh
r[repo-index.incremental-refresh]

The index MUST update the active path view for added, changed, unchanged,
and deleted files without re-extracting facts for unchanged digests.

#### Scenario: Unchanged content reuses extracted facts

- GIVEN a path whose current bytes hash to a digest already stored in the index
- WHEN a new refresh commits
- THEN the active path row reuses the extracted facts already stored for that digest

#### Scenario: Deleted file disappears from the active view

- GIVEN a path present in the committed generation
- WHEN the file is absent on the next successful refresh
- THEN the path no longer appears in active path query results

#### Scenario: New file becomes visible

- GIVEN a newly tracked file
- WHEN the next refresh commits
- THEN the file appears in path and corpus-stat queries

### Requirement: repo-index.cross-reference
r[repo-index.cross-reference]

The index MUST extract cross-reference facts needed to navigate
`verified-logic` code and specs.

#### Scenario: Requirement definition and uses

- GIVEN a requirement id from `docs/requirements.md` or active OpenSpec markdown
- WHEN that id is queried
- THEN the result includes its defining location and the indexed `r[...]` reference sites that mention it

#### Scenario: File facts

- GIVEN a tracked Rust source file
- WHEN its path is queried
- THEN the result includes extracted items and requirement references found in that file

#### Scenario: Active OpenSpec facts

- GIVEN an active change spec file under `openspec/changes/`
- WHEN it is indexed
- THEN the result includes its change name, domain, requirement headings, and requirement definitions

### Requirement: repo-index.query
r[repo-index.query]

The index MUST support deterministic local queries over committed index
state.

#### Scenario: Symbol lookup

- GIVEN an indexed symbol name
- WHEN it is queried
- THEN the result returns all defining or declared locations recorded for that symbol

#### Scenario: Requirement lookup

- GIVEN an indexed requirement id
- WHEN it is queried
- THEN the result returns the definition plus related implementation, verification, dependency, or related reference sites

#### Scenario: Index stats

- GIVEN a committed generation
- WHEN stats are queried
- THEN the result reports counts for tracked paths and extracted records in that generation

### Requirement: Trellis repository tooling filesystem authority is capability-rooted
r[trellis.repo_index.cap_std_boundary.root_wrappers] Trellis MUST represent repository, index database, proof-evidence input, and artifact output filesystem authority as typed capability roots opened by repository tooling shells.

#### Scenario: Valid repository tooling path stays under the declared root
r[trellis.repo_index.cap_std_boundary.tests.positive]
- GIVEN Trellis has opened an operator-declared repository, index, input, or output root
- WHEN repo-index or proof-evidence tooling reads, writes, indexes, or emits a valid relative path through the typed root
- THEN the operation MUST remain confined to that declared root while preserving BLAKE3 identity, requirement extraction, and valid proof-evidence behavior.

#### Scenario: Invalid repository tooling path cannot escape the root
r[trellis.repo_index.cap_std_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, a symlink escape attempt, or an index database/output escape attempt
- WHEN Trellis tooling resolves the path through the typed root
- THEN Trellis MUST fail closed before reading, writing, indexing, or emitting artifacts outside the declared root.

### Requirement: Capability dependency stays outside verified logic
r[trellis.repo_index.cap_std_boundary.dependency] Trellis MUST add `cap-std` only to repository tooling modules that own filesystem effects.

#### Scenario: Verus modules stay filesystem-neutral
r[trellis.repo_index.cap_std_boundary.conversion]
- GIVEN verified-logic modules, spec/exec functions, proof functions, and pure model code are reviewed
- WHEN cap-std adoption is complete
- THEN those modules MUST operate on in-memory values, proof facts, content identities, or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths directly.

### Requirement: Capability boundary is documented without proof overclaims
r[trellis.repo_index.cap_std_boundary.docs] Trellis MUST document that capability roots bound local repository-tooling filesystem authority only and do not prove new verified-logic properties, proof soundness, artifact truth, or consumer release readiness.

#### Scenario: Closeout evidence includes repository tooling boundary validation
r[trellis.repo_index.cap_std_boundary.validation]
- GIVEN the cap-std repository-tooling change is ready to archive
- WHEN focused repo-index, proof-evidence, Cairn validation, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible verified-logic non-claim documentation.
