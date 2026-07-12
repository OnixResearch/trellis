let contracts = import "upgrade-trial-contracts.ncl" in
let probe_timeout_seconds_value = 600 in
let maximum_saved_log_bytes_value = 65536 in
{
  schema_version = "trellis-octet-verus-upgrade-trial/v1",
  trial_id = "trellis-octet-verus-fc38f593-upgrade-trial",
  baseline = {
    verifier = "verus@0.2026.03.22.5e66329",
    expected_result = "passed",
    expected_verified_count = 10710,
    expected_error_count = 0,
  },
  candidate = {
    provider = "octet",
    provider_revision = "fc38f59330b626961d166febfdf1a5aa6575460f",
    profile_id = "octet-verus-0.2026.04.03.21dfcd2",
    verifier = "verus@0.2026.04.03.21dfcd2",
    expected_result = "proof-failure",
    expected_verified_count = 10709,
    expected_error_count = 1,
    diagnostic_anchors = [
      "error: postcondition not satisfied",
      "src/nix_drv_serialize.rs:761:9",
    ],
  },
  rows = [
    {
      id = "paper-sosp24-artifact",
      source_revision = "b817b036fd9b873a58730af8c2defccb40b7ea83",
      mechanism = "artifact-microcase",
      promotion_role = "diagnostic-only",
      commands = [
        {
          id = "linked-list",
          argv = ["octet-production-verus", "--crate-type=lib", "milli/linked-list/verus.rs"],
        },
        {
          id = "doubly-linked-list",
          argv = ["octet-production-verus", "--crate-type=lib", "milli/doubly-linked-list/verus.rs"],
        },
      ],
      expected_outcome = "source-api-incompatible",
      diagnostic_anchors = [
        "loop must have a decreases clause",
        "unresolved import `vstd::ptr`",
      ],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "The two physically present Verus microcases are not the complete SOSP artifact or its external case studies.",
      ],
    },
    {
      id = "verified-ironkv",
      source_revision = "6b8830a47a2dd5a8ce141438ebb2decb6550298a",
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "host-proof-root",
        argv = ["octet-production-verus", "--crate-type=dylib", "--expand-errors", "-C", "opt-level=3", "ironsht/src/lib.rs"],
      }],
      expected_outcome = "source-api-incompatible",
      diagnostic_anchors = [
        "unresolved import `vstd::imap`",
        "unresolved import `vstd::iset_lib`",
      ],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "The probe covers the host proof root only and does not establish distributed refinement or crash recovery.",
      ],
    },
    {
      id = "verified-memory-allocator",
      source_revision = "847354efbb51a7999deb3eef0ed2bfc199bccac1",
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "allocator-proof-root",
        argv = ["octet-production-verus", "--extern", "libc=<candidate-rust-rlib>", "verus-mimalloc/lib.rs"],
      }],
      expected_outcome = "source-api-incompatible",
      diagnostic_anchors = [
        "cannot find function `ptr_mut_ref` in this scope",
        "vstd/raw_ptr.rs",
      ],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "Building the pinned libc rlib admits a verifier dependency only; it does not establish allocator integration or production readiness.",
      ],
    },
    {
      id = "verified-node-replication",
      source_revision = "eb8e27244a4f1b7714defe55aa495c6173a1c330",
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "required-new-mut-ref",
        argv = ["octet-production-verus", "-V", "new-mut-ref", "--crate-type=lib", "verified-node-replication/src/lib.rs"],
      }],
      expected_outcome = "verifier-internal-error",
      diagnostic_anchors = [
        "Verus Internal Error: var_local_id failed",
        "verified-node-replication/src/exec/rwlock.rs",
      ],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "Node replication remains a local NUMA technique and is not network replication or consensus.",
      ],
    },
    {
      id = "verus-belt",
      source_revision = "b116e9db550e84896674df781b93570c35a9f0da",
      mechanism = "semantic-reference",
      promotion_role = "audit-only",
      commands = [{
        id = "rocq-boundary-audit",
        argv = ["audit", "README.md", "Makefile", "src/**/*.v", "verus/rust-toolchain.toml"],
      }],
      expected_outcome = "reference-audit-only",
      diagnostic_anchors = [
        "This contains the Rocq development",
        "Rocq (formerly Coq) 8.20.0",
      ],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "Rocq validation is not candidate Verus execution and cannot certify a later Verus release.",
      ],
    },
  ],
  decision = {
    expected = "blocked",
    preserve_current_pin = true,
    octet_compatibility_status = "local-pin",
    blockers = [
      "trellis-proof-failure",
      "verified-ironkv-source-api-incompatible",
      "verified-memory-allocator-source-api-incompatible",
      "verified-node-replication-verifier-internal-error",
    ],
  },
  claim_scope = "bounded-verifier-upgrade-compatibility-trial",
  non_claims = [
    "The trial does not prove verifier, solver, compiler, Rocq, or upstream soundness.",
    "Source identity, formatting, diagnostic microcases, and reference audits are not proof receipts.",
    "A blocked trial does not authorize changing Trellis's verifier pin or Octet compatibility status.",
  ],
} | contracts.UpgradeTrial
