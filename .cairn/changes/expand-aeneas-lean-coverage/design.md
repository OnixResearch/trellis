## Context

The pilot archived `supported` for the exact runtime item
`src/arithmetic.rs::abs_diff`. Growth needs two instruments that the pilot
deliberately excluded: a measured eligibility frontier over the module tree,
and a registry that maps existing Trellis requirement IDs to Lean statement
identities. This change lands those two instruments and leaves the proof batch
for a later step.

The rail's supported subset is safe, sequential Rust inside the reviewed
Charon subset. Unsafe Rust, concurrent Rust, unsupported nested-loop control
transfer, unresolved external definitions, and unsupported extraction features
stay blocked. Trellis already forbids `unsafe_code`, so the unsafe limit costs
little. The concurrency and external-definition limits matter for specific
module groups.

## Goals

- Make coverage a measured fact per class, not a plan promise.
- Keep every Lean statement tied to an existing Trellis requirement ID.
- Reuse the pilot and provider contracts without local pinning.
- Bound the first batch so review can judge cost before expansion.

## Non-Goals

- No proof of semantic equivalence between Verus specifications and Aeneas
  models.
- No replacement of Verus proof authority.
- No claim of whole-library proof coverage.
- No closure of the 16 `external_body` trust boundaries in this change.
- No hand-written Lean models for blake3, ed25519, redb, regex, serde_json, or
  bs58 in this change.

## Decisions

### 1. Coverage is counted per class, per requirement

**Choice:** The coverage receipt counts modules and requirements by measured
class. It never prints a library-wide "proved in Lean" percentage.

**Rationale:** The unit of a Lean claim is one theorem over one generated
model with one trust closure. A module count hides blocked modules and
unregistered requirements. A percentage invites an overclaim that the pilot
claim-boundary rules reject.

### 2. Eligibility classes come from measured attempts

**Choice:** The frontier records one class per top-level module from these
values: `tier-1-primitive`, `tier-2-collection`, `tier-3-external`,
`tier-4-trust-boundary`, `excluded-construct`, `blocked-<stage>`, or
`unmeasured`. A source scan may propose a hypothesis class. Only a recorded
extraction, translation, or admission observation sets the measured class.

**Rationale:** A source scan cannot see the supported subset. Macro erasure,
borrow-check options, generic mutable-reference instantiation, and unresolved
definitions change the outcome. The pilot already requires a deterministic
supported/blocked outcome; the frontier reuses that observation shape.

### 3. The statement registry owns Lean statement authority

**Choice:** One registry row per targeted requirement records:

- the existing `r[...]` requirement ID;
- the approved Lean statement BLAKE3;
- the claim class;
- the measured frontier class of the owning module;
- integer, overflow, and model notes;
- the declared trust closure;
- the pilot receipt reference; and
- an explicit non-target reason when the requirement stays out of scope.

**Rationale:** The pilot requires a reviewed statement hash, requirement ID,
claim class, and complete trust closure for each theorem. The registry makes
those facts countable and diffable across batches.

### 4. One bounded batch, then review

**Choice:** The frontier declares `modules_per_batch` (8),
`theorems_per_module` (4), and `active_batches` (1). Per-stage time, log,
memory, and retained-artifact limits come from the reviewed provider profile
and stay named in the receipt. Expansion needs a new change with its own
bounds.

**Rationale:** A successful microcase does not establish cost or stability
for the remaining corpus. A named bound keeps the review honest and keeps the
record small.

### 5. Reuse pilot gates and the provider contract

**Choice:** Batch work consumes the reviewed Octet provider package, the
extraction profile, and the pilot's freshness, adequacy, tamper, budget, and
manifest checks. Trellis does not reconstruct provider pins and does not fork
Aeneas, Charon, OCaml, or Lean package logic.

**Rationale:** Octet owns checked Rust proof execution. Duplicate local gates
would create a second, weaker authority.

### 6. Start only after the pilot reports

**Choice:** The pilot archived with a supported result, so the frontier and
registry work proceeds. The measured proof batch stays out of this change
until a provider run records per-module observations and the selected module
has a maintained Lean theorem.

**Rationale:** The pilot exists to measure tractability. A blocked pilot is a
valid result, and this plan must not convert it into a proof campaign.

### 7. Counts are not completion evidence

**Choice:** The receipt reports counts of `measured`, `blocked`, `excluded`,
and `unmeasured` modules and of `targeted`, `proved`, and `non-target`
requirements. The change does not use those counts as a completion or
progress claim.

**Rationale:** Stack rules reject commit counts and closure counts as
completion evidence. The receipt is an instrument, not a score.

## Risks / Trade-offs

- **Pilot blocks at extraction.** Mitigation: the frontier records the exact
  stage and construct, and the change stops before proof work.
- **Lean proof cost exceeds the batch budget.** Mitigation: per-module budget
  outcome is a recorded result, not a failure of the plan.
- **Statement-registry rows drift from requirement text.** Mitigation: the
  existing Tracey and coverage checks stay authoritative for requirement
  identity, and registry validation binds each row to a requirement ID that
  exists in `docs/requirements.md`.
- **Coverage counts read as progress.** Mitigation: the claim-boundary checker
  rejects whole-library and equivalence wording in receipts and docs.
- **Partial independence.** Mitigation: receipts name rustc, Charon, Aeneas,
  Lean, and every model or axiom as shared trust inputs. The change never
  claims verifier independence as a proved fact.
