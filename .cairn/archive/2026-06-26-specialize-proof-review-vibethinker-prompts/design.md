## Context

`proof-review-candidates.rs` ranks proof-review targets and writes Markdown packets for advisory VibeThinker review. The current packet asks a single broad question. That makes VibeThinker repeat scanner findings and sometimes suggest vague proof/test work instead of producing actionable postconditions, tests, or spec wording gaps.

## Decisions

### 1. Rule-specific prompt hints

**Choice:** Map each finding rule to a small deterministic prompt snippet. For example, `vacuous-ensures-true` asks for concrete non-vacuous postconditions using existing specs; `empty-nontrivial-proof` asks whether the proof is definitional, vacuous, or missing a lemma; `missing-negative-test-hints` asks only for executable negative cases.

**Rationale:** The scanner already knows why a target was selected. Passing that reason into the advisory prompt makes VibeThinker spend its output budget on the missing reasoning instead of rediscovering the rule.

### 2. Structured output contract

**Choice:** Add a packet-level requested output shape: proof postconditions, executable tests, spec wording gaps, patch critique, and `no focused concerns` when appropriate. Explicitly forbid testing proof functions, adding trust boundaries, and repeating scanner findings.

**Rationale:** This matches the way the advisory results have been useful in practice and makes packets easier to paste directly into `vibethinker_consult`.

### 3. No runtime model dependency

**Choice:** Keep the mapping in the Rust script rather than adding runtime Nickel execution or a separate model prompt template loader.

**Rationale:** The script remains a deterministic standalone repository tool. Policy freshness stays checked through the existing Nickel policy JSON path.

## Risks / Trade-offs

- Hard-coded prompt strings can drift from rule semantics. The mitigation is script self-test coverage and keeping snippets short and rule-local.
- More prompt text can make packets longer. The mapping only emits snippets for top packet findings, capped by the existing finding limit.
