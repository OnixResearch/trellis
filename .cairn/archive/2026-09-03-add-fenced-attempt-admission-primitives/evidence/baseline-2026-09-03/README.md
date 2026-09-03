# Baseline evidence

The baseline source is `5965871d734b56d132cde1cdd0e6b25962c11dc4`.
No fenced-attempt implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,969 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,196.

See the sibling command transcripts and `status.txt`.
