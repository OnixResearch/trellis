## Why

Four modules — `balance`, `transfer`, `double_entry`, `journal` — form a
financial accounting domain with 30+ Verus proofs and 30+ tests but no
formal specification in `openspec/specs/`. The requirements they verify
(conservation of value, checked arithmetic, double-entry identity, journal
validity) are scattered across doc comments and ensures clauses. A
spec document captures these requirements in one place, makes them
reviewable, and lets tracey verify coverage.

## What Changes

- **New spec domain**: `financial-accounting` with requirements covering
  all four modules
- **Tracey annotations**: update `r[depends]`/`r[impl]`/`r[verify]` tags
  in source to match spec requirement IDs

## Capabilities

### New Capabilities
- `financial-accounting-spec`: Formal requirements document for balance,
  transfer, double-entry, and journal modules

### Modified Capabilities
- Existing tracey annotations in `balance.rs`, `transfer.rs`,
  `double_entry.rs`, `journal.rs` updated to reference spec requirements

## Impact

- **Files**: `openspec/specs/financial-accounting/spec.md` (new),
  `src/balance.rs`, `src/transfer.rs`, `src/double_entry.rs`,
  `src/journal.rs` (annotation updates)
- **APIs**: No changes
- **Dependencies**: None
- **Testing**: `cargo test` remains green, `tracey query uncovered` clean
