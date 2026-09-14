# Aeneas/Lean coverage frontier

The Aeneas/Lean coverage frontier records which Trellis modules have a measured
Aeneas eligibility class. It separates a source-scan hypothesis from a measured
class. It never claims whole-library proof coverage.

The exact-source cross-verifier pilot for `src/arithmetic.rs::abs_diff` is
archived at `.cairn/archive/1970-01-01-add-aeneas-cross-verifier-pilot/`. That
pilot records `supported` for one module. This frontier generalizes the
instrument, not the proof.

## Files

| Path | Role |
|---|---|
| `aeneas-frontier/observations.json` | recorded provider observations, one per measured module |
| `aeneas-frontier/targets.json` | selected requirement statements with approved hashes and receipts |
| `aeneas-frontier/generated/frontier.json` | generated frontier artifact with class counts |
| `aeneas-frontier/generated/registry.json` | generated requirement statement registry |
| `scripts/aeneas_frontier/model.rs` | typed records for observations, frontier rows, and registry rows |
| `scripts/aeneas_frontier/validation.rs` | pure classification, construction, and validation |
| `scripts/aeneas_frontier/tests.rs` | positive and negative self-tests |
| `scripts/check-aeneas-frontier.rs` | bounded shell: inventory scan, generated-file publication, exit status |

## Classes

| Class | Meaning |
|---|---|
| `tier-1-primitive` | dependency-free module; safe, sequential primitive logic |
| `tier-2-collection` | module that uses `Vec`, `&mut`, or slices |
| `tier-3-external` | module that references an external crate |
| `tier-4-trust-boundary` | module that contains an `#[verifier::external_body]` site |
| `excluded-construct` | measured provider rejection; the observation names the construct |
| `blocked-<stage>` | measured stage blocker; the observation names the stage and diagnostic class |
| `unmeasured` | no positive measure. A failed tool run also stays unmeasured |

A source scan may propose one of the four tier classes as a hypothesis. Only a
recorded observation sets the measured class.

## Bounds

The frontier declares `modules_per_batch` (8), `theorems_per_module` (4), and
`active_batches` (1). The target validator rejects a selection above a bound.

## Registry rules

The registry holds one row per unique `r[...]` requirement ID in
`docs/requirements.md`. A targeted row records the approved Lean statement
BLAKE3 identities, the claim class, the module, the trust closure, and the
proof receipt. A non-target row records an explicit non-target reason. One
statement identity cannot serve two requirements.

## Commands

```bash
./scripts/check-aeneas-frontier.rs --check
./scripts/check-aeneas-frontier.rs --self-test
```

Run `--write` after a change to `observations.json` or `targets.json`.

## Non-claims

A frontier row is an eligibility observation. It does not prove Aeneas, Charon,
or Lean soundness, Verus/Aeneas semantic equivalence, verifier independence,
whole-library coverage, or release eligibility. Class counts are not completion
evidence.
