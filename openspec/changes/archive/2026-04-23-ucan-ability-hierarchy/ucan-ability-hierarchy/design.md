## Context

The landed `src/ucan_ability.rs` module already implements and proves a
narrow string-ability relation for UCAN-style hierarchical abilities.
The active change artifacts still describe broader greenfield work such
as parsing helpers and integration proofs that the landed module does not
provide. This design reconciles the change to the code that actually
exists.

## Goals / Non-Goals

**Goals:** Document the landed byte-segment model, top-ability special
case, single-level wildcard behavior, containment alias, and the exact
proof/test evidence that closes the current module.

**Non-Goals:** Recursive `**`, `parse_ability(...)`, Unicode
normalization, replacement of `subsumption.rs`, or a cross-module proof
that plugs string abilities into a larger `authorizes` decomposition.

## Decisions

### 1. Abilities stay as raw segment byte vectors

**Choice:** Runtime abilities are stored as `Vec<Vec<u8>>`, with proofs
reasoning over `Seq<Seq<u8>>`.

**Rationale:** This matches the landed code exactly and avoids inventing a
parser or normalized string format that does not exist in the module.

### 2. The top ability is a distinguished special case

**Choice:** `top_ability_spec()` is the single segment `"*"`, but it is
not treated like an ordinary one-level wildcard pattern.

**Rationale:** In landed code, `ability_subsumes_spec(...)` first checks
whether the pattern equals `top_ability_spec()` and, if so, returns true
for any ability depth. That distinguishes top from the per-segment
single-level wildcard used inside longer patterns.

### 3. Multi-segment `*` remains single-level only

**Choice:** Inside a multi-segment pattern such as `crud/*`, `*` matches
exactly one segment and never deeper nesting.

**Rationale:** The landed matcher requires equal segment counts for all
non-top patterns, so `crud/*` can match `crud/read` but not
`crud/nested/deep`.

### 4. Containment is a proof-layer alias to subsumption

**Choice:** `ability_contains_spec(parent, child)` is defined directly as
`ability_subsumes_spec(parent, child)` for the landed module.

**Rationale:** That is the exact code in `src/ucan_ability.rs`. The
containment-monotone proof is therefore a second-order property over the
same underlying matcher, not a distinct exec API.

### 5. Integration claims are deferred

**Choice:** This archived change stops at the direct ability relations in
`src/ucan_ability.rs` and does not claim a separate proof of larger
authorization decomposition.

**Rationale:** The landed source contains no proof or exec surface for the
broader `authorizes = subsumes && covers && mode_permits` story. Keeping
that out of scope avoids spec drift.

## Verification Mapping

### Requirement-level mapping

- `r[ucan-ability.subsumption-reflexive]` — `src/ucan_ability.rs:ability_subsumes_reflexive` plus `src/ucan_ability.rs:test_reflexive`
- `r[ucan-ability.subsumption-transitive]` — `src/ucan_ability.rs:ability_subsumes_transitive`
- `r[ucan-ability.top-subsumes-all]` — `src/ucan_ability.rs:top_subsumes_all` plus `src/ucan_ability.rs:test_top_subsumes_all`
- `r[ucan-ability.namespace-isolation]` — `src/ucan_ability.rs:namespace_isolation` plus `src/ucan_ability.rs:test_namespace_isolation`
- `r[ucan-ability.glob-one-level]` — `src/ucan_ability.rs:glob_one_level_matches` plus `src/ucan_ability.rs:test_glob_one_level`
- `r[ucan-ability.containment-monotone]` — `src/ucan_ability.rs:containment_monotone`

### Scenario-level mapping

- `r[ucan-ability.subsumption-reflexive.scenario-exact-match]` — `src/ucan_ability.rs:test_reflexive`
- `r[ucan-ability.subsumption-transitive.scenario-top-and-glob]` — `src/ucan_ability.rs:ability_subsumes_transitive`
- `r[ucan-ability.top-subsumes-all.scenario-star-matches-any-ability]` — `src/ucan_ability.rs:test_top_subsumes_all`
- `r[ucan-ability.namespace-isolation.scenario-msg-vs-crud]` — `src/ucan_ability.rs:test_namespace_isolation`
- `r[ucan-ability.glob-one-level.scenario-one-level-match]` — `src/ucan_ability.rs:test_glob_one_level`
- `r[ucan-ability.glob-one-level.scenario-nested-rejected]` — `src/ucan_ability.rs:test_glob_one_level`
- `r[ucan-ability.containment-monotone.scenario-delegation-narrows-ability]` — `src/ucan_ability.rs:containment_monotone`

## Validation

Archive-ready validation uses:

- `openspec_gate stage=proposal change=ucan-ability-hierarchy`
- `openspec_gate stage=design change=ucan-ability-hierarchy`
- `openspec_gate stage=tasks change=ucan-ability-hierarchy`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
- `./scripts/verus.sh src/lib.rs`

## Risks / Trade-offs

**[No parser surface]** → Callers must construct segmented abilities
outside this module. This is acceptable because the landed code never
claimed a parser.

**[ASCII-by-convention only]** → Segment bytes are compared literally.
That matches current UCAN practice but leaves any normalization policy to
higher layers.