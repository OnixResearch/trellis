# Baseline evidence

The baseline source is `891690467fcd40f989c520c590f8ceda5b4305ce`.
No typed normal-form implementation file existed when these checks ran.

- `nix develop --builders '' -c cargo test --quiet` passed.
- The exact-source Nix Verus check passed with 10,948 verified obligations and
  zero errors.
- Tracey reported zero uncovered and zero untested rules out of 2,161.

See the sibling command transcripts and `status.txt`.
