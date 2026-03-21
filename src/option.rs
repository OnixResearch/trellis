//! Verified Option combinators.

// r[impl option.unwrap-or-default]
/// Returns the contained value or the provided default.
pub fn unwrap_or_default<T>(opt: Option<T>, default: T) -> T {
    match opt {
        Some(v) => v,
        None => default,
    }
}

// r[impl option.map-preserves-none]
/// Maps an `Option<T>` to `Option<U>` by applying `f` to the contained value.
///
/// Returns `None` if the input is `None`.
pub fn map<T, U>(opt: Option<T>, f: impl FnOnce(T) -> U) -> Option<U> {
    match opt {
        Some(v) => Some(f(v)),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify option.unwrap-or-default]
    #[test]
    fn test_unwrap_or_default() {
        assert_eq!(unwrap_or_default(Some(42u64), 0), 42);
        assert_eq!(unwrap_or_default(None::<u64>, 99), 99);
    }

    // r[verify option.map-preserves-none]
    #[test]
    fn test_map_preserves_none() {
        let none: Option<u64> = None;
        assert_eq!(map(none, |x| x + 1), None);
        assert_eq!(map(Some(5u64), |x| x * 2), Some(10));
    }
}
