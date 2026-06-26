## Context

`src/lib.rs` is the canonical public module declaration surface. README is the human-facing catalog. The catalog should not silently omit new public modules, but descriptions are still best written by humans.

## Goals / Non-Goals

**Goals:**
- Detect public module declarations missing from the README catalog.
- Detect README catalog entries for modules no longer declared in `src/lib.rs`, unless explicitly exempted.
- Keep output deterministic and easy to fix.

**Non-Goals:**
- No automatic README rewrite in the first slice.
- No semantic validation of description quality.
- No enforcement for archived docs.

## Decisions

### Compare declarations to catalog entries

**Choice:** Parse `pub mod <name>;` declarations and compare them to backtick module names in README catalog sections or a small explicit allowlist.

**Rationale:** This catches the drift that has occurred without requiring a brittle full Markdown schema.

### Fixture or synthetic negative test

**Choice:** Include a negative test that removes or omits a known module from a fixture README and expects failure.

**Rationale:** A current-tree pass alone would not prove the checker can fail.

## Risks / Trade-offs

**False positives for intentionally undocumented modules** → Require an explicit documented exclusion list with rationale.

**Markdown parsing brittleness** → Use conservative line-based extraction and stable diagnostics.
