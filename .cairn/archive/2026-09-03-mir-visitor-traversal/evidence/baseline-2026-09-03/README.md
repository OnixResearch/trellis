# Baseline evidence

The baseline source is `41cc039ce8c72e494cba4bfb598132cae1083790`.
No MIR visitor implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,898 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,109.

See the sibling command transcripts and `status.txt`.
