let contracts = import "../../upgrade-trial-contracts.ncl" in
let trial = import "../../upgrade-trial.ncl" in
(trial & { decision.expected | force = "promote" }) | contracts.UpgradeTrial
