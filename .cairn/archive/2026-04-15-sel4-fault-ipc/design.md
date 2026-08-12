## Context

Fault delivery in seL4 turns structured fault information into IPC-visible state. A dedicated change gives the repo a reusable proof layer for preserving fault class and payload through encode/decode and handler delivery.

## Goals / Non-Goals

**Goals:**
- Preserve fault class and payload across encode/decode.
- Deliver fault records to a handler context.
- Keep payload fields uncorrupted.

**Non-Goals:**
- Exact message-register ABI
- Architecture-specific trap-entry sequences
- Reply-cap handling after fault response

## Data Model

### Fault variants

Version 1 models two structured fault classes:
- `PageFault { address: u64, access: AccessKind, ip: u64 }`
- `Generic { code: u64, data: u64, ip: u64 }`

`AccessKind` is an explicit enum with `Read`, `Write`, and `Execute` values. This is enough to cover the page-fault scenario plus a second payload-bearing class without dragging in full ABI coverage.

### Message representation

The intermediate representation is a fixed semantic message:
- `FaultMessage { class: FaultClass, word0: u64, word1: u64, word2: u64 }`

`encode_fault` maps each structured fault into those three payload words. `decode_fault` is total over all `FaultMessage` values; for page-fault messages it interprets unknown access words conservatively as `Read`.

### Delivery model

The handler-facing state is:
- `HandlerContext { handler_tcb: Option<usize>, inbox: Option<FaultMessage> }`

Delivery is intentionally separate from endpoint queue internals: `deliver_fault(&mut HandlerContext, Fault) -> bool`. If `handler_tcb` is configured, `deliver_fault` returns `true`, stores `Some(encode_fault(fault))` in `inbox`, and leaves `handler_tcb` unchanged; otherwise it returns `false` and leaves state unchanged. Handler-side observation goes through `received_fault`, which decodes `inbox` back into structured fault data.

### Module layout

Landed layout is:
- `src/sel4_fault_ipc.rs` — fault enums, semantic encode/decode, handler delivery, proofs, tests
- `src/lib.rs` — module doc line and `pub mod sel4_fault_ipc;`
- `docs/requirements.md` — Tracey requirement anchors for `sel4-fault-ipc.*`
- `openspec/specs/sel4-fault-ipc/spec.md` — canonical synced spec text

No `src/properties.rs` additions are needed for v1.

## Constructor vs requires policy

Public exec helpers use constructor-normalized inputs instead of range-heavy `requires`:
- `Fault` values are well formed by construction.
- `encode_fault` and `deliver_fault` take structured `Fault` values directly.
- `decode_fault` accepts any `FaultMessage`; it does not require pre-validation.

## Trusted boundary expectation

This design expects `0` new `#[verifier::external_body]` or `assume_specification[...]` bridges. The encode/decode/delivery logic stays fully internalized.

## Decisions

### 1. Model structured faults first

**Choice:** Model structured faults first
**Rationale:** Correctness is about preserving fault meaning, not raw word layout.

### 2. Encode/decode is a semantic roundtrip

**Choice:** Encode/decode is a semantic roundtrip over `Fault` ↔ `FaultMessage`
**Rationale:** This gives a strong, reusable postcondition for handler-side reasoning.

### 3. Keep delivery separate from endpoint internals

**Choice:** Keep delivery separate from endpoint internals
**Rationale:** This change states what arrives, not how the endpoint queue is implemented.

### 4. Mirror Tracey anchors into `docs/requirements.md`

**Choice:** Add `r[sel4-fault-ipc.preservation]`, `r[sel4-fault-ipc.delivery]`, and `r[sel4-fault-ipc.payload]` to `docs/requirements.md` and attach `r[impl]` / `r[verify]` tags to the defining code artifacts.
**Rationale:** Tracey coverage in this repo comes from `docs/requirements.md`, not OpenSpec specs alone.

## Verification Strategy

- `proof fn access_roundtrip` proves encoded page-fault access words decode back to the same `AccessKind` when they come from `encode_fault`.
- `proof fn encode_decode_roundtrip` proves `decode_fault_spec(encode_fault_spec(fault)) == fault` and therefore preserves fault class.
- `proof fn delivery_to_configured_handler` proves a configured handler context decodes the delivered inbox back to the original fault.
- `proof fn encode_decode_preserves_ip` proves the preserved instruction-pointer field survives encode/decode unchanged.
- Unit tests cover page-fault roundtrip, generic-fault payload preservation, configured delivery, and unconfigured no-op delivery.
- Proptests cover randomized page-fault roundtrips, generic-fault roundtrips, and configured delivery metadata preservation.

## Risks / Trade-offs

**[Too many fault classes could bloat the first version]** Keep v1 to two variants with the same proof shape.
**[Endpoint details may leak in]** Keep delivery as a handler-facing contract only.
**[Total decode weakens malformed-message precision]** The public contract only requires roundtrip for messages produced by `encode_fault`, so mapping unknown page-fault access words to `Read` keeps decode simple without weakening the tracked requirements.
