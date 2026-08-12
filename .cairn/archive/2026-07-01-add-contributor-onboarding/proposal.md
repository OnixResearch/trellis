## Why

Trellis has mature internal verification norms, but new contributors need a public, concise path for adding a module without breaking Tracey, Verus, README catalog, or proof-gap policy. A contributor guide reduces review churn and makes the repo easier to extend.

## What Changes

- Add `CONTRIBUTING.md` with repository-specific contribution workflow.
- Include module template guidance, requirement/Tracey annotation rules, positive and negative test expectations, and validation commands.
- Document when to create a Cairn change and how to archive it.

## Scope

In scope: contributor documentation, links to existing scripts/specs, and optional templates.

Out of scope:
- Changing code ownership or review policy.
- Relaxing verification requirements.
- Opening or managing pull requests.

## Verification

Run doc/link/catalog checks plus standard repository validation appropriate for documentation-only changes.
