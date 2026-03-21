{
  description = "verified-logic — Verified Rust library with Verus proofs and Tracey coverage";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Source filtering — include scripts/ and docs/ for verification checks
        src = let
          scriptFilter = path: _type:
            (builtins.match ".*\\.sh$" path) != null
            || (builtins.match ".*\\.md$" path) != null
            || (builtins.match ".*\\.styx$" path) != null;
          combinedFilter = path: type:
            (scriptFilter path type) || (craneLib.filterCargoSources path type);
        in pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = combinedFilter;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
          mold
        ];

        buildInputs = with pkgs; [
          openssl
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src nativeBuildInputs buildInputs;
        };

        verified-logic = craneLib.buildPackage {
          inherit src cargoArtifacts nativeBuildInputs buildInputs;
        };

        # --- Verification tools ---

        # Verus: fetch release binary + rustc 1.93.0 for shared libs
        rustc-1_93 = pkgs.rust-bin.stable."1.93.0".default;

        verus = pkgs.stdenv.mkDerivation {
          pname = "verus";
          version = "0.2026.01.30";
          src = pkgs.fetchzip {
            url = "https://github.com/verus-lang/verus/releases/download/release/0.2026.01.30.44ebdee/verus-0.2026.01.30.44ebdee-x86-linux.zip";
            hash = "sha256-FEdsMtZpl23i8qkKhFpc13GguDj6o0UHB1v0U78bcXs=";
            stripRoot = true;
          };
          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [ pkgs.stdenv.cc.cc.lib pkgs.zlib rustc-1_93 ];
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
            # Create a wrapper script that sets up the library path
            mkdir -p $out/bin
            cat > $out/bin/verus-verify <<WRAPPER
            #!/usr/bin/env bash
            export VERUS_Z3_PATH="$out/z3"
            export LD_LIBRARY_PATH="${rustc-1_93}/lib\''${LD_LIBRARY_PATH:+:\$LD_LIBRARY_PATH}"
            exec "$out/rust_verify" "\$@"
            WRAPPER
            chmod +x $out/bin/verus-verify
          '';
        };

        # Tracey: fetch release binary
        tracey = pkgs.stdenv.mkDerivation {
          pname = "tracey";
          version = "1.3.0";
          src = pkgs.fetchurl {
            url = "https://github.com/bearcove/tracey/releases/latest/download/tracey-x86_64-unknown-linux-gnu.tar.xz";
            hash = "sha256-+8DCXEQyjMsJcLJQkJX/KUEvpyy7xrADbEzujBKCH0c=";
          };
          sourceRoot = ".";
          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [ pkgs.stdenv.cc.cc.lib ];
          installPhase = ''
            mkdir -p $out/bin
            install -m 755 tracey-*/tracey $out/bin/tracey
          '';
        };

      in
      {
        packages = {
          default = verified-logic;
          verified-logic = verified-logic;
        };

        checks = {
          inherit verified-logic;

          # Cargo tests with nextest
          nextest = craneLib.cargoNextest {
            inherit src cargoArtifacts nativeBuildInputs buildInputs;
            partitions = 1;
            partitionType = "count";
          };

          # Clippy lints
          clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts nativeBuildInputs buildInputs;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          };

          # Format check
          fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Every pub fn has r[impl] annotation
          check-coverage = pkgs.runCommand "check-coverage" {} ''
            cd ${src}
            bash scripts/check-coverage.sh
            touch $out
          '';

          # Every pub fn in verus! has ensures
          check-ensures = pkgs.runCommand "check-ensures" {} ''
            cd ${src}
            bash scripts/check-ensures.sh
            touch $out
          '';

          # Verus formal verification
          verus-proofs = pkgs.runCommand "verus-proofs" {
            nativeBuildInputs = [ verus ];
          } ''
            cd ${src}
            verus-verify \
              --crate-type=lib \
              --extern vstd="${verus}/libvstd.rlib" \
              --extern builtin="${verus}/libverus_builtin.rlib" \
              --extern builtin_macros="${verus}/libverus_builtin_macros.so" \
              --extern state_machines_macros="${verus}/libverus_state_machines_macros.so" \
              -L "${verus}/lib" \
              --edition 2021 \
              src/lib.rs
            touch $out
          '';

          # Tracey requirement coverage
          tracey-coverage = pkgs.runCommand "tracey-coverage" {
            nativeBuildInputs = [ tracey ];
          } ''
            cd ${src}
            export HOME=$(mktemp -d)

            UNCOVERED=$(tracey query uncovered 2>&1)
            echo "$UNCOVERED"
            echo "$UNCOVERED" | grep -q "0 uncovered" || {
              echo "FAIL: uncovered requirements found"
              exit 1
            }

            UNTESTED=$(tracey query untested 2>&1)
            echo "$UNTESTED"
            echo "$UNTESTED" | grep -q "0 untested" || {
              echo "FAIL: untested requirements found"
              exit 1
            }

            touch $out
          '';
        };

        devShells.default = craneLib.devShell {
          inherit buildInputs;

          packages = with pkgs; [
            cargo-nextest
            cargo-watch
            rust-analyzer
          ];

          inputsFrom = [ verified-logic ];
        };
      }
    );
}
