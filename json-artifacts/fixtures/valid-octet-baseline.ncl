let Contracts = import "../contracts.ncl" in
{
  schema_version = 1,
  finding_schema_version = 2,
  metadata = {
    tool_name = "cargo-octet",
    tool_version = "0.1.0",
    rustc_version = "rustc fixture",
    toolchain = "nightly-fixture",
    profile_name = "workspace-metadata",
    profile_hash = "b3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    config_hash = "b3:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
  },
  baseline_hash = "b3:1111111111111111111111111111111111111111111111111111111111111111",
  findings = [{
    stable_id = "b3:0123456789abcdef",
    semantic_id = "b3:0123456789abcdef",
    identity_status = "computed",
    lint = "fixture_lint",
    group = "tigerstyle::fixture",
    severity = "warning",
    file = "src/fixture.rs",
    line = 1,
    column = 1,
    message = "fixture finding",
    crate_name = "verified_logic",
  }],
} | Contracts.OctetBaseline
