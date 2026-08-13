# Octet review

The pre-refresh review used immutable Octet revision `731571fbfffd0b7db11ac0cc9d3939105ddc71e2` against the prior baseline.

- Findings: 4,119 warnings and 0 errors.
- Baseline drift: 39 new stable IDs and 18 stale stable IDs.
- Prior baseline hash: `b3:39d6cb06e4c79a7907dc2d6463cb29b056c040b145a2282a87b3f6cbec54ea0a`.
- Review receipt: `octet-pre-refresh-review.txt`.

The drift came from the new ownership module and normalized snippets in the changed choreography files. The review also found two avoidable collection-growth warnings. The implementation now reserves each result vector from the admitted input length. An unrelated formatting change in `src/typed_durable_reference.rs` was removed before refresh.

The remaining findings are warning-only structural, naming, import, bounded-recursion, borrowed-argument, and platform-sized count findings. The bounded recursive lookup has an explicit Verus decrease over the finite admitted entry count. The `Vec` references and `usize` count preserve the executable Verus collection contract. The review did not add a trusted bridge, assumption, or proof-gap site.

The refreshed baseline has hash `b3:d9323b657fc87a9273b8c768ba2afd06991302ca778f3d77e0db08407f3f04b1` and 3,887 accepted stable IDs. The repository-pinned canonical gate uses Octet revision `374bd16b26cee2af34211a29bfa531c016811f51`. It passed with 4,119 warnings, 0 errors, 131 autofixable findings, 0 new IDs, and 0 stale IDs. See `pre-commit-octet.txt`.

This baseline records reviewed warning debt. It does not claim that the warnings are absent or that Octet proves semantic correctness.
