# Contributing to Trellis

Trellis is a verified Rust library. A good contribution keeps the public crate
identity `verified-logic`, preserves the spec/exec/proof style, and leaves the
local validation story reproducible.

## Shortest successful path

1. Pick or add a requirement ID in `docs/requirements.md` or the active Cairn
   change spec that describes the behavior you are changing.
2. Implement a pure `spec fn` for the mathematical model.
3. Implement the runtime `exec fn` with an `ensures result == *_spec(...)`
   contract wherever the function is inside `verus!`.
4. Add proof evidence (`proof fn`) or property evidence for the important
   behavior.
5. Add both positive and negative tests for every new test suite. Positive tests
   show accepted inputs; negative tests show malformed, boundary, or rejected
   inputs fail safely.
6. Run focused checks before and after core logic edits, then run the broader
   gates appropriate for the touched files.

Use `docs/module-template.md` as an illustrative starting point for new modules.
Replace every placeholder requirement ID before submitting the change.

## Tracey and requirement annotations

Tracey connects requirements to implementation and verification artifacts.

- Requirement prose lives in `docs/requirements.md` for code-traceable behavior.
- Implementation anchors use comments such as `// r[impl my.requirement-id]`.
- Proof, property, or test anchors use comments such as
  `// r[verify my.requirement-id]`.
- Helper-only requirements still need an implementation anchor on the defining
  spec/helper surface.

Run these focused checks when touching traceability:

```bash
tracey query uncovered
tracey query untested
./scripts/check-coverage.rs
```

## Baseline and final validation

For core logic, library code, or verified modules, run the smallest relevant
baseline before editing. Examples:

```bash
cargo test --quiet <module_or_test_filter>
./scripts/verus.sh src/lib.rs
tracey query uncovered
tracey query untested
```

After editing, rerun the focused baseline plus any new tests. Before finishing a
substantive change, run the maintained repository gates that match the scope:

```bash
cargo test --quiet
cargo clippy --all-targets -- -D warnings
./scripts/verus.sh src/lib.rs
./scripts/verify.rs
```

Use an isolated target directory if the shared target cache is stale:

```bash
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
```

## Proof-gap policy

Do not introduce untracked trusted boundaries. New uses of
`#[verifier::external_body]`, `assume_specification[...]`, unchecked
`assume(...)`, runtime-only exemptions, or no-ensures markers require an explicit
proof-gap inventory/spec update before the change can be considered complete.

Relevant checks:

```bash
./scripts/check-proof-gap-inventory.rs
./scripts/check-ensures.rs
```

If the change intentionally adds or reclassifies a proof gap, update the accepted
verification-gaps spec and `docs/proof-gap-sites.tsv` in the same change.

## Cairn changes

Create or update a Cairn change package when a contribution affects public
behavior, architecture, roadmap scope, validation/evidence contracts, or accepted
specs.

A complete package normally includes:

- `cairn/changes/<slug>/proposal.md`
- `cairn/changes/<slug>/design.md`
- `cairn/changes/<slug>/tasks.md`
- `cairn/changes/<slug>/specs/<domain>/spec.md`

Run gates before implementation, sync accepted specs after the implementation is
validated, then archive only after every task is complete:

```bash
nix run path:/home/brittonr/git/cairn#cairn -- gate proposal <slug> --root .
nix run path:/home/brittonr/git/cairn#cairn -- gate design <slug> --root .
nix run path:/home/brittonr/git/cairn#cairn -- gate tasks <slug> --root .
nix run path:/home/brittonr/git/cairn#cairn -- sync <slug> --root . --execute
CAIRN_ARCHIVE_DATE=$(date +%F) nix run path:/home/brittonr/git/cairn#cairn -- archive <slug> --root . --execute
```

## Contribution license

By contributing, you agree that your contribution is licensed under the Mozilla Public License 2.0 (`MPL-2.0`). You must have the authority to make that grant; do not submit employer-, client-, school-, or third-party-owned material without permission. Existing grants for earlier revisions remain valid.

This guide covers local contribution mechanics. It does not require any pull
request workflow.
