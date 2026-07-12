let contracts = import "../../upgrade-trial-contracts.ncl" in
let trial = import "../../upgrade-trial.ncl" in
let rows = std.array.map (fun row =>
  if row.id == "verified-ironkv" then
    row & { expected_outcome | force = "source-api-incompatible" }
  else
    row
) trial.rows in
(trial & { rows | force = rows }) | contracts.UpgradeTrial
