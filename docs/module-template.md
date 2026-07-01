# Verified Module Template

This template is illustrative. Copy the shape, then replace every placeholder
requirement ID, module name, type, and predicate with the behavior for your
change.

```rust
use vstd::prelude::*;

verus! {

// r[depends example.requirement-id]
pub open spec fn example_valid_spec(input: u64) -> bool {
    input > 0
}

// r[impl example.requirement-id]
pub fn example_valid(input: u64) -> (result: bool)
    ensures result == example_valid_spec(input)
{
    input > 0
}

// r[verify example.requirement-id]
pub proof fn positive_example_is_valid(input: u64)
    requires input > 0
    ensures example_valid_spec(input)
{}

// r[verify example.requirement-id]
pub proof fn zero_example_is_rejected()
    ensures !example_valid_spec(0)
{}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify example.requirement-id]
    #[test]
    fn accepts_positive_input() {
        assert!(example_valid(1));
    }

    // r[verify example.requirement-id]
    #[test]
    fn rejects_zero_input() {
        assert!(!example_valid(0));
    }
}
```

Checklist before finishing a real module:

- Add or update requirement prose for the real `r[...]` ID.
- Export the module from `src/lib.rs` only when the public API is ready.
- Keep non-trivial logic in pure specs/cores and leave I/O orchestration outside
  verified library functions.
- Include both positive and negative tests.
- Run focused tests before and after core edits.
- Run Verus and Tracey checks before marking implementation tasks complete.
