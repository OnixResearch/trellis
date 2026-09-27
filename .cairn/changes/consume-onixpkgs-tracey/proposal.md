## Why

`flake.nix` builds its own Tracey 1.3.0 from the moving `releases/latest` download URL. That copy duplicates the Tracey package that onixpkgs now publishes for the Onix stack, and the moving URL breaks the fixed hash as soon as upstream cuts a new release.

## What Changes

- Add the `onixpkgs` flake input, following Trellis's `nixpkgs`.
- Apply `onixpkgs.overlays.default` where Trellis instantiates nixpkgs, and point the `tracey-coverage` check at `pkgs.tracey`.
- Delete the inline Tracey derivation and its `releases/latest` URL.
- Lock only the new `onixpkgs` subtree in `flake.lock`.

## Impact

- **Files**: `flake.nix`, `flake.lock`, and this change package.
- **Testing**: Evaluate the Trellis outputs that use the package set, build `checks.x86_64-linux.tracey-coverage`, and run Cairn validation.
- **Boundary**: onixpkgs owns the Tracey package definition and source pin. This change does not alter Tracey requirements, annotations, or coverage queries.
