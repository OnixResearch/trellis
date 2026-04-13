## Why

The BEAM virtual machine has a distinctive concurrency model: lightweight
processes with isolated heaps, asynchronous message passing with per-process
mailboxes, selective receive with pattern matching, monitors and links for
failure propagation, and preemptive reduction-counted scheduling. These
primitives compose to produce Erlang/OTP's fault-tolerance guarantees.

This repo already models related concepts — process state machines
(`process_state`), signal algebra (`signal`), reference counting
(`ref_count`), message-passing channels (`zircon_channel`), and scheduling
disciplines (`fair_queue`, `deadline_sched`). What is missing is the
BEAM-specific composition: process isolation with per-process heaps,
mailbox ordering and selective receive semantics, bidirectional link
propagation, unidirectional monitor notification, reduction-based
preemption, and the process registry as a naming layer.

A verified model of these primitives would give downstream users a reusable
reference for actor-model concurrency, selective receive protocols, and
let-it-crash supervision semantics without needing a full BEAM emulator.

## What Changes

- **`beam_process`** — Process lifecycle: spawn with isolated heap,
  alive/exiting/dead state machine, process flags, and exit reason
  propagation. Proves state transitions are monotonic (alive → exiting →
  dead, never backwards).
- **`beam_mailbox`** — Per-process mailbox with FIFO append and selective
  receive. Proves receive returns the first message matching the pattern
  in mailbox order, that unmatched messages are preserved in order, and
  that `save_offset` cursor semantics are correct.
- **`beam_send`** — Asynchronous message delivery: send always succeeds
  from the sender's perspective, messages to live processes are appended
  to the recipient's mailbox, and messages to dead processes are silently
  dropped.
- **`beam_link`** — Bidirectional process links: link/unlink operations,
  exit signal propagation through live links, trap_exit flag semantics,
  unlink suppression, and kill-bypasses-trap semantics.
- **`beam_monitor`** — Unidirectional monitors: monitor/demonitor
  operations, DOWN message generation on monitored process death, and
  guaranteed delivery of DOWN even if the target is already dead at
  monitor time.
- **`beam_scheduler`** — Reduction-counted preemptive scheduling:
  reduction budget allocation, preemption on budget exhaustion, run-queue
  ordering, and priority levels (max, high, normal, low).
- **`beam_registry`** — Process registry as a partial map from names to
  pids. Register/unregister/whereis operations with uniqueness invariant
  (one name per pid, one pid per name).
- **Cross-module properties** — Proofs linking process death to link
  signal propagation, monitor DOWN delivery, mailbox finality, and
  registry cleanup.

## Capabilities

### New Capabilities

- `beam-process.lifecycle`: Verified monotonic process state transitions
- `beam-process.isolation`: Verified heap isolation for processes with distinct pids
- `beam-mailbox.ordering`: Verified FIFO mailbox and selective receive
- `beam-mailbox.save-offset`: Verified cursor correctness across receives
- `beam-send.async`: Verified fire-and-forget delivery semantics
- `beam-link.propagation`: Verified bidirectional exit signal propagation
- `beam-link.trap-exit`: Verified signal-to-message conversion
- `beam-monitor.down`: Verified DOWN message delivery guarantees
- `beam-monitor.late`: Verified immediate DOWN when target already dead
- `beam-scheduler.preemption`: Verified reduction-based preemption
- `beam-scheduler.priority`: Verified priority-level run queue ordering
- `beam-registry.uniqueness`: Verified bijective name ↔ pid mapping

## Impact

- **Files**: New `src/beam_*.rs` modules, `src/lib.rs` exports,
  `src/properties.rs` additions, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for process state, mailbox
  operations, send semantics, link/monitor propagation, scheduler
  decisions, and registry lookups
- **Dependencies**: Reuses existing `process_state`, `signal`,
  `ref_count` modules; no new runtime deps expected
- **Testing**: Verus proofs, unit tests for concrete scenarios, and
  proptest model oracles for message delivery ordering, link cascading,
  and scheduler fairness

## Out of Scope

- BEAM bytecode format, instruction set, or JIT compilation
- Distribution protocol (Erlang-to-Erlang node communication)
- ETS (Erlang Term Storage) table semantics
- Binary/bitstring construction and matching
- Garbage collection strategies and heap sizing
- OTP supervision tree composition (gen_server, supervisor behaviors)
- Port drivers and NIF (native implemented function) integration
- Timer wheel and `receive ... after` timeout scheduling
- IO and file descriptor management
