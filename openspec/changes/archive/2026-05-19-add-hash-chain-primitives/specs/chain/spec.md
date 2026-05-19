## ADDED Requirements

### Requirement: Chain Tip Commitment

The system MUST expose a chain-tip commitment primitive whose result is the genesis hash for an empty chain and the last entry hash for a non-empty chain. For any valid chain, the tip commitment MUST be usable as the compact commitment to the entire chain prefix under the existing collision-free/agreement assumptions.

r[chain.tip-commitment]

#### Scenario: Empty chain tip is genesis

- GIVEN an empty chain
- WHEN the chain-tip commitment is computed
- THEN the result is the configured genesis hash

#### Scenario: Non-empty chain tip is last entry hash

- GIVEN a chain with at least one entry
- WHEN the chain-tip commitment is computed
- THEN the result equals the final entry hash

### Requirement: Chain Extension Prefix Relation

The system MUST define a chain-extension predicate where one chain extends another exactly when the shorter chain is an unchanged prefix of the longer chain. Extension MUST be independent from hash validity, and separate proofs MUST show that a valid full chain preserves validity for any admitted prefix.

r[chain.extension-prefix]

#### Scenario: Extended chain preserves prefix entries

- GIVEN a prefix chain and a full chain where the full chain extends the prefix
- WHEN an index inside the prefix length is inspected
- THEN the entries at that index are equal in both chains

#### Scenario: Valid full chain yields valid prefix

- GIVEN a valid full chain and a prefix length within bounds
- WHEN the prefix at that length is selected
- THEN the selected prefix is also valid

### Requirement: Chain Checkpoint Anchor

The system MUST expose checkpoint anchoring over an admitted index and expected hash. A checkpoint MUST be valid only when the chain is valid, the index is in range, and the entry hash at that index equals the expected hash. Matching checkpoint anchors MUST authenticate the prefix through that index under the existing agreement and collision-free assumptions.

r[chain.checkpoint-anchor]

#### Scenario: Matching checkpoint authenticates prefix

- GIVEN a valid chain, an in-range checkpoint index, and the hash stored at that index
- WHEN checkpoint validation is evaluated
- THEN the checkpoint is accepted and commits to every entry up to and including that index

#### Scenario: Mismatched checkpoint is rejected

- GIVEN a valid chain, an in-range checkpoint index, and an expected hash different from the hash stored at that index
- WHEN checkpoint validation is evaluated
- THEN the checkpoint is rejected

### Requirement: Chain Fork Detection

The system MUST define fork-detection predicates for the first index where two chains differ. For two valid same-length chains, equal tip commitments MUST imply that no divergence exists. If a divergence exists at any index, the tip commitments of valid same-length chains MUST differ.

r[chain.fork-detection]

#### Scenario: Equal valid tips rule out divergence

- GIVEN two valid chains with the same length and the same tip commitment
- WHEN fork detection searches for a first divergence
- THEN no divergence is reported

#### Scenario: Divergence changes the tip

- GIVEN two valid chains with the same length and at least one differing entry
- WHEN their tip commitments are compared
- THEN the tip commitments differ

### Requirement: Chain Batch Append

The system MUST expose a batch append/fold primitive that applies a sequence of correctly hashed entries to an initial valid chain. Batch append MUST preserve chain validity by composing the existing single-entry append preservation proof at each step and MUST leave the original prefix unchanged.

r[chain.batch-append]

#### Scenario: Correctly hashed batch preserves validity

- GIVEN a valid base chain and a batch where every new entry is correctly hashed from the current fold tip
- WHEN the batch append primitive is evaluated
- THEN the resulting chain is valid and extends the base chain

#### Scenario: Incorrect batch entry is rejected

- GIVEN a valid base chain and a batch containing an entry whose stored hash does not match the current fold predecessor
- WHEN batch append admission is evaluated
- THEN the batch is rejected or yields no verified append-preserves claim for that entry

### Requirement: Chain Log Shape Validity

The system MUST define log-shape predicates separately from hash validity. Shape predicates MUST cover contiguous or strictly increasing index policy and the selected term policy. A combined log-validity predicate MAY compose shape validity with hash-chain validity, but hash validity alone MUST NOT imply index or term well-formedness.

r[chain.log-shape]

#### Scenario: Gap in shaped log is rejected

- GIVEN a hash-valid chain whose entry indices skip an expected position
- WHEN log-shape validity is evaluated under a contiguous-index policy
- THEN shape validity is rejected even if hash validity holds

#### Scenario: Combined log validity requires both shape and hash validity

- GIVEN a chain with valid hashes and valid index/term shape
- WHEN combined log validity is evaluated
- THEN the chain is accepted as combined-valid

### Requirement: Chain Tamper Classification

The system MUST expose named predicates or lemmas that classify detectable tampering of a valid chain as data identifier tampering, index tampering, term tampering, or predecessor/hash tampering. Classification MUST reuse the existing hash collision-free, agreement, and divergence facts rather than introducing new cryptographic assumptions.

r[chain.tamper-classification]

#### Scenario: Payload tamper is classified

- GIVEN two valid chains that first differ only by `data_id` at an index
- WHEN tamper classification is evaluated for that divergence
- THEN the divergence is classified as payload/data tampering and downstream hashes differ

#### Scenario: Metadata tamper is classified

- GIVEN two valid chains that first differ by index or term metadata at an index
- WHEN tamper classification is evaluated for that divergence
- THEN the divergence is classified as index or term tampering and downstream hashes differ

### Requirement: Chain Domain-Separated Hash Input

The system MUST define domain-separated hash input contracts for entry hashes and other chain-adjacent commitments such as checkpoints, receipts, and snapshots. A hash from one domain MUST NOT be accepted as verified evidence for a different domain unless an explicit compatibility wrapper proves the domains and serialized fields match.

r[chain.domain-separated-hash]

#### Scenario: Entry hash domain is distinct from checkpoint domain

- GIVEN identical numeric fields serialized under the entry domain and checkpoint domain
- WHEN the domain-separated hash input contract is evaluated
- THEN the two commitments are treated as different domains and are not interchangeable evidence

#### Scenario: Compatibility wrapper is explicit

- GIVEN an existing caller uses the legacy entry-hash wrapper
- WHEN domain-separated hashing is introduced
- THEN compatibility is provided only through an explicitly named wrapper with its own requirement coverage

### Requirement: Chain Runtime BLAKE3 Alignment

The system MUST align runtime chain hashing with the documented BLAKE3 serialization boundary or explicitly document any remaining deterministic test-mixer/runtime bridge as a trusted boundary. Runtime serialization MUST use a deterministic field order, fixed-width encoding, and domain tag when domain separation is enabled.

r[chain.runtime-blake3-alignment]

#### Scenario: Runtime hash uses documented serialization

- GIVEN an entry hash request with predecessor hash, index, term, and data identifier
- WHEN runtime chain hashing is executed under the BLAKE3-aligned implementation
- THEN the bytes are serialized in the documented field order with the documented domain tag before hashing

#### Scenario: Test mixer remains named trust boundary

- GIVEN the implementation still uses a deterministic mixer rather than real BLAKE3 for an executable surface
- WHEN proof-gap inventory and documentation are reviewed
- THEN the bridge is named as a runtime/trusted boundary and no cryptographic-strength claim is made for the mixer

### Requirement: Chain Compact History Proof Overlay

The system MAY define an optional compact history proof overlay, such as skip links, Merkle proofs, or a Merkle mountain range, for efficient membership or range proofs. The overlay MUST compose with base linear chain tips/checkpoints and MUST NOT replace or weaken `chain_valid_spec` for callers that require linear hash-chain validity.

r[chain.compact-history-proof]

#### Scenario: Compact proof composes with base tip

- GIVEN a compact membership or range proof and the corresponding base chain tip or checkpoint anchor
- WHEN compact proof verification is evaluated
- THEN acceptance implies the proof is anchored to the same base-chain commitment

#### Scenario: Base linear chain remains sufficient

- GIVEN a caller only requires ordinary linear chain validity and tip commitment
- WHEN compact-history overlay support is absent or disabled
- THEN the caller can still verify the base chain without depending on overlay predicates

