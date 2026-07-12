let contracts = import "../../upgrade-trial-contracts.ncl" in
let trial = import "../../upgrade-trial.ncl" in
let rows = std.array.map (fun row =>
  if row.id == "verified-memory-allocator" then
    row & {
      adaptation | force = row.adaptation & {
        patch_blake3 | force = "mutated-evidence",
      },
    }
  else
    row
) trial.rows in
(trial & { rows | force = rows }) | contracts.UpgradeTrial
