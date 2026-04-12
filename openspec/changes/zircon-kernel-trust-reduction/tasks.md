## Phase 1: Queue and Buffer Bridges

- [ ] Internalize `zircon_channel::{channel_signals, send_message, receive_message, close_endpoint}`
- [ ] Internalize `zircon_fifo::{fifo_write, fifo_read, fifo_signals}`
- [ ] Internalize `zircon_socket::{socket_signals, socket_write, socket_read}`

## Phase 2: Tree and Port Bridges

- [ ] Internalize `zircon_job::{well_formed_jobs, job_in_subtree, process_in_subtree, parent_kept_live}`
- [ ] Internalize `zircon_port::{pop_packet, deliver}`

## Phase 3: Re-verify and Shrink Inventory

- [ ] Update README + `openspec/specs/verification-gaps/spec.md` counts as bridges close
- [ ] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`
