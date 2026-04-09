//! Verified ATerm string escaping for Nix derivations.
//!
//! This is the phase-3 foundation: the string escaping primitive and the
//! derivation data structures used by later full serialization code.

#![allow(clippy::ptr_arg)]

use vstd::prelude::*;

verus! {

#[derive(Clone)]
pub struct DerivationOutput {
    pub name: Vec<u8>,
    pub path: Vec<u8>,
    pub hash_algo: Vec<u8>,
    pub hash: Vec<u8>,
}

#[derive(Clone)]
pub struct Derivation {
    pub outputs: Vec<DerivationOutput>,
    pub input_drvs: Vec<(Vec<u8>, Vec<Vec<u8>>)>,
    pub input_srcs: Vec<Vec<u8>>,
    pub platform: Vec<u8>,
    pub builder: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub env: Vec<(Vec<u8>, Vec<u8>)>,
}

pub closed spec fn escape_byte_spec(b: u8) -> Seq<u8> {
    if b == 92u8 {
        seq![92u8, 92u8]
    } else if b == 34u8 {
        seq![92u8, 34u8]
    } else if b == 10u8 {
        seq![92u8, 110u8]
    } else if b == 13u8 {
        seq![92u8, 114u8]
    } else if b == 9u8 {
        seq![92u8, 116u8]
    } else {
        seq![b]
    }
}

pub closed spec fn escape_prefix_spec(s: Seq<u8>, upto: nat) -> Seq<u8>
    recommends upto <= s.len()
    decreases upto
{
    if upto == 0 {
        Seq::<u8>::empty()
    } else {
        escape_prefix_spec(s, (upto - 1) as nat) + escape_byte_spec(s[(upto - 1) as int])
    }
}

// r[depends nix-drv-serialize.string-escape]
/// Spec: ATerm string escaping with surrounding quotes.
pub open spec fn escape_aterm_spec(s: Seq<u8>) -> Seq<u8> {
    seq![34u8] + escape_prefix_spec(s, s.len()) + seq![34u8]
}

proof fn append_escape_byte_len(b: u8)
    ensures 1 <= escape_byte_spec(b).len() <= 2
{}

// r[verify nix-drv-serialize.string-escape]
/// Escaped output is at least two bytes long because it is quoted.
pub proof fn escape_aterm_has_quotes(s: Seq<u8>)
    ensures
        escape_aterm_spec(s).len() >= 2,
        escape_aterm_spec(s)[0int] == 34u8,
        escape_aterm_spec(s)[escape_aterm_spec(s).len() - 1] == 34u8,
    decreases s.len()
{
    if s.len() != 0 {
        append_escape_byte_len(s[s.len() - 1]);
        escape_aterm_has_quotes(s.subrange(0, s.len() as int - 1));
    }
}

// r[impl nix-drv-serialize.string-escape]
/// Escape a byte string for ATerm serialization and wrap it in quotes.
pub fn escape_aterm(s: &Vec<u8>) -> (result: Vec<u8>)
    ensures result@ == escape_aterm_spec(s@)
{
    let mut result = Vec::new();
    result.push(34u8);

    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s.len(),
            result@ == seq![34u8] + escape_prefix_spec(s@, i as nat),
        decreases s.len() - i,
    {
        let b = s[i];
        if b == 92u8 {
            result.push(92u8);
            result.push(92u8);
        } else if b == 34u8 {
            result.push(92u8);
            result.push(34u8);
        } else if b == 10u8 {
            result.push(92u8);
            result.push(110u8);
        } else if b == 13u8 {
            result.push(92u8);
            result.push(114u8);
        } else if b == 9u8 {
            result.push(92u8);
            result.push(116u8);
        } else {
            result.push(b);
        }
        i = i + 1;
    }

    result.push(34u8);
    result
}

} // verus!

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    use super::*;

    fn v(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    // r[verify nix-drv-serialize.string-escape]
    #[test]
    fn test_escape_aterm_plain() {
        assert_eq!(escape_aterm(&v("hello")), v("\"hello\""));
    }

    // r[verify nix-drv-serialize.string-escape]
    #[test]
    fn test_escape_aterm_specials() {
        assert_eq!(escape_aterm(&b"a\n\r\t\\\"b".to_vec()), v("\"a\\n\\r\\t\\\\\\\"b\""));
    }

    // r[verify nix-drv-serialize.string-escape]
    #[test]
    fn test_derivation_structs_construct() {
        let output = DerivationOutput {
            name: v("out"),
            path: v("/nix/store/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-hello"),
            hash_algo: v("sha256"),
            hash: v("deadbeef"),
        };
        let drv = Derivation {
            outputs: vec![output],
            input_drvs: vec![(v("/nix/store/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb-input.drv"), vec![v("out")])],
            input_srcs: vec![v("/nix/store/cccccccccccccccccccccccccccccccc-src")],
            platform: v("x86_64-linux"),
            builder: v("/nix/store/dddddddddddddddddddddddddddddddd-bash/bin/bash"),
            args: vec![v("-c"), v("echo hi")],
            env: vec![(v("name"), v("hello"))],
        };
        assert_eq!(drv.outputs.len(), 1);
        assert_eq!(drv.args.len(), 2);
        assert_eq!(drv.env[0].0, v("name"));
    }

    proptest! {
        // r[verify nix-drv-serialize.string-escape]
        #[test]
        fn prop_escape_aterm_quotes_input(
            s in proptest::collection::vec(any::<u8>(), 0..64),
        ) {
            let escaped = escape_aterm(&s);
            prop_assert!(escaped.len() >= 2);
            prop_assert_eq!(escaped[0], b'"');
            prop_assert_eq!(escaped[escaped.len() - 1], b'"');
        }

        // r[verify nix-drv-serialize.string-escape]
        #[test]
        fn prop_escape_aterm_escapes_specials(
            s in proptest::collection::vec(any::<u8>(), 0..64),
        ) {
            let escaped = escape_aterm(&s);
            let inner = &escaped[1..escaped.len() - 1];
            let mut escaped_byte = false;
            for &b in inner {
                if escaped_byte {
                    prop_assert!(matches!(b, b'\\' | b'"' | b'n' | b'r' | b't'));
                    escaped_byte = false;
                } else if b == b'\\' {
                    escaped_byte = true;
                } else {
                    prop_assert!(!matches!(b, b'"' | b'\n' | b'\r' | b'\t'));
                }
            }
            prop_assert!(!escaped_byte, "escape sequence truncated at end of output");
        }
    }
}
