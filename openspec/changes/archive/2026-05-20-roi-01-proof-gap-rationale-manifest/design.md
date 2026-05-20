## Context

The current proof-gap checker loads `docs/proof-gap-sites.tsv` and validates the live `#[verifier::external_body]` site list plus classification drift. The manifest format is currently:

```text
path<TAB>function-signature<TAB>classification
```

All 16 current sites are classified as `permanent-trust-boundary`, while rationale text lives in README/OpenSpec prose. That separation makes rationale freshness hard to enforce.

## Goals / Non-Goals

**Goals:**
- Make the checked manifest the source of truth for per-site rationale.
- Keep rationale concise but non-empty and reviewable.
- Fail closed for missing/malformed rationale fields without mutating source.
- Preserve existing exact-site and classification drift behavior.

**Non-Goals:**
- Do not remove any trusted boundary.
- Do not claim cryptographic or codec correctness beyond existing trusted contracts.
- Do not introduce a new configuration language for this manifest; TSV remains sufficient.

## Decisions

### Extend the existing TSV manifest

**Choice:** Add a fourth tab-separated `rationale` field to `docs/proof-gap-sites.tsv` and update the header to `path<TAB>function-signature<TAB>classification<TAB>rationale`.

**Rationale:** The manifest is already checked, committed, and source-derived. Extending it avoids duplicating trust inventory across another generated artifact.

**Alternative:** Keep rationale only in README/OpenSpec prose. Rejected because prose is not currently tied to exact live sites.

### Validate structure and semantics in the Rust checker

**Choice:** `scripts/check-proof-gap-inventory.rs` will parse the fourth field, require non-empty rationale text, preserve exact-site matching, preserve classification validation, and reject malformed rows.

**Rationale:** A single deterministic checker gives local and Nix verification the same failure mode.

### Use controlled fixture/self-test modes for negative coverage

**Choice:** Add checker-controlled fixture scenarios for missing rationale, blank rationale, and malformed extra/missing columns.

**Rationale:** Negative coverage should not mutate repository source and should remain stable inside Nix builds.

## Risks / Trade-offs

**Overly verbose rationale rows** → Keep each rationale short enough for TSV review and reserve longer explanation for README/OpenSpec prose.

**TSV escaping complexity** → Disallow tabs inside rationale text and keep one line per site.

**Scope creep into proof closure** → This change only strengthens inventory evidence; proof-gap closure remains a later OpenSpec.

## Validation Plan

- `./scripts/check-proof-gap-inventory.rs`
- checker fixture/self-test modes for missing, blank, and malformed rationale fields
- Nix proof-gap inventory check if present, or add one if absent
- `git diff --check`
- `TMPDIR=/tmp ./scripts/verify.rs`
- `openspec validate roi-01-proof-gap-rationale-manifest --strict --json` before implementation and before archive
