# Baseline evidence

The baseline source is `782dea23b209471b26f790eb6b2469dba63b060c`.
No backend-codegen record implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,925 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,129.

See the sibling command transcripts and `status.txt`.
