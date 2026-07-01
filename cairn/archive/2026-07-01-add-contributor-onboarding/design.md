## Context

`AGENTS.md` contains detailed agent-maintainer notes, but it is not a public contributor guide. Contributors need stable guidance that explains the verified-library workflow without exposing session-specific notes.

## Goals

- Add a public contributor guide with a shortest successful path for small changes.
- Explain the spec/exec/proof/test pattern and required Tracey annotations.
- Require both positive and negative tests for new test suites.
- Explain baseline and final validation expectations for core logic changes.
- Describe Cairn change packages for public behavior, architecture, roadmap, or evidence-contract changes.

## Non-Goals

- Do not duplicate all maintainer-only gotchas from `AGENTS.md`.
- Do not make PR creation part of the workflow.
- Do not replace accepted specs or verification scripts as the source of truth.

## Decisions

### Public-safe guidance

The guide SHALL be written for humans and downstream contributors. It SHALL reference `AGENTS.md` only if appropriate and SHALL avoid transient local evidence claims.

### Template shape

If a module template is added, it SHALL include placeholders for requirement IDs, spec function, exec function with `ensures`, proof function, positive tests, and negative tests.

### Validation commands

The guide SHALL distinguish fast focused checks from full release-style verification and state when each is expected.
