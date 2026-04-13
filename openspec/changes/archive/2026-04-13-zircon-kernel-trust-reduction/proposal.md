## Why

`zircon-kernel-primitives` landed green, but it did not internalize every
exec/spec bridge. At the start of this change, the live tree had 16 closeable
`#[verifier::external_body]` sites across `src/zircon_channel.rs`,
`src/zircon_fifo.rs`, `src/zircon_job.rs`, `src/zircon_socket.rs`, and
`src/zircon_port.rs`. The README and
`openspec/specs/verification-gaps/spec.md` inventoried those sites explicitly.

That leaves the Zircon model semantically covered and tested, but not yet
fully internalized. Future trust-reduction work should close those bridges
without changing the public behavior or Tracey coverage.

## What We Know

- At change start, the green snapshot was `cargo test` (`3334 passed`),
  `cargo clippy --all-targets -- -D warnings`,
  `./scripts/verus.sh src/lib.rs` (`5902 verified, 0 errors`), and
  `./scripts/verify.sh` all pass.
- Closeable trusted sites:
  - `src/zircon_channel.rs`: `channel_signals`, `send_message`,
    `receive_message`, `close_endpoint`
  - `src/zircon_fifo.rs`: `fifo_write`, `fifo_read`, `fifo_signals`
  - `src/zircon_job.rs`: `well_formed_jobs`, `job_in_subtree`,
    `process_in_subtree`, `parent_kept_live`
  - `src/zircon_socket.rs`: `socket_signals`, `socket_write`, `socket_read`
  - `src/zircon_port.rs`: `pop_packet`, `deliver`
- Existing unit tests and proptests already pin the expected runtime behavior.
  The trust-reduction work is mostly about replacing external-body wrappers
  with verified loops/recursion and strengthening local proof structure.

## What Changes

- Internalize channel queue and handle-transfer exec wrappers.
- Internalize FIFO and socket buffer exec wrappers.
- Internalize job containment and parent-liveness queries.
- Internalize port dequeue and async-delivery logic.
- Keep README and verification-gap inventory counts in sync as boundaries close.

## Scope

- **In scope**: removing closeable Zircon `external_body` sites while
  preserving current semantics and green verification.
- **Out of scope**: expanding Zircon feature surface beyond the already
  shipped primitives.

## Evidence

Starting inventory snapshot:

- `69` total `#[verifier::external_body]` items across `16` files
- `15` permanent
- `16` closeable Zircon exec bridges
