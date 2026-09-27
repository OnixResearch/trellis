# Verification Tooling Delta

## ADDED Requirements

### Requirement: Tracey Comes From the Shared Package Set
r[verification-tooling.tracey-shared-package]

The Nix Tracey coverage check MUST use the Tracey package published by onixpkgs through Trellis's nixpkgs instance, and `flake.nix` MUST NOT define its own Tracey derivation or fetch Tracey from a moving `releases/latest` URL.

#### Scenario: Coverage check uses the onixpkgs Tracey

- GIVEN `flake.nix` applies `onixpkgs.overlays.default` to its nixpkgs instance
- WHEN `checks.<system>.tracey-coverage` evaluates
- THEN its Tracey input is `pkgs.tracey` from the overlay

#### Scenario: No local Tracey copy remains

- GIVEN the switch has landed
- WHEN `flake.nix` is searched for a Tracey derivation or a `tracey/releases/latest` URL
- THEN neither is present
