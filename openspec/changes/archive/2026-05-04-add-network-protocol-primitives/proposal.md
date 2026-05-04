## Why

Protocol-shaped data often mixes endpoint/address metadata, packet or frame
shape, header fields, parser admission, message ordering, checksum metadata, and
runtime transport behavior. A focused network-protocol primitive family gives
future changes exact finite building blocks for endpoint/address metadata,
packet/frame envelopes, header fields, state-free parser admission, message
sequencing metadata, checksum-shape metadata, and deterministic protocol
admission without claiming transport delivery, security, congestion control,
network stack, or external protocol correctness.

## What Changes

- Add a spec-only `add-network-protocol-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for endpoint/address metadata, packet/frame
  envelopes, header fields, parser admission, message sequencing metadata,
  checksum-shape metadata, payload bounds, and deterministic protocol admission.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.network-protocol.endpoint-address`: finite endpoint/address
  metadata envelopes.
- `domain-primitives.network-protocol.packet-frame`: finite packet/frame shape
  envelopes.
- `domain-primitives.network-protocol.header-field`: finite header-field
  admission metadata.
- `domain-primitives.network-protocol.parser-admission`: state-free deterministic
  parser admission envelopes.
- `domain-primitives.network-protocol.sequence`: finite message sequencing
  metadata.
- `domain-primitives.network-protocol.checksum-shape`: checksum-shape metadata
  envelopes.
- `domain-primitives.network-protocol.payload-bound`: finite payload length/type
  bound metadata.
- `domain-primitives.network-protocol.admission`: deterministic protocol
  admission/rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/net_*`, `src/protocol_*`, `src/packet_*`, `docs/requirements.md`, tests,
  and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing network/protocol modules now
- transport delivery, packet loss/retry behavior, ordering guarantees, flow or
  congestion control, socket behavior, OS/network-stack correctness, DNS,
  routing, MTU discovery, or distributed-network behavior
- cryptographic authentication, encryption, integrity, replay protection,
  confidentiality, authorization, or broad security claims
- external protocol standard conformance, interoperability, fuzzing coverage,
  streaming parser completeness, performance, allocation behavior, or real-time
  networking guarantees
- checksum error-detection strength, corruption detection, or probabilistic
  guarantees beyond declared finite checksum-shape metadata
