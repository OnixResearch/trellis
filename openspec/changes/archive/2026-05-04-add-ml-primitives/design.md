## Context

ML primitive proofs should focus on finite metadata and bounded data access:
rank/shape/stride, label vocabularies, disjoint dataset ranges, batch windows,
score ordering, output envelopes, and denominator-safe metrics. These are useful
without overclaiming training or inference correctness.

## Goals / Non-Goals

**Goals:** specify exact finite ML primitive contracts and implementation tasks.

**Non-Goals:** implement modules in this pass; prove model accuracy, fairness,
probabilistic guarantees, or floating-point error bounds.

## Decisions

### 1. Treat ML outputs as admitted artifacts, not proven predictions

**Choice:** `ml_inference_envelope` only checks that outputs satisfy declared
metadata bounds.

**Rationale:** This is verifiable and useful; it does not claim the model is good
or correct.

### 2. Put dataset and batching primitives before inference helpers

**Choice:** Include `ml_dataset_split` and `ml_batch_iterator` in the same ML spec.

**Rationale:** Data partition and window-safety bugs are common and proof-friendly.

## Validation Plan

- `openspec validate add-ml-primitives --strict --json`
- later implementation evidence per task: focused cargo tests, Verus, Tracey
