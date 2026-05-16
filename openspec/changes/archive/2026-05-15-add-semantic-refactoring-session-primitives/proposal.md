## Why

Unison keeps definitions immutable and treats refactoring as changing name metadata plus propagating compatible replacements, so the codebase remains valid mid-refactor. `verified-logic` can add finite primitives for alias movement, replacement compatibility, dependent propagation, and unresolved-dependent tracking.

## What Changes

- Specify name-to-definition bindings and immutable old/new definition identities.
- Specify compatible replacement and dependent propagation admission.
- Specify unresolved dependent tracking for incompatible replacements.
- Exclude real source editing, compiler typechecking, and IDE/UCM workflow behavior.

## Scope

In scope: finite structural refactoring-session predicates over names, definition hashes, type identities, and dependent edges.

Out of scope:
- Textual diff/merge behavior, source formatting, or editor integration.
- Real type inference or compiler compatibility checking.
- Automatically rewriting executable programs.

## Verification

Validate this OpenSpec package, then implement finite predicates, tests, Tracey coverage, and repository verification.
