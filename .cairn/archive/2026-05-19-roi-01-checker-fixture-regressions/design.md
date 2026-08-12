## Context

The repository now uses standalone Rust scripts for coverage, ensures, and iterative-exec validation. The scripts scan real source files, but a green full-tree run does not exercise known-bad cases such as missing annotations, missing `ensures`, or cyclic/qualified exec calls.

## Goals / Non-Goals

**Goals:**
- Make checker behavior regression-testable with small deterministic fixtures.
- Include negative fixtures that must fail for the expected reason.
- Keep developer invocation simple and compatible with the current Nix/Fenix script pattern.

**Non-Goals:**
- No new policy language in this slice.
- No migration from standalone scripts into a workspace crate.
- No unrelated checker-rule expansion.

## Decisions

### Fixture-first self-tests

**Choice:** Add tiny fixture source files and a harness that runs each checker against pass and fail fixtures.

**Rationale:** Fixtures are easier to review than mocking the parser internals and directly cover the command-line surface used by `flake.nix` and `scripts/verify.rs`.

**Implementation:** Add valid/invalid fixture directories. The harness must assert both exit status and at least one stable diagnostic substring for negative fixtures.

### Keep full-tree gates separate

**Choice:** Checker self-tests run before or alongside the live-tree checks, not instead of them.

**Rationale:** Fixtures prove rejection semantics; live-tree checks prove repository compliance.

## Risks / Trade-offs

**Brittle diagnostics** → Match stable substrings only, not entire stderr/stdout.

**Slow Nix checks** → Keep fixtures tiny and avoid invoking full verification from the fixture harness.
