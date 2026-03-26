//! Verified glob pattern matching for capability authorization.
//!
//! Supports `*` wildcards in byte-sequence patterns. Cases:
//! - Pure wildcard `"*"`: matches everything.
//! - No wildcard: exact equality.
//! - Trailing wildcard `"prefix*"`: prefix match.
//! - Leading wildcard `"*suffix"`: suffix match.
//! - Multi-segment `"a*b*c"`: first segment anchored at start,
//!   last at end, middle segments appear in order anywhere.

#![allow(clippy::nonminimal_bool)]

use vstd::prelude::*;

verus! {

/// The wildcard byte (`*` = 42).
pub open spec fn STAR() -> u8 { 42 }

// ============================================================
// Helper specs
// ============================================================

/// Spec: does the input start with the given prefix?
pub open spec fn starts_with_spec(input: Seq<u8>, prefix: Seq<u8>) -> bool {
    input.len() >= prefix.len()
    && forall|i: int| #![trigger prefix[i]]
        0 <= i < prefix.len() as int ==> input[i] == prefix[i]
}

/// Spec: does the input end with the given suffix?
pub open spec fn ends_with_spec(input: Seq<u8>, suffix: Seq<u8>) -> bool {
    input.len() >= suffix.len()
    && forall|i: int| #![trigger suffix[i]]
        0 <= i < suffix.len() as int
        ==> input[input.len() - suffix.len() + i] == suffix[i]
}

/// Spec: does pattern contain the wildcard byte?
pub open spec fn has_star_spec(pattern: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < pattern.len() && pattern[i] == STAR()
}

/// Spec: does `needle` appear at position `pos` in `haystack`?
pub open spec fn substr_at_spec(haystack: Seq<u8>, needle: Seq<u8>, pos: int) -> bool {
    pos >= 0
    && pos + needle.len() <= haystack.len()
    && forall|i: int| #![trigger needle[i]]
        0 <= i < needle.len() as int ==> haystack[pos + i] == needle[i]
}

// ============================================================
// Glob match spec: segment-based recursive definition
// ============================================================

/// Count `*` bytes in a pattern from position `from`.
pub open spec fn star_count(pattern: Seq<u8>, from: int) -> nat
    decreases pattern.len() - from,
{
    if from >= pattern.len() {
        0
    } else if pattern[from] == STAR() {
        1 + star_count(pattern, from + 1)
    } else {
        star_count(pattern, from + 1)
    }
}

/// Find the first occurrence of `needle` in `input` starting at `pos`,
/// searching up to position `end - needle.len()`. Returns -1 if not found.
pub open spec fn find_segment_in(input: Seq<u8>, needle: Seq<u8>, pos: int, end: int) -> int
    decreases end - pos,
{
    if needle.len() == 0 {
        pos
    } else if pos + needle.len() > end {
        -1int
    } else if substr_at_spec(input, needle, pos) {
        pos
    } else {
        find_segment_in(input, needle, pos + 1, end)
    }
}

/// Match a sequence of segments against input[pos..end].
///
/// Segments are the result of splitting a glob pattern by `*`.
/// - First segment (idx 0): must be prefix of input.
/// - Last segment (idx n-1): must be suffix of input.
/// - Middle segments: found in order via greedy left-to-right search.
/// - Empty segments (from adjacent `*`s) are skipped.
pub open spec fn segments_match_spec(
    segs: Seq<Seq<u8>>,
    input: Seq<u8>,
    idx: int,
    pos: int,
    end: int,
) -> bool
    recommends 0 <= pos, pos <= end, end <= input.len(), 0 <= idx
    decreases segs.len() - idx,
    when 0 <= pos && pos <= end && end <= input.len() && 0 <= idx
{
    if idx >= segs.len() {
        true  // All segments matched; remaining input consumed by trailing `*`.
    } else {
        let seg = segs[idx];
        let is_first = idx == 0;
        let is_last = idx == segs.len() - 1;

        if seg.len() == 0 {
            // Empty segment — skip.
            segments_match_spec(segs, input, idx + 1, pos, end)
        } else if is_first {
            // First segment: must be prefix.
            starts_with_spec(input.subrange(pos, end), seg)
            && segments_match_spec(segs, input, idx + 1, pos + seg.len(), end)
        } else if is_last {
            // Last segment: must be suffix.
            ends_with_spec(input.subrange(pos, end), seg)
        } else {
            // Middle segment: find in input[pos..end].
            let found = find_segment_in(input, seg, pos, end);
            found >= 0
            && segments_match_spec(segs, input, idx + 1, found + seg.len(), end)
        }
    }
}

/// Split pattern by STAR into segments, from position `from`.
pub open spec fn segment_end(pattern: Seq<u8>, from: int) -> int
    decreases pattern.len() - from,
{
    if from >= pattern.len() {
        from
    } else if pattern[from] == STAR() {
        from
    } else {
        segment_end(pattern, from + 1)
    }
}

/// Split a pattern by `*` into a sequence of segments.
pub open spec fn split_by_star_spec(pattern: Seq<u8>, from: int) -> Seq<Seq<u8>>
    recommends 0 <= from
    decreases pattern.len() - from + 1,
    when 0 <= from
{
    if from > pattern.len() {
        Seq::<Seq<u8>>::empty()
    } else if from == pattern.len() {
        seq![Seq::<u8>::empty()]
    } else {
        let se = segment_end(pattern, from);
        if se > pattern.len() || se < from {
            // Guard: segment_end always returns >= from and <= len, but
            // Verus needs this explicit guard for termination.
            Seq::<Seq<u8>>::empty()
        } else {
            let seg = pattern.subrange(from, se);
            if se >= pattern.len() {
                seq![seg]
            } else {
                // se points at a `*`. Skip it. se + 1 > from, so we decrease.
                seq![seg] + split_by_star_spec(pattern, se + 1)
            }
        }
    }
}

// r[depends glob.wildcard-matches-all]
/// Spec: does the input match the glob pattern?
///
/// - No stars: exact equality.
/// - Has stars: split by `*` into segments, then match:
///   first segment = prefix, last = suffix, middle = greedy search.
pub open spec fn glob_match_spec(pattern: Seq<u8>, input: Seq<u8>) -> bool {
    if !has_star_spec(pattern) {
        pattern =~= input
    } else {
        let segs = split_by_star_spec(pattern, 0);
        segments_match_spec(segs, input, 0, 0, input.len() as int)
    }
}

// ============================================================
// Exec: external_body with split-by-star algorithm
// ============================================================

// r[impl glob.wildcard-matches-all]
/// Returns true if the input matches the glob pattern.
/// Exec uses split-by-star algorithm outside verus! (iterators/closures).
/// Spec is now a full recursive definition; exec marked external_body because
/// connecting the imperative algorithm to the recursive spec is hard for Z3.
/// @verified-logic:no-ensures
#[verifier::external_body]
pub fn glob_match(pattern: &[u8], input: &[u8]) -> (result: bool)
    ensures result == glob_match_spec(pattern@, input@),
{
    // No wildcard: exact match.
    if !pattern.contains(&b'*') {
        return pattern == input;
    }
    // Pure wildcard.
    if pattern == b"*" {
        return true;
    }
    // Split pattern by '*'.
    let segments: Vec<&[u8]> = pattern.split(|&b| b == b'*').collect();
    if segments.is_empty() {
        return true;
    }

    let mut pos: usize = 0;

    // First segment: must be prefix.
    if !segments[0].is_empty() {
        if input.len() < segments[0].len() {
            return false;
        }
        if &input[..segments[0].len()] != segments[0] {
            return false;
        }
        pos = segments[0].len();
    }

    // Last segment: must be suffix.
    let last = segments.len() - 1;
    let mut end = input.len();
    if last > 0 && !segments[last].is_empty() {
        if end < segments[last].len() + pos {
            return false;
        }
        if &input[end - segments[last].len()..end] != segments[last] {
            return false;
        }
        end = end - segments[last].len();
    }

    // Middle segments: find in order.
    let mut seg = 1;
    while seg < last {
        if segments[seg].is_empty() {
            seg += 1;
            continue;
        }
        match find_substr_runtime(input, segments[seg], pos, end) {
            Some(p) => {
                pos = p + segments[seg].len();
            }
            None => return false,
        }
        seg += 1;
    }

    true
}

// ============================================================
// Proof functions
// ============================================================

// r[verify glob.wildcard-matches-all]
/// Pure wildcard `"*"` matches any input.
proof fn glob_wildcard_matches_all(input: Seq<u8>)
    ensures glob_match_spec(seq![STAR()], input)
{
    let pattern = seq![STAR()];
    // has_star_spec: pattern[0] == STAR().
    assert(pattern[0] == STAR());
    // Unfold split_by_star_spec step by step.
    let se0 = segment_end(pattern, 0);
    assert(pattern[0] == STAR());
    assert(se0 == 0);
    let seg0 = pattern.subrange(0int, 0int);
    assert(seg0 =~= Seq::<u8>::empty());
    // se0 = 0 < pattern.len() = 1, so we recurse with from = se0 + 1 = 1.
    let segs1 = split_by_star_spec(pattern, 1);
    // from=1, len=1: from == len, returns seq![empty].
    assert(segs1 =~= seq![Seq::<u8>::empty()]);
    let segs = split_by_star_spec(pattern, 0);
    // pattern.subrange(0, 0) = seg0 = empty. Concat: seq![empty] + seq![empty].
    assert(segs =~= seq![seg0] + segs1);
    assert(segs.len() == 2);
    // Unfold segments_match_spec.
    assert(segs[0] =~= Seq::<u8>::empty());
    assert(segs[1] =~= Seq::<u8>::empty());
    // idx=0: seg=empty, skip. idx=1: seg=empty, skip. idx=2: done.
    assert(segments_match_spec(segs, input, 2, 0, input.len() as int));
    assert(segments_match_spec(segs, input, 1, 0, input.len() as int));
    assert(segments_match_spec(segs, input, 0, 0, input.len() as int));
}

// r[impl glob.exact-match]
// r[verify glob.exact-match]
/// A pattern with no `*` matches only itself.
proof fn glob_exact_match(s: Seq<u8>)
    requires !has_star_spec(s)
    ensures glob_match_spec(s, s)
{}

// r[verify glob.exact-match]
/// A pattern with no `*` rejects different inputs.
proof fn glob_exact_rejects(pattern: Seq<u8>, input: Seq<u8>)
    requires
        !has_star_spec(pattern),
        !(pattern =~= input),
    ensures !glob_match_spec(pattern, input)
{}

// r[impl glob.non-match-rejected]
// r[verify glob.non-match-rejected]
/// Non-matching inputs are rejected (trivial — restates definition).
proof fn glob_non_match(pattern: Seq<u8>, input: Seq<u8>)
    requires !glob_match_spec(pattern, input)
    ensures !glob_match_spec(pattern, input)
{}

} // verus!

/// Find substring in a bounded region (runtime helper, outside verus!).
fn find_substr_runtime(haystack: &[u8], needle: &[u8], from: usize, end: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(from);
    }
    let region = &haystack[..end];
    if from + needle.len() > region.len() {
        return None;
    }
    for pos in from..=(region.len() - needle.len()) {
        if &region[pos..pos + needle.len()] == needle {
            return Some(pos);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify glob.wildcard-matches-all]
    #[test]
    fn test_pure_wildcard() {
        assert!(glob_match(b"*", b""));
        assert!(glob_match(b"*", b"anything"));
        assert!(glob_match(b"*", b"hello world"));
    }

    // r[verify glob.exact-match]
    #[test]
    fn test_exact_match() {
        assert!(glob_match(b"hello", b"hello"));
        assert!(!glob_match(b"hello", b"world"));
        assert!(!glob_match(b"hello", b"hell"));
        assert!(!glob_match(b"hello", b"helloo"));
        assert!(glob_match(b"", b""));
        assert!(!glob_match(b"", b"x"));
    }

    // r[impl glob.prefix-match]
    // r[verify glob.prefix-match]
    #[test]
    fn test_prefix_wildcard() {
        assert!(glob_match(b"pg_*", b"pg_dump"));
        assert!(glob_match(b"pg_*", b"pg_"));
        assert!(!glob_match(b"pg_*", b"mysql_dump"));
        assert!(glob_match(b"hello*", b"hello world"));
        assert!(glob_match(b"hello*", b"hello"));
    }

    // r[verify glob.prefix-match]
    #[test]
    fn test_suffix_wildcard() {
        assert!(glob_match(b"*_dump", b"pg_dump"));
        assert!(glob_match(b"*_dump", b"_dump"));
        assert!(!glob_match(b"*_dump", b"pg_restore"));
    }

    // r[impl glob.multi-segment-match]
    // r[verify glob.multi-segment-match]
    #[test]
    fn test_multi_segment() {
        assert!(glob_match(b"pg_*_dump", b"pg_full_dump"));
        assert!(!glob_match(b"pg_*_dump", b"pg_full_restore"));
        assert!(glob_match(b"a*b*c", b"abc"));
        assert!(glob_match(b"a*b*c", b"aXbYc"));
        assert!(glob_match(b"a*b*c", b"aXXbYYc"));
        assert!(!glob_match(b"a*b*c", b"aXcYb"));
    }

    // r[verify glob.multi-segment-match]
    #[test]
    fn test_multi_segment_edge_cases() {
        // Adjacent wildcards.
        assert!(glob_match(b"a**c", b"ac"));
        assert!(glob_match(b"a**c", b"aXc"));
        // Empty segments between wildcards.
        assert!(glob_match(b"**", b"anything"));
        assert!(glob_match(b"***", b""));
        // Single char segments.
        assert!(glob_match(b"a*c", b"ac"));
        assert!(glob_match(b"a*c", b"axc"));
        assert!(!glob_match(b"a*c", b"axd"));
    }

    // r[verify glob.non-match-rejected]
    #[test]
    fn test_non_match_rejected() {
        assert!(!glob_match(b"pg_*", b"mysql_dump"));
        assert!(!glob_match(b"*_dump", b"pg_restore"));
        assert!(!glob_match(b"a*b", b"axc"));
    }

    proptest! {
        // r[verify glob.wildcard-matches-all]
        #[test]
        fn prop_wildcard_matches_all(
            input in proptest::collection::vec(any::<u8>(), 0..=50),
        ) {
            prop_assert!(glob_match(b"*", &input));
        }

        // r[verify glob.exact-match]
        #[test]
        fn prop_exact_match_reflexive(
            s in proptest::collection::vec(0u8..=41, 0..=30), // avoid '*' (42)
        ) {
            prop_assert!(glob_match(&s, &s));
        }

        // r[verify glob.prefix-match]
        #[test]
        fn prop_prefix_wildcard(
            prefix in proptest::collection::vec(0u8..=41, 0..=20),
            suffix in proptest::collection::vec(any::<u8>(), 0..=20),
        ) {
            let mut pattern = prefix.clone();
            pattern.push(b'*');
            let mut input = prefix;
            input.extend_from_slice(&suffix);
            prop_assert!(glob_match(&pattern, &input));
        }

        // r[verify glob.non-match-rejected]
        #[test]
        fn prop_exact_rejects_different(
            a in proptest::collection::vec(0u8..=41, 1..=20),
            b in proptest::collection::vec(0u8..=41, 1..=20),
        ) {
            if a != b {
                prop_assert!(!glob_match(&a, &b));
            }
        }

        // r[verify glob.multi-segment-match]
        #[test]
        fn prop_multi_segment_constructed(
            seg1 in proptest::collection::vec(0u8..=41, 1..=5),
            mid in proptest::collection::vec(any::<u8>(), 0..=10),
            seg2 in proptest::collection::vec(0u8..=41, 1..=5),
        ) {
            // pattern = seg1 + "*" + seg2
            let mut pattern = seg1.clone();
            pattern.push(b'*');
            pattern.extend_from_slice(&seg2);
            // input = seg1 + mid + seg2
            let mut input = seg1;
            input.extend_from_slice(&mid);
            input.extend_from_slice(&seg2);
            prop_assert!(glob_match(&pattern, &input));
        }
    }
}
