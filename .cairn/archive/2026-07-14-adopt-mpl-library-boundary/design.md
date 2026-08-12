## Context

Trellis supplies verified building blocks rather than an operator service. MPL-2.0 preserves source-file reciprocity and is compatible with later AGPL shells without imposing strong copyleft on unrelated linked application files.

## Decisions

### Use MPL-2.0 for Trellis-owned source

The `verified-logic` Cargo package, current README, and checked-in license artifact identify MPL-2.0. No Incompatible-With-Secondary-Licenses notice is added.

### Preserve proof boundaries

The license change does not strengthen verification claims. Trellis still proves only its stated local properties, and consumers must revalidate their own integration and receipts.

### Preserve third-party terms

Verifier tools, referenced corpora, generated evidence, and any copied upstream material retain their original terms. Consumer vendoring must retain the MPL text and notices.

### Keep prior grants intact

Previously distributed AGPL copies remain usable under their received terms; the new declaration applies to the newly distributed Trellis-owned tree.
