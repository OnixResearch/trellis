## Why

The README proves breadth, but users still need help choosing the right module for a concrete job. An intent-oriented module guide turns the catalog into a usable map.

## What Changes

- Add `docs/module-guide.md` grouped by user goals and problem domains.
- Link each goal to stable modules, example paths, and relevant requirement IDs.
- Call out model-only, composition facade, and trusted-boundary-adjacent surfaces.

## Scope

In scope: user-facing guide documentation and drift checks for linked modules/examples.

Out of scope:
- Renaming modules.
- Reclassifying proof strength without source/spec changes.
- Replacing the README catalog.

## Verification

Run documentation/catalog drift checks and focused link/path checks, then run the standard repository verification gates affected by doc/spec changes.
