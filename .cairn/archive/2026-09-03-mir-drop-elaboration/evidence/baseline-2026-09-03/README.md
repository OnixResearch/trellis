# Baseline evidence

The baseline source is `44226097b7e73add55e7a809fc7353c2645e4971`.
No finite destruction-decision implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,962 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,185.

See the sibling command transcripts and `status.txt`.
