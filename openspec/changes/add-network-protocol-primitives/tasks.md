## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.network-protocol.endpoint-address` | `src/net_*`, `src/protocol_*` finite endpoint/address metadata |
| `domain-primitives.network-protocol.packet-frame` | packet/frame envelope shape |
| `domain-primitives.network-protocol.header-field` | header-field admission metadata |
| `domain-primitives.network-protocol.parser-admission` | state-free parser admission |
| `domain-primitives.network-protocol.sequence` | message sequencing metadata |
| `domain-primitives.network-protocol.checksum-shape` | checksum-shape metadata |
| `domain-primitives.network-protocol.payload-bound` | payload length/type bound metadata |
| `domain-primitives.network-protocol.admission` | deterministic protocol admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only network-protocol primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.network-protocol.endpoint-address,domain-primitives.network-protocol.packet-frame,domain-primitives.network-protocol.header-field,domain-primitives.network-protocol.parser-admission,domain-primitives.network-protocol.sequence,domain-primitives.network-protocol.checksum-shape,domain-primitives.network-protocol.payload-bound,domain-primitives.network-protocol.admission]`

## Phase 2: Future implementation

- [ ] Add docs/requirements entries and Tracey IDs for finite endpoint/address metadata.
- [ ] Implement and verify packet/frame envelope admission primitives.
- [ ] Implement and verify finite header-field admission metadata.
- [ ] Implement and verify state-free parser admission envelopes.
- [ ] Implement and verify finite message sequencing metadata.
- [ ] Implement and verify checksum-shape metadata envelopes.
- [ ] Implement and verify payload length/type bound metadata.
- [ ] Implement deterministic protocol admission/rejection composition.
- [ ] Add focused positive and negative tests for each network-protocol primitive.
- [ ] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [ ] Capture reviewable evidence and archive the change after all implementation tasks complete.
