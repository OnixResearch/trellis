## Context

The cheapest archiveable slice is a single-line prefetch core rather than a
full queue or hardware scheduler model. This repo already has a verified MESI
line model, so prefetch can be expressed as a pure wrapper around those
transitions plus a tiny replacement/writeback helper.

## Goals / Non-Goals

**Goals:**
- keep architectural state separate from cache state
- model read-prefetch and write-prefetch as pure functions
- reuse MESI-compatible line-state transitions
- model dropped-prefetch explicitly as identity
- model dirty-victim replacement against a finite memory vector

**Non-Goals:**
- hardware prefetch queues or stream detection
- multi-line set associativity or victim selection policy proofs
- TLB/page-table modeling beyond the caller-supplied `addr_valid` bit
- timing-channel or speculative side-channel analysis

## Decisions

### 1. Architectural state stays explicit and unchanged

**Choice:** represent architectural state as `{ regs, memory, pc }` and return
that state unchanged from each prefetch helper.

**Rationale:** this directly captures architectural invisibility while keeping
execution pure and testable.

### 2. Cache effect reuses existing MESI transitions

**Choice:** import `crate::mesi::*` directly, model prefetch-for-read with
`mesi::full_read_spec(...)`, and model prefetch-for-write as
`invalidate_others + requester := Exclusive(clean)`.

**Rationale:** the repo already proved the demand-access line transitions. The
prefetch model should reuse those semantics instead of inventing a second cache
protocol. Because the module imports `crate::mesi::*`, verification evidence is
captured via `./scripts/verus.sh src/lib.rs` rather than a per-file run.

### 3. Invalid-address behavior is an explicit no-op branch

**Choice:** gate read/write prefetch behavior on `addr_valid: bool`; when false,
return cloned architectural and cache state unchanged.

**Rationale:** this captures the “hint only” contract without pulling page-table
state into the module.

### 4. Replacement uses an explicit victim slot

**Choice:** model replacement with `(set, memory, victim, incoming) ->
(set', memory')`, where `victim` is a caller-supplied index and dirty victims
write back to `memory[victim.tag]` when that index is in range.

**Rationale:** this isolates the safety property from victim-selection policy and
keeps the proof surface small.

### 5. Bus action is a pure observation helper

**Choice:** keep `prefetch_bus_action(...)` separate from the state-mutating
prefetch helpers.

**Rationale:** this makes the coherence requirement explicit without forcing the
cache-state helpers to return effect bundles.

## Risks / Trade-offs

**[No full prefetch trace model]** -> Accept. A dropped-prefetch identity step
plus per-step architectural invisibility is enough for this initial core.

**[Single-line cache view only]** -> Accept. The module is meant to verify the
prefetch primitive, not a full cache implementation.