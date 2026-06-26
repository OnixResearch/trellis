## Why

UCAN abilities are hierarchical strings such as `crud/read` and
`msg/send`, not the fixed integer variants used by the older
capability-auth subsumption model. The landed `src/ucan_ability.rs`
module adds a UCAN-native ability surface: slash-separated path
segments, a distinguished top ability `*`, and single-level `*`
matching inside multi-segment patterns.

The important property here is not generic parsing or wildcard syntax
for its own sake. It is that the string-based model preserves the
structural guarantees later UCAN validation code expects: exact-match
reflexivity, transitive subsumption, cross-namespace isolation,
single-level wildcard depth, and containment monotonicity.

## What Changes

A landed module `src/ucan_ability.rs` providing:

**Data model:**
- `Ability` — runtime carrier storing ability segments as `Vec<Vec<u8>>`
- a distinguished spec-level top ability `top_ability_spec() == ["*"]`

**Public exec surface:**
- `ability_subsumes(pattern, ability) -> bool` — runtime segment-by-segment
  matching with a special-case for the top ability

**Spec / proof surface:**
- `ability_subsumes_spec(pattern, ability)` — exact subsumption model
- `ability_contains_spec(parent, child)` — containment alias used for the
  monotonicity proof
- `top_ability_spec()` and `is_glob_segment_spec(...)`

**Verified properties:**
- reflexive subsumption for concrete non-glob abilities
- transitive subsumption
- top ability subsumes all abilities
- namespace isolation for non-top patterns
- single-level `*` matches exactly one segment in multi-segment patterns
- containment monotonicity (`parent` subsumes anything a contained child subsumes)

The older integer-tag `subsumption.rs` model remains unchanged. This
change archives the landed string-ability module as a parallel surface;
it does not replace the older model or add ability parsing helpers.

## Non-Goals

- Recursive glob support such as `**`
- Byte-string parsing helpers such as `parse_ability(...)`
- Unicode normalization or non-ASCII identifier policy
- Replacing the existing integer-tag `subsumption.rs` model
- Proving end-to-end authorization decomposition integration in this
  change; the landed module only proves the string-ability relations
  directly present in `src/ucan_ability.rs`

## Capabilities

### Landed Capabilities
- `ucan-ability.subsumption-reflexive`
- `ucan-ability.subsumption-transitive`
- `ucan-ability.top-subsumes-all`
- `ucan-ability.namespace-isolation`
- `ucan-ability.glob-one-level`
- `ucan-ability.containment-monotone`

## Impact

- **Files**: `src/ucan_ability.rs` plus the synchronized OpenSpec artifacts
- **APIs**: one public exec matcher plus spec/proof helpers
- **Dependencies**: none; pure sequence/byte logic
- **Testing**: focused runtime tests for top/reflexive/glob/namespace behavior and full-lib Verus verification

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for `proposal`, `design`, and `tasks`, plus fresh validation
transcripts for:

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
- `./scripts/verus.sh src/lib.rs`

The resulting gate/runtime/proof evidence must be captured alongside the
archive closeout and post-archive active-change recount.