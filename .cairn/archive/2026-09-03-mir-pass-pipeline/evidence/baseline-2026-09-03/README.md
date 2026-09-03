# Baseline Evidence

Source: `df0d0121d5cdcd0d67bc6720ae0332ef33a7f6e3` (`origin/main` before implementation).

- Full Cargo tests returned status `0`.
- Full-library Verus returned status `0`.
- Tracey reported `0 uncovered out of 2087 rules` on the direct rerun.
- Tracey reported `0 untested out of 2087 rules`.

The first uncovered query hit the known Tracey daemon-start cancellation. The rerun used an unchanged detached worktree and produced the recorded result.
