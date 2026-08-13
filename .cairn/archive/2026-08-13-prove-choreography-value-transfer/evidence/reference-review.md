# Wyzer Reference Review

## Source and disposition

- Repository reference: Wyzer
- Immutable revision: `7f6acf0f60cf34357e06de50112eb2b1ed833a0c`
- Public license at the reviewed revision: MIT
- Additional disposition: the project operator reports direct MIT confirmation from the author.

The author confirmation is an operator-supplied fact. This change does not claim independent verification of that private communication.

## Selected concepts

- Values are located at roles.
- Transfer is explicit in choreography syntax.
- A moved value cannot be transferred again by its former owner.
- Negative moved-value examples are required evidence.

Trellis owns the resulting representation, executable semantics, specifications, and proofs. No Wyzer runtime code is copied.

## Rejected components

This change does not import Wyzer networking, queues, persistence, Perceus, functional-but-in-place update, compile-time execution, memory management, or C ABI components.

The selected model proves only an atomic semantic ownership update over supplied finite facts.
