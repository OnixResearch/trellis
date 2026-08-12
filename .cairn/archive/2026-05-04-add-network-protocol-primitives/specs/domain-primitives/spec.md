# Network Protocol Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite network-protocol primitives.
These primitives MUST use exact finite endpoint/address metadata, packet/frame
envelopes, header-field admission metadata, state-free parser admission, message
sequencing metadata, checksum-shape metadata, payload bounds, and deterministic
protocol admission/rejection. They MUST avoid transport delivery, socket or OS
network-stack behavior, DNS, routing, congestion control, distributed-network
behavior, cryptographic authentication, encryption, integrity, replay protection,
authorization, external protocol conformance, streaming parser completeness,
performance, or network security claims unless a later focused OpenSpec owns
those models.

## ADDED Requirements

### Requirement: Endpoint Address Metadata [r[domain-primitives.network-protocol.endpoint-address]]
The system MUST provide finite endpoint/address metadata envelopes that record
address-family tags, host/address identifiers, port/service metadata, scope tags,
and deterministic endpoint admission results without consulting DNS, sockets, or
network interfaces.

#### Scenario: Unknown endpoint family is rejected explicitly [r[domain-primitives.network-protocol.endpoint-address.scenario.unknown-endpoint-family-is-rejected-explicitly]]

- GIVEN finite endpoint/address metadata and a declared set of admitted address-family tags
- WHEN a candidate endpoint uses an unknown address-family tag
- THEN the endpoint primitive returns an explicit rejection without consulting DNS or the OS network stack

### Requirement: Packet Frame Envelope [r[domain-primitives.network-protocol.packet-frame]]
The system MUST provide finite packet/frame envelopes that record byte-span
bounds, frame-kind tags, declared header ranges, payload ranges, and deterministic
frame admission results.

#### Scenario: Payload range must fit inside frame [r[domain-primitives.network-protocol.packet-frame.scenario.payload-range-must-fit-inside-frame]]

- GIVEN a finite packet/frame envelope with header and payload ranges
- WHEN the payload range is outside the declared frame bounds
- THEN the packet-frame primitive rejects the envelope before parser admission is composed

### Requirement: Header Field Metadata [r[domain-primitives.network-protocol.header-field]]
The system MUST provide finite header-field metadata envelopes for field names or
IDs, field ranges, required/optional status, duplicate policy, and deterministic
header admission results.

#### Scenario: Missing required header is reported [r[domain-primitives.network-protocol.header-field.scenario.missing-required-header-is-reported]]

- GIVEN finite header-field metadata and required header definitions
- WHEN a required header field is absent
- THEN the header-field primitive returns explicit missing-header metadata

### Requirement: State-Free Parser Admission [r[domain-primitives.network-protocol.parser-admission]]
The system MUST provide deterministic state-free parser admission envelopes over
one finite packet, frame, or message under declared endpoint, header, payload,
and checksum-shape metadata.

#### Scenario: Parser admission does not claim stream completeness [r[domain-primitives.network-protocol.parser-admission.scenario.parser-admission-does-not-claim-stream-completeness]]

- GIVEN one finite packet/frame/message envelope
- WHEN parser admission accepts the envelope under declared finite rules
- THEN the result admits only that envelope and does not claim streaming parser completeness, reassembly, delivery, or session correctness

### Requirement: Message Sequencing Metadata [r[domain-primitives.network-protocol.sequence]]
The system MUST provide finite message sequencing metadata for declared sequence
numbers, acknowledgement-like fields, monotone/window bounds, duplicate markers,
and deterministic sequencing admission results.

#### Scenario: Sequence metadata does not imply delivery ordering [r[domain-primitives.network-protocol.sequence.scenario.sequence-metadata-does-not-imply-delivery-ordering]]

- GIVEN finite message sequencing metadata
- WHEN a candidate sequence field is admitted
- THEN the sequencing primitive records only declared metadata and does not claim transport delivery, ordering, replay protection, or liveness

### Requirement: Checksum Shape Metadata [r[domain-primitives.network-protocol.checksum-shape]]
The system MUST provide finite checksum-shape metadata envelopes for checksum
field location, width, covered ranges, stored value shape, and deterministic
shape admission without claiming checksum strength or corruption detection.

#### Scenario: Checksum shape does not imply integrity [r[domain-primitives.network-protocol.checksum-shape.scenario.checksum-shape-does-not-imply-integrity]]

- GIVEN finite checksum-shape metadata and a candidate packet/frame
- WHEN the checksum field shape is admitted
- THEN the checksum-shape primitive records field-shape metadata only and does not claim integrity, authentication, or corruption detection

### Requirement: Payload Bound Metadata [r[domain-primitives.network-protocol.payload-bound]]
The system MUST provide finite payload-bound metadata for payload byte ranges,
length-prefix agreement, payload-kind tags, empty-payload policy, and
deterministic payload admission results.

#### Scenario: Payload length mismatch is rejected [r[domain-primitives.network-protocol.payload-bound.scenario.payload-length-mismatch-is-rejected]]

- GIVEN finite payload-bound metadata and a declared payload length
- WHEN the payload range length does not match the declared payload length
- THEN the payload-bound primitive returns explicit length-mismatch rejection metadata

### Requirement: Protocol Admission Envelope [r[domain-primitives.network-protocol.admission]]
The system MUST provide deterministic protocol admission/rejection envelopes that
compose endpoint/address metadata, packet/frame envelopes, header fields,
state-free parser admission, sequencing metadata, checksum-shape metadata, and
payload bounds over finite protocol metadata.

#### Scenario: Admission requires all finite protocol checks [r[domain-primitives.network-protocol.admission.scenario.admission-requires-all-finite-protocol-checks]]

- GIVEN finite protocol metadata and a candidate packet/frame/message
- WHEN any endpoint, frame, header, parser, sequence, checksum-shape, or payload-bound check rejects
- THEN the protocol-admission primitive returns an explicit rejection decision before any transport, security, delivery, or external protocol claim is made
