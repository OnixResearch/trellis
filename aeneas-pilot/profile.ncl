# r[impl trellis.aeneas_cross_verifier.provider_consumption]
# r[impl trellis.aeneas_cross_verifier.exact_source]
# r[impl trellis.aeneas_cross_verifier.translation_manifest]
# r[impl trellis.aeneas_cross_verifier.theorem_map]
# r[impl trellis.aeneas_cross_verifier.theorem_adequacy]
let Contracts = import "contracts.ncl" in
let OctetRevision = "890f424cd7b1b2015f39c0eb5747f1bd1e576028" in
let ProviderIdentity = "b3:cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625" in
let CompatibilityIdentity = "b3:403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5" in
let ConsumerContractIdentity = "b3:63ea1e9e78916728a4a83719c29a6443a8952dbaadeeaae2a7420365afa7ff07" in
let SourceIdentity = "b3:3064658eb9d27a1c1c423abbad9e9c8878756594f07a1d01eeb6b3efef4fa8a4" in
let SymmetryIdentity = "b3:a67c10d4699ae7b0ddb5b35eb87552b893fcf038c93fd5c6b4c776d5b172e3fb" in
let SelfZeroIdentity = "b3:b3df314ff917bb6eb9e26f7a8d56b7c5bc8439da5e5c5076c47a4bee58c7ade4" in
let ResourceIdentity = "b3:4a2728758ddaf1f6214e8e9e6efe3ebb2c2c03cc235caef680010ffbc54d6023" in
let SourceBeginLine = 21 in
let SourceEndLine = 25 in
let ProviderTimeoutMilliseconds = 900000 in
let RetainedLogBytes = 1048576 in
let PointerWidthBytes = 8 in
let LlbcBytes = 76688 in
let TranslationBytes = 632 in
let GeneratedLeanBytes = 748 in
let SymmetryBytes = 495 in
let SelfZeroBytes = 390 in
let AxiomAuditBytes = 133 in
let ResourcePolicyBytes = 408 in
let NegativeCatalogBytes = 1837 in
let RequiredEnvironment = [
  "PATH",
  "CHARON_TOOLCHAIN_IS_IN_PATH",
  "LD_LIBRARY_PATH",
  "HOME",
  "MATHLIB_NO_CACHE_ON_UPDATE",
] in
let RequiredNonClaims = [
  "does not prove Aeneas or Charon translation soundness",
  "does not prove Lean or verifier soundness",
  "does not prove whole-program correctness",
  "does not prove semantic equivalence",
  "does not establish release eligibility",
  "does not prove rustc compiler correctness",
  "does not prove verifier independence",
  "does not merge Verus and Aeneas proof authority",
] in
let ExternalModels = [
  {
    input_id = "octet.aeneas-provider-profile",
    identity = ProviderIdentity,
  },
] in
let TrustInputs = [
  {
    input_id = "lean.propext",
    identity = "b3:f684452a5c056e2867a5935716df006d4bb1e99c38b5f753d168141ba879c98f",
  },
  {
    input_id = "lean.Classical.choice",
    identity = "b3:c4255580ac1986a44f37669982f36b10cdbb58dc2342e36edf468a569639a923",
  },
  {
    input_id = "lean.Quot.sound",
    identity = "b3:fab7533a97460e46812b181679ac641cc62ba94187588f865751fd7df34dae85",
  },
] in
let ConsumerTheorems = [
  {
    path = "aeneas-pilot/theorems/AbsDiffSymmetric.lean",
    theorem_id = "trellis.abs-diff.symmetry",
    declaration_name = "trellis_abs_diff_symmetric",
    requirement_id = "arithmetic.abs-diff",
    claim_class = "named-theorem-over-generated-model",
    approved_statement_blake3 = SymmetryIdentity,
    source_disposition = "maintained",
  },
  {
    path = "aeneas-pilot/theorems/AbsDiffSelfZero.lean",
    theorem_id = "trellis.abs-diff.self-zero",
    declaration_name = "trellis_abs_diff_self_is_zero",
    requirement_id = "arithmetic.abs-diff",
    claim_class = "named-theorem-over-generated-model",
    approved_statement_blake3 = SelfZeroIdentity,
    source_disposition = "maintained",
  },
] in
{
  schema_version = "trellis-aeneas-cross-verifier-profile/v1",
  pilot_id = "trellis.abs-diff.aeneas-cross-verifier-v1",
  outcome = "supported",
  octet_source = {
    repository = "https://github.com/OnixResearch/octet",
    revision = OctetRevision,
    consumer_contract_path = "standards/proof-rails/aeneas-lean/consumer-contracts.ncl",
    consumer_contract_identity = ConsumerContractIdentity,
    policy_artifact_identity = "b3:cc07811243f39a70c08d80b2d145fbc97596644d56b74284d5f870d93826f605",
    provider_profile_identity = ProviderIdentity,
    compatibility_matrix_identity = CompatibilityIdentity,
  },
  consumer = {
    schema_version = "octet-aeneas-consumer-profile/v1",
    profile_id = "trellis-abs-diff-aeneas-consumer-v1",
    consumer_id = "trellis.abs-diff",
    provider = {
      backend = "aeneas-lean",
      profile_id = "octet-aeneas-lean-0.1.0-bc2eb6f",
      profile_identity = ProviderIdentity,
      compatibility_matrix_id = "octet-aeneas-lean-compatibility-0.1.0-bc2eb6f-cb50ff16",
      compatibility_matrix_identity = CompatibilityIdentity,
      network_allowed = false,
    },
    source = {
      root = ".",
      crate_name = "verified_logic",
      paths = [
        { path = "src/arithmetic.rs", identity = SourceIdentity },
      ],
      selected_items = [
        {
          qualified_name = "verified_logic::arithmetic::abs_diff",
          kind = "function",
          source_file = "src/arithmetic.rs",
          source_identity = SourceIdentity,
          begin_line = SourceBeginLine,
          end_line = SourceEndLine,
          generated_name = "verified_logic.arithmetic.abs_diff",
        },
      ],
      broad_selection_allowed = false,
    },
    theorems = ConsumerTheorems,
    external_models = ExternalModels,
    trust_inputs = TrustInputs,
    resource_policy = {
      timeout_ms = ProviderTimeoutMilliseconds,
      stdout_max_bytes = RetainedLogBytes,
      stderr_max_bytes = RetainedLogBytes,
      environment_allowlist = RequiredEnvironment,
    },
    outcomes = ["supported", "blocked", "failed"],
    trust_closure_complete = true,
    non_claims = RequiredNonClaims,
  },
  provider_tools = {
    package = "octet-aeneas-lean-provider-closure",
    identity_schema = "octet-aeneas-lean-provider-closure/v1",
    aeneas_revision = "bc2eb6f941369fa3c695e30598b5758b11e4f0de",
    charon_revision = "cb50ff16b9f1066b8a97dc06da704de2da2fa41c",
    lean_toolchain = "leanprover/lean4:v4.31.0",
    aeneas_binary_blake3 = "428ce05d0f32c3bc3eceb24c1ead80b0dc742bfda22cba9c4b91b6a7f40a3819",
    charon_binary_blake3 = "64ee129c4553574e1686d87560af0699dba9010c998dd68fe4a6b25f6b2e64b5",
    lean_binary_blake3 = "e9e24ff8e9efd3e7f7a692d2411e0bb39316b626aeda1c3f1bce21bf21daea5d",
    lake_binary_blake3 = "9c0b74bac6d9436481da3be2f158f9abfb76b940a18aa723d2c493b29ac9f2b5",
    leantar_binary_blake3 = "b9f350bec0f839bb2e6157fdac2c8cc877f26a961ce81b16e5bcc97957d257fe",
    network_sandbox_blake3 = "c87d8bcc11f0c5d61e2598dbc08b801d7e1220753ba47b543525835bc1288b6b",
  },
  extraction = {
    charon_argv = [
      "charon",
      "cargo",
      "--preset",
      "aeneas",
      "--dest-file",
      "llbc.llbc",
      "--start-from",
      "crate::arithmetic::abs_diff",
    ],
    cargo_argv = ["--lib"],
    aeneas_argv = ["aeneas", "-backend", "lean", "-emit-json", "-dest", ".", "-no-progress-bar", "llbc.llbc"],
    lean_argv = ["lake", "build"],
    expected_has_errors = false,
    expected_skip_borrowck = false,
    expected_target = "x86_64-unknown-linux-gnu",
    expected_pointer_width_bytes = PointerWidthBytes,
    expected_little_endian = true,
  },
  resource_policy = {
    path = "aeneas-pilot/generated/resource-policy.json",
    identity = ResourceIdentity,
  },
  artifacts = [
    { role = "llbc", path = "aeneas-pilot/generated/llbc.llbc", bytes = LlbcBytes, blake3 = "b3:85ad114c09426020bd1c0ba56bc5e71ba301b7552a5f6b0b9219357b671cb872" },
    { role = "translation-manifest", path = "aeneas-pilot/generated/translation.json", bytes = TranslationBytes, blake3 = "b3:c7df4ccc0437a026d9dff206324ff5d42f3cf1c4e8663022cc4cba34ddc536a4" },
    { role = "generated-lean", path = "aeneas-pilot/generated/Llbc.lean", bytes = GeneratedLeanBytes, blake3 = "b3:4fe6569363d2ffef0075c278a0a49b91ccfb83a9387826927df658a7e5e6644b" },
    { role = "symmetry-theorem", path = "aeneas-pilot/theorems/AbsDiffSymmetric.lean", bytes = SymmetryBytes, blake3 = SymmetryIdentity },
    { role = "self-zero-theorem", path = "aeneas-pilot/theorems/AbsDiffSelfZero.lean", bytes = SelfZeroBytes, blake3 = SelfZeroIdentity },
    { role = "axiom-audit", path = "aeneas-pilot/theorems/AxiomAudit.lean", bytes = AxiomAuditBytes, blake3 = "b3:cb3d3675beba0ac1595c4747e97b73380083e15bc81a7e50eafb200c0c35b197" },
    { role = "resource-policy", path = "aeneas-pilot/generated/resource-policy.json", bytes = ResourcePolicyBytes, blake3 = ResourceIdentity },
    { role = "negative-catalog", path = "aeneas-pilot/generated/negative-cases.json", bytes = NegativeCatalogBytes, blake3 = "b3:a024bc771e73f8abd7badcf49f235ae8e67e202c7ae06975c4822521d6f94a89" },
  ],
  theorem_mappings = [
    {
      theorem_id = "trellis.abs-diff.symmetry",
      path = "aeneas-pilot/theorems/AbsDiffSymmetric.lean",
      declaration_name = "trellis_abs_diff_symmetric",
      requirement_id = "arithmetic.abs-diff",
      claim_class = "named-theorem-over-generated-model",
      approved_statement_blake3 = SymmetryIdentity,
      verus_proof_symbol = "arithmetic::abs_diff_symmetric",
      lean_theorem_symbol = "trellis_abs_diff_symmetric",
      integer_model = "Aeneas.Std.U64 with a 64-bit little-endian target",
      overflow_model = "branch-selected subtraction proves the selected subtraction is in range",
      axioms = ["propext"],
    },
    {
      theorem_id = "trellis.abs-diff.self-zero",
      path = "aeneas-pilot/theorems/AbsDiffSelfZero.lean",
      declaration_name = "trellis_abs_diff_self_is_zero",
      requirement_id = "arithmetic.abs-diff",
      claim_class = "named-theorem-over-generated-model",
      approved_statement_blake3 = SelfZeroIdentity,
      verus_proof_symbol = "arithmetic::abs_diff_self_is_zero",
      lean_theorem_symbol = "trellis_abs_diff_self_is_zero",
      integer_model = "Aeneas.Std.U64 with a 64-bit little-endian target",
      overflow_model = "self subtraction is in range and returns zero",
      axioms = ["propext", "Classical.choice", "Quot.sound"],
    },
  ],
  trust_closure = {
    complete = true,
    selected_axioms = ["propext", "Classical.choice", "Quot.sound"],
    selected_opaque_definitions = [],
  },
  verus = {
    receipt_path = "docs/proof-evidence-bundle.json",
    requirement_id = "arithmetic.abs-diff",
    proof_symbols = ["arithmetic::abs_diff_symmetric", "arithmetic::abs_diff_self_is_zero"],
  },
  negative_fixture_catalog = "aeneas-pilot/generated/negative-cases.json",
  required_non_claims = RequiredNonClaims,
} | Contracts.Profile
