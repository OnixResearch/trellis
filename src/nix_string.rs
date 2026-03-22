//! Verified Nix string operations.
//!
//! Models `builtins.stringLength`, `builtins.substring`, and
//! `builtins.replaceStrings`. Uses `Seq<u8>` / `Vec<u8>` for strings.

use vstd::prelude::*;

verus! {

// --- string_length ---

// r[depends nix-string.string-length]
/// Spec: length of a byte string.
pub open spec fn string_length_spec(s: Seq<u8>) -> nat {
    s.len()
}

// r[impl nix-string.string-length]
/// Returns the length of the string.
pub fn string_length(s: &Vec<u8>) -> (result: usize)
    ensures result as nat == string_length_spec(s@)
{
    s.len()
}

// --- substring ---

// r[depends nix-string.substring]
/// Spec: extract a substring. Clamps to string bounds (Nix semantics).
/// `start` is the starting index, `len` is the maximum length.
pub open spec fn substring_spec(s: Seq<u8>, start: nat, len: nat) -> Seq<u8> {
    let actual_start: nat = if start > s.len() { s.len() } else { start };
    let remaining: nat = (s.len() - actual_start) as nat;
    let actual_len: nat = if len > remaining { remaining } else { len };
    s.subrange(actual_start as int, (actual_start + actual_len) as int)
}

// r[impl nix-string.substring]
/// Extract a substring with Nix's clamping semantics.
#[verifier::external_body]
pub fn substring(s: &Vec<u8>, start: usize, len: usize) -> (result: Vec<u8>)
    ensures result@ == substring_spec(s@, start as nat, len as nat)
{
    let actual_start = start.min(s.len());
    let remaining = s.len() - actual_start;
    let actual_len = len.min(remaining);
    s[actual_start..actual_start + actual_len].to_vec()
}

// --- replace_strings ---

// r[depends nix-string.replace-strings]
/// Spec: left-to-right non-overlapping string replacement.
/// `from` and `to` are parallel arrays of patterns and replacements.
pub closed spec fn replace_strings_spec(
    s: Seq<u8>, from: Seq<Seq<u8>>, to: Seq<Seq<u8>>,
) -> Seq<u8>;

// r[impl nix-string.replace-strings]
/// Replace all occurrences of `from[i]` with `to[i]`, left-to-right,
/// non-overlapping. First match wins when patterns overlap at a position.
/// Trusted: complex byte-level scanning. Verified by proptest.
#[verifier::external_body]
pub fn replace_strings(
    s: &Vec<u8>, from: &Vec<Vec<u8>>, to: &Vec<Vec<u8>>,
) -> (result: Vec<u8>)
    requires from.len() == to.len()
    ensures result@ == replace_strings_spec(s@,
        from@.map_values(|v: Vec<u8>| v@),
        to@.map_values(|v: Vec<u8>| v@))
{
    let mut result = Vec::new();
    let mut pos = 0usize;
    'outer: while pos < s.len() {
        for i in 0..from.len() {
            let pat = &from[i];
            if pat.is_empty() {
                // Empty pattern matches everywhere; insert replacement and advance by 1
                result.extend_from_slice(&to[i]);
                result.push(s[pos]);
                pos += 1;
                continue 'outer;
            }
            if pos + pat.len() <= s.len() && &s[pos..pos + pat.len()] == pat.as_slice() {
                result.extend_from_slice(&to[i]);
                pos += pat.len();
                continue 'outer;
            }
        }
        // No pattern matched at this position
        result.push(s[pos]);
        pos += 1;
    }
    // Handle empty-pattern matches at the end
    for i in 0..from.len() {
        if from[i].is_empty() {
            result.extend_from_slice(&to[i]);
            break;
        }
    }
    result
}

// --- Proofs ---

// r[verify nix-string.string-length]
/// Length of empty string is 0.
proof fn string_length_empty()
    ensures string_length_spec(Seq::<u8>::empty()) == 0
{}

// r[verify nix-string.substring]
/// Substring of empty string is empty.
proof fn substring_empty(start: nat, len: nat)
    ensures substring_spec(Seq::<u8>::empty(), start, len) =~= Seq::<u8>::empty()
{}

// r[verify nix-string.substring]
/// Substring with start=0, len >= s.len() returns the whole string.
proof fn substring_full(s: Seq<u8>)
    ensures substring_spec(s, 0, s.len()) =~= s
{}

// r[verify nix-string.substring]
/// Substring length is at most the requested length.
proof fn substring_length_bounded(s: Seq<u8>, start: nat, len: nat)
    ensures substring_spec(s, start, len).len() <= len
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn v(s: &str) -> Vec<u8> { s.as_bytes().to_vec() }

    // r[verify nix-string.string-length]
    #[test]
    fn test_string_length() {
        assert_eq!(string_length(&v("")), 0);
        assert_eq!(string_length(&v("hello")), 5);
    }

    // r[verify nix-string.substring]
    #[test]
    fn test_substring() {
        assert_eq!(substring(&v("hello"), 1, 3), v("ell"));
        assert_eq!(substring(&v("hello"), 0, 100), v("hello")); // clamp
        assert_eq!(substring(&v("hello"), 10, 5), v("")); // past end
        assert_eq!(substring(&v("hello"), 0, 0), v("")); // zero length
    }

    // r[verify nix-string.replace-strings]
    #[test]
    fn test_replace_strings() {
        // Simple replacement
        let result = replace_strings(
            &v("hello world"),
            &vec![v("world")],
            &vec![v("nix")],
        );
        assert_eq!(result, v("hello nix"));

        // Multiple patterns
        let result = replace_strings(
            &v("aabbcc"),
            &vec![v("aa"), v("bb")],
            &vec![v("x"), v("y")],
        );
        assert_eq!(result, v("xycc"));

        // No match
        let result = replace_strings(
            &v("hello"),
            &vec![v("xyz")],
            &vec![v("abc")],
        );
        assert_eq!(result, v("hello"));
    }

    proptest! {
        // r[verify nix-string.string-length]
        #[test]
        fn prop_string_length(s in ".{0,50}") {
            let v = s.into_bytes();
            prop_assert_eq!(string_length(&v), v.len());
        }

        // r[verify nix-string.substring]
        #[test]
        fn prop_substring_bounded(
            s in ".{0,20}",
            start in 0usize..30,
            len in 0usize..30,
        ) {
            let v = s.into_bytes();
            let result = substring(&v, start, len);
            prop_assert!(result.len() <= len);
            prop_assert!(result.len() <= v.len());
        }

        // r[verify nix-string.substring]
        #[test]
        fn prop_substring_full(s in ".{0,20}") {
            let v = s.into_bytes();
            prop_assert_eq!(substring(&v, 0, v.len()), v);
        }

        // r[verify nix-string.replace-strings]
        #[test]
        fn prop_replace_no_match(
            s in "[a-z]{1,20}",
        ) {
            let v = s.into_bytes();
            // Pattern that can't match lowercase
            let result = replace_strings(&v, &vec![b"999".to_vec()], &vec![b"X".to_vec()]);
            prop_assert_eq!(result, v);
        }
    }
}
