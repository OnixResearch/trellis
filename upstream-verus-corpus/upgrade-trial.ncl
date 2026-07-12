let contracts = import "upgrade-trial-contracts.ncl" in
let probe_timeout_seconds_value = 600 in
let maximum_saved_log_bytes_value = 65536 in
{
  schema_version = "trellis-octet-verus-upgrade-trial/v1",
  trial_id = "trellis-octet-verus-may17-upgrade-trial",
  baseline = {
    verifier = "verus@0.2026.03.22.5e66329",
    expected_result = "passed",
    expected_verified_count = 10712,
    expected_error_count = 0,
  },
  candidate = {
    provider = "octet",
    provider_revision = "374bd16b26cee2af34211a29bfa531c016811f51",
    profile_id = "octet-verus-0.2026.05.17.e479cce",
    verifier = "verus@0.2026.05.17.e479cce",
    expected_result = "passed",
    expected_verified_count = 10712,
    expected_error_count = 0,
    result_anchors = ["verification results:: 10712 verified, 0 errors"],
  },
  rows = [
    {
      id = "paper-sosp24-artifact",
      source_revision = "b817b036fd9b873a58730af8c2defccb40b7ea83",
      source_variant = "unmodified",
      adaptation = {
        patch_path = "none",
        patch_blake3 = "none",
        semantic_audit = [],
      },
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
      expected_exit_codes = [1, 1],
      result_anchors = [
        "to dereference a mutable reference parameter in a postcondition",
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
      source_variant = "adapted",
      adaptation = {
        patch_path = "upstream-verus-corpus/adaptations/0.2026.05.17.e479cce/verified-ironkv.patch",
        patch_blake3 = "cedf509e48f6e83431f4884521fad46a8cd6be7908f28ef8de9b493a77f57937",
        semantic_audit = [
          "Verus PR 2486 names ISet and IMap as the old possibly-infinite Set and Map semantics introduced after the candidate release.",
          "The patch maps those post-split names and constructors back to the candidate's generalized extensional collections.",
        ],
      },
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "adapted-host-proof-root",
        argv = ["octet-production-verus", "--crate-type=dylib", "--expand-errors", "-C", "opt-level=3", "ironsht/src/lib.rs"],
      }],
      expected_outcome = "passed-adapted",
      expected_exit_codes = [0],
      result_anchors = ["verification results:: 312 verified, 0 errors"],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "The result covers the hash-bound adapted host proof root only and does not certify the unmodified revision, distributed refinement, or crash recovery.",
      ],
    },
    {
      id = "verified-memory-allocator",
      source_revision = "847354efbb51a7999deb3eef0ed2bfc199bccac1",
      source_variant = "adapted",
      adaptation = {
        patch_path = "upstream-verus-corpus/adaptations/0.2026.05.17.e479cce/verified-memory-allocator.patch",
        patch_blake3 = "399027aba3425050a667a60194767c1bd5e84af7f6c93bc5941b6a34f47b3f5b",
        semantic_audit = [
          "The patch preserves arithmetic lemma signatures and contracts while replacing the unavailable optional Singular tactic with explicit proofs.",
          "The segment-size repair proves the existing postcondition without changing executable behavior or trusted boundaries.",
        ],
      },
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "adapted-allocator-proof-root",
        argv = ["octet-production-verus", "--extern", "libc=<candidate-rust-rlib>", "verus-mimalloc/lib.rs"],
      }],
      expected_outcome = "passed-adapted",
      expected_exit_codes = [0],
      result_anchors = ["verification results:: 732 verified, 0 errors"],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "Building the pinned libc rlib and verifying adapted source do not establish allocator integration or production readiness.",
      ],
    },
    {
      id = "verified-node-replication",
      source_revision = "eb8e27244a4f1b7714defe55aa495c6173a1c330",
      source_variant = "unmodified",
      adaptation = {
        patch_path = "none",
        patch_blake3 = "none",
        semantic_audit = [],
      },
      mechanism = "native-verus",
      promotion_role = "required-native",
      commands = [{
        id = "native-proof-root",
        argv = ["octet-production-verus", "--crate-type=lib", "verified-node-replication/src/lib.rs"],
      }],
      expected_outcome = "passed-unmodified",
      expected_exit_codes = [0],
      result_anchors = ["verification results:: 254 verified, 0 errors"],
      timeout_seconds = probe_timeout_seconds_value,
      maximum_saved_log_bytes = maximum_saved_log_bytes_value,
      non_claims = [
        "Node replication remains a local NUMA technique and is not network replication or consensus.",
      ],
    },
    {
      id = "verus-belt",
      source_revision = "b116e9db550e84896674df781b93570c35a9f0da",
      source_variant = "reference-only",
      adaptation = {
        patch_path = "none",
        patch_blake3 = "none",
        semantic_audit = [],
      },
      mechanism = "semantic-reference",
      promotion_role = "audit-only",
      commands = [{
        id = "rocq-boundary-audit",
        argv = ["audit", "README.md", "Makefile", "src/**/*.v", "verus/rust-toolchain.toml"],
      }],
      expected_outcome = "reference-audit-only",
      expected_exit_codes = [0],
      result_anchors = [
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
    expected = "promote",
    preserve_current_pin = false,
    octet_compatibility_status = "native",
    recommended_octet_compatibility_status = "native",
    blockers = [],
  },
  claim_scope = "bounded-verifier-upgrade-compatibility-trial",
  non_claims = [
    "The trial does not prove verifier, solver, compiler, Vstd, Rocq, or upstream soundness.",
    "Source identity, formatting, diagnostic microcases, and reference audits are not proof receipts.",
    "Adapted-source passes bind their patches and do not certify the unmodified upstream revisions or authorize production adoption.",
    "Promotion changes only the reviewed Trellis verifier relationship; downstream proof and release authority remain local.",
    "The exact published Octet GitHub input binds the committed provider tree; package identity still does not transfer proof authority.",
  ],
} | contracts.UpgradeTrial
