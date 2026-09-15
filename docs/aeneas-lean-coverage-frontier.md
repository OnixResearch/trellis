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
| `aeneas-frontier/evidence/batch-2026-09-14.json` | local extraction batch evidence: item, status, seconds, LLBC bytes, LLBC BLAKE3, tool identities |
| `aeneas-frontier/evidence/batch-2026-09-14-reviewed.json` | reviewed-provider batch evidence: Charon and Aeneas binary BLAKE3, LLBC identity, generated Lean identity, translation manifest identity, budget record |
| `aeneas-frontier/evidence/batch-2026-09-15-sandboxed.json` | sandboxed batch evidence: denied-network provider run with per-stage status, time, and identities |
| `aeneas-frontier/evidence/verify-2026-09-15.txt` | repo verification transcript: all checks passed |
| `aeneas-frontier/evidence/octet-receipt-blocker-2026-09-15.txt` | why no `octet-proof-receipt` row existed at that time, with the pinned CLI build error |
| `aeneas-frontier/evidence/octet-rail-clamp-2026-09-15/` | the Octet formal-proof envelope receipt for `bounds.clamp`, its status, and its working conditions |
| `aeneas-frontier/theorems/rail/` | theorem copies that import the rail's `Llbc` module |
| `aeneas-frontier/evidence/lean-2026-09-14.json` | local Lean build evidence: theorem statements, axiom closure, generated-model identities, library `sorry` sites, local receipt |
| `aeneas-frontier/evidence/lean-build-2026-09-14.txt`, `-b.txt`, `-c.txt` | bounded build transcripts: command, axiom lines, summary |
| `aeneas-frontier/evidence/lean-negatives-2026-09-14.txt` | bounded transcript for the false and vacuous Lean runs |
| `aeneas-frontier/evidence/octet-adequacy-2026-09-14.txt` | Octet adequacy rejection of the vacuous theorem, with no receipt written |
| `aeneas-frontier/generated/lean/` | generated Lean models and translation manifests from the reviewed batch |
| `aeneas-frontier/theorems/` | maintained Lean theorems and the axiom audit |
| `aeneas-frontier/generated/frontier.json` | generated frontier artifact with class counts |
| `aeneas-frontier/generated/registry.json` | generated requirement statement registry |
| `scripts/aeneas_frontier/model.rs` | typed records for observations, frontier rows, and registry rows |
| `scripts/aeneas_frontier/validation.rs` | pure classification, construction, and validation |
| `scripts/aeneas_frontier/tests.rs` | positive and negative self-tests |
| `scripts/check-aeneas-frontier.rs` | bounded shell: inventory scan, generated-file publication, exit status |

## Measurement status

The measured classes come from the reviewed Octet provider binaries: Charon
extraction (`charon_binary_blake3` `b3:64ee129c…`) and Aeneas Lean translation
(`aeneas_binary_blake3` `b3:428ce05d…`). Eight items over six modules reached
the translation stage, and the archived pilot reached the Lean theorem stage
for `src/arithmetic.rs::abs_diff`. Five batch theorems now check in Lean through
a local build: `bounds.clamp`, `bounds.in-range`, `interval.contains`,
`ordering.min` (selection), and `ordering.min` (commutativity, the independent
counterpart of the Verus `ordering::min_commutative` proof). The `bounds.clamp`
requirement also carries an Octet formal-proof envelope receipt produced by the
pinned CLI; that receipt's working conditions and deviations are recorded with
it. All eight batch items also ran through Charon extraction and Aeneas Lean
translation inside the provider sandbox with a denied network namespace. Each
theorem carries the `propext` axiom closure; the local registry receipts carry
the `local-lake-build` class. The Aeneas Lean library itself
contains four `sorry` sites (`Aeneas/Std/Slice.lean` and
`Aeneas/Std/StringIter.lean`); the theorem axiom audit reports only `propext`,
and the sorry sites stay recorded as library trust facts. The batch runs used a
local work root and target directory, not the provider sandbox, so the reviewed
execution shell stays open. A measured class is an eligibility observation, not
proof coverage.

## Classes

| Class | Meaning |
|---|---|
| `tier-1-primitive` | dependency-free module; safe, sequential primitive logic |
| `tier-2-collection` | module that uses `Vec`, `&mut`, or slices |
| `tier-3-external` | module that references an external crate |
| `tier-4-trust-boundary` | module that contains an `#[verifier::external_body]` site |
| `excluded-construct` | measured provider rejection; the observation names the construct |
| `blocked-<stage>` | measured stage blocker; the observation names the stage and diagnostic class |
| `unmeasured` | no positive measure. A failed tool run also stays unmeasured; a failed observation records one closed diagnostic class (`stage-timeout`, `process-spawn-failure`, `provider-identity-drift`, `artifact-read-failure`) |

A source scan may propose one of the four tier classes as a hypothesis. Only a
recorded observation sets the measured class, and a measured class requires
`reviewed: true` on that observation. An unreviewed local run can never produce
a measured class.

## Bounds

The frontier declares `modules_per_batch` (8), `theorems_per_module` (4), and
`active_batches` (1). The target validator rejects a selection above a bound.

## Registry rules

The registry holds one row per unique `r[...]` requirement ID in
`docs/requirements.md`. A targeted row records the approved Lean statement
BLAKE3 identities, the claim class, the module, the trust closure, and the
proof receipt. A non-target row records an explicit non-target reason. One
statement identity cannot serve two requirements.

Every targeted row also records a receipt class and a receipt evidence path:

- `trellis-proof-receipt` is a Trellis receipt built from the reviewed provider
  stage evidence. The archived pilot uses this class.
- `octet-proof-receipt` is an Octet `cargo octet prove` formal-proof envelope.
  The `bounds.clamp` row uses this class; the receipt evidence records the
  working conditions and deviations of that run.
- `local-lake-build` is a local Lean build transcript. It proves the named
  theorem over the recorded generated model, and it carries no rail authority.

## Commands

```bash
./scripts/check-aeneas-frontier.rs --check
./scripts/check-aeneas-frontier.rs --self-test
```

Run `--write` after a change to `observations.json` or `targets.json`. Both
`--write` and `--check` scan this document and
`docs/aeneas-cross-verifier-pilot.md` for affirmative overclaims, and reject a
sentence that claims whole-library coverage, progress, semantic equivalence,
verifier independence, verifier soundness, or trust-boundary closure.

## Non-claims

A frontier row is an eligibility observation. It does not prove Aeneas, Charon,
or Lean soundness, Verus/Aeneas semantic equivalence, verifier independence,
whole-library coverage, or release eligibility. Class counts are not completion
evidence.
