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
        verusProofSource = pkgs.runCommand "trellis-verus-proof-source" { } ''
          mkdir -p $out/src
          cp -R ${./src}/. $out/src/
        '';

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
        octetProductionVerus = builtins.getAttr "octet-production-verus" octetPackages;
        verusfmt = builtins.getAttr "octet-verusfmt" octetPackages;
        verusToolchainProfile = builtins.getAttr "octet-verus-toolchain-profile" octetPackages;
        upgradeTrialProfile =
          builtins.fromJSON (builtins.readFile ./upstream-verus-corpus/generated/upgrade-trial.json);
        upgradeTrialRows = builtins.listToAttrs (
          map (row: {
            name = row.id;
            value = row;
          }) upgradeTrialProfile.rows
        );
        allocatorLibcCrate = pkgs.fetchurl {
          url = "https://crates.io/api/v1/crates/libc/0.2.147/download";
          hash = "sha256-tGaPsOqGHB3wlBJ6xfHaNAmoIRakunT8ouWO+ScVm7M=";
        };
        allocatorRlimit = 70;
        timeoutExitCode = 124;
        upgradeTrialSavedEvidence =
          ./cairn/archive/2026-07-11-trial-octet-verus-upgrade/evidence/trial;

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

        verusUpgradeTrial = pkgs.runCommand "trellis-octet-verus-upgrade-trial" {
          nativeBuildInputs = (with pkgs; [
            b3sum
            coreutils
            diffutils
            findutils
            gawk
            gnugrep
            gnused
            gnutar
            gzip
            jq
            nickel
            stdenv.cc
          ]) ++ [ verus octetProductionVerus verusToolchainProfile ];
        } ''
          set -eu
          corpus_dir=${./upstream-verus-corpus}
          trellis_source=${verusProofSource}
          paper_source=${corpusSources.paper-sosp24-artifact}
          ironkv_source=${corpusSources.verified-ironkv}
          allocator_source=${corpusSources.verified-memory-allocator}
          node_source=${corpusSources.verified-node-replication}
          verus_belt_source=${corpusSources.verus-belt}
          candidate_root=${octetProductionVerus}/libexec/verus
          octet_profile_json=${verusToolchainProfile}/share/octet/verus-toolchain/profile.json
          output_dir="$out/share/trellis/verus-upgrade-trial"
          maximum_saved_log_bytes='${toString upgradeTrialRows.verified-node-replication.maximum_saved_log_bytes}'
          probe_timeout_seconds='${toString upgradeTrialRows.verified-node-replication.timeout_seconds}'
          mkdir -p "$output_dir"

          cd "$corpus_dir"
          nickel export --format json upgrade-trial.ncl > "$TMPDIR/upgrade-trial.json"
          diff --unified generated/upgrade-trial.json "$TMPDIR/upgrade-trial.json"
          nickel export --format json fixtures/valid/current_upgrade_trial.ncl > /dev/null
          for invalid_fixture in fixtures/invalid/upgrade_*.ncl; do
            if nickel export --format json "$invalid_fixture" > /dev/null 2>&1; then
              echo "invalid Verus upgrade trial fixture unexpectedly passed: $invalid_fixture" >&2
              exit 1
            fi
          done
          jq --exit-status '
            .schema_version == "trellis-octet-verus-upgrade-trial/v1"
            and .decision.expected == "blocked"
            and .decision.preserve_current_pin == true
            and .decision.octet_compatibility_status == "local-pin"
            and (.rows | length == 5)
            and ([.rows[].id] == ([.rows[].id] | sort))
          ' "$TMPDIR/upgrade-trial.json" > /dev/null
          jq --exit-status '
            .profile_id == "octet-verus-0.2026.04.03.21dfcd2"
            and .verifier.release == "0.2026.04.03.21dfcd2"
            and any(.compatibility[];
              .consumer == "trellis"
              and .status == "local-pin"
              and .verifier_release == "0.2026.03.22.5e66329")
          ' "$octet_profile_json" > /dev/null

          normalize_log() {
            input="$1"
            output="$2"
            normalized_log="$TMPDIR/normalized.log"
            sed \
              -e "s#$trellis_source#<TRELLIS_SOURCE>#g" \
              -e "s#$paper_source#<PAPER_ARTIFACT_SOURCE>#g" \
              -e "s#$ironkv_source#<IRONKV_SOURCE>#g" \
              -e "s#$allocator_source#<ALLOCATOR_SOURCE>#g" \
              -e "s#$node_source#<NODE_REPLICATION_SOURCE>#g" \
              -e "s#$verus_belt_source#<VERUS_BELT_SOURCE>#g" \
              -E -e "s/thread 'rustc' \([0-9]+\)/thread 'rustc' (<PID>)/" \
              -E -e 's/[0-9]+ ms/<DURATION_MS>/g' \
              "$input" | head --bytes "$maximum_saved_log_bytes" > "$normalized_log"
            awk '
              { lines[NR] = $0; if ($0 !~ /^[[:space:]]*$/) last_nonblank = NR }
              END { for (line = 1; line <= last_nonblank; line++) print lines[line] }
            ' "$normalized_log" > "$output"
          }
          file_blake3() {
            digest="$(b3sum "$1")"
            printf '%s' "''${digest%% *}"
          }

          baseline_binary_blake3="$(file_blake3 ${verus}/rust_verify)"
          baseline_solver_blake3="$(file_blake3 ${verus}/z3)"
          cat > "$output_dir/baseline-identity.txt" <<EOF
          proof-verifier: ${upgradeTrialProfile.baseline.verifier}
          proof-verifier-binary-blake3: $baseline_binary_blake3
          proof-solver-blake3: $baseline_solver_blake3
          proof-verifier-nix-output: ${verus}
          trust-boundary: baseline identity and execution do not establish verifier or solver soundness
          EOF
          octet-production-verus --identity > "$output_dir/candidate-identity.txt"
          grep -F 'proof-verifier: ${upgradeTrialProfile.candidate.verifier}' "$output_dir/candidate-identity.txt" > /dev/null

          set +e
          timeout "$probe_timeout_seconds" verus-verify \
            --triggers-mode silent \
            --crate-type=lib \
            --extern vstd="${verus}/libvstd.rlib" \
            --extern builtin="${verus}/libverus_builtin.rlib" \
            --extern builtin_macros="${verus}/libverus_builtin_macros.so" \
            --extern state_machines_macros="${verus}/libverus_state_machines_macros.so" \
            -L "${verus}/lib" \
            --edition 2021 \
            "$trellis_source/src/lib.rs" \
            > "$TMPDIR/baseline.stdout" 2> "$TMPDIR/baseline.stderr"
          baseline_status="$?"
          timeout "$probe_timeout_seconds" octet-production-verus \
            --triggers-mode silent \
            --crate-type=lib \
            --extern vstd="$candidate_root/libvstd.rlib" \
            --extern builtin="$candidate_root/libverus_builtin.rlib" \
            --extern builtin_macros="$candidate_root/libverus_builtin_macros.so" \
            --extern state_machines_macros="$candidate_root/libverus_state_machines_macros.so" \
            -L "$candidate_root" \
            --edition 2021 \
            "$trellis_source/src/lib.rs" \
            > "$TMPDIR/candidate.stdout" 2> "$TMPDIR/candidate.stderr"
          candidate_status="$?"
          set -e
          test "$baseline_status" -eq 0
          test "$candidate_status" -eq 1
          grep -Fx 'verification results:: ${toString upgradeTrialProfile.baseline.expected_verified_count} verified, ${toString upgradeTrialProfile.baseline.expected_error_count} errors' "$TMPDIR/baseline.stdout" > /dev/null
          grep -Fx 'verification results:: ${toString upgradeTrialProfile.candidate.expected_verified_count} verified, ${toString upgradeTrialProfile.candidate.expected_error_count} errors' "$TMPDIR/candidate.stdout" > /dev/null
          ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
            grep -F '${anchor}' "$TMPDIR/candidate.stderr" > /dev/null
          '') upgradeTrialProfile.candidate.diagnostic_anchors}
          normalize_log "$TMPDIR/baseline.stdout" "$output_dir/baseline.stdout"
          normalize_log "$TMPDIR/baseline.stderr" "$output_dir/baseline.stderr"
          normalize_log "$TMPDIR/candidate.stdout" "$output_dir/candidate.stdout"
          normalize_log "$TMPDIR/candidate.stderr" "$output_dir/candidate.stderr"

          set +e
          timeout "$probe_timeout_seconds" octet-production-verus --crate-type=lib \
            "$paper_source/milli/linked-list/verus.rs" \
            > "$TMPDIR/paper-linked.stdout" 2> "$TMPDIR/paper-linked.stderr"
          paper_linked_status="$?"
          timeout "$probe_timeout_seconds" octet-production-verus --crate-type=lib \
            "$paper_source/milli/doubly-linked-list/verus.rs" \
            > "$TMPDIR/paper-doubly.stdout" 2> "$TMPDIR/paper-doubly.stderr"
          paper_doubly_status="$?"
          set -e
          test "$paper_linked_status" -eq 1
          test "$paper_doubly_status" -eq 1
          ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
            grep -F '${anchor}' "$TMPDIR/paper-linked.stderr" "$TMPDIR/paper-doubly.stderr" > /dev/null
          '') upgradeTrialRows.paper-sosp24-artifact.diagnostic_anchors}
          grep -F 'Commit: `097ac7ed283ae60375cd9b2b6017b3c629883b2b`' \
            "$paper_source/external-repository-versions.md" > /dev/null
          {
            cat "$TMPDIR/paper-linked.stdout" "$TMPDIR/paper-doubly.stdout"
          } > "$TMPDIR/paper.stdout"
          {
            cat "$TMPDIR/paper-linked.stderr" "$TMPDIR/paper-doubly.stderr"
          } > "$TMPDIR/paper.stderr"
          normalize_log "$TMPDIR/paper.stdout" "$output_dir/paper-sosp24-artifact.stdout"
          normalize_log "$TMPDIR/paper.stderr" "$output_dir/paper-sosp24-artifact.stderr"

          set +e
          timeout "$probe_timeout_seconds" octet-production-verus \
            --crate-type=dylib --expand-errors -C opt-level=3 \
            "$ironkv_source/ironsht/src/lib.rs" \
            > "$TMPDIR/ironkv.stdout" 2> "$TMPDIR/ironkv.stderr"
          ironkv_status="$?"
          set -e
          test "$ironkv_status" -eq 1
          ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
            grep -F '${anchor}' "$TMPDIR/ironkv.stderr" > /dev/null
          '') upgradeTrialRows.verified-ironkv.diagnostic_anchors}
          normalize_log "$TMPDIR/ironkv.stdout" "$output_dir/verified-ironkv.stdout"
          normalize_log "$TMPDIR/ironkv.stderr" "$output_dir/verified-ironkv.stderr"

          allocator_work="$TMPDIR/allocator-work"
          mkdir -p "$allocator_work/vendor" "$allocator_work/cargo-home" "$allocator_work/target"
          cp -R "$allocator_source/test_libc" "$allocator_work/test_libc"
          chmod -R u+w "$allocator_work/test_libc"
          tar --extract --gzip --file ${allocatorLibcCrate} --directory "$allocator_work/vendor"
          chmod -R u+w "$allocator_work/vendor"
          cat > "$allocator_work/vendor/libc-0.2.147/.cargo-checksum.json" <<'EOF'
          {"files":{},"package":"b4668fb0ea861c1df094127ac5f1da3409a82116a4ba74fca2e58ef927159bb3"}
          EOF
          cat > "$allocator_work/cargo-home/config.toml" <<EOF
          [source.crates-io]
          replace-with = "vendored-sources"
          [source.vendored-sources]
          directory = "$allocator_work/vendor"
          EOF
          candidate_cargo="$($candidate_root/rustup which cargo)"
          candidate_rustc="$($candidate_root/rustup which rustc)"
          CARGO_HOME="$allocator_work/cargo-home" RUSTC="$candidate_rustc" \
            "$candidate_cargo" build --release --locked --offline \
            --manifest-path "$allocator_work/test_libc/Cargo.toml" \
            --target-dir "$allocator_work/target" \
            > "$TMPDIR/allocator-cargo.stdout" 2> "$TMPDIR/allocator-cargo.stderr"
          allocator_libc_rlib="$(find "$allocator_work/target/release/deps" -name 'liblibc-*.rlib' -print -quit)"
          test -n "$allocator_libc_rlib"
          set +e
          (
            cd "$allocator_source/verus-mimalloc"
            timeout "$probe_timeout_seconds" octet-production-verus \
              --triggers-mode silent --no-auto-recommends-check --rlimit ${toString allocatorRlimit} \
              --extern libc="$allocator_libc_rlib" lib.rs --time -- -Zproc-macro-backtrace
          ) > "$TMPDIR/allocator.stdout" 2> "$TMPDIR/allocator.stderr"
          allocator_status="$?"
          set -e
          test "$allocator_status" -eq 1
          ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
            grep -F '${anchor}' "$TMPDIR/allocator.stderr" > /dev/null
          '') upgradeTrialRows.verified-memory-allocator.diagnostic_anchors}
          normalize_log "$TMPDIR/allocator.stdout" "$output_dir/verified-memory-allocator.stdout"
          normalize_log "$TMPDIR/allocator.stderr" "$output_dir/verified-memory-allocator.stderr"

          set +e
          timeout "$probe_timeout_seconds" octet-production-verus -V new-mut-ref --crate-type=lib \
            "$node_source/verified-node-replication/src/lib.rs" \
            > "$TMPDIR/node.stdout" 2> "$TMPDIR/node.stderr"
          node_status="$?"
          set -e
          test "$node_status" -eq 1
          ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
            grep -F '${anchor}' "$TMPDIR/node.stderr" > /dev/null
          '') upgradeTrialRows.verified-node-replication.diagnostic_anchors}
          normalize_log "$TMPDIR/node.stdout" "$output_dir/verified-node-replication.stdout"
          normalize_log "$TMPDIR/node.stderr" "$output_dir/verified-node-replication.stderr"

          grep -F 'This contains the Rocq development' "$verus_belt_source/README.md" > /dev/null
          grep -F 'Rocq (formerly Coq) 8.20.0' "$verus_belt_source/README.md" > /dev/null
          grep -F 'coq_makefile' "$verus_belt_source/Makefile" > /dev/null
          verus_belt_rocq_files="$(find "$verus_belt_source/src" -type f -name '*.v' | wc -l)"
          test "$verus_belt_rocq_files" -gt 0
          cat > "$output_dir/verus-belt-audit.txt" <<EOF
          status=reference-audit-only
          mechanism=rocq-semantic-reference
          rocq_source_files=$verus_belt_rocq_files
          candidate_verus_executed=false
          modeled_verus_snapshot=0.2026.03.16.b1c2a95
          trust-boundary=Rocq semantic validation is not a proof receipt for the Octet or Trellis verifier release
          EOF

          profile_blake3="$(file_blake3 "$octet_profile_json")"
          trial_matrix_blake3="$(file_blake3 "$TMPDIR/upgrade-trial.json")"
          corpus_catalog_blake3="$(file_blake3 "$corpus_dir/generated/catalog.json")"
          source_blake3="$(file_blake3 "$trellis_source/src/lib.rs")"
          baseline_identity_blake3="$(file_blake3 "$output_dir/baseline-identity.txt")"
          candidate_identity_blake3="$(file_blake3 "$output_dir/candidate-identity.txt")"
          baseline_stdout_blake3="$(file_blake3 "$output_dir/baseline.stdout")"
          baseline_stderr_blake3="$(file_blake3 "$output_dir/baseline.stderr")"
          candidate_stdout_blake3="$(file_blake3 "$output_dir/candidate.stdout")"
          candidate_stderr_blake3="$(file_blake3 "$output_dir/candidate.stderr")"
          paper_stdout_blake3="$(file_blake3 "$output_dir/paper-sosp24-artifact.stdout")"
          paper_stderr_blake3="$(file_blake3 "$output_dir/paper-sosp24-artifact.stderr")"
          ironkv_stdout_blake3="$(file_blake3 "$output_dir/verified-ironkv.stdout")"
          ironkv_stderr_blake3="$(file_blake3 "$output_dir/verified-ironkv.stderr")"
          allocator_stdout_blake3="$(file_blake3 "$output_dir/verified-memory-allocator.stdout")"
          allocator_stderr_blake3="$(file_blake3 "$output_dir/verified-memory-allocator.stderr")"
          node_stdout_blake3="$(file_blake3 "$output_dir/verified-node-replication.stdout")"
          node_stderr_blake3="$(file_blake3 "$output_dir/verified-node-replication.stderr")"
          verus_belt_audit_blake3="$(file_blake3 "$output_dir/verus-belt-audit.txt")"

          jq --null-input \
            --slurpfile matrix "$TMPDIR/upgrade-trial.json" \
            --slurpfile catalog "$corpus_dir/generated/catalog.json" \
            --arg profile_blake3 "$profile_blake3" \
            --arg trial_matrix_blake3 "$trial_matrix_blake3" \
            --arg corpus_catalog_blake3 "$corpus_catalog_blake3" \
            --arg execution_system '${system}' \
            --arg source_blake3 "$source_blake3" \
            --arg source_nix_input "$trellis_source" \
            --arg baseline_identity_blake3 "$baseline_identity_blake3" \
            --arg candidate_identity_blake3 "$candidate_identity_blake3" \
            --arg baseline_nix_output '${verus}' \
            --arg candidate_nix_output '${octetProductionVerus}' \
            --arg paper_source_nix_output "$paper_source" \
            --arg ironkv_source_nix_output "$ironkv_source" \
            --arg allocator_source_nix_output "$allocator_source" \
            --arg node_source_nix_output "$node_source" \
            --arg verus_belt_source_nix_output "$verus_belt_source" \
            --arg baseline_stdout_blake3 "$baseline_stdout_blake3" \
            --arg baseline_stderr_blake3 "$baseline_stderr_blake3" \
            --arg candidate_stdout_blake3 "$candidate_stdout_blake3" \
            --arg candidate_stderr_blake3 "$candidate_stderr_blake3" \
            --arg paper_stdout_blake3 "$paper_stdout_blake3" \
            --arg paper_stderr_blake3 "$paper_stderr_blake3" \
            --arg ironkv_stdout_blake3 "$ironkv_stdout_blake3" \
            --arg ironkv_stderr_blake3 "$ironkv_stderr_blake3" \
            --arg allocator_stdout_blake3 "$allocator_stdout_blake3" \
            --arg allocator_stderr_blake3 "$allocator_stderr_blake3" \
            --arg node_stdout_blake3 "$node_stdout_blake3" \
            --arg node_stderr_blake3 "$node_stderr_blake3" \
            --arg verus_belt_audit_blake3 "$verus_belt_audit_blake3" \
            --argjson baseline_exit "$baseline_status" \
            --argjson candidate_exit "$candidate_status" \
            --argjson paper_linked_exit "$paper_linked_status" \
            --argjson paper_doubly_exit "$paper_doubly_status" \
            --argjson ironkv_exit "$ironkv_status" \
            --argjson allocator_exit "$allocator_status" \
            --argjson node_exit "$node_status" \
            --argjson timeout_exit_code '${toString timeoutExitCode}' \
            --argjson verus_belt_rocq_files "$verus_belt_rocq_files" \
            '($baseline_exit == 0
              and $candidate_exit == 0
              and $ironkv_exit == 0
              and $allocator_exit == 0
              and $node_exit == 0) as $promotion_eligible
            | {
              payload_schema: "trellis-octet-verus-upgrade-trial-payload-v1",
              trial_id: $matrix[0].trial_id,
              claim_scope: $matrix[0].claim_scope,
              execution_system: $execution_system,
              trial_matrix_blake3: $trial_matrix_blake3,
              corpus_catalog_blake3: $corpus_catalog_blake3,
              execution_audit: {
                trellis_source_mutated: false,
                trellis_source_assumption_drift: "none-by-identical-immutable-source-input",
                verifier_library_assumption_drift: "not-evaluated",
                corpus_assumption_drift: "not-evaluated-probes-blocked-before-proof-completion",
                verifier_option_drift: false,
                trigger_changes: [],
                timeouts_observed: ([
                  $baseline_exit,
                  $candidate_exit,
                  $paper_linked_exit,
                  $paper_doubly_exit,
                  $ironkv_exit,
                  $allocator_exit,
                  $node_exit
                ] | any(. == $timeout_exit_code)),
                timeout_exit_code: $timeout_exit_code,
                trusted_boundary_transfer: false
              },
              baseline: ($matrix[0].baseline + {
                exit_code: $baseline_exit,
                source_blake3: $source_blake3,
                source_nix_input: $source_nix_input,
                identity_blake3: $baseline_identity_blake3,
                nix_output: $baseline_nix_output,
                stdout_blake3: $baseline_stdout_blake3,
                stderr_blake3: $baseline_stderr_blake3
              }),
              candidate: ($matrix[0].candidate + {
                exit_code: $candidate_exit,
                profile_blake3: $profile_blake3,
                source_blake3: $source_blake3,
                source_nix_input: $source_nix_input,
                identity_blake3: $candidate_identity_blake3,
                nix_output: $candidate_nix_output,
                stdout_blake3: $candidate_stdout_blake3,
                stderr_blake3: $candidate_stderr_blake3
              }),
              corpus_results: [
                {
                  id: "paper-sosp24-artifact",
                  mechanism: "artifact-microcase",
                  promotion_role: "diagnostic-only",
                  outcome: "source-api-incompatible",
                  source: (($catalog[0].entries[] | select(.id == "paper-sosp24-artifact")
                    | {revision, source_hash, source_hash_protocol})
                    + {nix_output: $paper_source_nix_output}),
                  exit_codes: [$paper_linked_exit, $paper_doubly_exit],
                  stdout_blake3: $paper_stdout_blake3,
                  stderr_blake3: $paper_stderr_blake3,
                  full_artifact_reproduced: false
                },
                {
                  id: "verified-ironkv",
                  mechanism: "native-verus",
                  promotion_role: "required-native",
                  outcome: "source-api-incompatible",
                  source: (($catalog[0].entries[] | select(.id == "verified-ironkv")
                    | {revision, source_hash, source_hash_protocol})
                    + {nix_output: $ironkv_source_nix_output}),
                  exit_code: $ironkv_exit,
                  stdout_blake3: $ironkv_stdout_blake3,
                  stderr_blake3: $ironkv_stderr_blake3
                },
                {
                  id: "verified-memory-allocator",
                  mechanism: "native-verus",
                  promotion_role: "required-native",
                  outcome: "source-api-incompatible",
                  source: (($catalog[0].entries[] | select(.id == "verified-memory-allocator")
                    | {revision, source_hash, source_hash_protocol})
                    + {nix_output: $allocator_source_nix_output}),
                  exit_code: $allocator_exit,
                  stdout_blake3: $allocator_stdout_blake3,
                  stderr_blake3: $allocator_stderr_blake3,
                  candidate_rust_libc_admitted: true,
                  build_dependency: {
                    crate: "libc",
                    version: "0.2.147",
                    fixed_output: "${allocatorLibcCrate}",
                    fixed_output_sha256: "sha256-tGaPsOqGHB3wlBJ6xfHaNAmoIRakunT8ouWO+ScVm7M="
                  }
                },
                {
                  id: "verified-node-replication",
                  mechanism: "native-verus",
                  promotion_role: "required-native",
                  outcome: "verifier-internal-error",
                  source: (($catalog[0].entries[] | select(.id == "verified-node-replication")
                    | {revision, source_hash, source_hash_protocol})
                    + {nix_output: $node_source_nix_output}),
                  exit_code: $node_exit,
                  stdout_blake3: $node_stdout_blake3,
                  stderr_blake3: $node_stderr_blake3
                },
                {
                  id: "verus-belt",
                  mechanism: "semantic-reference",
                  promotion_role: "audit-only",
                  outcome: "reference-audit-only",
                  source: (($catalog[0].entries[] | select(.id == "verus-belt")
                    | {revision, source_hash, source_hash_protocol})
                    + {nix_output: $verus_belt_source_nix_output}),
                  candidate_verus_executed: false,
                  rocq_source_files: $verus_belt_rocq_files,
                  audit_blake3: $verus_belt_audit_blake3
                }
              ],
              decision: ($matrix[0].decision + {
                observed: (if $promotion_eligible then "promote" else "blocked" end),
                promotion_eligible: $promotion_eligible,
                terminal_reason: (if $promotion_eligible
                  then "all-required-native-probes-passed"
                  else "required-native-probes-blocked"
                end)
              }),
              non_claims: $matrix[0].non_claims
            }' > "$output_dir/payload.json"
          jq --sort-keys --compact-output '.' "$output_dir/payload.json" \
            > "$output_dir/payload.canonical.json"
          receipt_blake3="$(file_blake3 "$output_dir/payload.canonical.json")"
          jq --null-input \
            --slurpfile payload "$output_dir/payload.json" \
            --arg receipt_blake3 "$receipt_blake3" \
            '{
              schema_version: "trellis-octet-verus-upgrade-trial-receipt-v1",
              hash_scope: "blake3-of-jq-sorted-compact-payload-json",
              payload: $payload[0],
              receipt_blake3: $receipt_blake3
            }' | jq --sort-keys '.' > "$output_dir/receipt.json"
          cp "$TMPDIR/upgrade-trial.json" "$output_dir/upgrade-trial.json"
          cp "$corpus_dir/generated/catalog.json" "$output_dir/catalog.json"
          cp "$octet_profile_json" "$output_dir/octet-profile.json"
          jq --exit-status '
            .payload.decision.expected == "blocked"
            and .payload.decision.observed == .payload.decision.expected
            and .payload.decision.promotion_eligible == false
            and .payload.decision.preserve_current_pin == true
            and .payload.decision.octet_compatibility_status == "local-pin"
            and .payload.baseline.expected_result == "passed"
            and .payload.candidate.expected_result == "proof-failure"
            and ([.payload.corpus_results[] | select(.promotion_role == "required-native") | .outcome]
              | all(. != "passed"))
          ' "$output_dir/receipt.json" > /dev/null
          ${pkgs.lib.optionalString (upgradeTrialSavedEvidence != null) ''
            diff --recursive --unified ${upgradeTrialSavedEvidence} "$output_dir"
          ''}
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
          verus-upgrade-trial = verusUpgradeTrial;
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
          verus-upgrade-trial = verusUpgradeTrial;

          # Verus formal verification
          verus-proofs = pkgs.runCommand "verus-proofs" {
            nativeBuildInputs = [ verus ];
          } ''
            verus-verify \
              --crate-type=lib \
              --extern vstd="${verus}/libvstd.rlib" \
              --extern builtin="${verus}/libverus_builtin.rlib" \
              --extern builtin_macros="${verus}/libverus_builtin_macros.so" \
              --extern state_machines_macros="${verus}/libverus_state_machines_macros.so" \
              -L "${verus}/lib" \
              --edition 2021 \
              ${verusProofSource}/src/lib.rs
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
