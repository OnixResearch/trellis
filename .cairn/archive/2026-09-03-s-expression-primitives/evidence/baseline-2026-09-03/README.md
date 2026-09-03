# Baseline evidence

The baseline source is `2a4529bcefdbeef07d4bf8cd4528a1b7e902a2e6`.
No canonical S-expression implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,942 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,150.

See the sibling command transcripts and `status.txt`.
