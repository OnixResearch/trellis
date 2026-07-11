let contracts = import "../../contracts.ncl" in
let catalog = import "../../catalog.ncl" in
(catalog & { claim_scope | force = "upstream-proofs-certify-trellis-and-downstream-runtimes" }) | contracts.CorpusCatalog
