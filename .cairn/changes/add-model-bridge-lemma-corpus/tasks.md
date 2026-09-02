## Tasks

- [ ] [serial] Define the bridge-lemma handshake contract: model requirement ID, code identifier, Verus proof identifier, assumption list, and one documented proof boundary. r[trellis.bridge_lemma.handshake]
- [ ] [serial] Add the bounded bridge-lemma corpus for the first protocol surface, with exact provenance metadata on every lemma. r[trellis.bridge_lemma.corpus]
- [ ] [parallel] Bind each lemma to its model requirement ID and code invariant and record imported model-layer facts as assumptions. r[trellis.bridge_lemma.provenance]
- [ ] [parallel] Add negative fixtures for a non-holding invariant pair, a mismatched requirement ID, an unrecorded assumption, and a missing non-claim. r[trellis.bridge_lemma.fixtures]
- [ ] [parallel] Enforce the bounded non-claim on every lemma and reject any lemma that omits it. r[trellis.bridge_lemma.nonclaim]
- [ ] [serial] Run the existing verification harness over positive and negative fixtures, then lifecycle validation, Cairn gates, and the relevant flake checks. r[trellis.bridge_lemma.validation]
