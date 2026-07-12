{
  description = "Trellis — composable verified-logic building blocks with Verus proofs and Tracey coverage";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    octet.url = "github:OnixResearch/octet?rev=fc38f59330b626961d166febfdf1a5aa6575460f";
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils, octet, ... }:
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
            || (builtins.match ".*/scripts/.*\\.rs$" path) != null
            || (builtins.match ".*\\.md$" path) != null
            || (builtins.match ".*\\.tsv$" path) != null
            || (builtins.match ".*\\.styx$" path) != null
            || (builtins.match ".*\\.ncl$" path) != null
            || (builtins.match ".*/tests/fixtures/.*\\.v$" path) != null
            || (builtins.match ".*/tests/fixtures/.*\\.sv$" path) != null
            || (builtins.baseNameOf path == "dylint.toml")
            || (builtins.baseNameOf path == "octet-baseline.json")
            || (builtins.baseNameOf path == "verusfmt-baseline.txt")
            || (builtins.match ".*/verification-policy/generated/.*\\.json$" path) != null
            || (builtins.match ".*/upstream-verus-corpus/generated/.*\\.json$" path) != null;
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

        octetConsumerVendorDir = craneLib.vendorCargoDeps {
          cargoLock = ./Cargo.lock;
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

        corpusCatalog =
          builtins.fromJSON (builtins.readFile ./upstream-verus-corpus/generated/catalog.json);
        corpusSources = builtins.listToAttrs (
          map (entry: {
            name = entry.id;
            value = pkgs.fetchzip {
              url = "${entry.repository_url}/archive/${entry.revision}.tar.gz";
              hash = entry.source_hash;
            };
          }) corpusCatalog.entries
        );

        octetPackages = octet.packages.${system};
        verusfmt = builtins.getAttr "octet-verusfmt" octetPackages;
        verusToolchainProfile = builtins.getAttr "octet-verus-toolchain-profile" octetPackages;

        upstreamVerusCorpus = pkgs.runCommand "trellis-upstream-verus-corpus" {
          nativeBuildInputs = with pkgs; [ b3sum diffutils jq nickel ];
        } ''
          corpus_dir=${./upstream-verus-corpus}
          mkdir -p "$out/share/trellis/upstream-verus-corpus"
          cd "$corpus_dir"
          nickel export --format json catalog.ncl > "$TMPDIR/catalog.json"
          diff --unified generated/catalog.json "$TMPDIR/catalog.json"
          nickel export --format json fixtures/valid/current_catalog.ncl > /dev/null
          for invalid_fixture in fixtures/invalid/*.ncl; do
            if nickel export --format json "$invalid_fixture" > /dev/null 2>&1; then
              echo "invalid corpus fixture unexpectedly passed: $invalid_fixture" >&2
              exit 1
            fi
          done
          jq --exit-status '
            .schema_version == "trellis-upstream-verus-corpus/v1"
            and .claim_scope == "source-intake-and-regression-selection-only"
            and (.entries | map(.id) == (map(.id) | sort))
            and (.entries | map(.id) | length == (unique | length))
            and (.global_non_claims | length > 0)
          ' "$TMPDIR/catalog.json" > /dev/null
          ${pkgs.lib.concatMapStringsSep "\n" (entry:
            let source = corpusSources.${entry.id};
            in pkgs.lib.concatMapStringsSep "\n" (sentinel: ''
              test -e ${source}/${sentinel}
            '') entry.sentinels
          ) corpusCatalog.entries}
          ${pkgs.lib.concatMapStringsSep "\n" (entry:
            let source = corpusSources.${entry.id};
            in ''
              printf '%s\t%s\t%s\n' '${entry.id}' '${entry.revision}' '${source}' >> "$TMPDIR/source-store-paths.tsv"
            ''
          ) corpusCatalog.entries}
          jq '{
            schema_version: "trellis-corpus-source-intake/v1",
            claim_scope,
            sources: [.entries[] | {
              id,
              repository_url,
              revision,
              source_hash,
              source_hash_protocol,
              sentinels,
              execution_policy
            }],
            non_claims: .global_non_claims
          }' "$TMPDIR/catalog.json" > "$out/share/trellis/upstream-verus-corpus/source-intake.json"
          cp "$TMPDIR/catalog.json" "$out/share/trellis/upstream-verus-corpus/catalog.json"
          cp "$TMPDIR/source-store-paths.tsv" "$out/share/trellis/upstream-verus-corpus/source-store-paths.tsv"
          b3sum "$out/share/trellis/upstream-verus-corpus/source-intake.json" \
            > "$out/share/trellis/upstream-verus-corpus/source-intake.blake3"
        '';

        verusfmtRatchetCheck = pkgs.runCommand "trellis-verusfmt-ratchet" {
          nativeBuildInputs = with pkgs; [ diffutils findutils jq ] ++ [ verusfmt verusToolchainProfile ];
        } ''
          corpus_dir=${./upstream-verus-corpus}
          source_root=${./src}
          profile_json=${verusToolchainProfile}/share/octet/verus-toolchain/profile.json
          baseline="$corpus_dir/verusfmt-baseline.txt"
          actual="$TMPDIR/verusfmt-nonconforming.txt"
          normalized_baseline="$TMPDIR/verusfmt-baseline-normalized.txt"

          compare_baseline() {
            diff --unified "$1" "$2"
          }
          validate_octet_profile() {
            jq --exit-status '
              .schema_version == "octet-verus-toolchain-profile/v1"
              and .profile_id == "octet-verus-0.2026.04.03.21dfcd2"
              and .formatter.version == "0.7.2"
              and any(.compatibility[];
                .consumer == "trellis"
                and .status == "local-pin"
                and .verifier_release == "0.2026.03.22.5e66329")
            ' "$1" > /dev/null
          }
          validate_octet_profile "$profile_json"
          jq '(.compatibility[] | select(.consumer == "trellis")).status = "native"' \
            "$profile_json" > "$TMPDIR/mismatched-profile.json"
          if validate_octet_profile "$TMPDIR/mismatched-profile.json"; then
            echo "mismatched Octet compatibility profile unexpectedly passed" >&2
            exit 1
          fi
          LC_ALL=C sort --unique "$baseline" > "$normalized_baseline"
          compare_baseline "$baseline" "$normalized_baseline"

          synthetic_expected="$TMPDIR/synthetic-expected.txt"
          synthetic_valid="$TMPDIR/synthetic-valid.txt"
          synthetic_new_drift="$TMPDIR/synthetic-new-drift.txt"
          synthetic_stale="$TMPDIR/synthetic-stale.txt"
          printf '%s\n' 'src/existing-unformatted.rs' > "$synthetic_expected"
          cp "$synthetic_expected" "$synthetic_valid"
          compare_baseline "$synthetic_expected" "$synthetic_valid"
          cp "$synthetic_expected" "$synthetic_new_drift"
          printf '%s\n' 'src/new-unformatted.rs' >> "$synthetic_new_drift"
          if compare_baseline "$synthetic_expected" "$synthetic_new_drift" > /dev/null; then
            echo "new formatter drift fixture unexpectedly matched its baseline" >&2
            exit 1
          fi
          : > "$synthetic_stale"
          if compare_baseline "$synthetic_expected" "$synthetic_stale" > /dev/null; then
            echo "stale formatter baseline fixture unexpectedly matched" >&2
            exit 1
          fi

          verusfmt --check --verus-only "$corpus_dir/fixtures/valid/formatted.rs"
          if verusfmt --check --verus-only "$corpus_dir/fixtures/invalid/unformatted.rs" > /dev/null 2>&1; then
            echo "unformatted Verus fixture unexpectedly passed" >&2
            exit 1
          fi
          : > "$actual"
          while IFS= read -r source_file; do
            relative_path="src/''${source_file#"$source_root"/}"
            if ! verusfmt --check --verus-only "$source_file" > /dev/null 2>&1; then
              printf '%s\n' "$relative_path" >> "$actual"
            fi
          done < <(find "$source_root" -type f -name '*.rs' -print | LC_ALL=C sort)
          compare_baseline "$baseline" "$actual"
          if verusfmt --update > /dev/null 2>&1; then
            echo "pinned verusfmt wrapper unexpectedly allowed self-update" >&2
            exit 1
          fi
          octet-verusfmt --identity > "$TMPDIR/formatter-identity.txt"
          grep -F 'formatter-profile: octet-verus-0.2026.04.03.21dfcd2' "$TMPDIR/formatter-identity.txt" > /dev/null
          grep -F 'formatter: verusfmt@0.7.2' "$TMPDIR/formatter-identity.txt" > /dev/null
          mkdir -p "$out"
        '';

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
          verusfmt = verusfmt;
          upstream-verus-corpus = upstreamVerusCorpus;
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

          # Octet/Tiger Style lints, shared with the local pre-commit hook.
          # Existing findings are accepted in octet-baseline.json; new stable IDs fail.
          octet = pkgs.runCommand "trellis-octet-baseline-check" {
            nativeBuildInputs = nativeBuildInputs ++ [ octet.packages.${system}.cargo-octet ];
            inherit buildInputs;
          } ''
            export HOME="$PWD/home"
            mkdir -p "$HOME"
            cp -r ${src} source
            chmod -R u+w source
            cd source
            export CARGO_HOME="$PWD/.cargo-home"
            mkdir -p "$CARGO_HOME"
            cp "${octetConsumerVendorDir}/config.toml" "$CARGO_HOME/config.toml"
            cargo-octet check --baseline octet-baseline.json -p verified-logic -- --lib
            mkdir -p "$out"
          '';

          # Every pub fn has r[impl] annotation
          check-coverage = pkgs.runCommand "check-coverage" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-coverage.rs "$TMPDIR/check-coverage.rs"
            rustc "$TMPDIR/check-coverage.rs" -o "$TMPDIR/check-coverage"
            "$TMPDIR/check-coverage"
            touch $out
          '';

          # Every pub fn in verus! has ensures
          check-ensures = pkgs.runCommand "check-ensures" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-ensures.rs "$TMPDIR/check-ensures.rs"
            rustc "$TMPDIR/check-ensures.rs" -o "$TMPDIR/check-ensures"
            "$TMPDIR/check-ensures"
            touch $out
          '';

          # Fixture-backed regression tests for checker scripts
          checker-fixtures = pkgs.runCommand "checker-fixtures" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            export CHECKER_FIXTURES_TMPDIR="$NIX_BUILD_TOP/checker-fixtures-tmp"
            mkdir -p "$CHECKER_FIXTURES_TMPDIR"
            cp scripts/checker-fixtures.rs "$TMPDIR/checker-fixtures.rs"
            rustc "$TMPDIR/checker-fixtures.rs" -o "$TMPDIR/checker-fixtures"
            "$TMPDIR/checker-fixtures"
            touch $out
          '';

          # Nickel-authored verification policy freshness
          verification-policy = pkgs.runCommand "verification-policy" {
            nativeBuildInputs = [ rustToolchain pkgs.clang pkgs.nickel ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-verification-policy.rs "$TMPDIR/check-verification-policy.rs"
            rustc "$TMPDIR/check-verification-policy.rs" -o "$TMPDIR/check-verification-policy"
            "$TMPDIR/check-verification-policy"
            touch $out
          '';

          # README module catalog drift
          readme-module-catalog = pkgs.runCommand "readme-module-catalog" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-readme-module-catalog.rs "$TMPDIR/check-readme-module-catalog.rs"
            rustc "$TMPDIR/check-readme-module-catalog.rs" -o "$TMPDIR/check-readme-module-catalog"
            "$TMPDIR/check-readme-module-catalog"
            "$TMPDIR/check-readme-module-catalog" --self-test
            touch $out
          '';

          # README headline metrics static drift
          readme-metrics = pkgs.runCommand "readme-metrics" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-readme-metrics.rs "$TMPDIR/check-readme-metrics.rs"
            rustc "$TMPDIR/check-readme-metrics.rs" -o "$TMPDIR/check-readme-metrics"
            "$TMPDIR/check-readme-metrics"
            "$TMPDIR/check-readme-metrics" --self-test
            touch $out
          '';

          # Proof-gap inventory exact-site, classification, and rationale drift
          proof-gap-inventory = pkgs.runCommand "proof-gap-inventory" {
            nativeBuildInputs = [ rustToolchain pkgs.clang ];
          } ''
            cd ${src}
            export TMPDIR=$(mktemp -d)
            cp scripts/check-proof-gap-inventory.rs "$TMPDIR/check-proof-gap-inventory.rs"
            rustc "$TMPDIR/check-proof-gap-inventory.rs" -o "$TMPDIR/check-proof-gap-inventory"
            "$TMPDIR/check-proof-gap-inventory"
            for fixture in missing-rationale blank-rationale malformed-rationale; do
              if "$TMPDIR/check-proof-gap-inventory" "--fixture-$fixture" >"$TMPDIR/$fixture.out" 2>&1; then
                echo "FAIL: --fixture-$fixture unexpectedly succeeded"
                cat "$TMPDIR/$fixture.out"
                exit 1
              fi
              grep -q "manifest malformed" "$TMPDIR/$fixture.out"
              echo "PASS: --fixture-$fixture failed closed"
            done
            touch $out
          '';

          # Pinned upstream source intake and exact formatter migration ratchet
          upstream-verus-corpus = upstreamVerusCorpus;
          verusfmt-ratchet = verusfmtRatchetCheck;

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

          packages = (with pkgs; [
            cargo-nextest
            cargo-watch
            rust-analyzer
            steel
            pre-commit
            nickel
          ]) ++ [ verusfmt ];

          inputsFrom = [ verified-logic ];
        };
      }
    );
}
