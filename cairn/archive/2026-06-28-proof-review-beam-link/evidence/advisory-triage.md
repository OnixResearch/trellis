# Advisory triage

Target: `src/beam_link.rs`

## Accepted actionable items

- Replace the empty proof bodies with explicit assertions over existing specs.
- Strengthen the unlink witness from a vacuous non-negative length claim to removal of both singleton directions plus non-membership.
- Strengthen exit-delivery witnesses to assert mailbox preservation for cascading/normal/kill paths and exact appended `EXIT` message details for trapped exits.
- Add executable public-runtime tests for repeated link idempotence, duplicate unlink removal while preserving unrelated links, normal-exit non-cascade, and inactive/dead process handling.

## Rejected or non-actionable items

- Do not change `deliver_exit_signal_spec` semantics for unknown reason values; existing requirements intentionally treat any non-normal, non-kill reason as abnormal.
- Do not add tests that violate proof `requires` clauses or attempt to call proof functions at runtime.
- Do not claim `kill` makes the peer immediately `BEAM_DEAD`; this model starts exit with reason `killed` (`BEAM_EXITING`) and later lifecycle completion reaches dead.
