## Why

The proof-review scanner selected `src/append_entries_build.rs` as the next bounded proof-hardening target. Capturing a dedicated Cairn package keeps the advisory packet, source patch, validation logs, and archive evidence reviewable.

## What Changes

- Review the scanner findings for `src/append_entries_build.rs`.
- Apply only proof/test hardening that does not add or hide trust boundaries.
- Capture focused validation, Tracey coverage, Cairn gates, and repo verification evidence before archive.

## Impact

- **Files**: expected source/test edits for `src/append_entries_build.rs`, this Cairn package, and archived evidence.
- **Testing**: focused tests where available, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`.
