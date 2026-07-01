## Context

Trellis has hundreds of public modules. Alphabetical or domain tables help maintainers, but a downstream user asks intent-first questions such as “how do I model a bounded queue?” or “which helper covers quorum admission?”.

## Goals

- Organize public modules by task-oriented questions.
- Include short, stable guidance for when to use a module and when not to use it.
- Link to examples, README catalog entries, accepted requirements, and source files.
- Keep the guide concise enough to be skimmed before reading source.

## Non-Goals

- Do not duplicate every README module row.
- Do not promise API stability beyond current repository policy.
- Do not hide trusted boundaries; guide entries MUST surface them when relevant.

## Decisions

### Guide shape

The guide SHALL use sections such as bounded arithmetic, consensus safety, authorization, serialization/codecs, memory/layout, compiler analysis, and verification tooling. Each section SHOULD include a “start here” module and follow-up modules.

### Drift resistance

Where practical, guide references SHALL use concrete module names that can be checked against `src/lib.rs`. Any unchecked prose-only reference SHOULD be short and locally auditable.

### Maturity labels

The guide MAY reference maturity labels defined by the module maturity index, but it SHALL remain useful even before the generated index exists.
