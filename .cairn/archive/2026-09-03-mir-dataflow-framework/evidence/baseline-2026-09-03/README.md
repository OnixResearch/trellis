# Baseline evidence

The baseline source is `8c3655903c51e32e3ae7cfc3f547ef03870e389c`.
No finite MIR analysis implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,955 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,173.

See the sibling command transcripts and `status.txt`.
