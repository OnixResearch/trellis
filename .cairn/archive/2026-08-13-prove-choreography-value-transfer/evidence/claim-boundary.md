# Claim Boundary

The implementation proves an atomic semantic owner change over a supplied finite ownership state.

A successful step:

- requires distinct source and destination roles;
- requires the source to own the selected value;
- advances to the stored continuation;
- replaces only the selected owner;
- preserves normalized value identities, entry count, and unrelated owners; and
- prevents immediate reuse by the former owner unless another admitted transfer moves ownership back.

The evidence does not prove transport delivery, queue behavior, network ordering, retries, persistence, allocation, memory reclamation, authorization, exactly-once effects, external observation uniqueness, crash recovery, or liveness.

Trellis emits semantic facts only. Consumers retain runtime authority and effect execution.
