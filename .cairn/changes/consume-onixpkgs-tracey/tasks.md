## Phase 1: Implementation

- [x] [serial] Add the `onixpkgs` input following `nixpkgs`, apply `onixpkgs.overlays.default` to the Trellis package set, point `tracey-coverage` at `pkgs.tracey`, delete the inline Tracey derivation, and lock only the `onixpkgs` subtree. r[verification-tooling.tracey-shared-package]
- [x] [serial] Evaluate the Trellis package, dev shell, and `tracey-coverage` outputs, build `checks.x86_64-linux.tracey-coverage`, and run Cairn validation. r[verification-tooling.tracey-shared-package]
