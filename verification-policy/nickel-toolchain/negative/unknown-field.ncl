let allowed = ["proof_tool"] in
let value = { proof_tool = "verus", ambient = true } in
let unknown = std.record.fields value |> std.array.filter (fun field => !std.array.elem field allowed) in
if std.array.length unknown == 0 then value else error "unknown-field"
