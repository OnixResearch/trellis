## Change Status

Status: scaffolded; implementation pending.

## Verification Coverage

- `r[verification-gaps.manifest-rationale-source]` — manifest schema update plus checker parse validation.
- `r[verification-gaps.manifest-rationale-drift-gate]` — checker negative fixtures/self-tests, Nix/local proof-gap inventory gates, and full verify.
- `r[verification-gaps.manifest-rationale-publication]` — README and canonical OpenSpec sync after archive.

## Phase 1: Scaffold

- [x] [serial] Create proposal, design, delta spec, and tasks for proof-gap rationale manifest validation. Completed 2026-05-20T01:16:31Z.
- [x] [serial] Validate the scaffold with strict OpenSpec validation before implementation. Completed 2026-05-20T01:16:31Z.

## Phase 2: Manifest and checker implementation

- [ ] [serial] Extend `docs/proof-gap-sites.tsv` to include a fourth non-empty rationale field for every current proof-gap site.
- [ ] [depends:manifest] Update `scripts/check-proof-gap-inventory.rs` to parse and validate rationale-bearing manifest rows while preserving count, exact-site, and classification drift checks.
- [ ] [depends:checker] Add deterministic negative fixture/self-test coverage for missing rationale, blank rationale, and malformed manifest rows.

## Phase 3: Integration and docs

- [ ] [depends:checker] Wire rationale validation through `scripts/verify.rs` and Nix checks, adding a Nix proof-gap inventory check if one is not already present.
- [ ] [depends:checker] Update README proof-gap documentation to identify `docs/proof-gap-sites.tsv` as the checked per-site rationale source.

## Phase 4: Validation and archive

- [ ] [depends:integration] Run focused proof-gap inventory checks, negative fixture/self-test checks, `git diff --check`, and `openspec validate roi-01-proof-gap-rationale-manifest --strict --json`; save evidence.
- [ ] [depends:validation] Run `TMPDIR=/tmp ./scripts/verify.rs`, confirm Tracey remains green if requirements/docs IDs changed, then sync and archive the change.
