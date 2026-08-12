## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.filesystem-path.segment` | `src/path_*`, `src/fs_path_*` finite path-segment envelopes |
| `domain-primitives.filesystem-path.form` | absolute/relative/rooted form metadata |
| `domain-primitives.filesystem-path.normalization` | explicit finite normalization decisions |
| `domain-primitives.filesystem-path.containment` | root/scope containment predicates |
| `domain-primitives.filesystem-path.traversal` | relative traversal rejection metadata |
| `domain-primitives.filesystem-path.extension-type` | extension/type metadata envelopes |
| `domain-primitives.filesystem-path.join-split` | deterministic join/split shape envelopes |
| `domain-primitives.filesystem-path.admission` | deterministic path admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only filesystem-path primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.filesystem-path.segment,domain-primitives.filesystem-path.form,domain-primitives.filesystem-path.normalization,domain-primitives.filesystem-path.containment,domain-primitives.filesystem-path.traversal,domain-primitives.filesystem-path.extension-type,domain-primitives.filesystem-path.join-split,domain-primitives.filesystem-path.admission]`

## Phase 2: Future implementation

- [x] Add docs/requirements entries and Tracey IDs for finite path-segment envelopes. `[covers=domain-primitives.filesystem-path.segment]`
- [x] Implement and verify path form admission primitives. `[covers=domain-primitives.filesystem-path.form]`
- [x] Implement and verify finite normalization decision envelopes. `[covers=domain-primitives.filesystem-path.normalization]`
- [x] Implement and verify root/scope containment predicates. `[covers=domain-primitives.filesystem-path.containment]`
- [x] Implement and verify relative traversal rejection metadata. `[covers=domain-primitives.filesystem-path.traversal]`
- [x] Implement and verify extension/type metadata envelopes. `[covers=domain-primitives.filesystem-path.extension-type]`
- [x] Implement and verify deterministic path join/split shape envelopes. `[covers=domain-primitives.filesystem-path.join-split]`
- [x] Implement deterministic path admission/rejection composition. `[covers=domain-primitives.filesystem-path.admission]`
- [x] Add focused positive and negative tests for each filesystem-path primitive.
- [x] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
