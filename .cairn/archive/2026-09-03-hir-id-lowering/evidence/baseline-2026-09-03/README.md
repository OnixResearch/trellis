# Baseline evidence

The baseline source is `792e6900477e4af42a5792ef4cb43bc97da4843f`.
No HIR ID-lowering implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,915 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,119.

See the sibling command transcripts and `status.txt`.
