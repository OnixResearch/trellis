# Repository Index Specification

## Purpose

Adds capability-bounded local filesystem roots for Trellis repository-index and proof-evidence tooling boundaries.

## ADDED Requirements

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
