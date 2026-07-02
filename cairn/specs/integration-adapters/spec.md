# Integration Adapters Specification

## Purpose

Defines the `integration-adapters` capability.

## Requirements

### Requirement: User-Facing Integration Adapters

The system MUST provide deterministic adapter surfaces only when they compose existing verified Trellis primitives and state their supported subset, validation evidence, and non-goals.

r[integration-adapters.user-facing-adapters]

#### Scenario: Adapter delegates to verified primitives

- GIVEN a public integration adapter is added
- WHEN its implementation is reviewed
- THEN its decision logic is factored through existing verified primitives or newly verified pure cores with explicit requirements

#### Scenario: Unsupported external behavior is not claimed

- GIVEN an adapter is named after or shaped like an external ecosystem
- WHEN adapter documentation is rendered
- THEN the documentation states the supported subset and does not claim full external conformance without matching tests and requirements

#### Scenario: Invalid domain input is rejected

- GIVEN an adapter receives malformed, incomplete, or out-of-subset domain input
- WHEN the adapter evaluates admission
- THEN it rejects the input deterministically and a negative test covers that rejection
