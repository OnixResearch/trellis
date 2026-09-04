{
  description = "Trellis — composable verified-logic building blocks with Verus proofs and Tracey coverage";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane/b556d7bbae5ff86e378451511873dfd07e4504cd";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    octet = {
      url = "github:OnixResearch/octet?rev=374bd16b26cee2af34211a29bfa531c016811f51";
      inputs.crane.follows = "crane";
    };
    # r[impl trellis.aeneas_cross_verifier.provider_consumption]
    octet-aeneas = {
      url = "github:OnixResearch/octet?rev=890f424cd7b1b2015f39c0eb5747f1bd1e576028";
      inputs.crane.follows = "crane";
    };
    trellis-consumer-release-src = {
      url = "git+ssh://git@github.com/OnixResearch/trellis.git?rev=1f03111b92cc36aade07359c96773e086908c95f";
      flake = false;
    };
    # r[impl trellis.nickel_toolchain.cohort]
    nickel-1-17-0.url = "github:nickel-lang/nickel/1320a983e6c3d1e2fb53dd2464b084b4903b1426";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      rust-overlay,
      flake-utils,
      octet,
      octet-aeneas,
      trellis-consumer-release-src,
      nickel-1-17-0,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        nickel = nickel-1-17-0.packages.${system}.default;

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Source filtering — include scripts/ and docs/ for verification checks
        src =
          let
            scriptFilter =
              path: _type:
              (builtins.match ".*\\.sh$" path) != null
              || (builtins.match ".*/scripts/.*\\.rs$" path) != null
              || (builtins.match ".*\\.md$" path) != null
              || (builtins.match ".*\\.tsv$" path) != null
              || (builtins.match ".*\\.styx$" path) != null
              || (builtins.match ".*\\.ncl$" path) != null
              || (builtins.match ".*/tests/fixtures/.*\\.v$" path) != null
              || (builtins.match ".*/tests/fixtures/.*\\.sv$" path) != null
              || (builtins.baseNameOf path == "dylint.toml")
              || (builtins.baseNameOf path == "LICENSE")
              || (builtins.baseNameOf path == "octet-baseline.json")
              || (builtins.baseNameOf path == "verusfmt-baseline.txt")
              || (builtins.match ".*/verification-policy/generated/.*\\.json$" path) != null
              || (builtins.match ".*/proof-review/generated/.*\\.json$" path) != null
              || (builtins.match ".*/cairn-policy/generated/.*\\.json$" path) != null
              || (builtins.match ".*/upstream-verus-corpus/generated/.*\\.json$" path) != null
              || (builtins.match ".*/consumer-profile/generated/.*\\.json$" path) != null
              || (builtins.match ".*/aeneas-pilot/.*\\.(json|lean|llbc)$" path) != null
              || (builtins.match ".*/docs/proof-evidence-bundle\\.json$" path) != null
              || (builtins.match ".*/docs/proof-artifact-manifest\\.json$" path) != null;
            isSessionOrLifecycleState = path: (builtins.match ".*/(\\.cairn|\\.pi)(/.*)?" path) != null;
            combinedFilter =
              path: type:
              !(isSessionOrLifecycleState path)
              && ((scriptFilter path type) || (craneLib.filterCargoSources path type));
          in
          pkgs.lib.cleanSourceWith {
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

        buildInputs =
          with pkgs;
          [
            openssl
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
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
          inherit
            src
            cargoArtifacts
            nativeBuildInputs
            buildInputs
            ;
        };

        # --- Verification tools ---

        # Historical verifier retained only as the comparison baseline in the
        # May 17 promotion receipt. Shipped proofs consume Octet's package below.
        baselineRustToolchain = pkgs.rust-bin.stable."1.94.0".default;

        baselineVerus = pkgs.stdenv.mkDerivation {
          pname = "verus";
          version = "0.2026.03.22";
          src = pkgs.fetchzip {
            url = "https://github.com/verus-lang/verus/releases/download/release/0.2026.03.22.5e66329/verus-0.2026.03.22.5e66329-x86-linux.zip";
            hash = "sha256-/rSsNizWruoGgAJciXt8oWUfLUqqs8vBhcSKqxWzhl8=";
            stripRoot = true;
          };
          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [
            pkgs.stdenv.cc.cc.lib
            pkgs.zlib
            baselineRustToolchain
          ];
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
            mkdir -p $out/bin
            cat > $out/bin/verus-verify <<WRAPPER
            #!/usr/bin/env bash
            export VERUS_Z3_PATH="$out/z3"
            export LD_LIBRARY_PATH="${baselineRustToolchain}/lib\''${LD_LIBRARY_PATH:+:\$LD_LIBRARY_PATH}"
            exec "$out/rust_verify" "\$@"
            WRAPPER
            chmod +x $out/bin/verus-verify
          '';
        };

        corpusCatalog = builtins.fromJSON (
          builtins.readFile ./upstream-verus-corpus/generated/catalog.json
        );
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
        octetSourceRevision = octet.rev or "unversioned";
        octetProductionVerus = builtins.getAttr "octet-production-verus" octetPackages;
        verusfmt = builtins.getAttr "octet-verusfmt" octetPackages;
        verusToolchainProfile = builtins.getAttr "octet-verus-toolchain-profile" octetPackages;
        octetAeneasPackages = octet-aeneas.packages.${system};
        octetAeneasSource = octet-aeneas.outPath;
        octetAeneasProvider = builtins.getAttr "octet-aeneas-lean-provider-closure" octetAeneasPackages;
        octetAeneasCargoOctet = octetAeneasPackages.cargo-octet;
        octetAeneasPolicyArtifacts = octetAeneasPackages.octet-policy-artifacts;
        aeneasTimeoutSeconds = 900;
        aeneasRetainedLogBytes = 1048576;
        aeneasFileBlockBytes = 512;
        aeneasRetainedArtifactBytes = 8388608;
        upgradeTrialProfile = builtins.fromJSON (
          builtins.readFile ./upstream-verus-corpus/generated/upgrade-trial.json
        );
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
        trellisRlimit = 20;
        upgradeTrialCorpusRowCount = 5;
        upgradeTrialAdaptedRowCount = 2;
        timeoutExitCode = 124;
        upgradeTrialSavedEvidence = ./cairn/archive/2026-07-12-repair-octet-verus-upgrade/evidence/trial;

        upstreamVerusCorpus =
          pkgs.runCommand "trellis-upstream-verus-corpus"
            {
              nativeBuildInputs = with pkgs; [
                b3sum
                diffutils
                jq
                nickel
              ];
            }
            ''
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
              ${pkgs.lib.concatMapStringsSep "\n" (
                entry:
                let
                  source = corpusSources.${entry.id};
                in
                pkgs.lib.concatMapStringsSep "\n" (sentinel: ''
                  test -e ${source}/${sentinel}
                '') entry.sentinels
              ) corpusCatalog.entries}
              ${pkgs.lib.concatMapStringsSep "\n" (
                entry:
                let
                  source = corpusSources.${entry.id};
                in
                ''
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

        verusfmtRatchetCheck =
          pkgs.runCommand "trellis-verusfmt-ratchet"
            {
              nativeBuildInputs =
                with pkgs;
                [
                  diffutils
                  findutils
                  jq
                ]
                ++ [
                  verusfmt
                  verusToolchainProfile
                ];
            }
            ''
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
                  and .profile_id == "octet-verus-0.2026.05.17.e479cce"
                  and .verifier.source_revision == "e479cce36490b8fa4b0fd7755aa742aec354372c"
                  and .formatter.version == "0.7.2"
                  and any(.compatibility[];
                    .consumer == "trellis"
                    and .status == "${upgradeTrialProfile.decision.octet_compatibility_status}"
                    and .verifier_release == "${
                      if upgradeTrialProfile.decision.octet_compatibility_status == "native" then
                        "0.2026.05.17.e479cce"
                      else
                        "0.2026.03.22.5e66329"
                    }")
                ' "$1" > /dev/null
              }
              validate_octet_profile "$profile_json"
              jq '(.compatibility[] | select(.consumer == "trellis")).status = "${
                if upgradeTrialProfile.decision.octet_compatibility_status == "native" then
                  "local-pin"
                else
                  "native"
              }"' \
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
              grep -F 'formatter-profile: octet-verus-0.2026.05.17.e479cce' "$TMPDIR/formatter-identity.txt" > /dev/null
              grep -F 'formatter: verusfmt@0.7.2' "$TMPDIR/formatter-identity.txt" > /dev/null
              mkdir -p "$out"
            '';

        verusUpgradeTrial =
          pkgs.runCommand "trellis-octet-verus-upgrade-trial"
            {
              nativeBuildInputs =
                (with pkgs; [
                  b3sum
                  clang
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
                  patch
                  stdenv.cc
                ])
                ++ [
                  baselineVerus
                  octetProductionVerus
                  verusToolchainProfile
                ];
            }
            ''
              set -eu
              corpus_dir=${./upstream-verus-corpus}
              trellis_source=${verusProofSource}
              paper_source=${corpusSources.paper-sosp24-artifact}
              ironkv_source=${corpusSources.verified-ironkv}
              allocator_source=${corpusSources.verified-memory-allocator}
              node_source=${corpusSources.verified-node-replication}
              verus_belt_source=${corpusSources.verus-belt}
              ironkv_patch=${./upstream-verus-corpus/adaptations/0.2026.05.17.e479cce/verified-ironkv.patch}
              allocator_patch=${./upstream-verus-corpus/adaptations/0.2026.05.17.e479cce/verified-memory-allocator.patch}
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
                and .decision.expected == "promote"
                and .decision.preserve_current_pin == false
                and .decision.recommended_octet_compatibility_status == "native"
                and (.decision.blockers | length == 0)
                and (.rows | length == ${toString upgradeTrialCorpusRowCount})
                and ([.rows[].id] == ([.rows[].id] | sort))
                and ([.rows[] | select(.promotion_role == "required-native") | .expected_outcome]
                  | all(. == "passed-adapted" or . == "passed-unmodified"))
                and ([.rows[] | select(.source_variant == "adapted")] | length == ${toString upgradeTrialAdaptedRowCount})
              ' "$TMPDIR/upgrade-trial.json" > /dev/null
              test '${octetSourceRevision}' = '${upgradeTrialProfile.candidate.provider_revision}'
              jq --exit-status '
                .profile_id == "octet-verus-0.2026.05.17.e479cce"
                and .verifier.release == "0.2026.05.17.e479cce"
                and .verifier.source_revision == "e479cce36490b8fa4b0fd7755aa742aec354372c"
                and any(.compatibility[];
                  .consumer == "trellis"
                  and .status == "${upgradeTrialProfile.decision.octet_compatibility_status}")
              ' "$octet_profile_json" > /dev/null
              cd "$TMPDIR"
              printf '%s\n' 'upgrade-trial: profile and typed matrix accepted'

              normalize_log() {
                input="$1"
                output="$2"
                normalized_log="$TMPDIR/normalized.log"
                sed \
                  -e "s#$TMPDIR#<TRIAL_TMP>#g" \
                  -e "s#$trellis_source#<TRELLIS_SOURCE>#g" \
                  -e "s#$paper_source#<PAPER_ARTIFACT_SOURCE>#g" \
                  -e "s#$ironkv_source#<IRONKV_SOURCE>#g" \
                  -e "s#$allocator_source#<ALLOCATOR_SOURCE>#g" \
                  -e "s#$node_source#<NODE_REPLICATION_SOURCE>#g" \
                  -e "s#$verus_belt_source#<VERUS_BELT_SOURCE>#g" \
                  -E -e "s/thread 'rustc' \([0-9]+\)/thread 'rustc' (<PID>)/" \
                  -E -e 's/[0-9]+ ms/<DURATION_MS>/g' \
                  -E -e 's/: +<DURATION_MS>/: <DURATION_MS>/g' \
                  -E -e 's/in [0-9]+([.][0-9]+)?s/in <DURATION_S>/g' \
                  -E -e 's/[[:space:]]+$//' \
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
              tree_blake3() {
                root="$1"
                manifest="$TMPDIR/tree-blake3.manifest"
                : > "$manifest"
                find "$root" -type f -print0 | LC_ALL=C sort --zero-terminated | while IFS= read -r -d "" file; do
                  relative="$(realpath --relative-to="$root" "$file")"
                  printf '%s\t%s\n' "$relative" "$(file_blake3 "$file")"
                done > "$manifest"
                file_blake3 "$manifest"
              }
              marker_count() {
                root="$1"
                pattern="$2"
                (grep --recursive --binary-files=without-match --extended-regexp --only-matching \
                  "$pattern" "$root" 2>/dev/null || true) | wc --lines | tr --delete ' '
              }
              require_patch_hash() {
                patch_path="$1"
                expected="$2"
                test "$(file_blake3 "$patch_path")" = "$expected"
              }

              baseline_binary_blake3="$(file_blake3 ${baselineVerus}/rust_verify)"
              baseline_solver_blake3="$(file_blake3 ${baselineVerus}/z3)"
              cat > "$output_dir/baseline-identity.txt" <<EOF
              proof-verifier: ${upgradeTrialProfile.baseline.verifier}
              proof-verifier-binary-blake3: $baseline_binary_blake3
              proof-solver-blake3: $baseline_solver_blake3
              proof-verifier-nix-output: ${baselineVerus}
              trust-boundary: baseline identity and execution do not establish verifier or solver soundness
              EOF
              octet-production-verus --identity > "$output_dir/candidate-identity.txt"
              grep -F 'proof-verifier: ${upgradeTrialProfile.candidate.verifier}' "$output_dir/candidate-identity.txt" > /dev/null
              grep -F 'proof-verifier-source-revision: e479cce36490b8fa4b0fd7755aa742aec354372c' "$output_dir/candidate-identity.txt" > /dev/null

              trellis_rlimit='${toString trellisRlimit}'
              set +e
              timeout "$probe_timeout_seconds" verus-verify \
                --triggers-mode silent \
                --rlimit "$trellis_rlimit" \
                --crate-type=lib \
                --extern vstd="${baselineVerus}/libvstd.rlib" \
                --extern builtin="${baselineVerus}/libverus_builtin.rlib" \
                --extern builtin_macros="${baselineVerus}/libverus_builtin_macros.so" \
                --extern state_machines_macros="${baselineVerus}/libverus_state_machines_macros.so" \
                -L "${baselineVerus}/lib" \
                --edition 2021 \
                "$trellis_source/src/lib.rs" \
                > "$TMPDIR/baseline.stdout" 2> "$TMPDIR/baseline.stderr"
              baseline_status="$?"
              timeout "$probe_timeout_seconds" octet-production-verus \
                --triggers-mode silent \
                --rlimit "$trellis_rlimit" \
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
              test "$candidate_status" -eq 0
              grep -Fx 'verification results:: ${toString upgradeTrialProfile.baseline.expected_verified_count} verified, ${toString upgradeTrialProfile.baseline.expected_error_count} errors' "$TMPDIR/baseline.stdout" > /dev/null
              grep -Fx 'verification results:: ${toString upgradeTrialProfile.candidate.expected_verified_count} verified, ${toString upgradeTrialProfile.candidate.expected_error_count} errors' "$TMPDIR/candidate.stdout" > /dev/null
              ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
                grep -F '${anchor}' "$TMPDIR/candidate.stdout" > /dev/null
              '') upgradeTrialProfile.candidate.result_anchors}
              normalize_log "$TMPDIR/baseline.stdout" "$output_dir/baseline.stdout"
              normalize_log "$TMPDIR/baseline.stderr" "$output_dir/baseline.stderr"
              normalize_log "$TMPDIR/candidate.stdout" "$output_dir/candidate.stdout"
              normalize_log "$TMPDIR/candidate.stderr" "$output_dir/candidate.stderr"
              printf '%s\n' 'upgrade-trial: baseline and candidate Trellis proofs passed'

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
              test "$paper_linked_status" -eq '${toString (builtins.elemAt upgradeTrialRows.paper-sosp24-artifact.expected_exit_codes 0)}'
              test "$paper_doubly_status" -eq '${toString (builtins.elemAt upgradeTrialRows.paper-sosp24-artifact.expected_exit_codes 1)}'
              ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
                grep -F '${anchor}' "$TMPDIR/paper-linked.stderr" "$TMPDIR/paper-doubly.stderr" > /dev/null
              '') upgradeTrialRows.paper-sosp24-artifact.result_anchors}
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
              printf '%s\n' 'upgrade-trial: diagnostic-only paper microcases classified'

              ironkv_patch_expected='${upgradeTrialRows.verified-ironkv.adaptation.patch_blake3}'
              allocator_patch_expected='${upgradeTrialRows.verified-memory-allocator.adaptation.patch_blake3}'
              require_patch_hash "$ironkv_patch" "$ironkv_patch_expected"
              require_patch_hash "$allocator_patch" "$allocator_patch_expected"
              if grep --extended-regexp '^\+[^+].*(assume[[:space:]]*\(|admit[[:space:]]*\(|external_body|with_triggers|#\[trigger\])' "$ironkv_patch" "$allocator_patch" > /dev/null; then
                echo "compatibility patch adds a forbidden assumption, body, or trigger marker" >&2
                exit 1
              fi
              cp "$ironkv_patch" "$TMPDIR/mutated-ironkv.patch"
              chmod u+w "$TMPDIR/mutated-ironkv.patch"
              printf '\n' >> "$TMPDIR/mutated-ironkv.patch"
              if require_patch_hash "$TMPDIR/mutated-ironkv.patch" "$ironkv_patch_expected"; then
                echo "mutated IronKV patch unexpectedly retained its identity" >&2
                exit 1
              fi
              cat > "$output_dir/patch-hash-negative.txt" <<EOF
              mutated_patch=verified-ironkv.patch+newline
              expected_result=rejected
              identity_algorithm=blake3
              EOF
              printf '%s\n' 'upgrade-trial: patch identities and mutation rejection passed'

              ironkv_adapted="$TMPDIR/verified-ironkv-adapted"
              cp -R "$ironkv_source" "$ironkv_adapted"
              chmod -R u+w "$ironkv_adapted"
              ironkv_assume_before="$(marker_count "$ironkv_source/ironsht/src" 'assume[[:space:]]*\(')"
              ironkv_external_before="$(marker_count "$ironkv_source/ironsht/src" 'external_body')"
              ironkv_trusted_before="$(marker_count "$ironkv_source/ironsht/src" 'trusted')"
              patch --batch --forward --silent --directory "$ironkv_adapted/ironsht" --strip 1 < "$ironkv_patch"
              ironkv_assume_after="$(marker_count "$ironkv_adapted/ironsht/src" 'assume[[:space:]]*\(')"
              ironkv_external_after="$(marker_count "$ironkv_adapted/ironsht/src" 'external_body')"
              ironkv_trusted_after="$(marker_count "$ironkv_adapted/ironsht/src" 'trusted')"
              test "$ironkv_assume_after" -eq "$ironkv_assume_before"
              test "$ironkv_external_after" -eq "$ironkv_external_before"
              test "$ironkv_trusted_after" -eq "$ironkv_trusted_before"
              ironkv_adapted_tree_blake3="$(tree_blake3 "$ironkv_adapted")"
              printf '%s\n' 'upgrade-trial: IronKV patch applied and trust inventory preserved'
              cat > "$output_dir/verified-ironkv-adaptation.txt" <<EOF
              result-scope: adapted-source-only
              upstream-revision: ${upgradeTrialRows.verified-ironkv.source_revision}
              patch-blake3: $ironkv_patch_expected
              adapted-tree-blake3: $ironkv_adapted_tree_blake3
              assume-sites-before: $ironkv_assume_before
              assume-sites-after: $ironkv_assume_after
              external-bodies-before: $ironkv_external_before
              external-bodies-after: $ironkv_external_after
              trusted-markers-before: $ironkv_trusted_before
              trusted-markers-after: $ironkv_trusted_after
              EOF
              set +e
              (
                cd "$ironkv_adapted/ironsht"
                timeout "$probe_timeout_seconds" octet-production-verus \
                  --crate-type=dylib --expand-errors -C opt-level=3 src/lib.rs
              ) > "$TMPDIR/ironkv.stdout" 2> "$TMPDIR/ironkv.stderr"
              ironkv_status="$?"
              set -e
              if ! test "$ironkv_status" -eq '${toString (builtins.elemAt upgradeTrialRows.verified-ironkv.expected_exit_codes 0)}'; then
                echo "adapted IronKV exited $ironkv_status" >&2
                cat "$TMPDIR/ironkv.stdout" >&2
                cat "$TMPDIR/ironkv.stderr" >&2
                exit 1
              fi
              printf 'upgrade-trial: adapted IronKV exited %s\n' "$ironkv_status"
              ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
                grep -F '${anchor}' "$TMPDIR/ironkv.stdout" > /dev/null
              '') upgradeTrialRows.verified-ironkv.result_anchors}
              normalize_log "$TMPDIR/ironkv.stdout" "$output_dir/verified-ironkv.stdout"
              normalize_log "$TMPDIR/ironkv.stderr" "$output_dir/verified-ironkv.stderr"
              printf '%s\n' 'upgrade-trial: adapted IronKV proof passed'

              allocator_adapted="$TMPDIR/verified-memory-allocator-adapted"
              cp -R "$allocator_source" "$allocator_adapted"
              chmod -R u+w "$allocator_adapted"
              allocator_assume_before="$(marker_count "$allocator_source/verus-mimalloc" 'assume[[:space:]]*\(')"
              allocator_external_before="$(marker_count "$allocator_source/verus-mimalloc" 'external_body')"
              allocator_trusted_before="$(marker_count "$allocator_source/verus-mimalloc" 'trusted')"
              patch --batch --forward --silent --directory "$allocator_adapted" --strip 1 < "$allocator_patch"
              allocator_assume_after="$(marker_count "$allocator_adapted/verus-mimalloc" 'assume[[:space:]]*\(')"
              allocator_external_after="$(marker_count "$allocator_adapted/verus-mimalloc" 'external_body')"
              allocator_trusted_after="$(marker_count "$allocator_adapted/verus-mimalloc" 'trusted')"
              test "$allocator_assume_after" -eq "$allocator_assume_before"
              test "$allocator_external_after" -eq "$allocator_external_before"
              test "$allocator_trusted_after" -eq "$allocator_trusted_before"
              allocator_adapted_tree_blake3="$(tree_blake3 "$allocator_adapted")"
              printf '%s\n' 'upgrade-trial: allocator patch applied and trust inventory preserved'
              cat > "$output_dir/verified-memory-allocator-adaptation.txt" <<EOF
              result-scope: adapted-source-only
              upstream-revision: ${upgradeTrialRows.verified-memory-allocator.source_revision}
              patch-blake3: $allocator_patch_expected
              adapted-tree-blake3: $allocator_adapted_tree_blake3
              assume-sites-before: $allocator_assume_before
              assume-sites-after: $allocator_assume_after
              external-bodies-before: $allocator_external_before
              external-bodies-after: $allocator_external_after
              trusted-markers-before: $allocator_trusted_before
              trusted-markers-after: $allocator_trusted_after
              EOF

              allocator_work="$TMPDIR/allocator-work"
              mkdir -p "$allocator_work/vendor" "$allocator_work/cargo-home" "$allocator_work/target"
              cp -R "$allocator_adapted/test_libc" "$allocator_work/test_libc"
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
                cd "$allocator_adapted/verus-mimalloc"
                timeout "$probe_timeout_seconds" octet-production-verus \
                  --triggers-mode silent --no-auto-recommends-check --rlimit ${toString allocatorRlimit} \
                  --extern libc="$allocator_libc_rlib" lib.rs --time -- -Zproc-macro-backtrace
              ) > "$TMPDIR/allocator.stdout" 2> "$TMPDIR/allocator.stderr"
              allocator_status="$?"
              set -e
              if ! test "$allocator_status" -eq '${toString (builtins.elemAt upgradeTrialRows.verified-memory-allocator.expected_exit_codes 0)}'; then
                echo "adapted allocator exited $allocator_status" >&2
                cat "$TMPDIR/allocator.stdout" >&2
                cat "$TMPDIR/allocator.stderr" >&2
                exit 1
              fi
              printf 'upgrade-trial: adapted allocator exited %s\n' "$allocator_status"
              ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
                grep -F '${anchor}' "$TMPDIR/allocator.stdout" > /dev/null
              '') upgradeTrialRows.verified-memory-allocator.result_anchors}
              normalize_log "$TMPDIR/allocator-cargo.stdout" "$output_dir/verified-memory-allocator-cargo.stdout"
              normalize_log "$TMPDIR/allocator-cargo.stderr" "$output_dir/verified-memory-allocator-cargo.stderr"
              normalize_log "$TMPDIR/allocator.stdout" "$output_dir/verified-memory-allocator.stdout"
              normalize_log "$TMPDIR/allocator.stderr" "$output_dir/verified-memory-allocator.stderr"
              printf '%s\n' 'upgrade-trial: adapted allocator proof passed'

              set +e
              timeout "$probe_timeout_seconds" octet-production-verus --crate-type=lib \
                "$node_source/verified-node-replication/src/lib.rs" \
                > "$TMPDIR/node.stdout" 2> "$TMPDIR/node.stderr"
              node_status="$?"
              set -e
              if ! test "$node_status" -eq '${toString (builtins.elemAt upgradeTrialRows.verified-node-replication.expected_exit_codes 0)}'; then
                echo "unmodified node replication exited $node_status" >&2
                cat "$TMPDIR/node.stdout" >&2
                cat "$TMPDIR/node.stderr" >&2
                exit 1
              fi
              printf 'upgrade-trial: unmodified node replication exited %s\n' "$node_status"
              ${pkgs.lib.concatMapStringsSep "\n" (anchor: ''
                grep -F '${anchor}' "$TMPDIR/node.stdout" > /dev/null
              '') upgradeTrialRows.verified-node-replication.result_anchors}
              normalize_log "$TMPDIR/node.stdout" "$output_dir/verified-node-replication.stdout"
              normalize_log "$TMPDIR/node.stderr" "$output_dir/verified-node-replication.stderr"
              printf '%s\n' 'upgrade-trial: unmodified node-replication proof passed'

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

              mkdir -p "$output_dir/adaptations"
              cp "$ironkv_patch" "$output_dir/adaptations/verified-ironkv.patch"
              cp "$allocator_patch" "$output_dir/adaptations/verified-memory-allocator.patch"
              profile_blake3="$(file_blake3 "$octet_profile_json")"
              trial_matrix_blake3="$(file_blake3 "$TMPDIR/upgrade-trial.json")"
              corpus_catalog_blake3="$(file_blake3 "$corpus_dir/generated/catalog.json")"
              source_blake3="$(tree_blake3 "$trellis_source/src")"
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
              ironkv_adaptation_audit_blake3="$(file_blake3 "$output_dir/verified-ironkv-adaptation.txt")"
              allocator_cargo_stdout_blake3="$(file_blake3 "$output_dir/verified-memory-allocator-cargo.stdout")"
              allocator_cargo_stderr_blake3="$(file_blake3 "$output_dir/verified-memory-allocator-cargo.stderr")"
              allocator_stdout_blake3="$(file_blake3 "$output_dir/verified-memory-allocator.stdout")"
              allocator_stderr_blake3="$(file_blake3 "$output_dir/verified-memory-allocator.stderr")"
              allocator_adaptation_audit_blake3="$(file_blake3 "$output_dir/verified-memory-allocator-adaptation.txt")"
              node_stdout_blake3="$(file_blake3 "$output_dir/verified-node-replication.stdout")"
              node_stderr_blake3="$(file_blake3 "$output_dir/verified-node-replication.stderr")"
              verus_belt_audit_blake3="$(file_blake3 "$output_dir/verus-belt-audit.txt")"
              patch_hash_negative_blake3="$(file_blake3 "$output_dir/patch-hash-negative.txt")"

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
                --arg baseline_nix_output '${baselineVerus}' \
                --arg candidate_nix_output '${octetProductionVerus}' \
                --arg provider_source_revision '${octetSourceRevision}' \
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
                --arg ironkv_patch_blake3 "$ironkv_patch_expected" \
                --arg ironkv_adapted_tree_blake3 "$ironkv_adapted_tree_blake3" \
                --arg ironkv_adaptation_audit_blake3 "$ironkv_adaptation_audit_blake3" \
                --arg ironkv_stdout_blake3 "$ironkv_stdout_blake3" \
                --arg ironkv_stderr_blake3 "$ironkv_stderr_blake3" \
                --arg allocator_patch_blake3 "$allocator_patch_expected" \
                --arg allocator_adapted_tree_blake3 "$allocator_adapted_tree_blake3" \
                --arg allocator_adaptation_audit_blake3 "$allocator_adaptation_audit_blake3" \
                --arg allocator_cargo_stdout_blake3 "$allocator_cargo_stdout_blake3" \
                --arg allocator_cargo_stderr_blake3 "$allocator_cargo_stderr_blake3" \
                --arg allocator_stdout_blake3 "$allocator_stdout_blake3" \
                --arg allocator_stderr_blake3 "$allocator_stderr_blake3" \
                --arg node_stdout_blake3 "$node_stdout_blake3" \
                --arg node_stderr_blake3 "$node_stderr_blake3" \
                --arg verus_belt_audit_blake3 "$verus_belt_audit_blake3" \
                --arg patch_hash_negative_blake3 "$patch_hash_negative_blake3" \
                --argjson baseline_exit "$baseline_status" \
                --argjson candidate_exit "$candidate_status" \
                --argjson paper_linked_exit "$paper_linked_status" \
                --argjson paper_doubly_exit "$paper_doubly_status" \
                --argjson ironkv_exit "$ironkv_status" \
                --argjson allocator_exit "$allocator_status" \
                --argjson node_exit "$node_status" \
                --argjson ironkv_assume_before "$ironkv_assume_before" \
                --argjson ironkv_assume_after "$ironkv_assume_after" \
                --argjson ironkv_external_before "$ironkv_external_before" \
                --argjson ironkv_external_after "$ironkv_external_after" \
                --argjson ironkv_trusted_before "$ironkv_trusted_before" \
                --argjson ironkv_trusted_after "$ironkv_trusted_after" \
                --argjson allocator_assume_before "$allocator_assume_before" \
                --argjson allocator_assume_after "$allocator_assume_after" \
                --argjson allocator_external_before "$allocator_external_before" \
                --argjson allocator_external_after "$allocator_external_after" \
                --argjson allocator_trusted_before "$allocator_trusted_before" \
                --argjson allocator_trusted_after "$allocator_trusted_after" \
                --argjson timeout_exit_code '${toString timeoutExitCode}' \
                --argjson verus_belt_rocq_files "$verus_belt_rocq_files" \
                'def row($id): $matrix[0].rows[] | select(.id == $id);
                ($baseline_exit == 0
                  and $candidate_exit == 0
                  and $ironkv_exit == 0
                  and $allocator_exit == 0
                  and $node_exit == 0) as $promotion_eligible
                | {
                  payload_schema: "trellis-octet-verus-upgrade-trial-payload-v2",
                  trial_id: $matrix[0].trial_id,
                  claim_scope: $matrix[0].claim_scope,
                  execution_system: $execution_system,
                  trial_matrix_blake3: $trial_matrix_blake3,
                  corpus_catalog_blake3: $corpus_catalog_blake3,
                  execution_audit: {
                    trellis_source_mutated: false,
                    trellis_source_assumption_drift: "none-by-identical-immutable-source-input",
                    verifier_library_assumption_drift: "not-claimed",
                    corpus_assumption_drift: "adapted-source-marker-counts-unchanged",
                    verifier_option_drift: false,
                    trigger_changes: [],
                    patch_hash_negative_rejected: true,
                    patch_hash_negative_blake3: $patch_hash_negative_blake3,
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
                    source_hash_scope: "blake3-of-sorted-relative-path-and-file-blake3-manifest",
                    source_nix_input: $source_nix_input,
                    identity_blake3: $baseline_identity_blake3,
                    nix_output: $baseline_nix_output,
                    stdout_blake3: $baseline_stdout_blake3,
                    stderr_blake3: $baseline_stderr_blake3
                  }),
                  candidate: ($matrix[0].candidate + {
                    exit_code: $candidate_exit,
                    profile_blake3: $profile_blake3,
                    provider_source_revision: $provider_source_revision,
                    source_blake3: $source_blake3,
                    source_hash_scope: "blake3-of-sorted-relative-path-and-file-blake3-manifest",
                    source_nix_input: $source_nix_input,
                    identity_blake3: $candidate_identity_blake3,
                    nix_output: $candidate_nix_output,
                    stdout_blake3: $candidate_stdout_blake3,
                    stderr_blake3: $candidate_stderr_blake3
                  }),
                  corpus_results: [
                    {
                      id: "paper-sosp24-artifact",
                      mechanism: row("paper-sosp24-artifact").mechanism,
                      promotion_role: row("paper-sosp24-artifact").promotion_role,
                      source_variant: row("paper-sosp24-artifact").source_variant,
                      outcome: row("paper-sosp24-artifact").expected_outcome,
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
                      mechanism: row("verified-ironkv").mechanism,
                      promotion_role: row("verified-ironkv").promotion_role,
                      source_variant: row("verified-ironkv").source_variant,
                      outcome: row("verified-ironkv").expected_outcome,
                      source: (($catalog[0].entries[] | select(.id == "verified-ironkv")
                        | {revision, source_hash, source_hash_protocol})
                        + {nix_output: $ironkv_source_nix_output}),
                      adaptation: (row("verified-ironkv").adaptation + {
                        result_scope: "adapted-source-only",
                        patch_blake3: $ironkv_patch_blake3,
                        adapted_tree_blake3: $ironkv_adapted_tree_blake3,
                        audit_blake3: $ironkv_adaptation_audit_blake3,
                        trust_inventory: {
                          assume_sites: {before: $ironkv_assume_before, after: $ironkv_assume_after},
                          external_bodies: {before: $ironkv_external_before, after: $ironkv_external_after},
                          trusted_markers: {before: $ironkv_trusted_before, after: $ironkv_trusted_after}
                        }
                      }),
                      exit_code: $ironkv_exit,
                      stdout_blake3: $ironkv_stdout_blake3,
                      stderr_blake3: $ironkv_stderr_blake3
                    },
                    {
                      id: "verified-memory-allocator",
                      mechanism: row("verified-memory-allocator").mechanism,
                      promotion_role: row("verified-memory-allocator").promotion_role,
                      source_variant: row("verified-memory-allocator").source_variant,
                      outcome: row("verified-memory-allocator").expected_outcome,
                      source: (($catalog[0].entries[] | select(.id == "verified-memory-allocator")
                        | {revision, source_hash, source_hash_protocol})
                        + {nix_output: $allocator_source_nix_output}),
                      adaptation: (row("verified-memory-allocator").adaptation + {
                        result_scope: "adapted-source-only",
                        patch_blake3: $allocator_patch_blake3,
                        adapted_tree_blake3: $allocator_adapted_tree_blake3,
                        audit_blake3: $allocator_adaptation_audit_blake3,
                        trust_inventory: {
                          assume_sites: {before: $allocator_assume_before, after: $allocator_assume_after},
                          external_bodies: {before: $allocator_external_before, after: $allocator_external_after},
                          trusted_markers: {before: $allocator_trusted_before, after: $allocator_trusted_after}
                        }
                      }),
                      exit_code: $allocator_exit,
                      stdout_blake3: $allocator_stdout_blake3,
                      stderr_blake3: $allocator_stderr_blake3,
                      candidate_rust_libc_admitted: true,
                      cargo_stdout_blake3: $allocator_cargo_stdout_blake3,
                      cargo_stderr_blake3: $allocator_cargo_stderr_blake3,
                      build_dependency: {
                        crate: "libc",
                        version: "0.2.147",
                        fixed_output: "${allocatorLibcCrate}",
                        fixed_output_sha256: "sha256-tGaPsOqGHB3wlBJ6xfHaNAmoIRakunT8ouWO+ScVm7M="
                      }
                    },
                    {
                      id: "verified-node-replication",
                      mechanism: row("verified-node-replication").mechanism,
                      promotion_role: row("verified-node-replication").promotion_role,
                      source_variant: row("verified-node-replication").source_variant,
                      outcome: row("verified-node-replication").expected_outcome,
                      source: (($catalog[0].entries[] | select(.id == "verified-node-replication")
                        | {revision, source_hash, source_hash_protocol})
                        + {nix_output: $node_source_nix_output}),
                      exit_code: $node_exit,
                      stdout_blake3: $node_stdout_blake3,
                      stderr_blake3: $node_stderr_blake3
                    },
                    {
                      id: "verus-belt",
                      mechanism: row("verus-belt").mechanism,
                      promotion_role: row("verus-belt").promotion_role,
                      source_variant: row("verus-belt").source_variant,
                      outcome: row("verus-belt").expected_outcome,
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
                      then "all-complete-and-required-native-proofs-passed"
                      else "complete-or-required-native-proof-blocked"
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
                  schema_version: "trellis-octet-verus-upgrade-trial-receipt-v2",
                  hash_scope: "blake3-of-jq-sorted-compact-payload-json",
                  payload: $payload[0],
                  receipt_blake3: $receipt_blake3
                }' | jq --sort-keys '.' > "$output_dir/receipt.json"
              cp "$TMPDIR/upgrade-trial.json" "$output_dir/upgrade-trial.json"
              cp "$corpus_dir/generated/catalog.json" "$output_dir/catalog.json"
              cp "$octet_profile_json" "$output_dir/octet-profile.json"
              jq --exit-status '
                .payload.decision.expected == "promote"
                and .payload.decision.observed == .payload.decision.expected
                and .payload.decision.promotion_eligible == true
                and .payload.decision.preserve_current_pin == false
                and .payload.decision.recommended_octet_compatibility_status == "native"
                and (.payload.decision.blockers | length == 0)
                and .payload.baseline.expected_result == "passed"
                and .payload.candidate.expected_result == "passed"
                and .payload.candidate.provider_revision == .payload.candidate.provider_source_revision
                and ([.payload.corpus_results[] | select(.promotion_role == "required-native") | .outcome]
                  | all(. == "passed-adapted" or . == "passed-unmodified"))
                and ([.payload.corpus_results[] | select(.source_variant == "adapted")
                  | .adaptation.trust_inventory
                  | (.assume_sites.before == .assume_sites.after)
                    and (.external_bodies.before == .external_bodies.after)
                    and (.trusted_markers.before == .trusted_markers.after)]
                  | all)
                and .payload.execution_audit.patch_hash_negative_rejected == true
                and .payload.execution_audit.timeouts_observed == false
                and .payload.execution_audit.trusted_boundary_transfer == false
              ' "$output_dir/receipt.json" > /dev/null
              printf '%s\n' 'upgrade-trial: canonical promotion receipt validated'
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
          nickel-1-17-0 = nickel;
          default = verified-logic;
          verified-logic = verified-logic;
          verus = octetProductionVerus;
          octet-production-verus = octetProductionVerus;
          verusfmt = verusfmt;
          upstream-verus-corpus = upstreamVerusCorpus;
          verus-upgrade-trial = verusUpgradeTrial;
          octet-aeneas-provider = octetAeneasProvider;
        };

        checks = {
          inherit verified-logic;

          # r[verify trellis.nickel_toolchain.cohort]
          nickel-cohort-exact =
            pkgs.runCommand "trellis-nickel-cohort-exact"
              {
                nativeBuildInputs = [ nickel ];
              }
              ''
                test '${nickel-1-17-0.rev}' = '1320a983e6c3d1e2fb53dd2464b084b4903b1426'
                test "$(command -v nickel)" = '${nickel}/bin/nickel'
                test "$(nickel --version)" = 'nickel-lang-cli nickel 1.17.0 (rev 1320a98)'
                if grep -q 'pkgs\.nickel' ${self}/flake.nix; then
                  echo 'older, floating, ambient, or mixed Nickel evaluator reference found' >&2
                  exit 1
                fi
                mkdir -p "$out"
              '';

          # r[verify trellis.nickel_toolchain.compatibility]
          # r[verify trellis.nickel_toolchain.evidence]
          nickel-toolchain-fixtures =
            pkgs.runCommand "trellis-nickel-toolchain-fixtures"
              {
                nativeBuildInputs = [
                  nickel
                  pkgs.diffutils
                ];
              }
              ''
                fixtures=${src}/verification-policy/nickel-toolchain
                nickel export --format json ${src}/verification-policy/nickel-evaluator.ncl \
                  > evaluator.json
                diff --unified ${src}/verification-policy/generated/nickel-evaluator.json evaluator.json
                for fixture in "$fixtures"/positive/*.ncl; do
                  nickel typecheck "$fixture"
                  nickel export --format json "$fixture" > /dev/null
                done
                for fixture in "$fixtures"/negative/*.ncl; do
                  if nickel eval "$fixture" >negative.out 2>negative.err; then
                    echo "negative Nickel fixture unexpectedly passed: $fixture" >&2
                    exit 1
                  fi
                  test -s negative.err
                done
                mkdir -p "$out"
              '';

          # Cargo tests with nextest
          nextest = craneLib.cargoNextest {
            inherit
              src
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              ;
            partitions = 1;
            partitionType = "count";
          };

          # Clippy lints
          clippy = craneLib.cargoClippy {
            inherit
              src
              cargoArtifacts
              nativeBuildInputs
              buildInputs
              ;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          };

          # Format check
          fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Octet/Tiger Style lints, shared with the local pre-commit hook.
          # Existing findings are accepted in octet-baseline.json; new stable IDs fail.
          octet =
            pkgs.runCommand "trellis-octet-baseline-check"
              {
                nativeBuildInputs = nativeBuildInputs ++ [ octet.packages.${system}.cargo-octet ];
                inherit buildInputs;
              }
              ''
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
          check-coverage =
            pkgs.runCommand "check-coverage"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
                cd ${src}
                export TMPDIR=$(mktemp -d)
                cp scripts/check-coverage.rs "$TMPDIR/check-coverage.rs"
                rustc "$TMPDIR/check-coverage.rs" -o "$TMPDIR/check-coverage"
                "$TMPDIR/check-coverage"
                touch $out
              '';

          # Every pub fn in verus! has ensures
          check-ensures =
            pkgs.runCommand "check-ensures"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
                cd ${src}
                export TMPDIR=$(mktemp -d)
                cp scripts/check-ensures.rs "$TMPDIR/check-ensures.rs"
                rustc "$TMPDIR/check-ensures.rs" -o "$TMPDIR/check-ensures"
                "$TMPDIR/check-ensures"
                touch $out
              '';

          # Fixture-backed regression tests for checker scripts
          checker-fixtures =
            pkgs.runCommand "checker-fixtures"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
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
          verification-policy =
            pkgs.runCommand "verification-policy"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                  nickel
                ];
              }
              ''
                cd ${src}
                export TMPDIR=$(mktemp -d)
                cp scripts/check-verification-policy.rs "$TMPDIR/check-verification-policy.rs"
                rustc "$TMPDIR/check-verification-policy.rs" -o "$TMPDIR/check-verification-policy"
                "$TMPDIR/check-verification-policy"
                touch $out
              '';

          # Trellis-owned direct dependency and vendored snapshot release profile.
          consumer-release-profile =
            pkgs.runCommand "consumer-release-profile"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                  pkgs.mold
                  nickel
                  pkgs.b3sum
                ];
              }
              ''
                cd ${src}
                export HOME="$TMPDIR/home"
                export CARGO_HOME="$TMPDIR/cargo-home"
                export CARGO_TARGET_DIR="$TMPDIR/cargo-target"
                mkdir -p "$HOME" "$CARGO_HOME" "$CARGO_TARGET_DIR"
                cp "${octetConsumerVendorDir}/config.toml" "$CARGO_HOME/config.toml"
                checker_dir="$TMPDIR/consumer-profile-checker"
                mkdir -p "$checker_dir/src"
                cp scripts/check-verified-logic-consumer-profile.rs "$checker_dir/src/main.rs"
                cat > "$checker_dir/Cargo.toml" <<'EOF'
                [package]
                name = "trellis-consumer-profile-checker"
                version = "0.1.0"
                edition = "2024"

                [dependencies]
                serde = { version = "1", features = ["derive"] }
                serde_json = "1"
                EOF
                cargo run --quiet --offline --manifest-path "$checker_dir/Cargo.toml" -- . \
                  --source-root ${trellis-consumer-release-src} \
                  --source-revision ${trellis-consumer-release-src.rev}
                cargo run --quiet --offline --manifest-path "$checker_dir/Cargo.toml" -- --self-test
                touch $out
              '';

          # README module catalog drift
          readme-module-catalog =
            pkgs.runCommand "readme-module-catalog"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
                cd ${src}
                export TMPDIR=$(mktemp -d)
                cp scripts/check-readme-module-catalog.rs "$TMPDIR/check-readme-module-catalog.rs"
                rustc "$TMPDIR/check-readme-module-catalog.rs" -o "$TMPDIR/check-readme-module-catalog"
                "$TMPDIR/check-readme-module-catalog"
                "$TMPDIR/check-readme-module-catalog" --self-test
                touch $out
              '';

          # README headline metrics static drift
          readme-metrics =
            pkgs.runCommand "readme-metrics"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
                cd ${src}
                export TMPDIR=$(mktemp -d)
                cp scripts/check-readme-metrics.rs "$TMPDIR/check-readme-metrics.rs"
                rustc "$TMPDIR/check-readme-metrics.rs" -o "$TMPDIR/check-readme-metrics"
                "$TMPDIR/check-readme-metrics"
                "$TMPDIR/check-readme-metrics" --self-test
                touch $out
              '';

          # Proof-gap inventory exact-site, classification, and rationale drift
          proof-gap-inventory =
            pkgs.runCommand "proof-gap-inventory"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.clang
                ];
              }
              ''
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

          # r[verify trellis.aeneas_cross_verifier.provider_consumption]
          # r[verify trellis.aeneas_cross_verifier.exact_source]
          # r[verify trellis.aeneas_cross_verifier.translation_manifest]
          # r[verify trellis.aeneas_cross_verifier.feasibility]
          # r[verify trellis.aeneas_cross_verifier.generated_boundary]
          # r[verify trellis.aeneas_cross_verifier.theorem_map]
          # r[verify trellis.aeneas_cross_verifier.theorem_adequacy]
          # r[verify trellis.aeneas_cross_verifier.resource_budget]
          # r[verify trellis.aeneas_cross_verifier.manifest]
          # r[verify trellis.aeneas_cross_verifier.negative_evidence]
          # r[verify trellis.aeneas_cross_verifier.claim_boundary]
          # r[verify trellis.aeneas_cross_verifier.final_validation]
          aeneas-cross-verifier-pilot =
            pkgs.runCommand "trellis-aeneas-cross-verifier-pilot"
              {
                nativeBuildInputs = [
                  rustToolchain
                  pkgs.b3sum
                  pkgs.clang
                  pkgs.coreutils
                  pkgs.diffutils
                  pkgs.findutils
                  pkgs.gnugrep
                  pkgs.jq
                  pkgs.mold
                  nickel
                  octetAeneasCargoOctet
                  octetAeneasProvider
                ];
                inherit buildInputs;
              }
              ''
                set -euo pipefail
                source_root="$PWD/source"
                work_root="$TMPDIR/aeneas-work"
                logs="$TMPDIR/logs"
                provider_closure='${octetAeneasProvider}'
                provider="$provider_closure/share/octet/aeneas-lean"
                timeout_seconds='${toString aeneasTimeoutSeconds}'
                retained_log_bytes='${toString aeneasRetainedLogBytes}'
                retained_capture_bytes="$((retained_log_bytes + 1))"
                retained_artifact_bytes='${toString aeneasRetainedArtifactBytes}'
                octet_policy_artifacts_dir="$TMPDIR/octet-policy-artifacts"
                mkdir -p "$source_root" "$work_root" "$logs"
                '${octetAeneasPolicyArtifacts}/bin/octet-policy-artifacts' "$octet_policy_artifacts_dir"
                cp -R ${src}/. "$source_root/"
                chmod -R u+w "$source_root" "$work_root"

                test '${octet-aeneas.rev}' = '890f424cd7b1b2015f39c0eb5747f1bd1e576028'
                test "$(b3sum '${octetAeneasSource}/standards/proof-rails/aeneas-lean/consumer-contracts.ncl' | cut -d' ' -f1)" \
                  = '63ea1e9e78916728a4a83719c29a6443a8952dbaadeeaae2a7420365afa7ff07'
                test "$(b3sum '${octetAeneasSource}/standards/proof-rails/aeneas-lean/generated/profile.json' | cut -d' ' -f1)" \
                  = 'cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625'
                test "$(b3sum '${octetAeneasSource}/standards/proof-rails/aeneas-lean/generated/compatibility-matrix.json' | cut -d' ' -f1)" \
                  = '403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5'
                test "$(b3sum "$octet_policy_artifacts_dir/octet-policies.json" | cut -d' ' -f1)" \
                  = 'cc07811243f39a70c08d80b2d145fbc97596644d56b74284d5f870d93826f605'

                cd "$source_root"
                nickel export --format json aeneas-pilot/resource-policy.ncl > "$TMPDIR/resource-policy.json"
                diff --unified aeneas-pilot/generated/resource-policy.json "$TMPDIR/resource-policy.json"
                nickel export --format json aeneas-pilot/negative-cases.ncl > "$TMPDIR/negative-cases.json"
                diff --unified aeneas-pilot/generated/negative-cases.json "$TMPDIR/negative-cases.json"
                nickel export --format json aeneas-pilot/profile.ncl > "$TMPDIR/profile.json"
                diff --unified aeneas-pilot/generated/profile.json "$TMPDIR/profile.json"
                cat > "$TMPDIR/octet-consumer-wrapper.ncl" <<EOF
                let contracts = import "${octetAeneasSource}/standards/proof-rails/aeneas-lean/consumer-contracts.ncl" in
                (import "$source_root/aeneas-pilot/profile.ncl").consumer | contracts.AeneasConsumerProfile | contracts.AeneasConsumerProfileRules
                EOF
                nickel export --format json "$TMPDIR/octet-consumer-wrapper.ncl" > "$TMPDIR/octet-consumer.json"
                jq --sort-keys '.consumer' aeneas-pilot/generated/profile.json > "$TMPDIR/consumer-from-profile.json"
                jq --sort-keys '.' "$TMPDIR/octet-consumer.json" > "$TMPDIR/consumer-from-octet.json"
                diff --unified "$TMPDIR/consumer-from-profile.json" "$TMPDIR/consumer-from-octet.json"

                "$provider_closure/bin/octet-aeneas-lean" --identity > "$TMPDIR/provider-identity.json"
                jq --exit-status '
                  .schema_version == "octet-aeneas-lean-provider-closure/v1"
                  and .status == "packaged"
                  and .profile_blake3 == "cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625"
                  and .aeneas_revision == "bc2eb6f941369fa3c695e30598b5758b11e4f0de"
                  and .charon_revision == "cb50ff16b9f1066b8a97dc06da704de2da2fa41c"
                  and .aeneas_binary_blake3 == "428ce05d0f32c3bc3eceb24c1ead80b0dc742bfda22cba9c4b91b6a7f40a3819"
                  and .charon_binary_blake3 == "64ee129c4553574e1686d87560af0699dba9010c998dd68fe4a6b25f6b2e64b5"
                  and .lean_binary_blake3 == "e9e24ff8e9efd3e7f7a692d2411e0bb39316b626aeda1c3f1bce21bf21daea5d"
                  and .lake_binary_blake3 == "9c0b74bac6d9436481da3be2f158f9abfb76b940a18aa723d2c493b29ac9f2b5"
                  and .leantar_binary_blake3 == "b9f350bec0f839bb2e6157fdac2c8cc877f26a961ce81b16e5bcc97957d257fe"
                  and .network_sandbox_blake3 == "c87d8bcc11f0c5d61e2598dbc08b801d7e1220753ba47b543525835bc1288b6b"
                ' "$TMPDIR/provider-identity.json" > /dev/null

                checker_dir="$TMPDIR/checker"
                mkdir -p "$checker_dir/src/aeneas_cross_verifier" "$TMPDIR/cargo-home" "$TMPDIR/cargo-target"
                cp scripts/check-aeneas-cross-verifier.rs "$checker_dir/src/main.rs"
                cp scripts/aeneas_cross_verifier/*.rs "$checker_dir/src/aeneas_cross_verifier/"
                cp '${octetConsumerVendorDir}/config.toml' "$TMPDIR/cargo-home/config.toml"
                cat > "$checker_dir/Cargo.toml" <<'EOF'
                [package]
                name = "trellis-aeneas-cross-verifier-checker"
                version = "0.1.0"
                edition = "2024"
                publish = false

                [dependencies]
                blake3 = "1"
                serde = { version = "1", features = ["derive"] }
                serde_json = "1"
                EOF
                export CARGO_HOME="$TMPDIR/cargo-home"
                export CARGO_TARGET_DIR="$TMPDIR/cargo-target"
                cargo clippy --quiet --offline --manifest-path "$checker_dir/Cargo.toml" -- -D warnings
                cargo build --quiet --offline --manifest-path "$checker_dir/Cargo.toml"
                checker="$CARGO_TARGET_DIR/debug/trellis-aeneas-cross-verifier-checker"
                "$checker" --root "$source_root" --check
                "$checker" --root "$source_root" --self-test

                cp -R "$source_root/." "$work_root/"
                mkdir -p "$work_root/.cargo-home"
                cp '${octetConsumerVendorDir}/config.toml' "$work_root/.cargo-home/config.toml"
                sandbox=(
                  "$provider/bwrap"
                  --die-with-parent
                  --unshare-net
                  --dev /dev
                  --proc /proc
                  --dir /tmp
                  --ro-bind /nix/store /nix/store
                  --bind "$work_root" "$work_root"
                  --chdir "$work_root"
                  --
                )
                environment=(
                  '${pkgs.coreutils}/bin/env'
                  -i
                  "HOME=$work_root"
                  "CARGO_HOME=$work_root/.cargo-home"
                  "CARGO_TARGET_DIR=$work_root/target"
                  'PATH=${
                    pkgs.lib.makeBinPath [
                      rustToolchain
                      pkgs.clang
                      pkgs.mold
                      pkgs.coreutils
                      pkgs.findutils
                      pkgs.gnugrep
                    ]
                  }'
                  'CHARON_TOOLCHAIN_IS_IN_PATH=1'
                  'MATHLIB_NO_CACHE_ON_UPDATE=1'
                )

                check_log_bounds() {
                  local path="$1"
                  local bytes
                  bytes="$(wc -c < "$path")"
                  if [ "$bytes" -gt "$retained_log_bytes" ]; then
                    echo "retained log exceeded $retained_log_bytes bytes: $path" >&2
                    return 1
                  fi
                }

                run_stage() {
                  local stage="$1"
                  shift
                  local status
                  set +e
                  timeout --signal=KILL "$timeout_seconds" "$@" \
                    > >(head -c "$retained_capture_bytes" > "$logs/$stage.stdout") \
                    2> >(head -c "$retained_capture_bytes" > "$logs/$stage.stderr")
                  status="$?"
                  wait
                  set -e
                  if [ "$status" -ne 0 ]; then
                    echo "stage $stage failed with status $status" >&2
                    cat "$logs/$stage.stderr" >&2
                    return 1
                  fi
                  check_log_bounds "$logs/$stage.stdout"
                  check_log_bounds "$logs/$stage.stderr"
                }

                run_expected_failure() {
                  local stage="$1"
                  shift
                  local status
                  set +e
                  timeout --signal=KILL "$timeout_seconds" "$@" \
                    > >(head -c "$retained_capture_bytes" > "$logs/$stage.stdout") \
                    2> >(head -c "$retained_capture_bytes" > "$logs/$stage.stderr")
                  status="$?"
                  wait
                  set -e
                  if [ "$status" -eq 0 ] || [ "$status" -eq '${toString timeoutExitCode}' ]; then
                    echo "negative stage $stage returned invalid status $status" >&2
                    return 1
                  fi
                  check_log_bounds "$logs/$stage.stdout"
                  check_log_bounds "$logs/$stage.stderr"
                }

                cd "$work_root"
                run_stage charon \
                  "''${sandbox[@]}" "''${environment[@]}" \
                  "$provider/charon" cargo \
                  --preset aeneas \
                  --dest-file llbc.llbc \
                  --start-from 'crate::arithmetic::abs_diff' \
                  -- --lib
                test "$(wc -c < llbc.llbc)" -le "$retained_artifact_bytes"
                diff --unified aeneas-pilot/generated/llbc.llbc llbc.llbc

                run_stage aeneas \
                  "''${sandbox[@]}" "''${environment[@]}" \
                  "$provider/aeneas" \
                  -backend lean \
                  -emit-json \
                  -dest . \
                  -no-progress-bar \
                  llbc.llbc
                test "$(wc -c < translation.json)" -le "$retained_artifact_bytes"
                test "$(wc -c < Llbc.lean)" -le "$retained_artifact_bytes"
                diff --unified aeneas-pilot/generated/translation.json translation.json
                diff --unified aeneas-pilot/generated/Llbc.lean Llbc.lean

                cp -R "$provider/aeneas-lean-library" aeneas-lean-library
                chmod -R u+w aeneas-lean-library
                mkdir -p .lake
                cp "$provider/lake-overrides.json" .lake/package-overrides.json
                cp "$provider/lake-manifest.json" lake-manifest.json
                cp "$provider/aeneas-lean-library/lean-toolchain" lean-toolchain
                cp aeneas-pilot/theorems/AbsDiffSymmetric.lean AbsDiffSymmetric.lean
                cp aeneas-pilot/theorems/AbsDiffSelfZero.lean AbsDiffSelfZero.lean
                cp aeneas-pilot/theorems/AxiomAudit.lean AxiomAudit.lean
                cp aeneas-pilot/fixtures/negative/FalseTheorem.lean FalseTheorem.lean
                cp aeneas-pilot/fixtures/negative/VacuousTheorem.lean VacuousTheorem.lean
                cat > lakefile.lean <<'EOF'
                import Lake
                open Lake DSL
                package trellis_aeneas_pilot
                require aeneas from "./aeneas-lean-library"
                @[default_target]
                lean_lib TrellisAeneasPilot where
                  roots := #[`Llbc, `AbsDiffSymmetric, `AbsDiffSelfZero, `AxiomAudit]
                EOF
                run_stage lean-positive \
                  "''${sandbox[@]}" "''${environment[@]}" \
                  "$provider/lake" build
                grep --fixed-strings "'trellis_abs_diff_symmetric' depends on axioms: [propext]" \
                  "$logs/lean-positive.stdout" > /dev/null
                grep --fixed-strings "'trellis_abs_diff_self_is_zero' depends on axioms: [propext, Classical.choice, Quot.sound]" \
                  "$logs/lean-positive.stdout" > /dev/null
                run_expected_failure lean-false \
                  "''${sandbox[@]}" "''${environment[@]}" \
                  "$provider/lake" env lean FalseTheorem.lean
                grep --fixed-strings 'error: unsolved goals' "$logs/lean-false.stdout" > /dev/null
                run_stage lean-vacuous \
                  "''${sandbox[@]}" "''${environment[@]}" \
                  "$provider/lake" env lean VacuousTheorem.lean

                cd "$source_root"
                vacuous_policy="$TMPDIR/vacuous-policy"
                cp -R "$octet_policy_artifacts_dir" "$vacuous_policy"
                chmod -R u+w "$vacuous_policy"
                theorem_hash="$(b3sum aeneas-pilot/fixtures/negative/VacuousTheorem.lean | cut -d' ' -f1)"
                jq --arg theorem_hash "$theorem_hash" '
                  .policies += [
                    (.policies[] | select(.policy_id == "octet.aeneas_consumer_contract.external-fixture")
                      | .policy_id = "trellis.aeneas_cross_verifier.vacuous"
                      | .summary = "Trellis vacuous theorem rejection probe"
                      | .proof_rails[0].source_paths = ["src/arithmetic.rs"]
                      | .proof_rails[0].proof_paths = ["aeneas-pilot/fixtures/negative/VacuousTheorem.lean"]
                      | .proof_rails[0].theorem_id = "trellis.abs-diff.vacuous-negative"
                      | .proof_rails[0].approved_theorem_statement_blake3 = $theorem_hash
                      | .proof_rails[0].checked_scope = "formal-proof:negative:trellis.aeneas_cross_verifier.vacuous#rail-0")
                  ]
                ' "$octet_policy_artifacts_dir/octet-policies.json" > "$vacuous_policy/octet-policies.json"
                policy_bytes="$(wc -c < "$vacuous_policy/octet-policies.json")"
                policy_hash="$(b3sum "$vacuous_policy/octet-policies.json" | cut -d' ' -f1)"
                jq --argjson bytes "$policy_bytes" --arg hash "$policy_hash" '
                  (.artifacts[] | select(.path == "octet-policies.json") | .bytes) = $bytes
                  | (.artifacts[] | select(.path == "octet-policies.json") | .blake3) = $hash
                ' "$octet_policy_artifacts_dir/octet-artifact-manifest.json" \
                  > "$vacuous_policy/octet-artifact-manifest.json"
                set +e
                env \
                  OCTET_AENEAS_CHARON="$provider/charon" \
                  OCTET_AENEAS_AENEAS="$provider/aeneas" \
                  OCTET_AENEAS_LEAN="$provider/lake" \
                  OCTET_AENEAS_LEAN_LIBRARY="$provider/aeneas-lean-library" \
                  OCTET_AENEAS_NETWORK_SANDBOX="$provider/bwrap" \
                  '${octetAeneasCargoOctet}/bin/cargo-octet' prove \
                    --policy-artifacts-dir "$vacuous_policy" \
                    --source src/arithmetic.rs \
                    --rail 'trellis.aeneas_cross_verifier.vacuous#rail-0' \
                    --artifact-dir "$TMPDIR/vacuous-artifacts" \
                    --output-format json > "$logs/octet-vacuous.json" 2> "$logs/octet-vacuous.stderr"
                vacuous_status="$?"
                set -e
                test "$vacuous_status" -ne 0
                jq --exit-status '.diagnostics | index("Aeneas theorem adequacy rejected vacuous theorem") != null' \
                  "$logs/octet-vacuous.json" > /dev/null

                '${octetAeneasCargoOctet}/bin/cargo-octet' artifact verify \
                  --artifact-dir '${octetAeneasSource}/standards/proof-rails/aeneas-lean/fixtures/valid/artifacts' \
                  > "$logs/octet-artifact-verify.log"

                "$checker" --root "$source_root" --check
                "$checker" --root "$source_root" --self-test
                mkdir -p "$out/logs"
                cp -R "$logs/." "$out/logs/"
                cp "$TMPDIR/provider-identity.json" "$out/provider-identity.json"
                cp "$source_root/aeneas-pilot/generated/aeneas-proof-receipt.json" "$out/aeneas-proof-receipt.json"
                cp "$source_root/aeneas-pilot/generated/cross-verifier-manifest.json" "$out/cross-verifier-manifest.json"
              '';

          # Pinned upstream source intake and exact formatter migration ratchet
          upstream-verus-corpus = upstreamVerusCorpus;
          verusfmt-ratchet = verusfmtRatchetCheck;
          verus-upgrade-trial = verusUpgradeTrial;

          # Verus formal verification through Octet's published production package.
          verus-proofs =
            pkgs.runCommand "verus-proofs"
              {
                nativeBuildInputs = [ octetProductionVerus ];
              }
              ''
                candidate_root=${octetProductionVerus}/libexec/verus
                octet-production-verus \
                  --triggers-mode silent \
                  --rlimit ${toString trellisRlimit} \
                  --crate-type=lib \
                  --extern vstd="$candidate_root/libvstd.rlib" \
                  --extern builtin="$candidate_root/libverus_builtin.rlib" \
                  --extern builtin_macros="$candidate_root/libverus_builtin_macros.so" \
                  --extern state_machines_macros="$candidate_root/libverus_state_machines_macros.so" \
                  -L "$candidate_root" \
                  --edition 2021 \
                  ${verusProofSource}/src/lib.rs
                touch $out
              '';

          # Tracey requirement coverage
          tracey-coverage =
            pkgs.runCommand "tracey-coverage"
              {
                nativeBuildInputs = [ tracey ];
              }
              ''
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

          packages =
            (with pkgs; [
              cargo-nextest
              cargo-watch
              rust-analyzer
              steel
              pre-commit
              nickel
            ])
            ++ [ verusfmt ];

          inputsFrom = [ verified-logic ];
        };
      }
    );
}
