let contracts = import "../../contracts.ncl" in
let catalog = import "../../catalog.ncl" in
let first = std.array.first catalog.entries in
let invalid_first = first & { revision | force = "main" } in
let invalid_entries = [invalid_first] in
(catalog & { entries | force = invalid_entries }) | contracts.CorpusCatalog
