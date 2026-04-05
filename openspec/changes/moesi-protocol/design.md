## Context

The existing `mesi.rs` models four states with processor and bus events
driving transitions. MOESI adds the Owned state between Modified and
Shared. A cache in O holds dirty data and can supply it to other caches,
but other caches may also hold the line in S. This module extends the
verification approach from `mesi.rs` to the five-state protocol.

## Goals / Non-Goals

**Goals:**
- Five-state model: M, O, E, S, I with all valid transitions
- Single-writer proof: at most one cache in M or O at any time
- Owned-supplies-data proof: O cache is the data source, not memory
- MESI refinement: MOESI restricted to non-O transitions matches MESI
- Transition completeness: every (state, event) pair is defined

**Non-Goals:**
- Bus arbitration or transaction ordering
- MOESI variants (e.g., AMD's actual HyperTransport protocol details)
- Performance modeling or cache latency
- Multi-level MOESI (single-level only)

## Decisions

### 1. State Representation

**Choice:** `u8` constants: `STATE_M = 0`, `STATE_O = 1`, `STATE_E = 2`,
`STATE_S = 3`, `STATE_I = 4`. Consistent with `mesi.rs` style but with O
inserted.

**Rationale:** Matches the existing `mesi.rs` pattern. The numbering
puts M and O adjacent since they share the "dirty data holder" property.

### 2. Multi-Cache Model

**Choice:** A `Seq<u8>` of per-cache states for a single line, plus a
`bool` for whether memory is up-to-date.

**Rationale:** MOESI's key invariant (at most one M or O) is a property
across all caches for one line. Modeling as a sequence lets us quantify
over all caches in the proof. The memory-up-to-date flag tracks whether
any cache holds dirty data (M or O means memory is stale).

### 3. Transition Function

**Choice:** Two functions: `processor_transition(state, event) -> (new_state, bus_action)` and `snoop_transition(state, bus_action) -> new_state`.
Bus actions: BusRead, BusReadX, BusUpgrade, Flush, FlushOpt.

**Rationale:** Separating processor-initiated and snoop-initiated transitions
matches the hardware architecture and keeps each function's case analysis
manageable. FlushOpt is the new bus action for MOESI: O supplies data
without writing back to memory.

### 4. MESI Refinement

**Choice:** A mapping function `moesi_to_mesi(state) -> mesi_state` that
maps O→S (the closest MESI equivalent) and M/E/S/I to themselves. Prove
that for transitions not involving the O state, `moesi_transition` composed
with the mapping equals `mesi_transition`.

**Rationale:** This is the backward compatibility proof. Systems that
don't use the O optimization should see identical behavior.

### 5. Single-Writer Proof Structure

**Choice:** An invariant predicate `single_writer(caches: Seq<u8>) -> bool`
that checks: `count(caches, M) + count(caches, O) <= 1`. Prove by induction
that every transition preserves this invariant.

**Rationale:** M and O are the two "dirty owner" states. The invariant is
that at most one cache is the dirty owner. The inductive proof checks each
transition case: M→O (on remote read) doesn't create a second owner because
the transition is on the same cache; I→M (on local write with no other
owner) is granted only when the invariant holds.

## Risks / Trade-offs

**[O→S transition ambiguity]** → In some MOESI variants, O can transition
to S when another cache takes ownership. The exact trigger depends on the
bus protocol. We model the most common variant (O stays O until eviction
or local write) and document the alternative.

**[Memory staleness tracking]** → Tracking whether memory is up-to-date
adds a boolean to the model but simplifies the owned-supplies-data proof.
Without it, you'd need to search all caches for M or O on every memory
read, which is what the directory protocol does — out of scope here.
