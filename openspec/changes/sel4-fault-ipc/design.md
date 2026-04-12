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

## Decisions

### 1. Model structured faults first

**Choice:** Model structured faults first
**Rationale:** Correctness is about preserving fault meaning, not raw word layout.

### 2. Encode/decode is a semantic roundtrip

**Choice:** Encode/decode is a semantic roundtrip
**Rationale:** This gives a strong, reusable postcondition for handler-side reasoning.

### 3. Keep delivery separate from endpoint internals

**Choice:** Keep delivery separate from endpoint internals
**Rationale:** This change states what arrives, not how the endpoint queue is implemented.

## Risks / Trade-offs

**[Too many fault classes could bloat the first version]** Start with a small structured fault sum type and extensible payload fields.
**[Endpoint details may leak in]** Keep delivery as a handler-facing contract only.
