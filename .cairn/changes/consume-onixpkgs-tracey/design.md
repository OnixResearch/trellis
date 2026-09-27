## Context

Trellis instantiates nixpkgs once per system with the rust-overlay overlay. The only Tracey consumer is the `tracey-coverage` check, which took a `let`-bound Tracey derivation fetched from `https://github.com/bearcove/tracey/releases/latest/download/tracey-x86_64-unknown-linux-gnu.tar.xz`. onixpkgs `pkgs/tracey` packages the same 1.3.0 release from versioned URLs with the same x86_64-linux artifact hash, plus aarch64-linux and aarch64-darwin artifacts.

## Decisions

### Decision: Consume Tracey through the onixpkgs overlay

**Choice:** Add `onixpkgs.overlays.default` after rust-overlay in Trellis's `import nixpkgs` and use `pkgs.tracey` in the check.

**Rationale:** Trellis already instantiates nixpkgs with overlays, so the overlay builds Tracey over Trellis's own package set, as the onixpkgs README recommends. No other onixpkgs attribute name is referenced in `flake.nix`, so the overlay adds Tracey without shadowing a package Trellis uses.

### Decision: Follow Trellis's nixpkgs

**Choice:** Set `inputs.onixpkgs.inputs.nixpkgs.follows = "nixpkgs"`. Trellis has no `treefmt-nix` input, so onixpkgs keeps its own.

**Rationale:** One nixpkgs revision keeps the lock small and keeps Tracey built against the same package set as the rest of Trellis.

## Risks / Trade-offs

- The Tracey derivation now comes from a private flake input, so locking or updating it needs SSH access to `OnixResearch/onixpkgs`. Trellis already locks a private `git+ssh` input (`trellis-consumer-release-src`).
- `pkgs.tracey` throws on systems onixpkgs does not package (for example x86_64-darwin). The old inline copy fetched the x86_64-linux binary on every system, so no system lost a working Tracey.
