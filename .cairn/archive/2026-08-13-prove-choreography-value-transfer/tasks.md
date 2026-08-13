## Phase 1: Baseline and representation

- [x] [serial] Record focused choreography tests, full-library Verus, proof-gap inventory, Tracey, and Cairn baselines before semantic changes. r[choreography.value_ownership.validation]
- [x] [serial] Review the payload-tag newtype pilot and select a distinct `ValueId` representation with an explicit proof-cost record. r[choreography.value_ownership.identity]
- [x] [serial] Record the exact Wyzer MIT reference revision, author-confirmed license disposition, selected design concepts, and rejected runtime components. r[choreography.value_ownership.boundary]

## Phase 2: Ownership core

- [x] [serial] Define normalized ownership entries, admission, lookup, update specifications, and executable refinements. r[choreography.value_ownership.state]
- [x] [depends:value-ownership-state] Add the global owned-transfer node and pure atomic transfer step over production types. r[choreography.value_ownership.transfer]
- [x] [depends:value-transfer-step] Prove successful owner replacement, former-owner exclusion, unrelated-value preservation, entry-count preservation, and normalization preservation. r[choreography.value_ownership.conservation]
- [x] [depends:value-transfer-step] Prove self-transfer, missing-value, wrong-source, and stale-source rejection without replacement state. r[choreography.value_ownership.transfer]

## Phase 3: Projection and correspondence

- [x] [serial] Add exact local send-owned and receive-owned nodes and projection behavior for involved and uninvolved roles. r[choreography.value_transfer_projection.shape]
- [x] [depends:owned-transfer-projection] Extend projectability and merge with exact peer, label, payload-tag, value-identity, and direction matching. r[choreography.value_transfer_projection.merge]
- [x] [depends:owned-transfer-projection] Prove owned-transfer projection duality and global-to-local step correspondence over the production projection path. r[properties.choreography_transfer_correspondence]
- [x] [depends:owned-transfer-proofs] Re-run every existing choreography theorem and reject any new trusted bridge, assumption, or proof gap. r[choreography.value_ownership.validation]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive tests for initial ownership, successful transfer, chained transfer, return transfer, exact projection, merge, and unrelated-value preservation. r[choreography.value_ownership.evidence]
- [x] [parallel] Add negative tests for duplicate values, missing values, self-transfer, wrong source, stale source, wrong peer, wrong value, mismatched payload tag, and conflicting transfer heads. r[choreography.value_ownership.evidence]
- [x] [parallel] Add property tests that compare executable owner updates with an independent pointwise oracle over generated normalized states. r[choreography.value_ownership.evidence]
- [x] [depends:value-identity-representation] Add compile-fail evidence that role, label, and payload-tag values cannot occupy a `ValueId` position without explicit construction. r[choreography.value_ownership.evidence]

## Phase 5: Documentation and closeout

- [x] [serial] Update canonical requirements, public API documentation, `docs/requirements.md`, implementation markers, and verification markers. r[choreography.value_ownership.validation] r[choreography.value_ownership.boundary]
- [x] [serial] Document atomic semantic transfer and explicit transport, persistence, memory, authorization, exactly-once, and liveness non-claims. r[choreography.value_ownership.boundary]
- [x] [depends:value-transfer-evidence] Run focused and full Cargo tests, Clippy, full-library Verus, proof-gap checks, Tracey coverage, Cairn validation, and lifecycle gates. r[choreography.value_ownership.validation]
- [x] [depends:value-transfer-validation] Review sync and archive plans and archive only with fresh non-contradictory evidence. r[choreography.value_ownership.validation]
