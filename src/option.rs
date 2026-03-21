//! Verified Option combinators.

use vstd::prelude::*;

verus! {

// r[depends option.unwrap-or-default]
/// Spec: returns the contained value or the default.
pub open spec fn unwrap_or_default_spec(opt: Option<u64>, default: u64) -> u64 {
    match opt {
        Some(v) => v,
        None => default,
    }
}

// r[impl option.unwrap-or-default]
/// Returns the contained value or the provided default.
pub fn unwrap_or_default(opt: Option<u64>, default: u64) -> (result: u64)
    ensures result == unwrap_or_default_spec(opt, default)
{
    match opt {
        Some(v) => v,
        None => default,
    }
}

// r[depends option.map-preserves-none]
/// Spec: mapping over an option.
pub open spec fn map_spec(opt: Option<u64>, f: spec_fn(u64) -> u64) -> Option<u64> {
    match opt {
        Some(v) => Some(f(v)),
        None => None,
    }
}

// r[verify option.unwrap-or-default]
proof fn unwrap_or_default_some(v: u64, default: u64)
    ensures unwrap_or_default_spec(Some(v), default) == v
{}

// r[verify option.unwrap-or-default]
proof fn unwrap_or_default_none(default: u64)
    ensures unwrap_or_default_spec(None, default) == default
{}

// r[verify option.map-preserves-none]
proof fn map_none_is_none(f: spec_fn(u64) -> u64)
    ensures map_spec(None, f) == None::<u64>
{}

// r[verify option.map-preserves-none]
proof fn map_some_applies(v: u64, f: spec_fn(u64) -> u64)
    ensures map_spec(Some(v), f) == Some(f(v))
{}

} // verus!

// r[impl option.map-preserves-none]
/// Maps an `Option<T>` by applying `f` to the contained value.
pub fn map<T, U>(opt: Option<T>, f: impl FnOnce(T) -> U) -> Option<U> {
    match opt {
        Some(v) => Some(f(v)),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify option.unwrap-or-default]
    #[test]
    fn test_unwrap_or_default() {
        assert_eq!(unwrap_or_default(Some(42), 0), 42);
        assert_eq!(unwrap_or_default(None, 99), 99);
    }

    // r[verify option.map-preserves-none]
    #[test]
    fn test_map_preserves_none() {
        let none: Option<u64> = None;
        assert_eq!(map(none, |x| x + 1), None);
        assert_eq!(map(Some(5u64), |x| x * 2), Some(10));
    }

    proptest! {
        // r[verify option.unwrap-or-default]
        #[test]
        fn prop_unwrap_or_default_some(v: u64, default: u64) {
            prop_assert_eq!(unwrap_or_default(Some(v), default), v);
        }

        // r[verify option.unwrap-or-default]
        #[test]
        fn prop_unwrap_or_default_none(default: u64) {
            prop_assert_eq!(unwrap_or_default(None, default), default);
        }

        // r[verify option.map-preserves-none]
        #[test]
        fn prop_map_none_is_none(seed: u64) {
            let none: Option<u64> = None;
            prop_assert_eq!(map(none, |x| x.wrapping_add(seed)), None);
        }
    }
}
