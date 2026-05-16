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
          nix
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
        rustc-1_94 = pkgs.rust-bin.stable."1.94.0".default;

        verus = pkgs.stdenv.mkDerivation {
          pname = "verus";
          version = "0.2026.03.22";
          src = pkgs.fetchzip {
            url = "https://github.com/verus-lang/verus/releases/download/release/0.2026.03.22.5e66329/verus-0.2026.03.22.5e66329-x86-linux.zip";
            hash = "sha256-/rSsNizWruoGgAJciXt8oWUfLUqqs8vBhcSKqxWzhl8=";
            stripRoot = true;
          };
          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [ pkgs.stdenv.cc.cc.lib pkgs.zlib rustc-1_94 ];
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
            # Create a wrapper script that sets up the library path
            mkdir -p $out/bin
            cat > $out/bin/verus-verify <<WRAPPER
            #!/usr/bin/env bash
            export VERUS_Z3_PATH="$out/z3"
            export LD_LIBRARY_PATH="${rustc-1_94}/lib\''${LD_LIBRARY_PATH:+:\$LD_LIBRARY_PATH}"
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
          check-ensures = pkgs.runCommand "check-ensures" {
            nativeBuildInputs = [ pkgs.python3 ];
          } ''
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

            # Tracey may return a transient daemon-start "Error: Cancelled" on
            # the first query. Warm the daemon and retry boundedly before
            # treating coverage as failed.
            tracey query status || true

            run_tracey_check() {
              query="$1"
              expected="$2"
              label="$3"

              for attempt in 1 2 3 4; do
                output=$(tracey query "$query" 2>&1) || true
                echo "$output"
                if echo "$output" | grep -q "$expected"; then
                  return 0
                fi
                if [ "$attempt" -lt 4 ]; then
                  sleep 2
                fi
              done

              echo "FAIL: $label requirements found"
              exit 1
            }

            run_tracey_check uncovered "0 uncovered" uncovered
            run_tracey_check untested "0 untested" untested

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
