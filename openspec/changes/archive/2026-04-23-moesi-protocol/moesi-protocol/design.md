## Context

`src/mesi.rs` already proves the standard four-state MESI line model.
MOESI extends that model with `Owned`, a dirty shared supplier that lets
a cache answer a remote read without making memory current first. The
landed design deliberately keeps the scope at the same one-line level as
MESI so the proof burden stays local and composable.

## Goals / Non-Goals

**Goals:** Model the five line states, prove dirty-owner exclusivity,
prove the defining `Modified -> Owned` remote-read path, model owner
writeback on eviction, keep read freshness at the cache-vs-memory source
level, and prove pointwise MESI refinement outside `Owned` paths.

**Non-Goals:** multi-line protocols, exact payload bytes, bus ordering,
fence behavior, directory metadata, or MESIF/Forward-state semantics.

## Decisions

### 1. Reuse MESI numeric encodings for shared states

**Choice:** `Invalid`, `Shared`, `Exclusive`, and `Modified` keep the
same numeric values as `src/mesi.rs`; `Owned` is added as a fifth value.

**Rationale:** This keeps MESI refinement straightforward because the
shared subset already matches the existing MESI state constants.

### 2. Model whole-line operations with pointwise sequence transforms

**Choice:** `snoop_read_all_spec(...)` and `invalidate_others_spec(...)`
are direct `Seq::new(...)` transforms rather than recursive scans.

**Rationale:** MOESI whole-line operations are pointwise on a single line
state per cache. The pointwise definition makes index-level facts,
length preservation, and refinement easier to prove than scan-style
recursion.

### 3. Separate dirty-owner state from memory freshness

**Choice:** Dirty ownership is the predicate `state in {Modified,
Owned}`. Memory freshness is tracked by a separate boolean constrained by
`memory_consistent_spec(caches, memory_up_to_date) <==>
memory_up_to_date == !dirty_owner_exists_spec(caches)`.

**Rationale:** This is the smallest model that can express owner supply,
writeback on owner eviction, and “reads never use stale memory” without
introducing full value semantics.

### 4. Remote read keeps `Owned` resident

**Choice:** On `BusRead`, `Modified -> Owned`, `Owned -> Owned`,
`Exclusive -> Shared`, and `Shared/Invalid` stay unchanged.

**Rationale:** This selects the common MOESI variant where the owner
continues supplying dirty data until eviction or local write. The proof
surface then matches the intended optimization directly.

### 5. MESI refinement is pointwise and excludes Owned-producing paths

**Choice:** Refinement is proved only for transitions whose source and
result stay inside `M/E/S/I`. In practice, that excludes the defining
`Modified + BusRead -> Owned` case and any source state already equal to
`Owned`.

**Rationale:** That is the precise boundary where MOESI differs from
MESI. Everything else should agree after mapping `Owned -> Shared`.

## Verification Mapping

### Requirement-level mapping

- `r[moesi.single-writer]` — `src/moesi.rs:single_writer_spec`, `src/moesi.rs:full_read_preserves_single_writer`, `src/moesi.rs:full_write_preserves_single_writer`, `src/moesi.rs:evict_preserves_single_writer`
- `r[moesi.owned-entry]` — `src/moesi.rs:modified_remote_read_promotes_owned`
- `r[moesi.owned-supply]` — `src/moesi.rs:dirty_data_supplied_spec`, `src/moesi.rs:owned_holder_supplies_dirty_data`
- `r[moesi.writeback]` — `src/moesi.rs:evict_writes_back_spec`, `src/moesi.rs:owned_evict_writes_back`, `src/moesi.rs:shared_evict_skips_writeback`, and the unit tests `src/moesi.rs:modified_evict_writes_back` / `src/moesi.rs:owned_evict_writes_back`
- `r[moesi.transitions]` — `src/moesi.rs:is_valid_state_spec`, `src/moesi.rs:processor_transition_state_spec`, `src/moesi.rs:processor_bus_action_spec`, `src/moesi.rs:snoop_transition_state_spec`, `src/moesi.rs:transition_completeness`
- `r[moesi.mesi-refinement]` — `src/moesi.rs:moesi_to_mesi_spec`, `src/moesi.rs:processor_read_refines_mesi`, `src/moesi.rs:processor_write_refines_mesi`, `src/moesi.rs:snoop_read_refines_mesi`, `src/moesi.rs:snoop_invalidate_refines_mesi`
- `r[moesi.no-stale-reads]` — `src/moesi.rs:memory_consistent_spec`, `src/moesi.rs:read_source_spec`, `src/moesi.rs:read_fresh_spec`, `src/moesi.rs:memory_consistency_implies_read_fresh`

### Scenario-level mapping

- `r[moesi.single-writer.scenario-mutual-exclusion]` — `src/moesi.rs:full_read_preserves_single_writer`, `src/moesi.rs:full_write_preserves_single_writer`
- `r[moesi.owned-entry.scenario-modified-remote-read]` — `src/moesi.rs:modified_remote_read_promotes_owned`
- `r[moesi.owned-supply.scenario-owned-supplies-data]` — `src/moesi.rs:owned_holder_supplies_dirty_data`
- `r[moesi.writeback.scenario-modified-eviction]` — `src/moesi.rs:modified_evict_writes_back`
- `r[moesi.writeback.scenario-owned-eviction]` — `src/moesi.rs:owned_evict_writes_back`
- `r[moesi.writeback.scenario-shared-eviction-clean]` — `src/moesi.rs:shared_evict_skips_writeback`
- `r[moesi.transitions.scenario-valid-next-state]` — `src/moesi.rs:transition_completeness`
- `r[moesi.mesi-refinement.scenario-non-owned-matches-mesi]` — `src/moesi.rs:processor_read_refines_mesi`, `src/moesi.rs:processor_write_refines_mesi`, `src/moesi.rs:snoop_read_refines_mesi`, `src/moesi.rs:snoop_invalidate_refines_mesi`
- `r[moesi.no-stale-reads.scenario-dirty-owner-blocks-memory]` — `src/moesi.rs:memory_consistency_implies_read_fresh`

## Validation

Archive-ready validation uses:

- `openspec_gate stage=proposal change=moesi-protocol`
- `openspec_gate stage=design change=moesi-protocol`
- `openspec_gate stage=tasks change=moesi-protocol`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib moesi`
- `./scripts/verus.sh src/lib.rs`

## Risks / Trade-offs

**[Source-level freshness only]** → The landed proof says reads use the
correct *source* (cache vs memory), not that a full payload value model
was carried through every transition.

**[Owned-stays-owned choice]** → The design commits to the variant where
an owner remains `Owned` on a remote read. Other MOESI-family variants
would need a different refinement story.
