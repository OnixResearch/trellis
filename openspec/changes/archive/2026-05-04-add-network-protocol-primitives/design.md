## Context

The domain-primitives umbrella now tracks serialization/codec,
identifier/namespace, filesystem-path, and storage/index families. Network
protocol data needs its own finite shape primitives so future proofs can reason
about packets, headers, endpoints, parser admission, and message metadata without
claiming runtime network-stack, security, or external protocol behavior.

## Goals / Non-Goals

**Goals:**

- specify exact finite endpoint/address metadata and packet/frame envelopes
- specify finite header-field and payload-bound admission metadata
- specify state-free parser admission over finite byte/message envelopes
- specify sequencing and checksum-shape metadata without claiming transport or
  security behavior
- specify deterministic protocol admission/rejection without implementation yet

**Non-Goals:**

- implementing runtime APIs in this spec-only change
- proving socket, OS network-stack, routing, DNS, MTU, delivery, ordering,
  liveness, congestion, or distributed-network behavior
- proving cryptographic authentication, encryption, integrity, replay, or broad
  security properties
- proving external protocol standard conformance, streaming parser completeness,
  performance, or interoperability

## Decisions

### 1. Model protocol data as finite envelopes

**Choice:** Protocol primitives use finite endpoint/address records,
packet/frame byte spans, header-field tables, payload bounds, and parser
admission metadata rather than live sockets or stream state.

**Rationale:** Finite envelopes compose with serialization/codec, data
validation, identifier/namespace, and resource-accounting primitives while
remaining Verus-friendly.

**Alternative:** Start with TCP/UDP/HTTP semantics. Rejected because runtime
network behavior and external protocol conformance are too broad for a reusable
primitive slice.

### 2. Keep parser admission state-free

**Choice:** Parser primitives admit or reject one finite message/frame envelope
under declared header/payload rules.

**Rationale:** State-free admission is useful for reusable shape proofs and
avoids hidden transport, reassembly, streaming, or session guarantees.

### 3. Treat sequencing and checksums as metadata

**Choice:** Sequencing primitives record finite counters/order metadata, and
checksum primitives record declared field shape and comparison metadata only.

**Rationale:** This supports deterministic bookkeeping without claiming delivery
ordering, anti-replay, corruption detection, or cryptographic integrity.

## Risks / Trade-offs

**Overlap with serialization/codec** → This child owns endpoint, header, packet,
message, sequencing, and protocol admission semantics; serialization/codec owns
byte-buffer, frame, schema, and encoder/decoder shape primitives.

**Security drift** → Requirements explicitly avoid authentication, encryption,
integrity, replay, authorization, and broad security claims. Security policies
must live in later focused changes.

**External-standard drift** → Requirements avoid protocol conformance and
interoperability claims unless a later focused exact model owns them.

## Validation Plan

- `openspec validate add-network-protocol-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about future implementation tasks
- future implementation must add `docs/requirements.md`, `r[impl]`,
  `r[depends]`, `r[verify]`, tests, Verus proof evidence, and Tracey evidence
