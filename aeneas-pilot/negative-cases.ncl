# r[impl trellis.aeneas_cross_verifier.negative_evidence]
let Contracts = import "contracts.ncl" in
{
  schema_version = "trellis-aeneas-negative-catalog/v1",
  cases = [
    { id = "provider-profile-drift", expected_diagnostic = "provider identity" },
    { id = "extraction-command-drift", expected_diagnostic = "extraction command" },
    { id = "source-identity-drift", expected_diagnostic = "source identity" },
    { id = "shadow-source-path", expected_diagnostic = "exact source path" },
    { id = "errorful-llbc", expected_diagnostic = "LLBC has errors" },
    { id = "skip-borrowck", expected_diagnostic = "borrow checking" },
    { id = "target-drift", expected_diagnostic = "target metadata" },
    { id = "stale-translation", expected_diagnostic = "translation mapping" },
    { id = "cross-run-def-id", expected_diagnostic = "run-local declaration" },
    { id = "stale-generated-lean", expected_diagnostic = "generated Lean identity" },
    { id = "theorem-hash-drift", expected_diagnostic = "theorem source identity" },
    { id = "theorem-symbol-drift", expected_diagnostic = "theorem mapping" },
    { id = "trust-closure-drift", expected_diagnostic = "trust closure" },
    { id = "budget-identity-drift", expected_diagnostic = "resource policy identity" },
    { id = "tool-failure-classification", expected_diagnostic = "stage outcome" },
    { id = "receipt-swap", expected_diagnostic = "receipt source identity" },
    { id = "missing-artifact-verification", expected_diagnostic = "artifact verification" },
    { id = "overclaim-text", expected_diagnostic = "required non-claim" },
  ],
} | Contracts.NegativeCatalog
