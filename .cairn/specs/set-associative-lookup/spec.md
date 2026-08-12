# Set-Associative Cache Lookup Specification

## Purpose

Defines requirements for verified address decomposition and N-way
set-associative cache lookup. Covers the bijection between addresses and
(tag, index, offset) triples, tag comparison, hit/miss classification,
set isolation, and duplicate prevention.

## Requirements

### Requirement: Decomposition Bijection
r[set-assoc.decomposition]

Decomposing an address into (tag, index, offset) and reconstructing MUST
yield the original address.

#### Scenario: Roundtrip

- GIVEN any 64-bit address and valid offset_bits, index_bits parameters
- WHEN the address is decomposed into (tag, index, offset)
- THEN `(tag << (offset_bits + index_bits)) | (index << offset_bits) | offset == address`

#### Scenario: Component bounds

- GIVEN an address decomposed with offset_bits=6 and index_bits=8
- WHEN the components are extracted
- THEN offset < 64, index < 256, and tag < 2^50

### Requirement: No Aliasing
r[set-assoc.no-aliasing]

Two distinct addresses with different tags MUST NOT produce a false hit
in the same set.

#### Scenario: Different tags same index

- GIVEN addresses A and B with the same index but different tags
- WHEN both are looked up in the same set
- THEN a hit on A does not match B's tag and vice versa

#### Scenario: Same tag same index

- GIVEN addresses A and B with the same tag and same index
- WHEN both are looked up
- THEN they map to the same cache line (same tag match)

### Requirement: Hit Correctness
r[cache-partition.isolation]

A cache hit MUST mean the tag stored in the matched way equals the
requested tag.

#### Scenario: Hit returns correct way

- GIVEN a set with tag T stored in way W
- WHEN a lookup for tag T is performed
- THEN the result is hit at way W

#### Scenario: Hit identifies requested line

- GIVEN a hit at way W
- WHEN the way's stored tag is inspected
- THEN it equals the requested tag and therefore identifies the requested cache line

### Requirement: Miss Completeness
r[set-assoc.miss]

A cache miss MUST mean no way in the selected set holds the requested tag.

#### Scenario: All ways checked

- GIVEN a set with N ways, none holding tag T
- WHEN a lookup for tag T is performed
- THEN the result is miss

#### Scenario: Miss is exhaustive

- GIVEN a miss result for tag T in set S
- WHEN every way in set S is inspected
- THEN no way holds tag T

### Requirement: Set Isolation
r[set-associative-lookup.set-isolation]

Addresses with different indices MUST map to different sets, and a lookup
in one set MUST NOT inspect or modify another set.

#### Scenario: Different indices different sets

- GIVEN addresses A and B with different indices
- WHEN A is looked up
- THEN only the set at A's index is accessed

#### Scenario: Cross-set independence

- GIVEN a write to set S1
- WHEN set S2 (S2 ≠ S1) is inspected
- THEN S2 is unchanged

### Requirement: No Duplicate Tags
r[set-assoc.no-duplicates]

Within any set, at most one way SHOULD hold a given tag at any time.

#### Scenario: Insert maintains uniqueness

- GIVEN a set with no duplicate tags
- WHEN a new tag is inserted (into an empty or evicted way)
- THEN the set still has no duplicate tags

#### Scenario: Duplicate detection

- GIVEN a set where way W1 and W2 both hold tag T
- WHEN the duplicate check is run
- THEN the violation is detected
