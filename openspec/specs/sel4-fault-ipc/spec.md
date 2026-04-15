# seL4 Fault IPC Specification

## Purpose

Defines requirements for verified `sel4_fault_ipc` semantics in this repo. Scope is fault handling behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Fault IPC Preserves Fault Metadata

r[sel4-fault-ipc.preservation]
Encoding a structured fault into an IPC message and decoding it again MUST recover the same fault class and required payload fields.

#### Scenario: Page fault roundtrip

- GIVEN a page fault with address `A` and access kind `K`
- WHEN it is encoded for IPC and decoded by the handler
- THEN the decoded fault has address `A` and access kind `K`

### Requirement: Fault IPC Delivers To The Handler Context

r[sel4-fault-ipc.delivery]
A modeled fault delivery step MUST deliver the encoded fault information to the configured fault-handler context.

#### Scenario: Configured handler receives fault

- GIVEN a thread with a configured fault handler
- WHEN the thread faults
- THEN the handler context receives that fault record

### Requirement: Fault IPC Payload Fields Are Uncorrupted

r[sel4-fault-ipc.payload]
Fault payload fields preserved by the model MUST not be modified by encode/decode or delivery steps.

#### Scenario: Instruction pointer preserved

- GIVEN a fault payload containing instruction pointer `IP`
- WHEN the fault is encoded and decoded
- THEN the decoded payload still contains `IP`
