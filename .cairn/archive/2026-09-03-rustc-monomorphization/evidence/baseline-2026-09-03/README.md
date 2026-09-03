# Baseline evidence

The baseline source is `5b9538e5088a1221a41aa29e974a871873804ff4`.
No concrete-item collection implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,937 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,140.

See the sibling command transcripts and `status.txt`.
